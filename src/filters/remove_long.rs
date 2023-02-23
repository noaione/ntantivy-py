use pyo3::prelude::*;
use tantivy as tv;

use crate::filters::TokenFilter;

#[pyclass(extends=TokenFilter)]
pub(crate) struct RemoveLongFilter {
    #[allow(dead_code)]
    pub(crate) inner: tv::tokenizer::RemoveLongFilter,
    length_limit: usize,
}

#[pymethods]
impl RemoveLongFilter {
    #[new]
    #[pyo3(signature = (
        length_limit = 40,
    ))]
    pub fn new(length_limit: usize) -> (Self, TokenFilter) {
        (
            RemoveLongFilter {
                inner: tv::tokenizer::RemoveLongFilter::limit(length_limit),
                length_limit,
            },
            TokenFilter::default(),
        )
    }

    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok("RemoveLongFilter".to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("RemoveLongFilter(limit={})", self.length_limit))
    }

    #[getter]
    fn length_limit(&self) -> PyResult<usize> {
        Ok(self.length_limit)
    }

    #[setter]
    fn set_length_limit(&mut self, new_limit: usize) -> PyResult<()> {
        self.length_limit = new_limit;
        self.rebuild();
        Ok(())
    }

    fn rebuild(&mut self) {
        self.inner =
            tv::tokenizer::RemoveLongFilter::limit(self.length_limit.clone());
    }
}
