use pyo3::prelude::*;
use tantivy::tokenizer::Language;

#[pyclass]
#[derive(Clone, Debug)]
pub(crate) enum LanguageFilter {
    Arabic,
    Danish,
    Dutch,
    English,
    Finnish,
    French,
    German,
    Greek,
    Hungarian,
    Italian,
    Norwegian,
    Portuguese,
    Romanian,
    Russian,
    Spanish,
    Swedish,
    Tamil,
    Turkish,
}

impl LanguageFilter {
    pub fn as_name(self) -> &'static str {
        use self::LanguageFilter::*;
        match self {
            Arabic => "Arabic",
            Danish => "Danish",
            Dutch => "Dutch",
            English => "English",
            Finnish => "Finnish",
            French => "French",
            German => "German",
            Greek => "Greek",
            Hungarian => "Hungarian",
            Italian => "Italian",
            Norwegian => "Norwegian",
            Portuguese => "Portuguese",
            Romanian => "Romanian",
            Russian => "Russian",
            Spanish => "Spanish",
            Swedish => "Swedish",
            Tamil => "Tamil",
            Turkish => "Turkish",
        }
    }

    pub fn as_iso6391(self) -> &'static str {
        use self::LanguageFilter::*;
        match self {
            Arabic => "ar",
            Danish => "da",
            Dutch => "nl",
            English => "en",
            Finnish => "fi",
            French => "fr",
            German => "de",
            Greek => "gr",
            Hungarian => "hu",
            Italian => "it",
            Norwegian => "no",
            Portuguese => "pt",
            Romanian => "ro",
            Russian => "ru",
            Spanish => "es",
            Swedish => "se",
            Tamil => "ta",
            Turkish => "tr",
        }
    }

    pub fn as_tantivy(self) -> Language {
        use self::LanguageFilter::*;
        match self {
            Arabic => Language::Arabic,
            Danish => Language::Danish,
            Dutch => Language::Dutch,
            English => Language::English,
            Finnish => Language::Finnish,
            French => Language::French,
            German => Language::German,
            Greek => Language::Greek,
            Hungarian => Language::Hungarian,
            Italian => Language::Italian,
            Norwegian => Language::Norwegian,
            Portuguese => Language::Portuguese,
            Romanian => Language::Romanian,
            Russian => Language::Russian,
            Spanish => Language::Spanish,
            Swedish => Language::Swedish,
            Tamil => Language::Tamil,
            Turkish => Language::Turkish,
        }
    }
}
