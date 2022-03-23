use nannou::app::App;
use nannou::draw::{Draw, Drawing};
use std::mem::transmute;
use pyo3::prelude::*;

use nannou::prelude::*;
use nannou::draw::properties::*;
use nannou::draw::primitive::*;
use nannou::draw::primitive::polygon::*;

use crate::event::*;

static mut INSTANCE: *mut AppState = 0 as *mut AppState;
static mut APP_INSTANCE: *mut App = 0 as *mut App;
static mut DRAW_INSTANCE: *mut Draw = 0 as *mut Draw;

pub fn instance_exists() -> bool {
    unsafe { INSTANCE != 0 as *mut AppState }
}

pub fn app_initialized() -> bool {
    unsafe { APP_INSTANCE != 0 as *mut App }
}

pub fn instance() -> &'static mut AppState<'static> {
    if instance_exists() {
        unsafe { &mut *INSTANCE }
    } else {
        panic!("Python instance is not found");
    }
}

pub fn set_instance(py_callback: AppState) {
    unsafe {
        INSTANCE = transmute(Box::new(py_callback));
    }
}

pub fn init_app(app: &App) {
    unsafe {
        APP_INSTANCE = transmute(app);
        DRAW_INSTANCE = transmute(Box::new(app.draw()));
    }
}

pub fn get_app() -> &'static mut App {
    if app_initialized() {
        unsafe { &mut *APP_INSTANCE }
    } else {
        panic!("App instance is not found");
    }
}

pub fn get_draw() -> Draw {
    if app_initialized() {
        unsafe {
            let state = instance();
            let draw = &mut *DRAW_INSTANCE;
            draw.transform(state.transform_matrix)
        }
    } else {
        panic!("Draw instance is not found");
    }
}

pub trait PythonCallback {
    fn setup(&mut self);
    fn update(&mut self);
    fn draw(&mut self);
    fn mouse_event(&mut self, event: &WindowEvent);
    fn key_event(&mut self, event: &WindowEvent);
}

pub struct AppState<'a> {
    pub py: Python<'a>,
    pub py_setup: &'a PyAny,
    pub py_update: &'a PyAny,
    pub py_draw: &'a PyAny,
    pub py_mouse_event: &'a PyAny,
    pub py_key_event: &'a PyAny,
    pub width: u32,
    pub height: u32,
    pub title: &'a str,
    drawing_style: DrawingStyle,
    transform_matrix: Mat4,
    matrix_stack: Vec<Mat4>,
}

impl<'a> AppState<'a> {
    pub fn new(
        py: Python<'a>, py_setup: &'a PyAny, py_update: &'a PyAny, py_draw: &'a PyAny,
        py_mouse_event: &'a PyAny, py_key_event: &'a PyAny
    ) -> Self {
        let matrix_stack = Vec::new();
        Self {
            py, py_setup, py_update, py_draw, py_mouse_event, py_key_event,
            width: 800,
            height: 800,
            title: "q5",
            drawing_style: DrawingStyle::new(),
            transform_matrix: Mat4::IDENTITY,
            matrix_stack,
        }
    }

    pub fn transform(&mut self, transform_matrix: Mat4) {
        self.transform_matrix = self.transform_matrix * transform_matrix;
    }

    pub fn reset_transform(&mut self) {
        self.transform_matrix = Mat4::IDENTITY;
    }

    pub fn push_matrix(&mut self) {
        self.matrix_stack.push(self.transform_matrix);
    }

    pub fn pop_matrix(&mut self) {
        self.transform_matrix = match self.matrix_stack.pop() {
            Some(mat) => mat,
            _ => panic!("Matrix stack is empty."),
        };
    }

    pub fn fill(&mut self, color: PColor) {
        self.drawing_style.fill_color = color;
    }

    pub fn no_fill(&mut self) {
        self.drawing_style.fill_color = PColor::NoColor;
    }

    pub fn stroke(&mut self, color: PColor) {
        self.drawing_style.stroke_color = color;
    }

    pub fn no_stroke(&mut self) {
        self.drawing_style.stroke_color = PColor::NoColor;
    }

    pub fn stroke_weight(&mut self, w: f32) {
        self.drawing_style.stroke_weight = w;
    }

    pub fn get_stroke_weight(&self) -> f32 {
        self.drawing_style.stroke_weight
    }
}

