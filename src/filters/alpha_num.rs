use pyo3::prelude::*;
use tantivy as tv;

use crate::filters::TokenFilter;

#[pyclass(extends=TokenFilter)]
pub(crate) struct AlphaNumericOnlyFilter {
    #[allow(dead_code)]
    pub(crate) inner: tv::tokenizer::AlphaNumOnlyFilter,
}

#[pymethods]
impl AlphaNumericOnlyFilter {
    #[new]
    fn new() -> (Self, TokenFilter) {
        (
            AlphaNumericOnlyFilter {
                inner: tv::tokenizer::AlphaNumOnlyFilter,
            },
            TokenFilter::default(),
        )
    }

    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok("AlphaNumericOnlyFilter".to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok("AlphaNumericOnlyFilter()".to_string())
    }
}
