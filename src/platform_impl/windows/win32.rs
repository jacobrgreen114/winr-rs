
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