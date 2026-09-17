use pyo3::prelude::*;

#[pyfunction]
fn extract_json(markdown: &str) -> String {
    emire_core::extract_json(markdown)
}

#[pymodule]
mod emire {
    #[pymodule_export]
    use super::extract_json;
}
