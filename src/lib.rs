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
fn run<'a>(
        py: Python<'a>,
        py_setup: &'a PyAny, py_update: &'a PyAny, py_draw: &'a PyAny
    ) {
    set_instance(AppState::new(py, py_setup, py_update, py_draw));
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    init_app(app);
    instance().setup();

    let _window = app.new_window()
        .view(view)
        .event(event)
        .size(instance().width, instance().height)
        .resizable(false)
        .build().unwrap();

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
        "width" => get_app().window_rect().w().to_object(py),
        "height" => get_app().window_rect().h().to_object(py),
        _ => {
            return Err(PyAttributeError::new_err(format!(
                "module 'q5' has no attribute '{}'", name
            )))
        }
    };
    Ok(value)
}

#[pyfunction]
fn size(width: u32, height: u32) {
    instance().size(width, height);
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
fn polygon(points: &PyList) {
    let points = points.extract::<Vec<(f32, f32)>>().unwrap();
    let draw = get_draw();
    draw.polygon()
        .fill_style()
        .stroke_style()
        .points(points.iter().map(|p| {
            (p.0, p.1)
        }));
}

#[pymodule]
fn engine(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(__getattr__, m)?)?;
    m.add_function(wrap_pyfunction!(run, m)?)?;
    m.add_function(wrap_pyfunction!(size, m)?)?;
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
