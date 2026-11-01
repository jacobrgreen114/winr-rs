// Copyright (c) 2026 Jacob Green
// SPDX-License-Identifier: MIT OR Apache-2.0

pub(crate) mod display;
pub(crate) mod window;
pub(crate) mod win32 {
    pub use windows::Win32::Foundation::*;
    pub use windows::Win32::Graphics::Gdi::*;
    pub use windows::Win32::UI::WindowsAndMessaging::*;
    pub use windows::core::*;
}
