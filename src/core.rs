use nannou::app::App;
use nannou::draw::{Draw, Drawing};
use std::mem::transmute;
use pyo3::prelude::*;

use nannou::prelude::*;
use nannou::draw::properties::*;
use nannou::draw::primitive::*;
use nannou::draw::primitive::polygon::*;

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
    fn update(&mut self);
    fn draw(&mut self);
}

pub struct AppState<'a> {
    pub py: Python<'a>,
    pub py_update: &'a PyAny,
    pub py_draw: &'a PyAny,
    drawing_style: DrawingStyle,
    transform_matrix: Mat4,
    matrix_stack: Vec<Mat4>,
}

impl<'a> AppState<'a> {
    pub fn new(py: Python<'a>, py_update: &'a PyAny, py_draw: &'a PyAny) -> Self {
        let matrix_stack = Vec::new();
        Self {
            py, py_update, py_draw,
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
        self.transform_matrix = Mat4::IDENTITY;
    }

    pub fn pop_matrix(&mut self) {
        self.transform_matrix = match self.matrix_stack.pop() {
            Some(mat) => mat,
            _ => panic!("Matrix stack is empty."),
        };
    }

    pub fn fill(&mut self, r: u8, g: u8, b: u8) {
        self.drawing_style.fill_color = PColor::Rgb8(r, g, b);
    }

    pub fn no_fill(&mut self) {
        self.drawing_style.fill_color = PColor::NoColor;
    }

    pub fn stroke(&mut self, r: u8, g: u8, b: u8) {
        self.drawing_style.stroke_color = PColor::Rgb8(r, g, b);
    }

    pub fn no_stroke(&mut self) {
        self.drawing_style.stroke_color = PColor::NoColor;
    }

    pub fn stroke_weight(&mut self, w: f32) {
        self.drawing_style.stroke_weight = w;
    }
}

impl<'a> PythonCallback for AppState<'a> {
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
    Gray(f32),
    Gray8(u8),
    Rgb(f32, f32, f32),
    Rgba(f32, f32, f32, f32),
    Rgb8(u8, u8, u8),
    Rgba8(u8, u8, u8, u8),
    NoColor,
}

pub trait ShapeStyle {
    fn fill_style(self) -> Self;
    fn stroke_style(self) -> Self;
}

impl<'a, T> ShapeStyle for Drawing<'a, T>
    where T: SetPolygon + SetStroke + SetColor<ColorScalar> + Into<Primitive>,
          Primitive: Into<Option<T>> {
    fn fill_style(self) -> Self {
        let state = instance();
        match state.drawing_style.fill_color {
            PColor::Gray8(lum) => self.color(rgb8(lum, lum, lum)),
            PColor::Rgb8(r, g, b) => self.color(rgb8(r, g, b)),
            PColor::NoColor => self.no_fill(),
            _ => {
                self.color(PLUM)
            },
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
            PColor::NoColor => self,
            _ => {
                self.stroke_color(PLUM)
                    .stroke_weight(state.drawing_style.stroke_weight)
            },
        }
    }
}
