use pyo3::prelude::*;
use numpy::{IntoPyArray, PyArrayDyn};

#[pyfunction]
fn some_func() -> u32{
    42
}

#[pyfunction]
fn add_numpy(py: Python, x: &PyArrayDyn<f64>, y: &PyArrayDyn<f64>) -> PyResult<Py<PyArrayDyn<f64>>> {
    unsafe{
        let x = x.as_array();
        let y = y.as_array();
        Ok((&x + &y).into_pyarray(py).to_owned())
    }
}

pub fn add_math_functions(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(some_func, m)?)?;
    m.add_function(wrap_pyfunction!(add_numpy, m)?)?;
    Ok(())
}
