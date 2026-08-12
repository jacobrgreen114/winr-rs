// Copyright (c) 2026 Jacob Green
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::platform;

//  Keyboard Input

macro_rules! virtual_keys {
    {$($variant:ident),* $(,)?} => {
        /// Virtual key codes
        ///
        /// !!! Important !!!
        ///
        /// `VirtualKey`'s values mirror platforms codes as closely as possible.
        /// Therefore, values are not stable across platforms.
        /// Additional attention to detail is required if serializing.
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, Copy, Clone)]
        pub enum VirtualKey {
            $($variant = platform::VirtualKeyConstants::$variant),*
        }

        #[allow(non_upper_case_globals)]
        pub(crate) trait PlatformVirtualKeyConstants {
            $(const $variant: u32;)*
        }
    };
}

virtual_keys! {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,

    Plus,
    Minus,

    LeftBracket,
    RightBracket,
    Slash,
    Backslash,
    Semicolon,
    Apostrophe,
    Comma,
    Period,
    Grave,

    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,

    NumpadDecimal,
    NumpadAdd,
    NumpadSubtract,
    NumpadMultiply,
    NumpadDivide,
    // NumpadEquals,
    NumpadEnter,

    LeftArrow,
    UpArrow,
    RightArrow,
    DownArrow,

    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,

    Space,
    Backspace,
    Tab,
    Enter,

    Escape,
    Pause,
    PrintScreen,


    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,

    CapsLock,
    NumLock,
    ScrollLock,

    LeftShift,
    RightShift,
    LeftControl,
    RightControl,
    LeftAlt,
    RightAlt,
    LeftSuper,
    RightSuper,
    Menu,

    VolumeUp,
    VolumeDown,
    VolumeMute,

    MediaPlay,
    MediaStop,
    MediaPrev,
    MediaNext,
}

/// Key states
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyState {
    Release = 0,
    Press = 1,
    Repeat = 2,
}

// bitflags::bitflags! {
//     #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
//     pub struct KeyModifiers : u32 {
//         const SHIFT = 1 << 0;
//         const CONTROL = 1 << 1;
//         const ALT = 1 << 2;
//         const SUPER = 1 << 3;
//     }
// }

//  Mouse Input

/// Mouse buttons
///
/// !!! Important !!!
/// `MouseButton`'s values mirror platforms codes as closely as possible.
/// Therefore, values are not stable across platforms.
/// Additional attention to detail is required if serializing.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Button4,
    Button5,
    Button6,
    Button7,
    Button8,
}

/// Button states
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ButtonState {
    Release = 0,
    Press = 1,
}
