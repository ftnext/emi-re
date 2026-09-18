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

#[pyfunction]
fn remove_spaces(text: &str) -> String {
    emire_core::remove_spaces(text)
}

#[pymodule]
mod emire {
    use pyo3::prelude::*;

    #[pymodule_export]
    use super::extract_json;
    #[pymodule_export]
    use super::remove_spaces;
    #[pymodule_export]
    use super::to_snake_case;

    #[pymodule]
    mod normalize {
        #[pymodule_export]
        use super::super::RemoveWhitespaceNormalizer;
    }

    #[pymodule_init]
    fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        PyModule::import(m.py(), "sys")?
            .getattr("modules")?
            .set_item("emire.normalize", m.getattr("normalize")?)?;
        Ok(())
    }
}

#[pyclass(module = "emire.normalize")]
struct RemoveWhitespaceNormalizer;

#[pymethods]
impl RemoveWhitespaceNormalizer {
    #[new]
    fn new() -> Self {
        Self
    }

    fn normalize(&self, text: &str) -> String {
        emire_core::remove_spaces(text)
    }
}
