// Copyright (c) 2026 Jacob Green
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::input::*;
use crate::platform::windows::*;
use crate::platform::*;
use crate::window::*;
use crate::*;

use std::mem::transmute;
use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::LazyLock;

pub type Win32WindowHandle = win32::HWND;

pub struct Win32WindowBuilder<C: WindowCallbacks> {
    callbacks: C,
    title: String,
    style: WindowStyle,
}

fn string_to_title(title: &str) -> Vec<u16> {
    let mut s: Vec<_> = title.encode_utf16().collect();
    match s.last().copied() {
        Some(0) => {}
        _ => s.push(0),
    }
    s
}

impl<C: WindowCallbacks> PlatformWindowBuilder<C> for Win32WindowBuilder<C> {
    type Window = Win32Window<C>;

    fn new(callbacks: C) -> Self {
        Win32WindowBuilder {
            callbacks,
            title: Default::default(),
            style: Default::default(),
        }
    }

    fn with_callbacks<D: WindowCallbacks>(self, callbacks: D) -> Win32WindowBuilder<D> {
        Win32WindowBuilder {
            callbacks,
            title: self.title,
            style: self.style,
        }
    }

    fn with_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    fn with_style(mut self, style: WindowStyle) -> Self {
        self.style = style;
        self
    }

    fn build(self) -> WinrResult<Ptr<Self::Window>> {
        let title = string_to_title(&self.title);

        let window = match &self.style {
            WindowStyle::Windowed(windowed) => {
                let (style, style_ex) = (windowed.into(), windowed.into());

                let (x, y, w, h) = {
                    let (x, y) = windowed
                        .origin
                        .map(|p| {
                            let x = i32::try_from(p.x).unwrap();
                            let y = i32::try_from(p.y).unwrap();
                            (x, y)
                        })
                        .unwrap_or((win32::CW_USEDEFAULT, win32::CW_USEDEFAULT));

                    let (w, h) = windowed
                        .extent
                        .map(|s| {
                            let x = i32::try_from(s.width).unwrap();
                            let y = i32::try_from(s.height).unwrap();
                            (x, y)
                        })
                        .unwrap_or((win32::CW_USEDEFAULT, win32::CW_USEDEFAULT));

                    (x, y, w, h)
                };

                let mut window = Ptr::new(Win32Window {
                    handle: Default::default(),
                    size_min: windowed.extent_min,
                    size_max: windowed.extent_max,
                    callbacks: self.callbacks,
                });

                let create_info = WindowCreateInfo {
                    window: NonNull::<Win32Window<C>>::from_mut(window.as_mut()).cast(),
                    proc: win_proc::<C>,
                };

                let _hwnd = unsafe {
                    win32::CreateWindowExW(
                        style_ex,
                        win32::PCWSTR(get_window_class().as_ptr()),
                        win32::PCWSTR(title.as_ptr()),
                        style,
                        x,
                        y,
                        w,
                        h,
                        None,
                        None,
                        None,
                        Some(core::ptr::from_ref::<WindowCreateInfo>(&create_info).cast()),
                    )
                }?;

                window
            }
        };

        debug_assert_ne!(window.handle, Default::default());

        Ok(window)
    }
}

pub struct Win32Window<C: WindowCallbacks> {
    handle: win32::HWND,
    size_min: Option<Extent>,
    size_max: Option<Extent>,
    callbacks: C,
}

impl<C: WindowCallbacks> Win32Window<C> {
    fn destroy(&mut self) -> WinrResult<()> {
        unsafe { win32::DestroyWindow(self.handle) }?;
        // Handle nullified in win_proc
        Ok(())
    }
}

impl<C: WindowCallbacks> Drop for Win32Window<C> {
    fn drop(&mut self) {
        if self.handle != Default::default() {
            let window = unsafe { transmute(&self) };
            C::on_closing(window, &EventClosing {});
            self.destroy().unwrap();
        }
    }
}

impl<C: WindowCallbacks> PlatformWindow<C> for Win32Window<C> {
    fn handle(&self) -> platform::WindowHandle {
        self.handle
    }

