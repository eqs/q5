use pyo3::prelude::*;

pub type Align = u32;
pub type MouseButton = u32;
pub type Key = u32;


pub const LEFT: Align = 0 as Align;
pub const RIGHT: Align = 1 as Align;
pub const TOP: Align = 2 as Align;
pub const BOTTOM: Align = 3 as Align;
pub const CENTER: Align = 4 as Align;
pub const MIDDLE: Align = 4 as Align;


pub const MOUSE_LEFT: MouseButton = 0 as MouseButton;
pub const MOUSE_RIGHT: MouseButton = 1 as MouseButton;
pub const MOUSE_MIDDLE: MouseButton = 2 as MouseButton;

pub const KEY_1: Key = nannou::event::Key::Key1 as Key;
pub const KEY_2: Key = nannou::event::Key::Key2 as Key;
pub const KEY_3: Key = nannou::event::Key::Key3 as Key;
pub const KEY_4: Key = nannou::event::Key::Key4 as Key;
pub const KEY_5: Key = nannou::event::Key::Key5 as Key;
pub const KEY_6: Key = nannou::event::Key::Key6 as Key;
pub const KEY_7: Key = nannou::event::Key::Key7 as Key;
pub const KEY_8: Key = nannou::event::Key::Key8 as Key;
pub const KEY_9: Key = nannou::event::Key::Key9 as Key;
pub const KEY_0: Key = nannou::event::Key::Key0 as Key;
pub const KEY_A: Key = nannou::event::Key::A as Key;
pub const KEY_B: Key = nannou::event::Key::B as Key;
pub const KEY_C: Key = nannou::event::Key::C as Key;
pub const KEY_D: Key = nannou::event::Key::D as Key;
pub const KEY_E: Key = nannou::event::Key::E as Key;
pub const KEY_F: Key = nannou::event::Key::F as Key;
pub const KEY_G: Key = nannou::event::Key::G as Key;
pub const KEY_H: Key = nannou::event::Key::H as Key;
pub const KEY_I: Key = nannou::event::Key::I as Key;
pub const KEY_J: Key = nannou::event::Key::J as Key;
pub const KEY_K: Key = nannou::event::Key::K as Key;
pub const KEY_L: Key = nannou::event::Key::L as Key;
pub const KEY_M: Key = nannou::event::Key::M as Key;
pub const KEY_N: Key = nannou::event::Key::N as Key;
pub const KEY_O: Key = nannou::event::Key::O as Key;
pub const KEY_P: Key = nannou::event::Key::P as Key;
pub const KEY_Q: Key = nannou::event::Key::Q as Key;
pub const KEY_R: Key = nannou::event::Key::R as Key;
pub const KEY_S: Key = nannou::event::Key::S as Key;
pub const KEY_T: Key = nannou::event::Key::T as Key;
pub const KEY_U: Key = nannou::event::Key::U as Key;
pub const KEY_V: Key = nannou::event::Key::V as Key;
pub const KEY_W: Key = nannou::event::Key::W as Key;
pub const KEY_X: Key = nannou::event::Key::X as Key;
pub const KEY_Y: Key = nannou::event::Key::Y as Key;
pub const KEY_Z: Key = nannou::event::Key::Z as Key;
pub const KEY_ESCAPE: Key = nannou::event::Key::Escape as Key;
pub const KEY_F1: Key = nannou::event::Key::F1 as Key;
pub const KEY_F2: Key = nannou::event::Key::F2 as Key;
pub const KEY_F3: Key = nannou::event::Key::F3 as Key;
pub const KEY_F4: Key = nannou::event::Key::F4 as Key;
pub const KEY_F5: Key = nannou::event::Key::F5 as Key;
pub const KEY_F6: Key = nannou::event::Key::F6 as Key;
pub const KEY_F7: Key = nannou::event::Key::F7 as Key;
pub const KEY_F8: Key = nannou::event::Key::F8 as Key;
pub const KEY_F9: Key = nannou::event::Key::F9 as Key;
pub const KEY_F10: Key = nannou::event::Key::F10 as Key;
pub const KEY_F11: Key = nannou::event::Key::F11 as Key;
pub const KEY_F12: Key = nannou::event::Key::F12 as Key;
pub const KEY_F13: Key = nannou::event::Key::F13 as Key;
pub const KEY_F14: Key = nannou::event::Key::F14 as Key;
pub const KEY_F15: Key = nannou::event::Key::F15 as Key;
pub const KEY_F16: Key = nannou::event::Key::F16 as Key;
pub const KEY_F17: Key = nannou::event::Key::F17 as Key;
pub const KEY_F18: Key = nannou::event::Key::F18 as Key;
pub const KEY_F19: Key = nannou::event::Key::F19 as Key;
pub const KEY_F20: Key = nannou::event::Key::F20 as Key;
pub const KEY_F21: Key = nannou::event::Key::F21 as Key;
pub const KEY_F22: Key = nannou::event::Key::F22 as Key;
pub const KEY_F23: Key = nannou::event::Key::F23 as Key;
pub const KEY_F24: Key = nannou::event::Key::F24 as Key;
pub const KEY_SNAPSHOT: Key = nannou::event::Key::Snapshot as Key;
pub const KEY_SCROLL: Key = nannou::event::Key::Scroll as Key;
pub const KEY_PAUSE: Key = nannou::event::Key::Pause as Key;
pub const KEY_INSERT: Key = nannou::event::Key::Insert as Key;
pub const KEY_HOME: Key = nannou::event::Key::Home as Key;
pub const KEY_DELETE: Key = nannou::event::Key::Delete as Key;
pub const KEY_END: Key = nannou::event::Key::End as Key;
pub const KEY_PAGE_DOWN: Key = nannou::event::Key::PageDown as Key;
pub const KEY_PAGE_UP: Key = nannou::event::Key::PageUp as Key;
pub const KEY_LEFT: Key = nannou::event::Key::Left as Key;
pub const KEY_UP: Key = nannou::event::Key::Up as Key;
pub const KEY_RIGHT: Key = nannou::event::Key::Right as Key;
pub const KEY_DOWN: Key = nannou::event::Key::Down as Key;
pub const KEY_BACK: Key = nannou::event::Key::Back as Key;
pub const KEY_RETURN: Key = nannou::event::Key::Return as Key;
pub const KEY_SPACE: Key = nannou::event::Key::Space as Key;
pub const KEY_COMPOSE: Key = nannou::event::Key::Compose as Key;
pub const KEY_CARET: Key = nannou::event::Key::Caret as Key;
pub const KEY_NUMLOCK: Key = nannou::event::Key::Numlock as Key;
pub const KEY_NUMPAD_0: Key = nannou::event::Key::Numpad0 as Key;
pub const KEY_NUMPAD_1: Key = nannou::event::Key::Numpad1 as Key;
pub const KEY_NUMPAD_2: Key = nannou::event::Key::Numpad2 as Key;
pub const KEY_NUMPAD_3: Key = nannou::event::Key::Numpad3 as Key;
pub const KEY_NUMPAD_4: Key = nannou::event::Key::Numpad4 as Key;
pub const KEY_NUMPAD_5: Key = nannou::event::Key::Numpad5 as Key;
pub const KEY_NUMPAD_6: Key = nannou::event::Key::Numpad6 as Key;
pub const KEY_NUMPAD_7: Key = nannou::event::Key::Numpad7 as Key;
pub const KEY_NUMPAD_8: Key = nannou::event::Key::Numpad8 as Key;
pub const KEY_NUMPAD_9: Key = nannou::event::Key::Numpad9 as Key;
pub const KEY_NUMPAD_ADD: Key = nannou::event::Key::NumpadAdd as Key;
pub const KEY_NUMPAD_DIVIDE: Key = nannou::event::Key::NumpadDivide as Key;
pub const KEY_NUMPAD_DECIMAL: Key = nannou::event::Key::NumpadDecimal as Key;
pub const KEY_NUMPAD_COMMA: Key = nannou::event::Key::NumpadComma as Key;
pub const KEY_NUMPAD_ENTER: Key = nannou::event::Key::NumpadEnter as Key;
pub const KEY_NUMPAD_EQUALS: Key = nannou::event::Key::NumpadEquals as Key;
pub const KEY_NUMPAD_MULTIPLY: Key = nannou::event::Key::NumpadMultiply as Key;
pub const KEY_NUMPAD_SUBTRACT: Key = nannou::event::Key::NumpadSubtract as Key;
pub const KEY_ABNTC_1: Key = nannou::event::Key::AbntC1 as Key;
pub const KEY_ABNTC_2: Key = nannou::event::Key::AbntC2 as Key;
pub const KEY_APOSTROPHE: Key = nannou::event::Key::Apostrophe as Key;
pub const KEY_APPS: Key = nannou::event::Key::Apps as Key;
pub const KEY_ASTERISK: Key = nannou::event::Key::Asterisk as Key;
pub const KEY_AT: Key = nannou::event::Key::At as Key;
pub const KEY_AX: Key = nannou::event::Key::Ax as Key;
pub const KEY_BACKSLASH: Key = nannou::event::Key::Backslash as Key;
pub const KEY_CALCULATOR: Key = nannou::event::Key::Calculator as Key;
pub const KEY_CAPITAL: Key = nannou::event::Key::Capital as Key;
pub const KEY_COLON: Key = nannou::event::Key::Colon as Key;
pub const KEY_COMMA: Key = nannou::event::Key::Comma as Key;
pub const KEY_CONVERT: Key = nannou::event::Key::Convert as Key;
pub const KEY_EQUALS: Key = nannou::event::Key::Equals as Key;
pub const KEY_GRAVE: Key = nannou::event::Key::Grave as Key;
pub const KEY_KANA: Key = nannou::event::Key::Kana as Key;
pub const KEY_KANJI: Key = nannou::event::Key::Kanji as Key;
pub const KEY_LALT: Key = nannou::event::Key::LAlt as Key;
pub const KEY_LBRACKET: Key = nannou::event::Key::LBracket as Key;
pub const KEY_LCONTROL: Key = nannou::event::Key::LControl as Key;
pub const KEY_LSHIFT: Key = nannou::event::Key::LShift as Key;
pub const KEY_LWIN: Key = nannou::event::Key::LWin as Key;
pub const KEY_MAIL: Key = nannou::event::Key::Mail as Key;
pub const KEY_MEDIA_SELECT: Key = nannou::event::Key::MediaSelect as Key;
pub const KEY_MEDIA_STOP: Key = nannou::event::Key::MediaStop as Key;
pub const KEY_MINUS: Key = nannou::event::Key::Minus as Key;
pub const KEY_MUTE: Key = nannou::event::Key::Mute as Key;
pub const KEY_MY_COMPUTER: Key = nannou::event::Key::MyComputer as Key;
pub const KEY_NAVIGATE_FORWARD: Key = nannou::event::Key::NavigateForward as Key;
pub const KEY_NAVIGATE_BACKWARD: Key = nannou::event::Key::NavigateBackward as Key;
pub const KEY_NEXT_TRACK: Key = nannou::event::Key::NextTrack as Key;
pub const KEY_NO_CONVERT: Key = nannou::event::Key::NoConvert as Key;
pub const KEY_OEM102: Key = nannou::event::Key::OEM102 as Key;
pub const KEY_PERIOD: Key = nannou::event::Key::Period as Key;
pub const KEY_PLAY_PAUSE: Key = nannou::event::Key::PlayPause as Key;
pub const KEY_PLUS: Key = nannou::event::Key::Plus as Key;
pub const KEY_POWER: Key = nannou::event::Key::Power as Key;
pub const KEY_PREV_TRACK: Key = nannou::event::Key::PrevTrack as Key;
pub const KEY_RALT: Key = nannou::event::Key::RAlt as Key;
pub const KEY_RBRACKET: Key = nannou::event::Key::RBracket as Key;
pub const KEY_RCONTROL: Key = nannou::event::Key::RControl as Key;
pub const KEY_RSHIFT: Key = nannou::event::Key::RShift as Key;
pub const KEY_RWIN: Key = nannou::event::Key::RWin as Key;
pub const KEY_SEMICOLON: Key = nannou::event::Key::Semicolon as Key;
pub const KEY_SLASH: Key = nannou::event::Key::Slash as Key;
pub const KEY_SLEEP: Key = nannou::event::Key::Sleep as Key;
pub const KEY_STOP: Key = nannou::event::Key::Stop as Key;
pub const KEY_SYSRQ: Key = nannou::event::Key::Sysrq as Key;
pub const KEY_TAB: Key = nannou::event::Key::Tab as Key;
pub const KEY_UNDERLINE: Key = nannou::event::Key::Underline as Key;
pub const KEY_UNLABELED: Key = nannou::event::Key::Unlabeled as Key;
pub const KEY_VOLUME_DOWN: Key = nannou::event::Key::VolumeDown as Key;
pub const KEY_VOLUME_UP: Key = nannou::event::Key::VolumeUp as Key;
pub const KEY_WAKE: Key = nannou::event::Key::Wake as Key;
pub const KEY_WEB_BACK: Key = nannou::event::Key::WebBack as Key;
pub const KEY_WEB_FAVORITES: Key = nannou::event::Key::WebFavorites as Key;
pub const KEY_WEB_FORWARD: Key = nannou::event::Key::WebForward as Key;
pub const KEY_WEB_HOME: Key = nannou::event::Key::WebHome as Key;
pub const KEY_WEB_REFRESH: Key = nannou::event::Key::WebRefresh as Key;
pub const KEY_WEB_SEARCH: Key = nannou::event::Key::WebSearch as Key;
pub const KEY_WEB_STOP: Key = nannou::event::Key::WebStop as Key;
pub const KEY_YEN: Key = nannou::event::Key::Yen as Key;
pub const KEY_COPY: Key = nannou::event::Key::Copy as Key;
pub const KEY_PASTE: Key = nannou::event::Key::Paste as Key;
pub const KEY_CUT: Key = nannou::event::Key::Cut as Key;

