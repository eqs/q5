use nannou::prelude::*;
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::exceptions::PyAttributeError;

mod system;
mod event;
mod numpy_lib;
mod math_utils;
mod constant;

use crate::system::*;
use crate::event::add_event_class;
use crate::numpy_lib::add_numpy_functions;
use crate::math_utils::add_math_functions;
use crate::constant::add_module_constants;

struct Model {
    _window: window::Id,
}

#[pyfunction]
fn run<'a>(
        py: Python<'a>,
        py_setup: &'a PyAny,
        py_update: &'a PyAny,
        py_draw: &'a PyAny,
        py_mouse_pressed: &'a PyAny,
        py_mouse_released: &'a PyAny,
        py_mouse_moved: &'a PyAny,
        py_mouse_entered: &'a PyAny,
        py_mouse_exited: &'a PyAny,
        py_key_pressed: &'a PyAny,
        py_key_released: &'a PyAny
    ) {
    set_instance(AppState::new(
            py, py_setup, py_update, py_draw, 
            py_mouse_pressed,
            py_mouse_released,
            py_mouse_moved,
            py_mouse_entered,
            py_mouse_exited,
            py_key_pressed,
            py_key_released,
    ));
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    init_app(app);
    let _window = app.new_window()
        .view(view)
        .event(event)
        .title(instance().title)
        .size(instance().width, instance().height)
        .resizable(false)
        .build().unwrap();
    instance().setup();

    Model { _window }
}

fn update(app: &App, _model: &mut Model, _update: Update) {
    update_app(app);
    instance().update();
}

fn event(_app: &App, _model: &mut Model, event: WindowEvent) {
    instance().mouse_event(&event);
    instance().key_event(&event);
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    instance().draw();
    draw.to_frame(app, &frame).unwrap();
    instance().reset_transform();
}

#[warn(non_snake_case)]
#[pyfunction]
fn __getattr__(py: Python, name: &str) -> PyResult<PyObject> {
    let value = match name {
        "frame_count" => get_app().elapsed_frames().to_object(py),
        "width" => get_app().window_rect().w().to_object(py),
        "height" => get_app().window_rect().h().to_object(py),
        "mouse_x" => get_app().mouse.x.to_object(py),
        "mouse_y" => get_app().mouse.y.to_object(py),
        "mouse_button" => {
            let code = instance().mouse_event_state.mouse_button_code();
            match code {
                Some(v) => v.to_object(py),
                _ => py.None(),
            }
        },
        "key" => instance().key_event_state.key_code().to_object(py),
        _ => {
            return Err(PyAttributeError::new_err(format!(
                "module 'q5' has no attribute '{}'", name
            )))
        }
    };
    Ok(value)
}

#[pyfunction]
fn title(title: &str) {
    get_app().main_window().set_title(title);
}

#[pyfunction]
fn size(width: u32, height: u32) {
    get_app().main_window().set_inner_size_points(width as f32, height as f32);
}
#[pyfunction]
fn full_screen(fullscreen: bool) {
    get_app().main_window().set_fullscreen(fullscreen);
}

#[pyfunction]
fn loop_forever() {
    get_app().set_loop_mode(LoopMode::RefreshSync);
}

#[pyfunction]
fn no_loop() {
    get_app().set_loop_mode(LoopMode::NTimes { number_of_updates: 1 });
}

#[pyfunction]
fn loop_ntimes(count: usize) {
    get_app().set_loop_mode(LoopMode::NTimes { number_of_updates: count });
}

#[pyfunction]
fn loop_wait() {
    get_app().set_loop_mode(LoopMode::Wait);
}

#[pyfunction]
fn scale(x: f32, y: Option<f32>) {
    let mat = match y {
        Some(y) => Mat4::from_scale(Vec3::new(x, y, 1.0)),
        None => Mat4::from_scale(Vec3::new(x, x, 1.0)),
    };
    instance().transform(mat);
}

#[pyfunction]
fn rotate(angle: f32) {
    let mat = Mat4::from_rotation_z(angle);
    instance().transform(mat);
}

#[pyfunction]
fn translate(x: f32, y: f32) {
    let mat = Mat4::from_translation(Vec3::new(x, y, 0.0));
    instance().transform(mat);
}

#[pyfunction]
fn push_matrix() {
    instance().push_matrix();
}

#[pyfunction]
fn pop_matrix() {
    instance().pop_matrix();
}

#[pyfunction]
fn fill(r: u8, g: Option<u8>, b: Option<u8>, a: Option<u8>) {
    instance().fill(PColor::create_color(r, g, b, a));
}

#[pyfunction]
fn no_fill() {
    instance().no_fill();
}

#[pyfunction]
fn stroke(r: u8, g: Option<u8>, b: Option<u8>, a: Option<u8>) {
    instance().stroke(PColor::create_color(r, g, b, a));
}

#[pyfunction]
fn no_stroke() {
    instance().no_stroke();
}

#[pyfunction]
fn stroke_weight(w: f32) {
    instance().stroke_weight(w);
}

#[pyfunction]
fn font_size(font_size: u32) {
    instance().font_size(font_size);
}

#[pyfunction]
fn text_leading(text_leading: f32) {
    instance().text_leading(text_leading);
}

