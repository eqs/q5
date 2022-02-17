use pyo3::prelude::*;
use pyo3::wrap_pyfunction;


#[pyfunction]
fn rust_func() -> usize {
    14
}

#[pymodule]
fn engine(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(rust_func))?;
    Ok(())
}
