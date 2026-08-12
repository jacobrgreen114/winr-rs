// Copyright (c) 2026 Jacob Green
// SPDX-License-Identifier: MIT OR Apache-2.0

mod application;

use application::*;
use std::convert::Into;

mod window;
use window::*;

mod display;
use display::*;

use super::*;

mod win32 {
    pub use windows::Win32::Foundation::*;
    pub use windows::Win32::Graphics::Gdi::*;
    pub use windows::Win32::UI::Input::KeyboardAndMouse::*;
    pub use windows::Win32::UI::WindowsAndMessaging::*;
    pub use windows::core::*;
}

pub type Win32Error = win32::Error;

// Api

impl PlatformWindowingApi for WindowingApi {
    type Error = Win32Error;

    type ApplicationBuilder<C: ApplicationCallbacks> = Win32ApplicationBuilder<C>;
    type WindowBuilder<C: WindowCallbacks> = Win32WindowBuilder<C>;

    type WindowHandle = Win32WindowHandle;
    
    type Displays = Win32Monitors;

    type VirtualKeyConstants = Win32VirtualKeyConstants;
}

pub struct Win32VirtualKeyConstants;

macro_rules! win32_virtual_key {
    ($expr:expr) => {
        $expr.0 as u32
    };
}

macro_rules! win32_virtual_key_constants {
    {$($variant:ident -> $value:expr),* $(,)?} => {
        #[allow(non_upper_case_globals)]
        impl PlatformVirtualKeyConstants for Win32VirtualKeyConstants {
            $(const $variant : u32 = $value;)*
        }
    };
}

macro_rules! extended {
    ($value:expr) => {
        (1 << 16) | $value
    };
}