pub fn add_module_constants(m: &PyModule) -> PyResult<()> {
    macro_rules! add_constant {
        ($name: ident) => {
            m.add(stringify!($name), $name)
        };
    }

    add_constant!(LEFT)?;
    add_constant!(RIGHT)?;
    add_constant!(TOP)?;
    add_constant!(BOTTOM)?;
    add_constant!(MIDDLE)?;
    add_constant!(CENTER)?;

    add_constant!(MOUSE_LEFT)?;
    add_constant!(MOUSE_RIGHT)?;
    add_constant!(MOUSE_MIDDLE)?;

    add_constant!(KEY_1)?;
    add_constant!(KEY_2)?;
    add_constant!(KEY_3)?;
    add_constant!(KEY_4)?;
    add_constant!(KEY_5)?;
    add_constant!(KEY_6)?;
    add_constant!(KEY_7)?;
    add_constant!(KEY_8)?;
    add_constant!(KEY_9)?;
    add_constant!(KEY_0)?;
    add_constant!(KEY_A)?;
    add_constant!(KEY_B)?;
    add_constant!(KEY_C)?;
    add_constant!(KEY_D)?;
    add_constant!(KEY_E)?;
    add_constant!(KEY_F)?;
    add_constant!(KEY_G)?;
    add_constant!(KEY_H)?;
    add_constant!(KEY_I)?;
    add_constant!(KEY_J)?;
    add_constant!(KEY_K)?;
    add_constant!(KEY_L)?;
    add_constant!(KEY_M)?;
    add_constant!(KEY_N)?;
    add_constant!(KEY_O)?;
    add_constant!(KEY_P)?;
    add_constant!(KEY_Q)?;
    add_constant!(KEY_R)?;
    add_constant!(KEY_S)?;
    add_constant!(KEY_T)?;
    add_constant!(KEY_U)?;
    add_constant!(KEY_V)?;
    add_constant!(KEY_W)?;
    add_constant!(KEY_X)?;
    add_constant!(KEY_Y)?;
    add_constant!(KEY_Z)?;
    add_constant!(KEY_ESCAPE)?;
    add_constant!(KEY_F1)?;
    add_constant!(KEY_F2)?;
    add_constant!(KEY_F3)?;
    add_constant!(KEY_F4)?;
    add_constant!(KEY_F5)?;
    add_constant!(KEY_F6)?;
    add_constant!(KEY_F7)?;
    add_constant!(KEY_F8)?;
    add_constant!(KEY_F9)?;
    add_constant!(KEY_F10)?;
    add_constant!(KEY_F11)?;
    add_constant!(KEY_F12)?;
    add_constant!(KEY_F13)?;
    add_constant!(KEY_F14)?;
    add_constant!(KEY_F15)?;
    add_constant!(KEY_F16)?;
    add_constant!(KEY_F17)?;
    add_constant!(KEY_F18)?;
    add_constant!(KEY_F19)?;
    add_constant!(KEY_F20)?;
    add_constant!(KEY_F21)?;
    add_constant!(KEY_F22)?;
    add_constant!(KEY_F23)?;
    add_constant!(KEY_F24)?;
    add_constant!(KEY_SNAPSHOT)?;
    add_constant!(KEY_SCROLL)?;
    add_constant!(KEY_PAUSE)?;
    add_constant!(KEY_INSERT)?;
    add_constant!(KEY_HOME)?;
    add_constant!(KEY_DELETE)?;
    add_constant!(KEY_END)?;
    add_constant!(KEY_PAGE_DOWN)?;
    add_constant!(KEY_PAGE_UP)?;
    add_constant!(KEY_LEFT)?;
    add_constant!(KEY_UP)?;
    add_constant!(KEY_RIGHT)?;
    add_constant!(KEY_DOWN)?;
    add_constant!(KEY_BACK)?;
    add_constant!(KEY_RETURN)?;
    add_constant!(KEY_SPACE)?;
    add_constant!(KEY_COMPOSE)?;
    add_constant!(KEY_CARET)?;
    add_constant!(KEY_NUMLOCK)?;
    add_constant!(KEY_NUMPAD_0)?;
    add_constant!(KEY_NUMPAD_1)?;
    add_constant!(KEY_NUMPAD_2)?;
    add_constant!(KEY_NUMPAD_3)?;
    add_constant!(KEY_NUMPAD_4)?;
    add_constant!(KEY_NUMPAD_5)?;
    add_constant!(KEY_NUMPAD_6)?;
    add_constant!(KEY_NUMPAD_7)?;
    add_constant!(KEY_NUMPAD_8)?;
    add_constant!(KEY_NUMPAD_9)?;
    add_constant!(KEY_NUMPAD_ADD)?;
    add_constant!(KEY_NUMPAD_DIVIDE)?;
    add_constant!(KEY_NUMPAD_DECIMAL)?;
    add_constant!(KEY_NUMPAD_COMMA)?;
    add_constant!(KEY_NUMPAD_ENTER)?;
    add_constant!(KEY_NUMPAD_EQUALS)?;
    add_constant!(KEY_NUMPAD_MULTIPLY)?;
    add_constant!(KEY_NUMPAD_SUBTRACT)?;
    add_constant!(KEY_ABNTC_1)?;
    add_constant!(KEY_ABNTC_2)?;
    add_constant!(KEY_APOSTROPHE)?;
    add_constant!(KEY_APPS)?;
    add_constant!(KEY_ASTERISK)?;
    add_constant!(KEY_AT)?;
    add_constant!(KEY_AX)?;
    add_constant!(KEY_BACKSLASH)?;
    add_constant!(KEY_CALCULATOR)?;
    add_constant!(KEY_CAPITAL)?;
    add_constant!(KEY_COLON)?;
    add_constant!(KEY_COMMA)?;
    add_constant!(KEY_CONVERT)?;
    add_constant!(KEY_EQUALS)?;
    add_constant!(KEY_GRAVE)?;
    add_constant!(KEY_KANA)?;
    add_constant!(KEY_KANJI)?;
    add_constant!(KEY_LALT)?;
    add_constant!(KEY_LBRACKET)?;
    add_constant!(KEY_LCONTROL)?;
    add_constant!(KEY_LSHIFT)?;
    add_constant!(KEY_LWIN)?;
    add_constant!(KEY_MAIL)?;
    add_constant!(KEY_MEDIA_SELECT)?;
    add_constant!(KEY_MEDIA_STOP)?;
    add_constant!(KEY_MINUS)?;
    add_constant!(KEY_MUTE)?;
    add_constant!(KEY_MY_COMPUTER)?;
    add_constant!(KEY_NAVIGATE_FORWARD)?;
    add_constant!(KEY_NAVIGATE_BACKWARD)?;
    add_constant!(KEY_NEXT_TRACK)?;
    add_constant!(KEY_NO_CONVERT)?;
    add_constant!(KEY_OEM102)?;
    add_constant!(KEY_PERIOD)?;
    add_constant!(KEY_PLAY_PAUSE)?;
    add_constant!(KEY_PLUS)?;
    add_constant!(KEY_POWER)?;
    add_constant!(KEY_PREV_TRACK)?;
    add_constant!(KEY_RALT)?;
    add_constant!(KEY_RBRACKET)?;
    add_constant!(KEY_RCONTROL)?;
    add_constant!(KEY_RSHIFT)?;
    add_constant!(KEY_RWIN)?;
    add_constant!(KEY_SEMICOLON)?;
    add_constant!(KEY_SLASH)?;
    add_constant!(KEY_SLEEP)?;
    add_constant!(KEY_STOP)?;
    add_constant!(KEY_SYSRQ)?;
    add_constant!(KEY_TAB)?;
    add_constant!(KEY_UNDERLINE)?;
    add_constant!(KEY_UNLABELED)?;
    add_constant!(KEY_VOLUME_DOWN)?;
    add_constant!(KEY_VOLUME_UP)?;
    add_constant!(KEY_WAKE)?;
    add_constant!(KEY_WEB_BACK)?;
    add_constant!(KEY_WEB_FAVORITES)?;
    add_constant!(KEY_WEB_FORWARD)?;
    add_constant!(KEY_WEB_HOME)?;
    add_constant!(KEY_WEB_REFRESH)?;
    add_constant!(KEY_WEB_SEARCH)?;
    add_constant!(KEY_WEB_STOP)?;
    add_constant!(KEY_YEN)?;
    add_constant!(KEY_COPY)?;
    add_constant!(KEY_PASTE)?;
    add_constant!(KEY_CUT)?;

    Ok(())
}
