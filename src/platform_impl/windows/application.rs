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
use super::window::*;

use std::ffi::c_void;
use std::pin::Pin;
use std::sync::{Arc, RwLock};
use std::ops::Deref;

pub struct Application {
    windows: RwLock<Vec<Pin<Arc<Window>>>>,
}

impl Application {
    fn default() -> Self {
        Self {
            windows: RwLock::new(Vec::new()),
        }
    }

    pub fn shutdown() {
        unsafe {
            win32::PostQuitMessage(0);
        }
    }

    pub fn wait_for_events(&self) {
        unsafe { win32::WaitMessage().unwrap() };
    }

    pub fn run<C: ApplicationController>(controller: C) -> C::ExitCode {
        let app = Application::default();
        controller.on_init(&app);

        let mut msg = win32::MSG::default();
        loop {
            controller.before_window_events(&app);
            while unsafe { win32::PeekMessageA(&mut msg, None, 0, 0, win32::PM_REMOVE).as_bool() } {
                unsafe {
                    win32::TranslateMessage(&msg);
                    win32::DispatchMessageA(&msg);
                }
            }
            if msg.message == win32::WM_QUIT {
                break;
            }

            controller.after_window_events(&app);
        }

        controller.on_exit(&app)
    }

    pub fn create_window<C: WindowController + 'static>(&self, controller: C) -> Result<(), ()> {
        let config = controller.get_config();
        let (style, style_ex) = create_window_style(&config);
        let title = std::ffi::CString::new(config.title).unwrap();

        let controller = Box::new(controller);
        let window = Arc::pin(Window {
            hwnd: win32::HWND::default(),
            controller,
        });

        let (width, height) = {
            let size = config.size.unwrap_or(Size {
                width: 640.0,
                height: 480.0,
            });

            let mut rect = win32::RECT {
                left: 0,
                top: 0,
                right: size.width as i32,
                bottom: size.height as i32,
            };
            unsafe { win32::AdjustWindowRectEx(&mut rect, style, false, style_ex) }.unwrap();
            (rect.right - rect.left, rect.bottom - rect.top)
        };

        let hwnd = unsafe {
            win32::CreateWindowExA(
                style_ex,
                win32::PCSTR::from_raw(WINDOW_CLASS.name().to_bytes_with_nul().as_ptr()),
                win32::PCSTR::from_raw(title.as_bytes().as_ptr()),
                style,
                config
                    .pos
                    .map(|pos| pos.x as i32)
                    .unwrap_or(win32::CW_USEDEFAULT),
                config
                    .pos
                    .map(|pos| pos.y as i32)
                    .unwrap_or(win32::CW_USEDEFAULT),
                width,
                height,
                win32::HWND::default(),
                win32::HMENU::default(),
                win32::HINSTANCE::default(),
                Some(window.deref() as *const Window as *const c_void),
            )
        };
        if hwnd.0 == 0 {
            return Err(());
        }

        self.windows.write().unwrap().push(window);
        Ok(())
    }
}