win32_virtual_key_constants! {
    A -> win32_virtual_key!(win32::VK_A),
    B -> win32_virtual_key!(win32::VK_B),
    C -> win32_virtual_key!(win32::VK_C),
    D -> win32_virtual_key!(win32::VK_D),
    E -> win32_virtual_key!(win32::VK_E),
    F -> win32_virtual_key!(win32::VK_F),
    G -> win32_virtual_key!(win32::VK_G),
    H -> win32_virtual_key!(win32::VK_H),
    I -> win32_virtual_key!(win32::VK_I),
    J -> win32_virtual_key!(win32::VK_J),
    K -> win32_virtual_key!(win32::VK_K),
    L -> win32_virtual_key!(win32::VK_L),
    M -> win32_virtual_key!(win32::VK_M),
    N -> win32_virtual_key!(win32::VK_N),
    O -> win32_virtual_key!(win32::VK_O),
    P -> win32_virtual_key!(win32::VK_P),
    Q -> win32_virtual_key!(win32::VK_Q),
    R -> win32_virtual_key!(win32::VK_R),
    S -> win32_virtual_key!(win32::VK_S),
    T -> win32_virtual_key!(win32::VK_T),
    U -> win32_virtual_key!(win32::VK_U),
    V -> win32_virtual_key!(win32::VK_V),
    W -> win32_virtual_key!(win32::VK_W),
    X -> win32_virtual_key!(win32::VK_X),
    Y -> win32_virtual_key!(win32::VK_Y),
    Z -> win32_virtual_key!(win32::VK_Z),

    _0 -> win32_virtual_key!(win32::VK_0),
    _1 -> win32_virtual_key!(win32::VK_1),
    _2 -> win32_virtual_key!(win32::VK_2),
    _3 -> win32_virtual_key!(win32::VK_3),
    _4 -> win32_virtual_key!(win32::VK_4),
    _5 -> win32_virtual_key!(win32::VK_5),
    _6 -> win32_virtual_key!(win32::VK_6),
    _7 -> win32_virtual_key!(win32::VK_7),
    _8 -> win32_virtual_key!(win32::VK_8),
    _9 -> win32_virtual_key!(win32::VK_9),

    Plus -> win32_virtual_key!(win32::VK_OEM_PLUS),
    Minus -> win32_virtual_key!(win32::VK_OEM_MINUS),
    LeftBracket -> win32_virtual_key!(win32::VK_OEM_4),
    RightBracket -> win32_virtual_key!(win32::VK_OEM_6),
    Slash -> win32_virtual_key!(win32::VK_OEM_2),
    Backslash -> win32_virtual_key!(win32::VK_OEM_5),
    Semicolon -> win32_virtual_key!(win32::VK_OEM_1),
    Apostrophe -> win32_virtual_key!(win32::VK_OEM_7),
    Comma -> win32_virtual_key!(win32::VK_OEM_COMMA),
    Period -> win32_virtual_key!(win32::VK_OEM_PERIOD),
    Grave -> win32_virtual_key!(win32::VK_OEM_3),

    Numpad0 -> win32_virtual_key!(win32::VK_NUMPAD0),
    Numpad1 -> win32_virtual_key!(win32::VK_NUMPAD1),
    Numpad2 -> win32_virtual_key!(win32::VK_NUMPAD2),
    Numpad3 -> win32_virtual_key!(win32::VK_NUMPAD3),
    Numpad4 -> win32_virtual_key!(win32::VK_NUMPAD4),
    Numpad5 -> win32_virtual_key!(win32::VK_NUMPAD5),
    Numpad6 -> win32_virtual_key!(win32::VK_NUMPAD6),
    Numpad7 -> win32_virtual_key!(win32::VK_NUMPAD7),
    Numpad8 -> win32_virtual_key!(win32::VK_NUMPAD8),
    Numpad9 -> win32_virtual_key!(win32::VK_NUMPAD9),

    NumpadDecimal -> win32_virtual_key!(win32::VK_DECIMAL),
    NumpadAdd -> win32_virtual_key!(win32::VK_ADD),
    NumpadSubtract -> win32_virtual_key!(win32::VK_SUBTRACT),
    NumpadMultiply -> win32_virtual_key!(win32::VK_MULTIPLY),
    NumpadDivide -> win32_virtual_key!(win32::VK_DIVIDE),
    // NumpadEquals -> ,
    NumpadEnter -> extended!(win32_virtual_key!(win32::VK_RETURN)),

    LeftArrow -> win32_virtual_key!(win32::VK_LEFT),
    UpArrow -> win32_virtual_key!(win32::VK_UP),
    RightArrow -> win32_virtual_key!(win32::VK_RIGHT),
    DownArrow -> win32_virtual_key!(win32::VK_DOWN),

    Insert -> win32_virtual_key!(win32::VK_INSERT),
    Delete -> win32_virtual_key!(win32::VK_DELETE),
    Home -> win32_virtual_key!(win32::VK_HOME),
    End -> win32_virtual_key!(win32::VK_END),
    PageUp -> win32_virtual_key!(win32::VK_PRIOR),
    PageDown -> win32_virtual_key!(win32::VK_NEXT),

    Space -> win32_virtual_key!(win32::VK_SPACE),
    Backspace -> win32_virtual_key!(win32::VK_BACK),
    Tab -> win32_virtual_key!(win32::VK_TAB),
    Enter -> win32_virtual_key!(win32::VK_RETURN),

    Escape -> win32_virtual_key!(win32::VK_ESCAPE),
    Pause -> win32_virtual_key!(win32::VK_PAUSE),
    PrintScreen -> win32_virtual_key!(win32::VK_SNAPSHOT),

    F1 -> win32_virtual_key!(win32::VK_F1),
    F2 -> win32_virtual_key!(win32::VK_F2),
    F3 -> win32_virtual_key!(win32::VK_F3),
    F4 -> win32_virtual_key!(win32::VK_F4),
    F5 -> win32_virtual_key!(win32::VK_F5),
    F6 -> win32_virtual_key!(win32::VK_F6),
    F7 -> win32_virtual_key!(win32::VK_F7),
    F8 -> win32_virtual_key!(win32::VK_F8),
    F9 -> win32_virtual_key!(win32::VK_F9),
    F10 -> win32_virtual_key!(win32::VK_F10),
    F11 -> win32_virtual_key!(win32::VK_F11),
    F12 -> win32_virtual_key!(win32::VK_F12),
    F13 -> win32_virtual_key!(win32::VK_F13),
    F14 -> win32_virtual_key!(win32::VK_F14),
    F15 -> win32_virtual_key!(win32::VK_F15),
    F16 -> win32_virtual_key!(win32::VK_F16),
    F17 -> win32_virtual_key!(win32::VK_F17),
    F18 -> win32_virtual_key!(win32::VK_F18),
    F19 -> win32_virtual_key!(win32::VK_F19),
    F20 -> win32_virtual_key!(win32::VK_F20),
    F21 -> win32_virtual_key!(win32::VK_F21),
    F22 -> win32_virtual_key!(win32::VK_F22),
    F23 -> win32_virtual_key!(win32::VK_F23),
    F24 -> win32_virtual_key!(win32::VK_F24),

    CapsLock -> win32_virtual_key!(win32::VK_CAPITAL),
    NumLock -> win32_virtual_key!(win32::VK_NUMLOCK),
    ScrollLock -> win32_virtual_key!(win32::VK_SCROLL),

    LeftShift -> win32_virtual_key!(win32::VK_SHIFT),
    RightShift -> extended!(win32_virtual_key!(win32::VK_SHIFT)),
    LeftControl -> win32_virtual_key!(win32::VK_CONTROL),
    RightControl -> extended!(win32_virtual_key!(win32::VK_CONTROL)),
    LeftAlt -> win32_virtual_key!(win32::VK_MENU),
    RightAlt -> extended!(win32_virtual_key!(win32::VK_MENU)),
    LeftSuper -> win32_virtual_key!(win32::VK_LWIN),
    RightSuper -> win32_virtual_key!(win32::VK_RWIN),
    Menu -> win32_virtual_key!(win32::VK_APPS),

    VolumeUp -> win32_virtual_key!(win32::VK_VOLUME_UP),
    VolumeDown -> win32_virtual_key!(win32::VK_VOLUME_DOWN),
    VolumeMute -> win32_virtual_key!(win32::VK_VOLUME_MUTE),

    MediaPlay -> win32_virtual_key!(win32::VK_MEDIA_PLAY_PAUSE),
    MediaStop -> win32_virtual_key!(win32::VK_MEDIA_STOP),
    MediaPrev -> win32_virtual_key!(win32::VK_MEDIA_PREV_TRACK),
    MediaNext -> win32_virtual_key!(win32::VK_MEDIA_NEXT_TRACK),
}
