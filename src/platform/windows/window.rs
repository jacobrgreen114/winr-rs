// Copyright (c) 2026 Jacob Green
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The only native window-procedure boundary. It never invokes application code.
use super::win32::*;
use crate::application::Event;
use crate::input::{self, ButtonState, KeyState, MouseButton, Utf16Decoder};
use crate::window::{WindowBuilder, WindowEvent, WindowId};
use crate::{Extent, Point, WinrError, WinrResult};
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::rc::Rc;
use std::sync::OnceLock;

pub(crate) struct Shared {
    pub events: RefCell<VecDeque<Event>>,
    pub panicked: Cell<bool>,
}
impl Default for Shared {
    fn default() -> Self {
        Self {
            events: RefCell::new(VecDeque::with_capacity(64)),
            panicked: Cell::new(false),
        }
    }
}

pub(crate) struct NativeWindow {
    pub id: WindowId,
    hwnd: Cell<HWND>,
    shared: Rc<Shared>,
    ready: Cell<bool>,
    destroying: Cell<bool>,
    min: Option<(i32, i32)>,
    max: Option<(i32, i32)>,
    text: RefCell<Utf16Decoder>,
}
impl NativeWindow {
    pub fn create(
        id: WindowId,
        shared: Rc<Shared>,
        builder: WindowBuilder,
    ) -> WinrResult<Rc<Self>> {
        let title = title(&builder.title)?;
        let min = builder.extent_min.map(Extent::checked_i32).transpose()?;
        let max = builder.extent_max.map(Extent::checked_i32).transpose()?;
        if let (Some(min), Some(max)) = (min, max)
            && (min.0 > max.0 || min.1 > max.1)
        {
            return Err(WinrError::InvalidWindowOptions);
        }
        let (x, y) = builder
            .origin
            .map(position)
            .transpose()?
            .unwrap_or((CW_USEDEFAULT, CW_USEDEFAULT));
        let (width, height) = builder
            .extent
            .map(Extent::checked_i32)
            .transpose()?
            .unwrap_or((CW_USEDEFAULT, CW_USEDEFAULT));
        if builder.extent.is_some()
            && (min.is_some_and(|m| width < m.0 || height < m.1)
                || max.is_some_and(|m| width > m.0 || height > m.1))
        {
            return Err(WinrError::InvalidWindowOptions);
        }
        let mut style = WS_SYSMENU | WS_MINIMIZEBOX | WS_CLIPCHILDREN | WS_CLIPSIBLINGS;
        style |= if builder.decorated {
            WS_CAPTION
        } else {
            WS_POPUP
        };
        if builder.resizable {
            style |= WS_THICKFRAME | WS_MAXIMIZEBOX;
        }
        let instance = instance()?;
        register_class(instance)?;
        let state = Rc::new(Self {
            id,
            hwnd: Cell::new(HWND::default()),
            shared,
            ready: Cell::new(false),
            destroying: Cell::new(false),
            min,
            max,
            text: RefCell::new(Utf16Decoder::default()),
        });
        // SAFETY: state remains alive throughout creation. WM_NCCREATE establishes
        // an additional Rc owner for userdata, released only at WM_NCDESTROY.
        let result = unsafe {
            CreateWindowExW(
                WS_EX_APPWINDOW,
                w!("winr.event-window.v1"),
                PCWSTR(title.as_ptr()),
                style,
                x,
                y,
                width,
                height,
                None,
                None,
                Some(instance),
                Some(Rc::as_ptr(&state).cast()),
            )
        };
        if state.shared.panicked.get() {
            let _ = state.close();
            return Err(WinrError::NativeProcedurePanicked);
        }
        result?;
        state.ready.set(true);
        state.emit(WindowEvent::Created);
        Ok(state)
    }
    pub fn handle(&self) -> WinrResult<HWND> {
        let hwnd = self.hwnd.get();
        if hwnd.is_invalid() || self.destroying.get() {
            Err(WinrError::WindowClosed)
        } else {
            Ok(hwnd)
        }
    }
    pub fn is_closed(&self) -> bool {
        self.hwnd.get().is_invalid()
    }
    pub fn close(&self) -> WinrResult<()> {
        if self.is_closed() || self.destroying.replace(true) {
            return Ok(());
        }
        // SAFETY: HWND belongs to this thread. Only Cell accesses span this call;
        // no exclusive reference or RefCell borrow survives synchronous messages.
        let result = unsafe { DestroyWindow(self.hwnd.get()) };
        if result.is_err() {
            self.destroying.set(false);
        }
        result.map_err(Into::into)
    }
    pub fn show(&self) -> WinrResult<()> {
        unsafe {
            let _ = ShowWindow(self.handle()?, SW_SHOW);
        }
        Ok(())
    }
    pub fn set_title(&self, value: &str) -> WinrResult<()> {
        let title = title(value)?;
        unsafe { SetWindowTextW(self.handle()?, PCWSTR(title.as_ptr())) }.map_err(Into::into)
    }
    pub fn set_pos(&self, value: Point) -> WinrResult<()> {
        let (x, y) = position(value)?;
        unsafe {
            SetWindowPos(
                self.handle()?,
                None,
                x,
                y,
                0,
                0,
                SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE,
            )
        }
        .map_err(Into::into)
    }
    pub fn set_size(&self, value: Extent) -> WinrResult<()> {
        let (w, h) = value.checked_i32()?;
        if self.min.is_some_and(|m| w < m.0 || h < m.1)
            || self.max.is_some_and(|m| w > m.0 || h > m.1)
        {
            return Err(WinrError::InvalidWindowOptions);
        }
        unsafe {
            SetWindowPos(
                self.handle()?,
                None,
                0,
                0,
                w,
                h,
                SWP_NOMOVE | SWP_NOZORDER | SWP_NOACTIVATE,
            )
        }
        .map_err(Into::into)
    }
    pub fn request_redraw(&self) -> WinrResult<()> {
        unsafe { InvalidateRect(Some(self.handle()?), None, false) }
            .ok()
            .map_err(Into::into)
    }
    fn emit(&self, event: WindowEvent) {
        if self.ready.get() {
            self.shared
                .events
                .borrow_mut()
                .push_back(Event::Window { id: self.id, event });
        }
    }
}
fn title(value: &str) -> WinrResult<Vec<u16>> {
    if value.contains('\0') {
        return Err(WinrError::InvalidWindowOptions);
    }
    Ok(value.encode_utf16().chain(Some(0)).collect())
}
fn position(value: Point) -> WinrResult<(i32, i32)> {
    Ok((
        value
            .x
            .try_into()
            .map_err(|_| WinrError::InvalidWindowOptions)?,
        value
            .y
            .try_into()
            .map_err(|_| WinrError::InvalidWindowOptions)?,
    ))
}
fn instance() -> windows::core::Result<HINSTANCE> {
    // SAFETY: None requests the current process module, which remains loaded.
    unsafe { windows::Win32::System::LibraryLoader::GetModuleHandleW(None) }
        .map(|module| HINSTANCE(module.0))
}
fn register_class(instance: HINSTANCE) -> windows::core::Result<()> {
    static CLASS: OnceLock<windows::core::Result<()>> = OnceLock::new();
    CLASS
        .get_or_init(|| {
            let class = WNDCLASSEXW {
                cbSize: size_of::<WNDCLASSEXW>() as u32,
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(window_proc),
                hInstance: instance,
                hCursor: unsafe { LoadCursorW(None, IDC_ARROW)? },
                lpszClassName: w!("winr.event-window.v1"),
                ..Default::default()
            };
            if unsafe { RegisterClassExW(&class) } == 0 {
                Err(Error::from_thread())
            } else {
                Ok(())
            }
        })
        .clone()
}