#[pyfunction]
fn background(r: u8, g: Option<u8>, b: Option<u8>, a: Option<u8>) {
    let draw = get_draw();
    let color = PColor::create_color(r, g, b, a);
    match color {
        PColor::Gray8(lum) => draw.background().color(rgb8(lum, lum, lum)),
        PColor::Rgb8(r, g, b) => draw.background().color(rgb8(r, g, b)),
        PColor::Rgba8(r, g, b, a) => draw.background().color(rgba8(r, g, b, a)),
        _ => panic!("Invalid color")
    };
}

#[pyfunction]
fn ellipse(x: f32, y: f32, w: f32, h: f32) {
    let draw = get_draw();
    draw.ellipse()
        .x_y(x, y)
        .w_h(w, h)
        .fill_style()
        .stroke_style();
}

#[pyfunction]
fn circle(x: f32, y: f32, r: f32) {
    ellipse(x, y, r, r);
}

#[pyfunction]
fn rect(x: f32, y: f32, w: f32, h: f32) {
    let draw = get_draw();
    draw.rect()
        .x_y(x, y)
        .w_h(w, h)
        .fill_style()
        .stroke_style();
}

#[pyfunction]
fn line(x1: f32, y1: f32, x2: f32, y2: f32) {
    let draw = get_draw();
    let p1 = vec2(x1, y1);
    let p2 = vec2(x2, y2);
    draw.line()
        .points(p1, p2)
        .path_style();
}

#[pyfunction]
fn arrow(
    x1: f32, y1: f32, x2: f32, y2: f32,
        head_length: Option<f32>, head_width: Option<f32>
    ) {
    let draw = get_draw();
    let p1 = vec2(x1, y1);
    let p2 = vec2(x2, y2);

    let head_length = head_length.unwrap_or(
        instance().get_stroke_weight() * 4.0
    );
    let head_width = head_width.unwrap_or(
        instance().get_stroke_weight() * 2.0
    );

    draw.arrow()
        .points(p1, p2)
        .head_length(head_length)
        .head_width(head_width)
        .path_style();
}

#[pyfunction]
fn polygon_list(points: &PyList) {
    let points = points.extract::<Vec<(f32, f32)>>().unwrap();
    let draw = get_draw();
    draw.polygon()
        .fill_style()
        .stroke_style()
        .points(points.iter().map(|p| {
            (p.0, p.1)
        }));
}

#[pyfunction]
fn polyline_list(points: &PyList) {
    let points = points.extract::<Vec<(f32, f32)>>().unwrap();
    let draw = get_draw();
    draw.polyline()
        .path_style()
        .points(points.iter().map(|p| {
            (p.0, p.1)
        }));
}

#[pyfunction]
fn text(text: &str, x: f32, y: f32, w: Option<f32>, h: Option<f32>) {
    let draw = get_draw();

    match (w, h) {
        (None, None) =>
            draw.text(text)
                .text_style()
                .x_y(x, y),
        (Some(w), Some(h)) =>
            draw.text(text)
                .text_style()
                .x_y(x, y)
                .w_h(w, h),
        _ => panic!("Invalid arguments")
    };
}

#[pyfunction]
fn save_frame(file_path: &str) {
    get_app().main_window().capture_frame(file_path);
}

#[pymodule]
fn engine(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(__getattr__, m)?)?;
    m.add_function(wrap_pyfunction!(run, m)?)?;
    m.add_function(wrap_pyfunction!(loop_forever, m)?)?;
    m.add_function(wrap_pyfunction!(no_loop, m)?)?;
    m.add_function(wrap_pyfunction!(loop_ntimes, m)?)?;
    m.add_function(wrap_pyfunction!(loop_wait, m)?)?;
    m.add_function(wrap_pyfunction!(title, m)?)?;
    m.add_function(wrap_pyfunction!(size, m)?)?;
    m.add_function(wrap_pyfunction!(full_screen, m)?)?;
    m.add_function(wrap_pyfunction!(push_matrix, m)?)?;
    m.add_function(wrap_pyfunction!(pop_matrix, m)?)?;
    m.add_function(wrap_pyfunction!(scale, m)?)?;
    m.add_function(wrap_pyfunction!(rotate, m)?)?;
    m.add_function(wrap_pyfunction!(translate, m)?)?;
    m.add_function(wrap_pyfunction!(fill, m)?)?;
    m.add_function(wrap_pyfunction!(no_fill, m)?)?;
    m.add_function(wrap_pyfunction!(stroke, m)?)?;
    m.add_function(wrap_pyfunction!(no_stroke, m)?)?;
    m.add_function(wrap_pyfunction!(stroke_weight, m)?)?;
    m.add_function(wrap_pyfunction!(font_size, m)?)?;
    m.add_function(wrap_pyfunction!(text_leading, m)?)?;
    m.add_function(wrap_pyfunction!(background, m)?)?;
    m.add_function(wrap_pyfunction!(ellipse, m)?)?;
    m.add_function(wrap_pyfunction!(circle, m)?)?;
    m.add_function(wrap_pyfunction!(rect, m)?)?;
    m.add_function(wrap_pyfunction!(line, m)?)?;
    m.add_function(wrap_pyfunction!(arrow, m)?)?;
    m.add_function(wrap_pyfunction!(polygon_list, m)?)?;
    m.add_function(wrap_pyfunction!(polyline_list, m)?)?;
    m.add_function(wrap_pyfunction!(text, m)?)?;
    m.add_function(wrap_pyfunction!(save_frame, m)?)?;

    add_event_class(&m)?;
    add_numpy_functions(&m)?;
    add_module_constants(&m)?;
    add_math_functions(&m)?;

    Ok(())
}
