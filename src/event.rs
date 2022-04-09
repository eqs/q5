use pyo3::prelude::*;
use nannou::event::{Key, MouseButton};


pub struct MouseEventState {
    mouse_x: f32,
    mouse_y: f32,
    pmouse_x: f32,
    pmouse_y: f32,
    mouse_button: Option<MouseButton>,
}

impl MouseEventState {
    pub fn new(mouse_x: f32, mouse_y: f32) -> Self {
        Self {
            mouse_x: mouse_x,
            mouse_y: mouse_y,
            pmouse_x: 0.0,
            pmouse_y: 0.0,
            mouse_button: None,
        }
    }

    pub fn mouse_x(&self) -> f32 {
        self.mouse_x
    }

    pub fn mouse_y(&self) -> f32 {
        self.mouse_y
    }

    pub fn pmouse_x(&self) -> f32 {
        self.pmouse_x
    }

    pub fn pmouse_y(&self) -> f32 {
        self.pmouse_y
    }

    pub fn mouse_button(&self) -> Option<MouseButton> {
        self.mouse_button
    }

    pub fn mouse_button_mut(&mut self) -> &mut Option<MouseButton> {
        &mut self.mouse_button
    }

    pub fn mouse_button_name(&self) -> &str {
        match self.mouse_button {
            Some(MouseButton::Left) => "left",
            Some(MouseButton::Right) => "right",
            Some(MouseButton::Middle) => "middle",
            _ => "",
        }
    }
}


pub struct KeyEventState {
    key: Option<Key>,
}

impl KeyEventState {
    pub fn new() -> Self {
        Self { key: None }
    }

    pub fn key(&self) -> Option<Key> {
        self.key
    }

    pub fn key_mut(&mut self) -> &mut Option<Key> {
        &mut self.key
    }

    pub fn key_code(&self) -> i32 {
        match self.key {
            Some(Key::Key0) => '0' as i32,
            Some(Key::Key1) => '1' as i32,
            Some(Key::Key2) => '2' as i32,
            Some(Key::Key3) => '3' as i32,
            Some(Key::Key4) => '4' as i32,
            Some(Key::Key5) => '5' as i32,
            Some(Key::Key6) => '6' as i32,
            Some(Key::Key7) => '7' as i32,
            Some(Key::Key8) => '8' as i32,
            Some(Key::Key9) => '9' as i32,
            Some(Key::A) => 'a' as i32,
            Some(Key::B) => 'b' as i32,
            Some(Key::C) => 'c' as i32,
            Some(Key::D) => 'd' as i32,
            Some(Key::E) => 'e' as i32,
            Some(Key::F) => 'f' as i32,
            Some(Key::G) => 'g' as i32,
            Some(Key::H) => 'h' as i32,
            Some(Key::I) => 'i' as i32,
            Some(Key::J) => 'j' as i32,
            Some(Key::K) => 'k' as i32,
            Some(Key::L) => 'l' as i32,
            Some(Key::M) => 'm' as i32,
            Some(Key::N) => 'n' as i32,
            Some(Key::O) => 'o' as i32,
            Some(Key::P) => 'p' as i32,
            Some(Key::Q) => 'q' as i32,
            Some(Key::R) => 'r' as i32,
            Some(Key::S) => 's' as i32,
            Some(Key::T) => 't' as i32,
            Some(Key::U) => 'u' as i32,
            Some(Key::V) => 'v' as i32,
            Some(Key::W) => 'w' as i32,
            Some(Key::X) => 'x' as i32,
            Some(Key::Y) => 'y' as i32,
            Some(Key::Z) => 'z' as i32,

            _ => 0,
        }
    }
}

pub fn add_event_class(m: &PyModule) -> PyResult<()> {
    // m.add_class::<MouseEventState>()?;
    // m.add_class::<KeyEventState>()?;
    Ok(())
}
