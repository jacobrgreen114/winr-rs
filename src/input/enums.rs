use crate::platform_impl;

pub(crate) type VirtualKeyT = u32;

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum KeyState {
    Press,
    Release,
    Repeat,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ButtonState {
    Release,
    Press,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    X1,
    X2,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VirtualKey {
    A = platform_impl::virtual_key::A,
    B = platform_impl::virtual_key::B,
    C = platform_impl::virtual_key::C,
    D = platform_impl::virtual_key::D,
    E = platform_impl::virtual_key::E,
    F = platform_impl::virtual_key::F,
    G = platform_impl::virtual_key::G,
    H = platform_impl::virtual_key::H,
    I = platform_impl::virtual_key::I,
    J = platform_impl::virtual_key::J,
    K = platform_impl::virtual_key::K,
    L = platform_impl::virtual_key::L,
    M = platform_impl::virtual_key::M,
    N = platform_impl::virtual_key::N,
    O = platform_impl::virtual_key::O,
    P = platform_impl::virtual_key::P,
    Q = platform_impl::virtual_key::Q,
    R = platform_impl::virtual_key::R,
    S = platform_impl::virtual_key::S,
    T = platform_impl::virtual_key::T,
    U = platform_impl::virtual_key::U,
    V = platform_impl::virtual_key::V,
    W = platform_impl::virtual_key::W,
    X = platform_impl::virtual_key::X,
    Y = platform_impl::virtual_key::Y,
    Z = platform_impl::virtual_key::Z,

    Num0 = platform_impl::virtual_key::NUM0,
    Num1 = platform_impl::virtual_key::NUM1,
    Num2 = platform_impl::virtual_key::NUM2,
    Num3 = platform_impl::virtual_key::NUM3,
    Num4 = platform_impl::virtual_key::NUM4,
    Num5 = platform_impl::virtual_key::NUM5,
    Num6 = platform_impl::virtual_key::NUM6,
    Num7 = platform_impl::virtual_key::NUM7,
    Num8 = platform_impl::virtual_key::NUM8,
    Num9 = platform_impl::virtual_key::NUM9,

    Numpad0 = platform_impl::virtual_key::NUMPAD0,
    Numpad1 = platform_impl::virtual_key::NUMPAD1,
    Numpad2 = platform_impl::virtual_key::NUMPAD2,
    Numpad3 = platform_impl::virtual_key::NUMPAD3,
    Numpad4 = platform_impl::virtual_key::NUMPAD4,
    Numpad5 = platform_impl::virtual_key::NUMPAD5,
    Numpad6 = platform_impl::virtual_key::NUMPAD6,
    Numpad7 = platform_impl::virtual_key::NUMPAD7,
    Numpad8 = platform_impl::virtual_key::NUMPAD8,
    Numpad9 = platform_impl::virtual_key::NUMPAD9,
    NumpadMultiply = platform_impl::virtual_key::NUMPAD_MULTIPLY,
    NumpadAdd = platform_impl::virtual_key::NUMPAD_ADD,
    NumpadSeparator = platform_impl::virtual_key::NUMPAD_SEPARATOR,
    NumpadSubtract = platform_impl::virtual_key::NUMPAD_SUBTRACT,
    NumpadDecimal = platform_impl::virtual_key::NUMPAD_DECIMAL,
    NumpadDivide = platform_impl::virtual_key::NUMPAD_DIVIDE,

    Backspace = platform_impl::virtual_key::BACKSPACE,
    Tab = platform_impl::virtual_key::TAB,
    Enter = platform_impl::virtual_key::ENTER,

    F1 = platform_impl::virtual_key::F1,
    F2 = platform_impl::virtual_key::F2,
    F3 = platform_impl::virtual_key::F3,
    F4 = platform_impl::virtual_key::F4,
    F5 = platform_impl::virtual_key::F5,
    F6 = platform_impl::virtual_key::F6,
    F7 = platform_impl::virtual_key::F7,
    F8 = platform_impl::virtual_key::F8,
    F9 = platform_impl::virtual_key::F9,
    F10 = platform_impl::virtual_key::F10,
    F11 = platform_impl::virtual_key::F11,
    F12 = platform_impl::virtual_key::F12,
    F13 = platform_impl::virtual_key::F13,
    F14 = platform_impl::virtual_key::F14,
    F15 = platform_impl::virtual_key::F15,
    F16 = platform_impl::virtual_key::F16,
    F17 = platform_impl::virtual_key::F17,
    F18 = platform_impl::virtual_key::F18,
    F19 = platform_impl::virtual_key::F19,
    F20 = platform_impl::virtual_key::F20,
    F21 = platform_impl::virtual_key::F21,
    F22 = platform_impl::virtual_key::F22,
    F23 = platform_impl::virtual_key::F23,
    F24 = platform_impl::virtual_key::F24,

    Shift = platform_impl::virtual_key::SHIFT,
    Control = platform_impl::virtual_key::CONTROL,
    Alt = platform_impl::virtual_key::ALT,

    LeftShift = platform_impl::virtual_key::LEFT_SHIFT,
    RightShift = platform_impl::virtual_key::RIGHT_SHIFT,
    LeftControl = platform_impl::virtual_key::LEFT_CONTROL,
    RightControl = platform_impl::virtual_key::RIGHT_CONTROL,
    LeftAlt = platform_impl::virtual_key::LEFT_ALT,
    RightAlt = platform_impl::virtual_key::RIGHT_ALT,
    LeftSuper = platform_impl::virtual_key::LEFT_SUPER,
    RightSuper = platform_impl::virtual_key::RIGHT_SUPER,

    Menu = platform_impl::virtual_key::MENU,

    CapsLock = platform_impl::virtual_key::CAPS_LOCK,
    NumLock = platform_impl::virtual_key::NUM_LOCK,
    ScrollLock = platform_impl::virtual_key::SCROLL_LOCK,

    Pause = platform_impl::virtual_key::PAUSE,
    Escape = platform_impl::virtual_key::ESCAPE,
    Space = platform_impl::virtual_key::SPACE,

    Insert = platform_impl::virtual_key::INSERT,
    Delete = platform_impl::virtual_key::DELETE,
    PageUp = platform_impl::virtual_key::PAGE_UP,
    PageDown = platform_impl::virtual_key::PAGE_DOWN,
    End = platform_impl::virtual_key::END,
    Home = platform_impl::virtual_key::HOME,

    Left = platform_impl::virtual_key::LEFT,
    Up = platform_impl::virtual_key::UP,
    Right = platform_impl::virtual_key::RIGHT,
    Down = platform_impl::virtual_key::DOWN,

    Clear = platform_impl::virtual_key::CLEAR,
    Select = platform_impl::virtual_key::SELECT,
    Print = platform_impl::virtual_key::PRINT,
    Execute = platform_impl::virtual_key::EXECUTE,
    PrintScreen = platform_impl::virtual_key::PRINT_SCREEN,
    Help = platform_impl::virtual_key::HELP,

    Colon = platform_impl::virtual_key::COLON,
    Plus = platform_impl::virtual_key::PLUS,
    Comma = platform_impl::virtual_key::COMMA,
    Minus = platform_impl::virtual_key::MINUS,
    Period = platform_impl::virtual_key::PERIOD,
    Slash = platform_impl::virtual_key::SLASH,
    Tilde = platform_impl::virtual_key::TILDE,
    LeftBracket = platform_impl::virtual_key::LEFT_BRACKET,
    Backslash = platform_impl::virtual_key::BACKSLASH,
    RightBracket = platform_impl::virtual_key::RIGHT_BRACKET,
    Quote = platform_impl::virtual_key::QUOTE,

    VolumeMute = platform_impl::virtual_key::VOLUME_MUTE,
    VolumeDown = platform_impl::virtual_key::VOLUME_DOWN,
    VolumeUp = platform_impl::virtual_key::VOLUME_UP,

    MediaNextTrack = platform_impl::virtual_key::MEDIA_NEXT_TRACK,
    MediaPreviousTrack = platform_impl::virtual_key::MEDIA_PREV_TRACK,
    MediaStop = platform_impl::virtual_key::MEDIA_STOP,
    MediaPlayPause = platform_impl::virtual_key::MEDIA_PLAY_PAUSE,
    // Sleep = platform_impl::virtual_key::SLEEP,
    // BrowserBack,
    // BrowserForward ,
    // BrowserRefresh,
    // BrowserStop,
    // BrowserSearch,
    // BrowserFavorites,
    // BrowserHome,

    // LaunchMail,
    // LaunchMediaSelect,
    // LaunchApp1,
    // LaunchApp2,
}

impl VirtualKey {
    pub(crate) const unsafe fn from_raw(value: VirtualKeyT) -> Self {
        std::mem::transmute(value)
    }
}
