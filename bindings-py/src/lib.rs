use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyfunction]
fn extract_json(markdown: &str) -> PyResult<&str> {
    emire_core::extract_json(markdown)
        .ok_or_else(|| PyValueError::new_err("No JSON block found in the provided markdown."))
}

#[pymodule]
mod emire {
    #[pymodule_export]
    use super::extract_json;
}
