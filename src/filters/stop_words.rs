use pyo3::prelude::*;
use tantivy as tv;

use crate::filters::TokenFilter;

use super::language::LanguageFilter;

#[pyclass(extends=TokenFilter)]
pub(crate) struct StopWordsFilter {
    #[allow(dead_code)]
    pub(crate) inner: tv::tokenizer::StopWordFilter,
    language: LanguageFilter,
}

#[pymethods]
impl StopWordsFilter {
    #[new]
    fn new(language: LanguageFilter) -> PyResult<(Self, TokenFilter)> {
        let tlang = language.clone().as_tantivy();
        if let Some(inner) = tv::tokenizer::StopWordFilter::new(tlang) {
            Ok((StopWordsFilter { inner, language }, TokenFilter::default()))
        } else {
            Err(pyo3::exceptions::PyValueError::new_err(
                "Invalid language provided for stop words filter.",
            ))
        }
    }

    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok("StopWordsFilter".to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("StopWordsFilter(language={:?})", self.language))
    }

    #[getter]
    fn language(&self) -> PyResult<LanguageFilter> {
        Ok(self.language.clone())
    }

    fn set_language(&mut self, language: LanguageFilter) -> PyResult<()> {
        self.language = language;
        self.rebuild()
    }

    fn rebuild(&mut self) -> PyResult<()> {
        let tlang = self.language.clone().as_tantivy();
        if let Some(inner) = tv::tokenizer::StopWordFilter::new(tlang) {
            self.inner = inner;
            Ok(())
        } else {
            Err(pyo3::exceptions::PyValueError::new_err(
                "Invalid language provided for stop words filter.",
            ))
        }
    }
}
