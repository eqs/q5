use pyo3::prelude::*;
use numpy::*;
use crate::*;

#[pyfunction]
fn polygon_ndarray(points: &PyArray<f64, Ix2>) {
    unsafe {
        let points = points.as_array();
        let draw = get_draw();
        draw.polygon()
            .fill_style()
            .stroke_style()
            .points(points.outer_iter().map(|p| {
                pt2(p[[0]] as f32, p[[1]] as f32)
            }));
    }
}

#[pyfunction]
fn polyline_ndarray(points: &PyArray<f64, Ix2>) {
    unsafe {
        let points = points.as_array();
        let draw = get_draw();
        draw.polyline()
            .path_style()
            .points(points.outer_iter().map(|p| {
                pt2(p[[0]] as f32, p[[1]] as f32)
            }));
    }
}

pub fn add_numpy_functions(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(polygon_ndarray, m)?)?;
    m.add_function(wrap_pyfunction!(polyline_ndarray, m)?)?;
    Ok(())
}
