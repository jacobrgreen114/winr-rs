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

use crate::input::VirtualKeyT;
use windows::Win32::UI::Input::KeyboardAndMouse::*;

const fn to_virtual_key(vk: VIRTUAL_KEY) -> VirtualKeyT {
    vk.0 as VirtualKeyT
}

macro_rules! vkey {
    ($id:ident, $val:expr) => {
        pub const $id: VirtualKeyT = to_virtual_key($val);
    };
}

vkey!(BACKSPACE, VK_BACK);
vkey!(TAB, VK_TAB);
vkey!(ENTER, VK_RETURN);

vkey!(SHIFT, VK_SHIFT);
vkey!(CONTROL, VK_CONTROL);
vkey!(ALT, VK_MENU);

vkey!(PAUSE, VK_PAUSE);
vkey!(CAPS_LOCK, VK_CAPITAL);
vkey!(ESCAPE, VK_ESCAPE);
vkey!(SPACE, VK_SPACE);
vkey!(PAGE_UP, VK_PRIOR);
vkey!(PAGE_DOWN, VK_NEXT);
vkey!(END, VK_END);
vkey!(HOME, VK_HOME);
vkey!(LEFT, VK_LEFT);
vkey!(UP, VK_UP);
vkey!(RIGHT, VK_RIGHT);
vkey!(DOWN, VK_DOWN);
vkey!(SELECT, VK_SELECT);
vkey!(PRINT, VK_PRINT);
vkey!(EXECUTE, VK_EXECUTE);
vkey!(PRINT_SCREEN, VK_SNAPSHOT);
vkey!(INSERT, VK_INSERT);
vkey!(DELETE, VK_DELETE);
vkey!(HELP, VK_HELP);
vkey!(CLEAR, VK_CLEAR);

vkey!(NUM0, VK_0);
vkey!(NUM1, VK_1);
vkey!(NUM2, VK_2);
vkey!(NUM3, VK_3);
vkey!(NUM4, VK_4);
vkey!(NUM5, VK_5);
vkey!(NUM6, VK_6);
vkey!(NUM7, VK_7);
vkey!(NUM8, VK_8);
vkey!(NUM9, VK_9);

vkey!(A, VK_A);
vkey!(B, VK_B);
vkey!(C, VK_C);
vkey!(D, VK_D);
vkey!(E, VK_E);
vkey!(F, VK_F);
vkey!(G, VK_G);
vkey!(H, VK_H);
vkey!(I, VK_I);
vkey!(J, VK_J);
vkey!(K, VK_K);
vkey!(L, VK_L);
vkey!(M, VK_M);
vkey!(N, VK_N);
vkey!(O, VK_O);
vkey!(P, VK_P);
vkey!(Q, VK_Q);
vkey!(R, VK_R);
vkey!(S, VK_S);
vkey!(T, VK_T);
vkey!(U, VK_U);
vkey!(V, VK_V);
vkey!(W, VK_W);
vkey!(X, VK_X);
vkey!(Y, VK_Y);
vkey!(Z, VK_Z);

vkey!(LEFT_SUPER, VK_LWIN);
vkey!(RIGHT_SUPER, VK_RWIN);
vkey!(MENU, VK_APPS);
vkey!(SLEEP, VK_SLEEP);

vkey!(NUMPAD0, VK_NUMPAD0);
vkey!(NUMPAD1, VK_NUMPAD1);
vkey!(NUMPAD2, VK_NUMPAD2);
vkey!(NUMPAD3, VK_NUMPAD3);
vkey!(NUMPAD4, VK_NUMPAD4);
vkey!(NUMPAD5, VK_NUMPAD5);
vkey!(NUMPAD6, VK_NUMPAD6);
vkey!(NUMPAD7, VK_NUMPAD7);
vkey!(NUMPAD8, VK_NUMPAD8);
vkey!(NUMPAD9, VK_NUMPAD9);

vkey!(NUMPAD_MULTIPLY, VK_MULTIPLY);
vkey!(NUMPAD_ADD, VK_ADD);
vkey!(NUMPAD_SEPARATOR, VK_SEPARATOR);
vkey!(NUMPAD_SUBTRACT, VK_SUBTRACT);
vkey!(NUMPAD_DECIMAL, VK_DECIMAL);
vkey!(NUMPAD_DIVIDE, VK_DIVIDE);

vkey!(F1, VK_F1);
vkey!(F2, VK_F2);
vkey!(F3, VK_F3);
vkey!(F4, VK_F4);
vkey!(F5, VK_F5);
vkey!(F6, VK_F6);
vkey!(F7, VK_F7);
vkey!(F8, VK_F8);
vkey!(F9, VK_F9);
vkey!(F10, VK_F10);
vkey!(F11, VK_F11);
vkey!(F12, VK_F12);
vkey!(F13, VK_F13);
vkey!(F14, VK_F14);
vkey!(F15, VK_F15);
vkey!(F16, VK_F16);
vkey!(F17, VK_F17);
vkey!(F18, VK_F18);
vkey!(F19, VK_F19);
vkey!(F20, VK_F20);
vkey!(F21, VK_F21);
vkey!(F22, VK_F22);
vkey!(F23, VK_F23);
vkey!(F24, VK_F24);

vkey!(NUM_LOCK, VK_NUMLOCK);
vkey!(SCROLL_LOCK, VK_SCROLL);

vkey!(LEFT_SHIFT, VK_LSHIFT);
vkey!(RIGHT_SHIFT, VK_RSHIFT);
vkey!(LEFT_CONTROL, VK_LCONTROL);
vkey!(RIGHT_CONTROL, VK_RCONTROL);
vkey!(LEFT_ALT, VK_LMENU);
vkey!(RIGHT_ALT, VK_RMENU);

vkey!(VOLUME_MUTE, VK_VOLUME_MUTE);
vkey!(VOLUME_DOWN, VK_VOLUME_DOWN);
vkey!(VOLUME_UP, VK_VOLUME_UP);

vkey!(MEDIA_NEXT_TRACK, VK_MEDIA_NEXT_TRACK);
vkey!(MEDIA_PREV_TRACK, VK_MEDIA_PREV_TRACK);
vkey!(MEDIA_STOP, VK_MEDIA_STOP);
vkey!(MEDIA_PLAY_PAUSE, VK_MEDIA_PLAY_PAUSE);

vkey!(COLON, VK_OEM_1);
vkey!(PLUS, VK_OEM_PLUS);
vkey!(COMMA, VK_OEM_COMMA);
vkey!(MINUS, VK_OEM_MINUS);
vkey!(PERIOD, VK_OEM_PERIOD);
vkey!(SLASH, VK_OEM_2);
vkey!(TILDE, VK_OEM_3);
vkey!(LEFT_BRACKET, VK_OEM_4);
vkey!(BACKSLASH, VK_OEM_5);
vkey!(RIGHT_BRACKET, VK_OEM_6);
vkey!(QUOTE, VK_OEM_7);
