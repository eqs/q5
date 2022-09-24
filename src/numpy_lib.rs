use pyo3::prelude::*;
use numpy::*;
use nannou::image;

use crate::*;


pub fn load_image_from_numpy(image: &PyArray<f64, Ix3>) -> image::DynamicImage {
    let shape = image.shape();
    image::DynamicImage::ImageRgba8(image::ImageBuffer::from_fn(shape[0] as u32, shape[1] as u32, |x, y| {
        return image::Rgba([(x as f32 / 255.0) as u8, (y as f32 / 255.0) as u8, 0, std::u8::MAX])
    }))
}

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
