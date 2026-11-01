// Copyright (c) 2026 Jacob Green
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Keyboard and pointer input. Key identities are independent of native numeric codes.
// One table generates both public identities and the checked Windows mapping.
macro_rules! virtual_keys {
    ($($variant:ident => $native:ident),* $(,)?) => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
        #[non_exhaustive]
        pub enum VirtualKey {
            $($variant,)*
            NumpadEnter, RightShift, RightControl, RightAlt,
            Unknown(u32),
        }
        #[cfg(windows)]
        pub(crate) fn virtual_key(code: u32, scan_code: u8, extended: bool) -> VirtualKey {
            use windows::Win32::UI::Input::KeyboardAndMouse::*;
            // Shift is distinguished by its scan code, not the extended-key bit.
            if code == VK_SHIFT.0 as u32 {
                return if scan_code == 0x36 { VirtualKey::RightShift } else { VirtualKey::LeftShift };
            }
            if extended {
                match code {
                    c if c == VK_CONTROL.0 as u32 => return VirtualKey::RightControl,
                    c if c == VK_MENU.0 as u32 => return VirtualKey::RightAlt,
                    c if c == VK_RETURN.0 as u32 => return VirtualKey::NumpadEnter,
                    _ => {}
                }
            }
            match code {
                $(c if c == $native.0 as u32 => VirtualKey::$variant,)*
                other => VirtualKey::Unknown(other),
            }
        }
    };
}
virtual_keys! {
    A => VK_A,
    B => VK_B,
    C => VK_C,
    D => VK_D,
    E => VK_E,
    F => VK_F,
    G => VK_G,
    H => VK_H,
    I => VK_I,
    J => VK_J,
    K => VK_K,
    L => VK_L,
    M => VK_M,
    N => VK_N,
    O => VK_O,
    P => VK_P,
    Q => VK_Q,
    R => VK_R,
    S => VK_S,
    T => VK_T,
    U => VK_U,
    V => VK_V,
    W => VK_W,
    X => VK_X,
    Y => VK_Y,
    Z => VK_Z,
    _0 => VK_0,
    _1 => VK_1,
    _2 => VK_2,
    _3 => VK_3,
    _4 => VK_4,
    _5 => VK_5,
    _6 => VK_6,
    _7 => VK_7,
    _8 => VK_8,
    _9 => VK_9,
    Plus => VK_OEM_PLUS,
    Minus => VK_OEM_MINUS,
    LeftBracket => VK_OEM_4,
    RightBracket => VK_OEM_6,
    Slash => VK_OEM_2,
    Backslash => VK_OEM_5,
    Semicolon => VK_OEM_1,
    Apostrophe => VK_OEM_7,
    Comma => VK_OEM_COMMA,
    Period => VK_OEM_PERIOD,
    Grave => VK_OEM_3,
    Numpad0 => VK_NUMPAD0,
    Numpad1 => VK_NUMPAD1,
    Numpad2 => VK_NUMPAD2,
    Numpad3 => VK_NUMPAD3,
    Numpad4 => VK_NUMPAD4,
    Numpad5 => VK_NUMPAD5,
    Numpad6 => VK_NUMPAD6,
    Numpad7 => VK_NUMPAD7,
    Numpad8 => VK_NUMPAD8,
    Numpad9 => VK_NUMPAD9,
    NumpadDecimal => VK_DECIMAL,
    NumpadAdd => VK_ADD,
    NumpadSubtract => VK_SUBTRACT,
    NumpadMultiply => VK_MULTIPLY,
    NumpadDivide => VK_DIVIDE,
    LeftArrow => VK_LEFT,
    UpArrow => VK_UP,
    RightArrow => VK_RIGHT,
    DownArrow => VK_DOWN,
    Insert => VK_INSERT,
    Delete => VK_DELETE,
    Home => VK_HOME,
    End => VK_END,
    PageUp => VK_PRIOR,
    PageDown => VK_NEXT,
    Space => VK_SPACE,
    Backspace => VK_BACK,
    Tab => VK_TAB,
    Enter => VK_RETURN,
    Escape => VK_ESCAPE,
    Pause => VK_PAUSE,
    PrintScreen => VK_SNAPSHOT,
    F1 => VK_F1,
    F2 => VK_F2,
    F3 => VK_F3,
    F4 => VK_F4,
    F5 => VK_F5,
    F6 => VK_F6,
    F7 => VK_F7,
    F8 => VK_F8,
    F9 => VK_F9,
    F10 => VK_F10,
    F11 => VK_F11,
    F12 => VK_F12,
    F13 => VK_F13,
    F14 => VK_F14,
    F15 => VK_F15,
    F16 => VK_F16,
    F17 => VK_F17,
    F18 => VK_F18,
    F19 => VK_F19,
    F20 => VK_F20,
    F21 => VK_F21,
    F22 => VK_F22,
    F23 => VK_F23,
    F24 => VK_F24,
    CapsLock => VK_CAPITAL,
    NumLock => VK_NUMLOCK,
    ScrollLock => VK_SCROLL,
    LeftShift => VK_SHIFT,
    LeftControl => VK_CONTROL,
    LeftAlt => VK_MENU,
    LeftSuper => VK_LWIN,
    RightSuper => VK_RWIN,
    Menu => VK_APPS,
    VolumeUp => VK_VOLUME_UP,
    VolumeDown => VK_VOLUME_DOWN,
    VolumeMute => VK_VOLUME_MUTE,
    MediaPlay => VK_MEDIA_PLAY_PAUSE,
    MediaStop => VK_MEDIA_STOP,
    MediaPrev => VK_MEDIA_PREV_TRACK,
    MediaNext => VK_MEDIA_NEXT_TRACK,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum KeyState {
    Release,
    Press,
    Repeat,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Button4,
    Button5,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ButtonState {
    Release,
    Press,
}

/// Incrementally decodes Windows UTF-16 character messages.
/// A malformed sequence produces the Unicode replacement character.
#[cfg(any(windows, test))]
#[derive(Default)]
pub(crate) struct Utf16Decoder {
    high: Option<u16>,
}
#[cfg(any(windows, test))]
impl Utf16Decoder {
    pub fn push(&mut self, unit: u16) -> [Option<char>; 2] {
        let mut result = [None, None];
        if let Some(high) = self.high.take() {
            if (0xdc00..=0xdfff).contains(&unit) {
                let scalar = 0x10000 + (((high as u32 - 0xd800) << 10) | (unit as u32 - 0xdc00));
                return [char::from_u32(scalar), None];
            }
            result[0] = Some(char::REPLACEMENT_CHARACTER);
        }
        let next = if (0xd800..=0xdbff).contains(&unit) {
            self.high = Some(unit);
            None
        } else {
            Some(char::from_u32(unit as u32).unwrap_or(char::REPLACEMENT_CHARACTER))
        };
        if result[0].is_none() {
            result[0] = next;
        } else {
            result[1] = next;
        }
        result
    }
}

#[cfg(any(windows, test))]
pub(crate) fn signed_point(bits: isize) -> crate::Point {
    crate::Point::new(
        bits as u16 as i16 as isize,
        ((bits >> 16) as u16 as i16) as isize,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn supplementary_characters_and_malformed_sequences() {
        let mut decoder = Utf16Decoder::default();
        assert_eq!(decoder.push(0xd83d), [None, None]);
        assert_eq!(decoder.push(0xde00), [Some('😀'), None]);
        assert_eq!(decoder.push(0xdc00), [Some('�'), None]);
        decoder.push(0xd800);
        assert_eq!(decoder.push(b'A' as u16), [Some('�'), Some('A')]);
        decoder.push(0xd800);
        assert_eq!(decoder.push(0xd83d), [Some('�'), None]);
        assert_eq!(decoder.push(0xde00), [Some('😀'), None]);
    }
    #[test]
    fn negative_coordinates_are_sign_extended() {
        assert_eq!(signed_point(0xfffeffff), crate::Point::new(-1, -2));
        assert_eq!(signed_point(0x7fff8000), crate::Point::new(-32768, 32767));
    }
    #[cfg(windows)]
    #[test]
    fn every_key_code_is_safe_and_modifiers_are_distinct() {
        for code in 0..=0xffff {
            let _ = virtual_key(code, 0, false);
            let _ = virtual_key(code, 0, true);
        }
        assert_eq!(virtual_key(0xff, 0, false), VirtualKey::Unknown(0xff));
        assert_eq!(virtual_key(0x10, 0x36, false), VirtualKey::RightShift);
        assert_eq!(virtual_key(0x11, 0, true), VirtualKey::RightControl);
        assert_eq!(virtual_key(0x0d, 0, true), VirtualKey::NumpadEnter);
    }
}
