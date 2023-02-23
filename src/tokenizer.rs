#![allow(clippy::new_ret_no_self)]

use pyo3::prelude::*;
use tantivy::tokenizer::{
    FacetTokenizer as TFacetTok, NgramTokenizer as TNGramTok, RawTokenizer as TRawTok, SimpleTokenizer as TSimpleTok,
    TextAnalyzer, WhitespaceTokenizer as TWhitespaceTok, LowerCaser,
};

use crate::{filters::{self, remove_long::RemoveLongFilter}};

#[pyclass(subclass)]
#[derive(Default)]
pub(crate) struct Tokenizer {
    pub(crate) analyzer: Option<TextAnalyzer>,
}

impl Clone for Tokenizer {
    fn clone(&self) -> Self {
        match &self.analyzer {
            Some(inner) => Tokenizer {
                analyzer: Some(inner.clone()),
            },
            None => Tokenizer::default(),
        }
    }
}

#[pymethods]
impl Tokenizer {
    pub(crate) fn add_filter(
        &mut self,
        mut filter: filters::TokenFilter,
    ) -> PyResult<Tokenizer> {
        filter.rebuild();
        match &self.analyzer {
            Some(inner) => {
                self.analyzer =
                    Some(inner.clone().filter(filter.inner.unwrap()));
            }
            None => {
                return Err(pyo3::exceptions::PyRuntimeError::new_err(
                    "Tokenizer not initialized",
                ));
            }
        }
        Ok(self.clone())
    }
}

#[pyclass(extends=Tokenizer)]
pub(crate) struct RawTokenizer {
    #[allow(dead_code)]
    pub(crate) analyzer: TextAnalyzer,
}

#[pymethods]
impl RawTokenizer {
    #[new]
    fn new() -> (Self, Tokenizer) {
        (
            RawTokenizer {
                analyzer: TextAnalyzer::from(TRawTok),
            },
            Tokenizer::default(),
        )
    }
}

// Also known as "default" or SimpleTokenizer
#[pyclass(extends=Tokenizer)]
pub(crate) struct StandardTokenizer {
    #[allow(dead_code)]
    pub(crate) analyzer: TextAnalyzer,
}

#[pymethods]
impl StandardTokenizer {
    #[new]
    fn new() -> (Self, Tokenizer) {
        (
            StandardTokenizer {
                analyzer: TextAnalyzer::from(TSimpleTok),
            },
            Tokenizer::default(),
        )
    }
}

impl Default for StandardTokenizer {
    fn default() -> Self {
        let mut tokenizer = StandardTokenizer {
            analyzer: TextAnalyzer::from(TSimpleTok),
        };
        let r_long = RemoveLongFilter::new(40).0;
        
        tokenizer.analyzer = tokenizer.analyzer
            .filter(r_long.inner.clone())
            .filter(LowerCaser);
        tokenizer
    }
}

#[pyclass(extends=Tokenizer)]
pub(crate) struct WhitespaceTokenizer {
    #[allow(dead_code)]
    pub(crate) analyzer: TextAnalyzer,
}

#[pymethods]
impl WhitespaceTokenizer {
    #[new]
    fn new() -> (Self, Tokenizer) {
        (
            WhitespaceTokenizer {
                analyzer: TextAnalyzer::from(TWhitespaceTok),
            },
            Tokenizer::default(),
        )
    }
}

#[pyclass(extends=Tokenizer)]
pub(crate) struct FacetTokenizer {
    #[allow(dead_code)]
    pub(crate) analyzer: TextAnalyzer,
}

#[pymethods]
impl FacetTokenizer {
    #[new]
    fn new() -> (Self, Tokenizer) {
        (
            FacetTokenizer {
                analyzer: TextAnalyzer::from(TFacetTok),
            },
            Tokenizer::default(),
        )
    }
}

#[pyclass(extends=Tokenizer)]
pub(crate) struct NgramTokenizer {
    #[allow(dead_code)]
    pub(crate) analyzer: TextAnalyzer,
}

#[pymethods]
impl NgramTokenizer {
    #[new]
    fn new(
        min_gram: usize,
        max_gram: usize,
        prefix_only: bool,
    ) -> (Self, Tokenizer) {
        (
            NgramTokenizer {
                analyzer: TextAnalyzer::from(TNGramTok::new(
                    min_gram,
                    max_gram,
                    prefix_only,
                )),
            },
            Tokenizer::default(),
        )
    }
}
