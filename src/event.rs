use pyo3::prelude::*;


#[pyclass(module="engine")]
pub struct MouseEventState {
    mouse_x: f32,
    mouse_y: f32,
    pmouse_x: f32,
    pmouse_y: f32,
}

#[pymethods]
impl MouseEventState {

    #[new]
    pub fn new() -> Self {
        Self {
            mouse_x: 0.0,
            mouse_y: 0.0,
            pmouse_x: 0.0,
            pmouse_y: 0.0,
        }
    }

    #[getter]
    pub fn mouse_x(&self) -> f32 {
        self.mouse_x
    }

    #[getter]
    pub fn mouse_y(&self) -> f32 {
        self.mouse_y
    }

    #[getter]
    pub fn pmouse_x(&self) -> f32 {
        self.pmouse_x
    }

    #[getter]
    pub fn pmouse_y(&self) -> f32 {
        self.pmouse_y
    }
}


#[pyclass(module="engine")]
struct KeyEventState {
    
}

pub fn add_event_class(m: &PyModule) -> PyResult<()> {
    m.add_class::<MouseEventState>()?;
    Ok(())
}
