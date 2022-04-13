use pyo3::prelude::*;
use nannou::event::{Key, MouseButton};

use crate::constant;


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

    pub fn mouse_button_code(&self) -> Option<constant::MouseButton> {
        match self.mouse_button {
            Some(MouseButton::Left) => Some(constant::MOUSE_LEFT),
            Some(MouseButton::Right) => Some(constant::MOUSE_RIGHT),
            Some(MouseButton::Middle) => Some(constant::MOUSE_MIDDLE),
            _ => None,
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
            Some(key) => key as i32,
            _ => 0,
        }
    }
}

pub fn add_event_class(m: &PyModule) -> PyResult<()> {
    Ok(())
}
