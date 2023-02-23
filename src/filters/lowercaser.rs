use pyo3::prelude::*;
use tantivy as tv;

use crate::filters::TokenFilter;

#[pyclass(extends=TokenFilter)]
pub(crate) struct LowerCaserFilter {
    #[allow(dead_code)]
    pub(crate) inner: tv::tokenizer::LowerCaser,
}

#[pymethods]
impl LowerCaserFilter {
    #[new]
    fn new() -> (Self, TokenFilter) {
        (
            LowerCaserFilter {
                inner: tv::tokenizer::LowerCaser,
            },
            TokenFilter::default(),
        )
    }

    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok("LowerCaserFilter".to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok("LowerCaserFilter()".to_string())
    }
}
