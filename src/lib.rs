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

extern crate lazy_static;
extern crate eventify;

#[cfg(target_os = "windows")]
extern crate windows;

mod input;
mod platform_impl;

pub use platform_impl::Application;
pub use platform_impl::Window;
pub use input::*;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Rect {
    pos: Point,
    size: Size,
}

#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub title: String,
    pub size: Option<Size>,
    pub pos: Option<Point>,
    pub decorated: bool,
    pub resizable: bool,
}

impl WindowConfig {
    pub const fn default() -> Self {
        Self {
            title: String::new(),
            size: None,
            pos: None,
            decorated: true,
            resizable: true,
        }
    }
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self::default()
    }
}

#[allow(unused_variables)]
pub trait WindowController {
    fn get_config(&self) -> WindowConfig {
        WindowConfig::default()
    }

    fn on_init(&self, window: &Window) {}
    fn on_close(&self, window: &Window) -> bool {
        true
    }
    fn on_closing(&self, window: &Window) {}
    fn on_closed(&self, window: &Window) {}

    fn on_moved(&self, window: &Window, event: &WindowMovedEvent) {}
    fn on_resized(&self, window: &Window, event: &WindowResizedEvent) {}

    fn on_key(&self, window: &Window, event: &KeyEvent) {}
    fn on_char(&self, window: &Window, event: &CharEvent) {}
    fn on_focus(&self, window: &Window, event: &FocusChangedEvent) {}

    fn on_mouse_move(&self, window: &Window, event: &MouseMoveEvent) {}
    fn on_mouse_button(&self, window: &Window, event: &MouseButtonEvent) {}
    fn on_mouse_wheel(&self, window: &Window, event: &MouseWheelEvent) {}
}

#[allow(unused_variables)]
pub trait ApplicationController {
    type ExitCode;

    fn on_init(&self, app: &Application);
    fn on_exit(&self, app: &Application) -> Self::ExitCode;

    fn before_window_events(&self, app: &Application) {}
    fn after_window_events(&self, app: &Application) {
        app.wait_for_events();
    }
}
