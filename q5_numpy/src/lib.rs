use pyo3::prelude::*;

#[pyfunction]
fn get_number() -> usize {
    42
}

#[pymodule]
fn numpy_lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_number, m)?)?;
    Ok(())
}