/// SAFETY: called by Windows on the creating thread with message-specific pointers.
/// userdata holds an Rc strong reference from NCCREATE until NCDESTROY. Each invocation
/// takes its own Rc so nested destruction cannot invalidate the current procedure.
unsafe extern "system" fn window_proc(hwnd: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    if msg == WM_NCCREATE {
        // SAFETY: WM_NCCREATE supplies a valid CREATESTRUCTW for this call.
        let create = unsafe { &*(lp.0 as *const CREATESTRUCTW) };
        let ptr = create.lpCreateParams.cast::<NativeWindow>();
        if ptr.is_null() {
            return LRESULT(0);
        }
        // SAFETY: create() owns an Rc for this pointer throughout CreateWindowExW.
        unsafe {
            Rc::increment_strong_count(ptr);
            SetLastError(ERROR_SUCCESS);
        }
        unsafe {
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, ptr as isize);
        }
        if unsafe { GetLastError() } != ERROR_SUCCESS {
            unsafe {
                drop(Rc::from_raw(ptr));
            }
            return LRESULT(0);
        }
        unsafe { &*ptr }.hwnd.set(hwnd);
        return LRESULT(1);
    }
    let ptr = unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) } as *const NativeWindow;
    if ptr.is_null() {
        return unsafe { DefWindowProcW(hwnd, msg, wp, lp) };
    }
    unsafe {
        Rc::increment_strong_count(ptr);
    }
    let state = unsafe { Rc::from_raw(ptr) };
    if msg == WM_NCDESTROY {
        // Detach before default processing; nested calls now take the default path.
        unsafe {
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
        }
        state.hwnd.set(HWND::default());
        unsafe {
            drop(Rc::from_raw(ptr));
        } // release userdata's owner exactly once
    }
    // No application code runs here. Contain internal unwinding failures and surface
    // them from the event loop after native dispatch (panic=abort cannot be caught).
    match catch_unwind(AssertUnwindSafe(|| unsafe {
        dispatch(&state, hwnd, msg, wp, lp)
    })) {
        Ok(result) => result,
        Err(_) => {
            state.shared.panicked.set(true);
            LRESULT(0)
        }
    }
}

