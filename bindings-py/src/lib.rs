use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyfunction]
fn extract_json(markdown: &str) -> PyResult<&str> {
    emire_core::extract_json(markdown)
        .ok_or_else(|| PyValueError::new_err("No JSON block found in the provided markdown."))
}

#[pyfunction]
fn to_snake_case(camel_case_str: &str) -> String {
    emire_core::to_snake_case(camel_case_str)
}

#[pymodule]
mod emire {
    #[pymodule_export]
    use super::extract_json;
    #[pymodule_export]
    use super::to_snake_case;
}
