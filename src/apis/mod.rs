// THIS FILE IS GENERATED BY api-generator, DO NOT EDIT DIRECTLY!
//
use pyo3::prelude::*;
mod r#storage;
pub(super) fn create_module(py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "apis")?;
    m.add_submodule(r#storage::create_module(py)?)?;
    Ok(m)
}
