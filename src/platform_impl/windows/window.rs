/*
 * Copyright 2023 Jacob R. Green
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::*;
use super::win32;

use std::ffi::CStr;
use std::ptr::NonNull;
use std::sync::Arc;

use eventify::event::*;

use lazy_static::lazy_static;

static WINDOW_CLASS_NAME: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"winman\0") };

type WindowCloseEvent = Event<()>;
type WindowCloseHook = EventHook<()>;

pub(crate) struct WindowClass {
    _atom: u16,
}

impl WindowClass {
    fn register() -> Result<Self, ()> {
        let atom = unsafe {
            win32::RegisterClassExA(&win32::WNDCLASSEXA {
                cbSize: std::mem::size_of::<win32::WNDCLASSEXA>() as u32,
                style: win32::CS_HREDRAW | win32::CS_VREDRAW,
                lpfnWndProc: Some(window_proc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: Default::default(),
                hIcon: Default::default(),
                hCursor: Default::default(),
                hbrBackground: Default::default(),
                lpszMenuName: win32::PCSTR::null(),
                lpszClassName: win32::PCSTR::from_raw(
                    WINDOW_CLASS_NAME.to_bytes_with_nul().as_ptr(),
                ),
                hIconSm: Default::default(),
            })
        };
        if atom == 0 {
            return Err(());
        }
        Ok(Self { _atom: atom })
    }

    pub fn name(&self) -> &CStr {
        WINDOW_CLASS_NAME
    }
}

impl Drop for WindowClass {
    fn drop(&mut self) {
        unsafe {
            win32::UnregisterClassA(
                win32::PCSTR::from_raw(WINDOW_CLASS_NAME.to_bytes_with_nul().as_ptr()),
                win32::HINSTANCE::default(),
            )
        }
        .unwrap();
    }
}

lazy_static! {
    pub(crate) static ref WINDOW_CLASS: WindowClass = WindowClass::register().unwrap();
}

pub(crate) fn create_window_style(
    config: &WindowConfig,
) -> (win32::WINDOW_STYLE, win32::WINDOW_EX_STYLE) {
    let style_ex = win32::WS_EX_APPWINDOW;

    let mut style = win32::WS_CLIPSIBLINGS | win32::WS_CLIPCHILDREN;
    if config.decorated {
        style |= win32::WS_CAPTION;
        if config.resizable {
            style |= win32::WS_SYSMENU
                | win32::WS_MINIMIZEBOX
                | win32::WS_MAXIMIZEBOX
                | win32::WS_THICKFRAME;
        }
    } else {
        style |= win32::WS_POPUP;
    }
    (style, style_ex)
}

pub struct Window {
    pub(crate) hwnd: win32::HWND,
    pub(crate) controller: Box<dyn WindowController>,
}

impl Window {
    pub fn show(&self) {
        unsafe {
            win32::ShowWindow(self.hwnd, win32::SW_SHOWDEFAULT);
        }
    }

    pub fn hide(&self) {
        unsafe {
            win32::ShowWindow(self.hwnd, win32::SW_HIDE);
        }
    }

    pub fn minimize(&self) {
        unsafe {
            win32::ShowWindow(self.hwnd, win32::SW_SHOWMINIMIZED);
        }
    }

    pub fn maximize(&self) {
        unsafe {
            win32::ShowWindow(self.hwnd, win32::SW_SHOWMAXIMIZED);
        }
    }

    fn close(&self) {
        unsafe {
            self.controller.on_closing(self);
            win32::DestroyWindow(self.hwnd).unwrap();
            self.controller.on_closed(self);
        }
    }

    pub fn set_title(&self, title: &str) {
        let title = std::ffi::CString::new(title).unwrap();
        unsafe {
            win32::SetWindowTextA(self.hwnd, win32::PCSTR::from_raw(title.as_bytes().as_ptr()))
        }
        .unwrap();
    }

    pub fn bind_close_event(&self, event: &WindowCloseEvent) {
        let hwnd = self.hwnd;
        event
            .add_handler(move |_| {
                let window = window_from_hwnd(hwnd).unwrap();
                window.close();
            })
            .leak();
    }
}

unsafe impl Sync for Window {}
unsafe impl Send for Window {}

extern "system" fn window_proc(
    hwnd: win32::HWND,
    msg: u32,
    wparam: win32::WPARAM,
    lparam: win32::LPARAM,
) -> win32::LRESULT {
    match msg {
        // Window Events
        win32::WM_CREATE => unsafe {
            let create_struct = NonNull::new(lparam.0 as *mut win32::CREATESTRUCTA).unwrap();
            let mut window =
                NonNull::new(create_struct.as_ref().lpCreateParams as *mut Window).unwrap();
            window.as_mut().hwnd = hwnd;
            win32::SetWindowLongPtrA(
                hwnd,
                win32::GWLP_USERDATA,
                window.as_ref() as *const _ as isize,
            );

            window.as_ref().controller.on_init(window.as_ref());
            win32::LRESULT::default()
        },
        win32::WM_CLOSE => {
            let window = window_from_hwnd(hwnd).unwrap();
            if window.controller.on_close(window) {
                window.close();
            }
            win32::LRESULT::default()
        }
        win32::WM_DESTROY => unsafe {
            // todo remove window from application
            win32::PostQuitMessage(0);
            win32::LRESULT::default()
        },

        win32::WM_MOVE => {
            let window = window_from_hwnd(hwnd).unwrap();
            let (x, y) = lparam_to_point(lparam);
            window
                .controller
                .on_moved(window, &WindowMovedEvent::new(x, y));
            win32::LRESULT::default()
        }

        win32::WM_SIZE => {
            let window = window_from_hwnd(hwnd).unwrap();
            let (width, height) = lparam_to_size(lparam);
            window
                .controller
                .on_resized(window, &WindowResizedEvent::new(width, height));
            win32::LRESULT::default()
        }

        win32::WM_PAINT => unsafe {
            // println!("WM_PAINT");
            let window = window_from_hwnd(hwnd).unwrap();
            let mut ps = win32::PAINTSTRUCT::default();
            let hdc = win32::BeginPaint(hwnd, &mut ps);
            let color = win32::HBRUSH((win32::COLOR_BACKGROUND.0 + 1) as isize);
            win32::FillRect(hdc, &ps.rcPaint, color);

            win32::EndPaint(hwnd, &ps).unwrap();
            win32::LRESULT::default()

            //
            // win32::DefWindowProcA(hwnd, msg, wparam, lparam)
        },

        // Keyboard Input
        win32::WM_KEYDOWN => {
            let window = window_from_hwnd(hwnd).unwrap();
            let key = wparam_to_vkey(wparam).unwrap();
            let repeat = lparam_to_prev_key_state(lparam);
            let state = if !repeat {
                KeyState::Press
            } else {
                KeyState::Repeat
            };
            window.controller.on_key(window, &KeyEvent::new(key, state));
            win32::LRESULT::default()
        }
        win32::WM_KEYUP => {
            let window = window_from_hwnd(hwnd).unwrap();
            let key = wparam_to_vkey(wparam).unwrap();
            window
                .controller
                .on_key(window, &KeyEvent::new(key, KeyState::Release));
            win32::LRESULT::default()
        }
        win32::WM_CHAR => {
            let window = window_from_hwnd(hwnd).unwrap();
            let c = wparam_char_code(wparam).unwrap();
            let repeat = lparam_to_prev_key_state(lparam);
            window
                .controller
                .on_char(window, &CharEvent::new(c, repeat));
            win32::LRESULT::default()
        }
        win32::WM_SETFOCUS => {
            let window = window_from_hwnd(hwnd).unwrap();
            window
                .controller
                .on_focus(window, &FocusChangedEvent::new(true));
            win32::LRESULT::default()
        }
        win32::WM_KILLFOCUS => {
            let window = window_from_hwnd(hwnd).unwrap();
            window
                .controller
                .on_focus(window, &FocusChangedEvent::new(false));
            win32::LRESULT::default()
        }

        // Mouse Input
        win32::WM_MOUSEMOVE => {
            let window = window_from_hwnd(hwnd).unwrap();
            let (x, y) = lparam_to_point(lparam);
            window
                .controller
                .on_mouse_move(window, &MouseMoveEvent::new(x, y));
            win32::LRESULT::default()
        }
        win32::WM_LBUTTONDOWN => {
            let window = window_from_hwnd(hwnd).unwrap();
            let (x, y) = lparam_to_point(lparam);
            window.controller.on_mouse_button(
                window,
                &MouseButtonEvent::new(MouseButton::Left, ButtonState::Press, x, y),
            );
            win32::LRESULT::default()
        }
        win32::WM_LBUTTONUP => {
            let window = window_from_hwnd(hwnd).unwrap();
            let (x, y) = lparam_to_point(lparam);
            window.controller.on_mouse_button(
                window,
                &MouseButtonEvent::new(MouseButton::Left, ButtonState::Release, x, y),
            );
            win32::LRESULT::default()
        }
        // win32::WM_LBUTTONDBLCLK => {
        //     let window = window_from_hwnd(hwnd).unwrap();
        //     let (x, y) = lparam_to_point(lparam);
        //     println!("LButtonDblClick: ({}, {})", x, y);
        //     win32::LRESULT::default()
        // }
        win32::WM_RBUTTONDOWN => {
            let window = window_from_hwnd(hwnd).unwrap();
            let (x, y) = lparam_to_point(lparam);
            window.controller.on_mouse_button(
                window,
                &MouseButtonEvent::new(MouseButton::Right, ButtonState::Press, x, y),
            );
            win32::LRESULT::default()
        }
        win32::WM_RBUTTONUP => {
            let window = window_from_hwnd(hwnd).unwrap();
            let (x, y) = lparam_to_point(lparam);
            window.controller.on_mouse_button(
                window,
                &MouseButtonEvent::new(MouseButton::Right, ButtonState::Release, x, y),
            );
            win32::LRESULT::default()
        }
        // win32::WM_RBUTTONDBLCLK => {
        //     let window = window_from_hwnd(hwnd).unwrap();
        //     let (x, y) = lparam_to_point(lparam);
        //     println!("RButtonDblClick: ({}, {})", x, y);
        //     win32::LRESULT::default()
        // }
        win32::WM_MBUTTONDOWN => {
            let window = window_from_hwnd(hwnd).unwrap();
            let (x, y) = lparam_to_point(lparam);
            window.controller.on_mouse_button(
                window,
                &MouseButtonEvent::new(MouseButton::Middle, ButtonState::Press, x, y),
            );
            win32::LRESULT::default()
        }
        win32::WM_MBUTTONUP => {
            let window = window_from_hwnd(hwnd).unwrap();
            let (x, y) = lparam_to_point(lparam);
            window.controller.on_mouse_button(
                window,
                &MouseButtonEvent::new(MouseButton::Middle, ButtonState::Release, x, y),
            );
            win32::LRESULT::default()
        }
        // win32::WM_MBUTTONDBLCLK => {
        //     let window = window_from_hwnd(hwnd).unwrap();
        //     let (x, y) = lparam_to_point(lparam);
        //     println!("MButtonDblClick: ({}, {})", x, y);
        //     win32::LRESULT::default()
        // }
        win32::WM_XBUTTONDOWN => {
            let window = window_from_hwnd(hwnd).unwrap();
            let (x, y) = lparam_to_point(lparam);
            let button = wparam_to_xkey(wparam);
            window.controller.on_mouse_button(
                window,
                &MouseButtonEvent::new(button, ButtonState::Press, x, y),
            );
            win32::LRESULT::default()
        }
        win32::WM_XBUTTONUP => {
            let window = window_from_hwnd(hwnd).unwrap();
            let (x, y) = lparam_to_point(lparam);
            let button = wparam_to_xkey(wparam);
            window.controller.on_mouse_button(
                window,
                &MouseButtonEvent::new(button, ButtonState::Release, x, y),
            );
            win32::LRESULT::default()
        }
        // win32::WM_XBUTTONDBLCLK => {
        //     let window = window_from_hwnd(hwnd).unwrap();
        //     let (x, y) = lparam_to_point(lparam);
        //     let button = wparam_to_xkey(wparam);
        //     println!("XButtonDblClick: {:?} ({}, {})", button, x, y);
        //     win32::LRESULT::default()
        // }
        win32::WM_MOUSEWHEEL => {
            let window = window_from_hwnd(hwnd).unwrap();
            let delta = wparam_to_wheel_delta(wparam);
            let (x, y) = lparam_to_point(lparam);
            window
                .controller
                .on_mouse_wheel(window, &MouseWheelEvent::new(delta as f32, 0.0, x, y));
            win32::LRESULT::default()
        }
        win32::WM_MOUSEHWHEEL => {
            let window = window_from_hwnd(hwnd).unwrap();
            let delta = wparam_to_wheel_delta(wparam);
            let (x, y) = lparam_to_point(lparam);
            window
                .controller
                .on_mouse_wheel(window, &MouseWheelEvent::new(0.0, delta as f32, x, y));
            win32::LRESULT::default()
        }

        _ => unsafe { win32::DefWindowProcA(hwnd, msg, wparam, lparam) },
    }
}

fn window_from_hwnd<'a>(hwnd: win32::HWND) -> Option<&'a Window> {
    NonNull::new(unsafe { win32::GetWindowLongPtrA(hwnd, win32::GWLP_USERDATA) } as *mut Window)
        .map(|ptr| unsafe { ptr.as_ref() })
}

fn lparam_to_point(lparam: win32::LPARAM) -> (f32, f32) {
    let x = win32::lparam_loword(lparam) as i16 as f32;
    let y = win32::lparam_hiword(lparam) as i16 as f32;
    (x, y)
}

fn lparam_to_size(lparam: win32::LPARAM) -> (f32, f32) {
    let width = win32::lparam_loword(lparam) as f32;
    let height = win32::lparam_hiword(lparam) as f32;
    (width, height)
}

fn wparam_to_vkey(wparam: win32::WPARAM) -> Option<VirtualKey> {
    Some(unsafe { VirtualKey::from_raw(wparam.0 as u32) })
}

fn lparam_to_prev_key_state(lparam: win32::LPARAM) -> bool {
    lparam.0 & (1 << 30) != 0
}

fn wparam_char_code(wparam: win32::WPARAM) -> Option<char> {
    char::from_u32(wparam.0 as u32)
}

fn wparam_to_wheel_delta(wparam: win32::WPARAM) -> i16 {
    win32::wparam_hiword(wparam) as i16
}

fn wparam_to_xkey(wparam: win32::WPARAM) -> MouseButton {
    match win32::wparam_hiword(wparam) {
        win32::XBUTTON1 => MouseButton::X1,
        win32::XBUTTON2 => MouseButton::X2,
        _ => panic!(),
    }
}
