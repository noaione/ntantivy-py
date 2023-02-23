#![allow(clippy::new_ret_no_self)]

use pyo3::prelude::*;
use tantivy as tv;

use crate::filters::TokenFilter;

#[pyclass(extends=TokenFilter)]
pub(crate) struct SplitCompoundWordsFilter {
    #[allow(dead_code)]
    pub(crate) inner: tv::tokenizer::SplitCompoundWords<usize>,
    dict: Vec<String>,
}

#[pymethods]
impl SplitCompoundWordsFilter {
    #[new]
    fn new(compound_words: Vec<String>) -> (Self, TokenFilter) {
        (
            SplitCompoundWordsFilter {
                inner: tv::tokenizer::SplitCompoundWords::from_dictionary(
                    compound_words.clone(),
                ),
                dict: compound_words,
            },
            TokenFilter::default(),
        )
    }

    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok("SplitCompoundWordsFilter".to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("SplitCompoundWordsFilter(words={:?})", self.dict))
    }

    #[getter]
    fn words(&self) -> PyResult<Vec<String>> {
        Ok(self.dict.clone())
    }

    fn add_word(&mut self, word: &str) {
        self.dict.push(word.to_owned());
        self.rebuild();
    }

    fn remove_word(&mut self, word: &str) {
        self.dict.retain(|w| w != word);
        self.rebuild();
    }

    fn rebuild(&mut self) {
        self.inner = tv::tokenizer::SplitCompoundWords::from_dictionary(
            self.dict.clone(),
        );
    }
}