    fn is_closed(&self) -> bool {
        self.handle == Default::default()
    }

    fn show(&mut self) -> WinrResult<()> {
        let _ = unsafe { win32::ShowWindow(self.handle, win32::SW_SHOW) };
        Ok(())
    }

    fn set_title(&mut self, title: String) -> WinrResult<()> {
        let title = string_to_title(&title);
        unsafe { win32::SetWindowTextW(self.handle, win32::PCWSTR(title.as_ptr())) }
            .map_err(Into::into)
    }

    fn set_pos(&mut self, pos: Point) -> WinrResult<()> {
        unsafe {
            win32::SetWindowPos(
                self.handle,
                None,
                pos.x.try_into().unwrap(),
                pos.y.try_into().unwrap(),
                0,
                0,
                win32::SWP_NOSIZE | win32::SWP_NOZORDER,
            )
        }
        .map_err(Into::into)
    }

    fn set_size(&mut self, size: Extent) -> WinrResult<()> {
        unsafe {
            win32::SetWindowPos(
                self.handle,
                None,
                0,
                0,
                size.width.try_into().unwrap(),
                size.height.try_into().unwrap(),
                win32::SWP_NOMOVE | win32::SWP_NOZORDER,
            )
        }
        .map_err(Into::into)
    }

    fn callbacks(&self) -> &C {
        &self.callbacks
    }

    fn callbacks_mut(&mut self) -> &mut C {
        &mut self.callbacks
    }
}

//
//  Window Class
//

fn get_window_class() -> &'static [u16] {
    static WINDOW_CLASS: LazyLock<(u16, &'static [u16])> = LazyLock::new(|| unsafe {
        static CLASS_NAME: &[u16] = &['w' as u16, 'i' as u16, 'n' as u16, 'r' as u16, 0];
        let class = win32::WNDCLASSEXW {
            cbSize: size_of::<win32::WNDCLASSEXW>() as u32,
            style: win32::CS_HREDRAW | win32::CS_VREDRAW,
            lpfnWndProc: Some(win_proc_initial),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: win32::HINSTANCE::default(),
            hIcon: win32::HICON::default(),
            hCursor: win32::LoadCursorW(None, win32::IDC_ARROW).unwrap(),
            hbrBackground: win32::HBRUSH::default(),
            lpszMenuName: win32::PCWSTR::null(),
            lpszClassName: win32::PCWSTR(CLASS_NAME.as_ptr()),
            hIconSm: win32::HICON::default(),
        };
        (win32::RegisterClassExW(&class), CLASS_NAME)
    });

    WINDOW_CLASS.1
}

//
//  Win32 Window Procedure
//

macro_rules! loword {
    ($value:expr) => {
        (($value) as u16)
    };
}

macro_rules! hiword {
    ($value:expr) => {
        (($value >> 16) as u16)
    };
}

struct WindowCreateInfo {
    window: NonNull<core::ffi::c_void>,
    proc: unsafe extern "system" fn(
        param0: win32::HWND,
        param1: u32,
        param2: win32::WPARAM,
        param3: win32::LPARAM,
    ) -> win32::LRESULT,
}

// Initial 'Any' win_proc before switching to the generic specialized version
pub unsafe extern "system" fn win_proc_initial(
    hwnd: win32::HWND,
    msg: u32,
    wparam: win32::WPARAM,
    lparam: win32::LPARAM,
) -> win32::LRESULT {
    unsafe {
        match msg {
            win32::WM_GETMINMAXINFO => win32::DefWindowProcW(hwnd, msg, wparam, lparam),
            win32::WM_NCCREATE => {
                let create = (lparam.0 as *const win32::CREATESTRUCTW).as_ref().unwrap();
                let create_info = create
                    .lpCreateParams
                    .cast::<WindowCreateInfo>()
                    .as_ref()
                    .unwrap();
                win32::SetWindowLongPtrW(
                    hwnd,
                    win32::GWLP_USERDATA,
                    create_info.window.as_ptr() as isize,
                );
                win32::SetWindowLongPtrW(hwnd, win32::GWLP_WNDPROC, create_info.proc as isize);

                win32::DefWindowProcW(hwnd, msg, wparam, lparam)
            }
            _ => panic!("Unexpected window message!"),
        }
    }
}

