use nannou::prelude::*;
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::exceptions::PyAttributeError;

mod core;
use crate::core::*;

struct Model {
    _window: window::Id,
}

#[pyfunction]
fn run<'a>(py: Python<'a>, py_update: &'a PyAny, py_draw: &'a PyAny) {
    set_instance(AppState::new(py, py_update, py_draw));
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window()
        .view(view)
        .event(event)
        .size(600, 600)
        .build().unwrap();
    init_app(app);
    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    instance().update();
}

fn event(_app: &App, _model: &mut Model, _event: WindowEvent) {}

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
        _ => {
            return Err(PyAttributeError::new_err(format!(
                "module 'q5' has no attribute '{}'", name
            )))
        }
    };
    Ok(value)
}

#[pyfunction]
fn scale(x: f32, y: f32) {
    let mat = Mat4::from_scale(Vec3::new(x, y, 1.0));
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
fn fill(r: u8, g: u8, b: u8) {
    instance().fill(r, g, b);
}

#[pyfunction]
fn no_fill() {
    instance().no_fill();
}

#[pyfunction]
fn stroke(r: u8, g: u8, b: u8) {
    instance().stroke(r, g, b);
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
fn background(r: u8, g: u8, b: u8) {
    let draw = get_draw();
    draw.background().color(rgb8(r, g, b));
}

#[pyfunction]
fn ellipse(x: f64, y: f64, w: f64, h: f64) {
    let draw = get_draw();
    draw.ellipse()
        .x_y(x as f32, y as f32)
        .w_h(w as f32, h as f32)
        .fill_style()
        .stroke_style();
}

#[pyfunction]
fn circle(x: f64, y: f64, r: f64) {
    ellipse(x, y, r, r);
}

#[pyfunction]
fn rect(x: f64, y: f64, w: f64, h: f64) {
    let draw = get_draw();
    draw.rect()
        .x_y(x as f32, y as f32)
        .w_h(w as f32, h as f32)
        .fill_style()
        .stroke_style();
}

#[pyfunction]
fn line(x1: f64, y1: f64, x2: f64, y2: f64) {
    let draw = get_draw();
    let p1 = vec2(x1 as f32, y1 as f32);
    let p2 = vec2(x2 as f32, y2 as f32);
    draw.line()
        .points(p1, p2)
        .path_style();
}

#[pyfunction]
fn arrow(
    x1: f64, y1: f64, x2: f64, y2: f64,
        head_length: Option<f32>, head_width: Option<f32>
    ) {
    let draw = get_draw();
    let p1 = vec2(x1 as f32, y1 as f32);
    let p2 = vec2(x2 as f32, y2 as f32);

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
fn polygon(points: &PyList) {
    let points = points.extract::<Vec<(f64, f64)>>().unwrap();
    let draw = get_draw();

    let points = points.iter().map(|p| {
        (p.0 as f32, p.1 as f32)
    });

    draw.polygon()
        .fill_style()
        .stroke_style()
        .points(points);
}

#[pymodule]
fn engine(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(__getattr__, m)?)?;
    m.add_function(wrap_pyfunction!(run, m)?)?;
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
    m.add_function(wrap_pyfunction!(background, m)?)?;
    m.add_function(wrap_pyfunction!(ellipse, m)?)?;
    m.add_function(wrap_pyfunction!(circle, m)?)?;
    m.add_function(wrap_pyfunction!(rect, m)?)?;
    m.add_function(wrap_pyfunction!(line, m)?)?;
    m.add_function(wrap_pyfunction!(arrow, m)?)?;
    m.add_function(wrap_pyfunction!(polygon, m)?)?;
    Ok(())
}
