use pyo3::prelude::*;
use tantivy as tv;

use crate::filters::TokenFilter;

use super::language::LanguageFilter;

#[pyclass]
#[derive(Clone)]
pub(crate) struct Stemmer {
    pub(crate) inner: tv::tokenizer::Stemmer,
    language: LanguageFilter,
}

#[pymethods]
impl Stemmer {
    #[new]
    fn new(language: LanguageFilter) -> Self {
        let tlang = language.clone().as_tantivy();
        Stemmer {
            inner: tv::tokenizer::Stemmer::new(tlang),
            language,
        }
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "Stemmer(language={:?})",
            self.language.clone().as_iso6391()
        ))
    }

    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok(self.language.clone().as_name().to_string())
    }

    #[getter]
    fn iso6391(&self) -> PyResult<String> {
        Ok(self.language.clone().as_iso6391().to_string())
    }
}

#[pyclass(extends=TokenFilter)]
pub(crate) struct StemmerFilter {
    #[allow(dead_code)]
    pub(crate) inner: tv::tokenizer::Stemmer,
    stemmer: Stemmer,
}

#[pymethods]
impl StemmerFilter {
    #[new]
    fn new(stemmer: &Stemmer) -> (Self, TokenFilter) {
        (
            StemmerFilter {
                inner: stemmer.inner.clone(),
                stemmer: stemmer.clone(),
            },
            TokenFilter::default(),
        )
    }

    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok("StemmerFilter".to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "StemmerFilter(language={:?})",
            self.stemmer.name().unwrap()
        ))
    }

    #[getter]
    fn stem_name(&self) -> PyResult<String> {
        self.stemmer.clone().name()
    }

    #[getter]
    fn stem_iso6391(&self) -> PyResult<String> {
        self.stemmer.clone().iso6391()
    }

    fn set_stemmer(&mut self, stemmer: &Stemmer) -> PyResult<()> {
        self.stemmer = stemmer.clone();
        self.rebuild()
    }

    fn rebuild(&mut self) -> PyResult<()> {
        self.inner = self.stemmer.inner.clone();
        Ok(())
    }
}
