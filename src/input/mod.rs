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

mod enums;
pub use enums::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct KeyEvent {
    key: VirtualKey,
    state: KeyState,
}

impl KeyEvent {
    pub(crate) const fn new(key: VirtualKey, state: KeyState) -> Self {
        Self { key, state }
    }

    pub const fn key(&self) -> VirtualKey {
        self.key
    }

    pub const fn state(&self) -> KeyState {
        self.state
    }

    pub fn is_key(&self, key: VirtualKey) -> bool {
        self.key == key
    }

    pub fn is_down(&self) -> bool {
        self.state == KeyState::Press
    }

    pub fn is_up(&self) -> bool {
        self.state == KeyState::Release
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CharEvent {
    c: char,
    repeat: bool,
}

impl CharEvent {
    pub(crate) const fn new(c: char, repeat: bool) -> Self {
        Self { c, repeat }
    }

    pub const fn char(&self) -> char {
        self.c
    }

    pub const fn repeat(&self) -> bool {
        self.repeat
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MouseMoveEvent {
    x: f32,
    y: f32,
}

impl MouseMoveEvent {
    pub(crate) const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub const fn pos(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MouseButtonEvent {
    button: MouseButton,
    state: ButtonState,
    x: f32,
    y: f32,
}

impl MouseButtonEvent {
    pub(crate) const fn new(button: MouseButton, state: ButtonState, x: f32, y: f32) -> Self {
        Self {
            button,
            state,
            x,
            y,
        }
    }

    pub const fn button(&self) -> MouseButton {
        self.button
    }

    pub const fn state(&self) -> ButtonState {
        self.state
    }

    pub const fn pos(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MouseWheelEvent {
    vert: f32,
    horz: f32,
    x: f32,
    y: f32,
}

impl MouseWheelEvent {
    pub(crate) const fn new(vert: f32, horz: f32, x: f32, y: f32) -> Self {
        Self { vert, horz, x, y }
    }

    pub const fn vert(&self) -> f32 {
        self.vert
    }

    pub const fn horz(&self) -> f32 {
        self.horz
    }

    pub const fn pos(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct WindowMovedEvent {
    x: f32,
    y: f32,
}

impl WindowMovedEvent {
    pub(crate) const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub const fn pos(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct WindowResizedEvent {
    width: f32,
    height: f32,
}

impl WindowResizedEvent {
    pub(crate) const fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub const fn size(&self) -> (f32, f32) {
        (self.width, self.height)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FocusChangedEvent {
    focused: bool,
}

impl FocusChangedEvent {
    pub(crate) const fn new(focused: bool) -> Self {
        Self { focused }
    }

    pub const fn focus(&self) -> bool {
        self.focused
    }
}
