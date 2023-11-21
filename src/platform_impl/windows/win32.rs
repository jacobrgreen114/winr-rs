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

pub use windows::core::*;
pub use windows::Win32::Foundation::*;
pub use windows::Win32::Graphics::Gdi::*;
pub use windows::Win32::UI::Input::KeyboardAndMouse::*;
pub use windows::Win32::UI::WindowsAndMessaging::*;

pub const fn lparam_loword(value: LPARAM) -> u16 {
    value.0 as u16
}

pub const fn lparam_hiword(value: LPARAM) -> u16 {
    (value.0 as u32 >> 16) as u16
}

pub const fn wparam_loword(value: WPARAM) -> u16 {
    value.0 as u16
}

pub const fn wparam_hiword(value: WPARAM) -> u16 {
    (value.0 as u32 >> 16) as u16
}
