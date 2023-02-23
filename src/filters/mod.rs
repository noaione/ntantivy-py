use pyo3::prelude::*;
use tantivy as tv;

pub mod alpha_num;
pub mod ascii_fold;
pub mod language;
pub mod lowercaser;
pub mod remove_long;
pub mod split_compound;
pub mod stemmer;
pub mod stop_words;

// Reimplementation of the TokenStream/TokenFilter
// for the python bindings
// This needs to be able to be used by tantivy
// and also be imported by the python bindings

#[pyclass(subclass)]
#[derive(Default)]
pub(crate) struct TokenFilter {
    pub(crate) inner: Option<tv::tokenizer::BoxTokenFilter>,
}

impl Clone for TokenFilter {
    fn clone(&self) -> Self {
        match &self.inner {
            Some(inner) => TokenFilter {
                inner: Some(inner.box_clone()),
            },
            None => TokenFilter::default(),
        }
    }
}

#[pymethods]
impl TokenFilter {
    #[getter]
    fn name(&self) -> PyResult<()> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "Not implemented",
        ))
    }

    pub fn rebuild(&mut self) {
        // This is a no-op for most filters
    }
}