impl<'a> PythonCallback for AppState<'a> {
    fn setup(&mut self) {
        if let Err(err) = self.py_setup.call0() {
            err.print(self.py);
        }
    }

    fn update(&mut self) {
        if let Err(err) = self.py_update.call0() {
            err.print(self.py);
        }
    }

    fn draw(&mut self) {
        if let Err(err) = self.py_draw.call0() {
            err.print(self.py);
        }
    }

    fn mouse_event(&mut self, event: &WindowEvent) {
        let app = get_app();
        let state = MouseEventState::new(
            app.mouse.x, app.mouse.y,
        );
        if let Err(err) = self.py_mouse_event.call1((state,)) {
            err.print(self.py);
        }
    }

    fn key_event(&mut self, event: &WindowEvent) {
        if let Err(err) = self.py_key_event.call1((42,)) {
            err.print(self.py);
        }
    }
}

pub struct DrawingStyle {
    pub stroke_color: PColor,
    pub fill_color: PColor,
    pub stroke_weight: f32,
}

impl DrawingStyle {
    pub fn new() -> DrawingStyle {
        DrawingStyle {
            stroke_color: PColor::Gray8(0),
            fill_color: PColor::Gray8(255),
            stroke_weight: 1.0,
        }
    }
}

pub enum PColor {
    Gray8(u8),
    Rgb8(u8, u8, u8),
    Rgba8(u8, u8, u8, u8),
    NoColor,
}

impl PColor {
    pub fn create_color(r: u8, x: Option<u8>, y: Option<u8>, z: Option<u8>) -> Self {
        match (x, y, z) {
            (None, None, None) => PColor::Gray8(r),
            (Some(a), None, None) => PColor::Rgba8(r, r, r, a),
            (Some(g), Some(b), None) => PColor::Rgb8(r, g, b),
            (Some(g), Some(b), Some(a)) => PColor::Rgba8(r, g, b, a),
            _ => panic!("Invalid color")
        }
    }
}

pub trait ShapeStyle {
    fn fill_style(self) -> Self;
    fn stroke_style(self) -> Self;
}

pub trait PathStyle {
    fn path_style(self) -> Self;
}

impl<'a, T> ShapeStyle for Drawing<'a, T>
    where T: SetPolygon + SetStroke + SetColor<ColorScalar> + Into<Primitive>,
          Primitive: Into<Option<T>> {
    fn fill_style(self) -> Self {
        let state = instance();
        match state.drawing_style.fill_color {
            PColor::Gray8(lum) => self.color(rgb8(lum, lum, lum)),
            PColor::Rgb8(r, g, b) => self.color(rgb8(r, g, b)),
            PColor::Rgba8(r, g, b, a) => self.color(rgba8(r, g, b, a)),
            PColor::NoColor => self.no_fill(),
        }
    }

    fn stroke_style(self) -> Self {
        let state = instance();
        match state.drawing_style.stroke_color {
            PColor::Gray8(lum) => {
                self.stroke_color(rgb8(lum, lum, lum))
                    .stroke_weight(state.drawing_style.stroke_weight)
            },
            PColor::Rgb8(r, g, b) => {
                self.stroke_color(rgb8(r, g, b))
                    .stroke_weight(state.drawing_style.stroke_weight)
            },
            PColor::Rgba8(r, g, b, a) => {
                self.stroke_color(rgba8(r, g, b, a))
                    .stroke_weight(state.drawing_style.stroke_weight)
            },
            PColor::NoColor => self,
        }
    }
}

impl<'a, T> PathStyle for Drawing<'a, T>
    where T: SetStroke + SetColor<f32> + Into<Primitive>,
          Primitive: Into<Option<T>> {

    fn path_style(self) -> Self {
        let state = instance();
        match state.drawing_style.stroke_color {
            PColor::Gray8(lum) => {
                self.rgb8(lum, lum, lum)
                    .stroke_weight(state.drawing_style.stroke_weight)
            },
            PColor::Rgb8(r, g, b) => {
                self.rgb8(r, g, b)
                    .stroke_weight(state.drawing_style.stroke_weight)
            },
            PColor::Rgba8(r, g, b, a) => {
                self.rgba8(r, g, b, a)
                    .stroke_weight(state.drawing_style.stroke_weight)
            },
            PColor::NoColor => self,
        }
    }
}
