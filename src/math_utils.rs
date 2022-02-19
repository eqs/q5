use pyo3::prelude::*;

#[pyfunction]
fn some_func() -> u32{
    42
}

pub fn add_math_functions(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(some_func, m)?)?;
    Ok(())
}