unsafe fn dispatch(state: &NativeWindow, hwnd: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    match msg {
        WM_NCDESTROY => {
            state.emit(WindowEvent::Closed);
            return unsafe { DefWindowProcW(hwnd, msg, wp, lp) };
        }
        WM_CLOSE => state.emit(WindowEvent::CloseRequested),
        WM_PAINT => {
            // Validate before returning; rendering is deferred until application dispatch.
            let mut paint = PAINTSTRUCT::default();
            unsafe {
                let _ = BeginPaint(hwnd, &mut paint);
                let _ = EndPaint(hwnd, &paint);
            }
            state.emit(WindowEvent::RedrawRequested);
        }
        WM_GETMINMAXINFO => {
            // Let Windows populate all defaults before applying cached outer constraints.
            unsafe {
                DefWindowProcW(hwnd, msg, wp, lp);
            }
            let info = unsafe { &mut *(lp.0 as *mut MINMAXINFO) };
            if let Some((x, y)) = state.min {
                info.ptMinTrackSize = POINT { x, y };
            }
            if let Some((x, y)) = state.max {
                info.ptMaxTrackSize = POINT { x, y };
            }
        }
        WM_CHAR => {
            let chars = state.text.borrow_mut().push(wp.0 as u16);
            for ch in chars.into_iter().flatten() {
                state.emit(WindowEvent::Character(ch));
            }
        }
        WM_UNICHAR => {
            if wp.0 as u32 == UNICODE_NOCHAR {
                return LRESULT(1);
            }
            state.emit(WindowEvent::Character(
                char::from_u32(wp.0 as u32).unwrap_or(char::REPLACEMENT_CHARACTER),
            ));
        }
        WM_KEYDOWN | WM_KEYUP | WM_SYSKEYDOWN | WM_SYSKEYUP => {
            let key = input::virtual_key(
                wp.0 as u32,
                ((lp.0 >> 16) & 0xff) as u8,
                lp.0 & (1 << 24) != 0,
            );
            let key_state = if msg == WM_KEYUP || msg == WM_SYSKEYUP {
                KeyState::Release
            } else if lp.0 & (1 << 30) != 0 {
                KeyState::Repeat
            } else {
                KeyState::Press
            };
            state.emit(WindowEvent::Key {
                key,
                state: key_state,
            });
            // Preserve system behavior such as Alt+F4 and the system menu.
            if msg == WM_SYSKEYDOWN || msg == WM_SYSKEYUP {
                return unsafe { DefWindowProcW(hwnd, msg, wp, lp) };
            }
        }
        WM_MOUSEMOVE => state.emit(WindowEvent::MouseMoved(input::signed_point(lp.0))),
        WM_LBUTTONDOWN | WM_LBUTTONUP | WM_RBUTTONDOWN | WM_RBUTTONUP | WM_MBUTTONDOWN
        | WM_MBUTTONUP | WM_XBUTTONDOWN | WM_XBUTTONUP => {
            let button = match msg {
                WM_LBUTTONDOWN | WM_LBUTTONUP => MouseButton::Left,
                WM_RBUTTONDOWN | WM_RBUTTONUP => MouseButton::Right,
                WM_MBUTTONDOWN | WM_MBUTTONUP => MouseButton::Middle,
                _ => {
                    if ((wp.0 >> 16) as u16) == XBUTTON1 {
                        MouseButton::Button4
                    } else {
                        MouseButton::Button5
                    }
                }
            };
            let pressed = matches!(
                msg,
                WM_LBUTTONDOWN | WM_RBUTTONDOWN | WM_MBUTTONDOWN | WM_XBUTTONDOWN
            );
            state.emit(WindowEvent::MouseButton {
                button,
                state: if pressed {
                    ButtonState::Press
                } else {
                    ButtonState::Release
                },
                position: input::signed_point(lp.0),
            });
            if msg == WM_XBUTTONDOWN || msg == WM_XBUTTONUP {
                return LRESULT(1);
            }
        }
        WM_MOUSEWHEEL | WM_MOUSEHWHEEL => {
            let p = input::signed_point(lp.0);
            let mut point = POINT {
                x: p.x as i32,
                y: p.y as i32,
            };
            if unsafe { ScreenToClient(hwnd, &mut point) }.as_bool() {
                let delta = (wp.0 >> 16) as u16 as i16 as i32;
                state.emit(WindowEvent::MouseWheel {
                    delta_x: if msg == WM_MOUSEHWHEEL { delta } else { 0 },
                    delta_y: if msg == WM_MOUSEWHEEL { delta } else { 0 },
                    position: Point::new(point.x as isize, point.y as isize),
                });
            }
        }
        WM_MOVE => state.emit(WindowEvent::Moved(input::signed_point(lp.0))),
        WM_SIZE => state.emit(WindowEvent::Resized(Extent::new(
            lp.0 as u16 as usize,
            (lp.0 >> 16) as u16 as usize,
        ))),
        WM_SETFOCUS | WM_KILLFOCUS => state.emit(WindowEvent::Focused(msg == WM_SETFOCUS)),
        WM_SHOWWINDOW => {
            state.emit(WindowEvent::Shown(wp.0 != 0));
            return unsafe { DefWindowProcW(hwnd, msg, wp, lp) };
        }
        _ => return unsafe { DefWindowProcW(hwnd, msg, wp, lp) },
    }
    LRESULT(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn userdata_owns_state_until_ncdestroy_and_releases_it() -> WinrResult<()> {
        let state = NativeWindow::create(
            WindowId(1),
            Rc::new(Shared::default()),
            WindowBuilder::new(),
        )?;
        assert_eq!(Rc::strong_count(&state), 2, "local + native userdata owner");
        let weak = Rc::downgrade(&state);
        state.close()?;
        assert_eq!(Rc::strong_count(&state), 1, "native owner must be released");
        drop(state);
        assert!(weak.upgrade().is_none());
        Ok(())
    }
}