unsafe fn lparam_to_point(lparam: win32::LPARAM) -> Point {
    Point {
        x: loword!(lparam.0).try_into().unwrap(),
        y: hiword!(lparam.0).try_into().unwrap(),
    }
}

unsafe fn lparam_to_size(lparam: win32::LPARAM) -> Extent {
    Extent {
        width: loword!(lparam.0).try_into().unwrap(),
        height: hiword!(lparam.0).try_into().unwrap(),
    }
}

unsafe fn wparam_to_xbutton(wparam: win32::WPARAM) -> MouseButton {
    match hiword!(wparam.0) {
        win32::XBUTTON1 => MouseButton::Button4,
        win32::XBUTTON2 => MouseButton::Button5,
        _ => unreachable!(),
    }
}

// fn wparam_to_mods(wparam: win32::WPARAM) -> KeyModifiers {
//     let mut mods = KeyModifiers::default();
//     if (wparam.0 | win32::MK_SHIFT.0 as usize) != 0 {
//         mods |= KeyModifiers::SHIFT
//     }
//     if (wparam.0 | win32::MK_CONTROL.0 as usize) != 0 {
//         mods |= KeyModifiers::CONTROL
//     }
//     mods
// }

fn win32_key_event<C: WindowCallbacks>(
    window: &mut crate::window::Window<C>,
    wparam: win32::WPARAM,
    lparam: win32::LPARAM,
    state: KeyState,
) -> win32::LRESULT {
    let extended = (lparam.0 >> 24) & 1 != 0;
    VirtualKey::try_from((wparam, extended))
        .ok()
        .inspect(|key| {
            // let scan_code = hiword!(lparam.0) & 0xff;
            let event = EventKey { key: *key, state };
            C::on_key(window, &event);
        });
    win32::LRESULT::default()
}

fn win32_mouse_button_event<C: WindowCallbacks>(
    window: &mut crate::window::Window<C>,
    wparam: win32::WPARAM,
    lparam: win32::LPARAM,
    button: MouseButton,
    state: ButtonState,
) -> win32::LRESULT {
    let event = EventMouseButton {
        button,
        state,
        position: unsafe { lparam_to_point(lparam) },
    };
    C::on_mouse_button(window, &event);
    win32::LRESULT::default()
}

