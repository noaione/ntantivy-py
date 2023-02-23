use pyo3::prelude::*;
use tantivy as tv;

use crate::filters::TokenFilter;

#[pyclass(extends=TokenFilter)]
pub(crate) struct AsciiFoldingFilter {
    #[allow(dead_code)]
    pub(crate) inner: tv::tokenizer::AsciiFoldingFilter,
}

#[pymethods]
impl AsciiFoldingFilter {
    #[new]
    fn new() -> (Self, TokenFilter) {
        (
            AsciiFoldingFilter {
                inner: tv::tokenizer::AsciiFoldingFilter,
            },
            TokenFilter::default(),
        )
    }

    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok("AsciiFoldingFilter".to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok("AsciiFoldingFilter()".to_string())
    }
}