pub unsafe extern "system" fn win_proc<C: WindowCallbacks>(
    hwnd: win32::HWND,
    msg: u32,
    wparam: win32::WPARAM,
    lparam: win32::LPARAM,
) -> win32::LRESULT {
    let window = unsafe { get_window::<C>(hwnd) }.unwrap();

    match msg {
        // Drawing
        win32::WM_PAINT => {
            // Validate whole client area to stop the system from sending WM_PAINT messages
            C::redraw_requested(window, &EventRedraw {});

            // todo - make window implementation mark region as clean

            win32::ValidateRect(Some(hwnd), None).expect("Failed to mark window as clean");

            win32::LRESULT::default()
        }

        // Character Input
        win32::WM_CHAR => {
            let mut ch = char::from_u32(wparam.0 as u32).unwrap();
            match ch {
                '\r' => ch = '\n',
                _ => {}
            }

            let event = EventChar { ch };

            C::on_char(window, &event).unwrap();

            win32::LRESULT::default()
        }

        win32::WM_UNICHAR => {
            let ch = wparam.0 as u32;
            if ch == win32::UNICODE_NOCHAR {
                win32::LRESULT(1)
            } else {
                let ch = char::from_u32(ch).unwrap();
                let event = EventChar { ch };

                C::on_char(window, &event);

                win32::LRESULT::default()
            }
        }

        // Key Pressed
        win32::WM_KEYDOWN | win32::WM_SYSKEYDOWN => {
            let prev_state = ((lparam.0 >> 30) & 1) != 0;
            let state = if !prev_state {
                KeyState::Press
            } else {
                KeyState::Repeat
            };
            win32_key_event::<C>(window, wparam, lparam, state)
        }

        // Key Released
        win32::WM_KEYUP | win32::WM_SYSKEYUP => {
            win32_key_event::<C>(window, wparam, lparam, KeyState::Release)
        }

        // Mouse Button
        win32::WM_LBUTTONDOWN => win32_mouse_button_event::<C>(
            window,
            wparam,
            lparam,
            MouseButton::Left,
            ButtonState::Press,
        ),

        win32::WM_LBUTTONUP => win32_mouse_button_event::<C>(
            window,
            wparam,
            lparam,
            MouseButton::Left,
            ButtonState::Release,
        ),

        win32::WM_RBUTTONDOWN => win32_mouse_button_event::<C>(
            window,
            wparam,
            lparam,
            MouseButton::Right,
            ButtonState::Press,
        ),

        win32::WM_RBUTTONUP => win32_mouse_button_event::<C>(
            window,
            wparam,
            lparam,
            MouseButton::Right,
            ButtonState::Release,
        ),

        win32::WM_MBUTTONDOWN => win32_mouse_button_event::<C>(
            window,
            wparam,
            lparam,
            MouseButton::Middle,
            ButtonState::Press,
        ),

        win32::WM_MBUTTONUP => win32_mouse_button_event::<C>(
            window,
            wparam,
            lparam,
            MouseButton::Middle,
            ButtonState::Release,
        ),

        win32::WM_XBUTTONDOWN => win32_mouse_button_event::<C>(
            window,
            wparam,
            lparam,
            unsafe { wparam_to_xbutton(wparam) },
            ButtonState::Press,
        ),

        win32::WM_XBUTTONUP => win32_mouse_button_event::<C>(
            window,
            wparam,
            lparam,
            unsafe { wparam_to_xbutton(wparam) },
            ButtonState::Release,
        ),

        // Mouse Move
        win32::WM_MOUSEMOVE => {
            let event = EventMouseMove {
                position: unsafe { lparam_to_point(lparam) },
            };
            C::on_mouse_move(window, &event);
            win32::LRESULT::default()
        }

        // Mouse Wheel
        win32::WM_MOUSEWHEEL => {
            let delta = hiword!(wparam.0) as i16;
            let event = EventMouseWheel {
                delta_y: delta as i32,
                delta_x: 0,
                position: unsafe { lparam_to_point(lparam) },
            };
            C::on_mouse_wheel(window, &event);
            win32::LRESULT::default()
        }

        win32::WM_MOUSEHWHEEL => {
            let delta = hiword!(wparam.0) as i16;
            let event = EventMouseWheel {
                delta_x: delta as i32,
                delta_y: 0,
                position: unsafe { lparam_to_point(lparam) },
            };
            C::on_mouse_wheel(window, &event);
            win32::LRESULT::default()
        }

        // Window Focused
        win32::WM_SETFOCUS => {
            C::on_focus(window, &EventFocused { is_focused: true });
            win32::LRESULT::default()
        }

        // Window Lost Focus
        win32::WM_KILLFOCUS => {
            C::on_focus(window, &EventFocused { is_focused: false });
            win32::LRESULT::default()
        }

        // Window Moved
        win32::WM_MOVE => {
            let event = EventMoved {
                origin: unsafe { lparam_to_point(lparam) },
            };
            C::on_moved(window, &event);
            win32::LRESULT::default()
        }

        // Window System Command
        win32::WM_SYSCOMMAND => {
            let command = wparam.0 as u32 & 0xfff0;
            match command {
                win32::SC_RESTORE => {
                    C::on_visibility(
                        window,
                        &EventVisibility {
                            visibility: Visibility::Normal,
                        },
                    );
                }
                win32::SC_MINIMIZE => {
                    C::on_visibility(
                        window,
                        &EventVisibility {
                            visibility: Visibility::Minimized,
                        },
                    );
                }
                win32::SC_MAXIMIZE => {
                    C::on_visibility(
                        window,
                        &EventVisibility {
                            visibility: Visibility::Maximized,
                        },
                    );
                }

                _ => {}
            }
            unsafe { win32::DefWindowProcW(hwnd, msg, wparam, lparam) }
        }

        // Window Resized
        win32::WM_SIZE => {
            let event = EventResized {
                extent: unsafe { lparam_to_size(lparam) },
            };
            C::on_resized(window, &event);
            win32::LRESULT::default()
        }

        win32::WM_GETMINMAXINFO => {
            let info = unsafe { (lparam.0 as *mut win32::MINMAXINFO).as_mut() }.unwrap();
            if let Some(min) = window.platform.size_min {
                info.ptMinTrackSize.x = min.width.try_into().unwrap();
                info.ptMinTrackSize.y = min.height.try_into().unwrap();
            }
            if let Some(max) = window.platform.size_max {
                info.ptMaxTrackSize.x = max.width.try_into().unwrap();
                info.ptMaxTrackSize.y = max.height.try_into().unwrap();
            }
            win32::LRESULT::default()
        }

        // Visibility
        win32::WM_SHOWWINDOW => {
            let shown = wparam.0 != win32::FALSE.0 as _;
            C::on_shown(window, &EventShown { shown });
            win32::LRESULT::default()
        }

        // Construction and Destruction
        win32::WM_CREATE => {
            // I::on_created called from window builder
            window.platform.handle = hwnd;
            win32::LRESULT::default()
        }
        win32::WM_CLOSE => {
            let should_close = C::on_close(window, &EventClose {}).unwrap();
            if should_close {
                C::on_closing(window, &EventClosing {});
                window
                    .platform
                    .destroy()
                    .expect("window should be destroyed");
            }
            win32::LRESULT::default()
        }

        win32::WM_DESTROY => {
            window.platform.handle = Default::default();
            C::on_closed(window, &EventClosed {});
            win32::LRESULT::default()
        }
        _ => unsafe { win32::DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

unsafe fn get_window<'a, C: WindowCallbacks>(
    hwnd: win32::HWND,
) -> Option<&'a mut crate::window::Window<C>> {
    unsafe {
        let ptr =
            win32::GetWindowLongPtrW(hwnd, win32::GWLP_USERDATA) as *mut crate::window::Window<C>;
        Some(NonNull::new(ptr)?.as_mut())
    }
}

//
//  Helpers
//

impl Into<win32::WINDOW_STYLE> for &Windowed {
    fn into(self) -> win32::WINDOW_STYLE {
        let mut s = win32::WS_SYSMENU
            | win32::WS_MINIMIZEBOX
            | win32::WS_CLIPSIBLINGS
            | win32::WS_CLIPCHILDREN;
        if self.decorated {
            s |= win32::WS_CAPTION | win32::WS_BORDER;

            if self.resizable {
                s |= win32::WS_THICKFRAME | win32::WS_MAXIMIZEBOX;
            }
        } else {
            s |= win32::WS_POPUP;
        }

        s
    }
}

impl Into<win32::WINDOW_EX_STYLE> for &Windowed {
    fn into(self) -> win32::WINDOW_EX_STYLE {
        win32::WS_EX_APPWINDOW
    }
}

impl TryFrom<(win32::WPARAM, bool)> for VirtualKey {
    type Error = ();

    fn try_from((value, ext): (win32::WPARAM, bool)) -> Result<Self, Self::Error> {
        // println!("{:x} {}", value.0, ext);
        if ext {
            let key = match win32::VIRTUAL_KEY(value.0 as u16) {
                win32::VK_SHIFT => VirtualKey::RightShift,
                win32::VK_CONTROL => VirtualKey::RightControl,
                win32::VK_MENU => VirtualKey::RightAlt,
                win32::VK_RETURN => VirtualKey::NumpadEnter,
                code => unsafe { transmute::<_, VirtualKey>(code.0 as u32) },
            };

            Ok(key)
        } else {
            Ok(unsafe { transmute::<_, VirtualKey>(value.0 as u16 as u32) })
        }
    }
}
