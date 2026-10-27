use crate::international_postal_code_api::candidate::Candidate;
use crate::sdk::has_param;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Lookup {
    pub input_id: String,
    pub country: String,
    pub locality: String,
    pub administrative_area: String,
    pub postal_code: String,
    pub language: Option<Language>,
    pub max_results: u32,

    pub results: Vec<Candidate>,
}

impl Lookup {
    pub(crate) fn into_param_array(self) -> Vec<(String, String)> {
        vec![
            has_param("input_id".to_string(), self.input_id),
            has_param("country".to_string(), self.country),
            has_param("locality".to_string(), self.locality),
            has_param("administrative_area".to_string(), self.administrative_area),
            has_param("postal_code".to_string(), self.postal_code),
            self.language
                .map(|language| ("language".to_string(), language.to_string())),
            has_param("max_results".to_string(), self.max_results),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

/// The language of the returned results, sent as the `language` query parameter.
///
/// A [`Lookup`] whose `language` is `None` omits the parameter entirely.
#[derive(Clone, Debug, PartialEq)]
pub enum Language {
    /// Results are in the language of the output country whenever possible.
    Native,
    /// Results use the Latin character set.
    Latin,
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Native => write!(f, "native"),
            Language::Latin => write!(f, "latin"),
        }
    }
}
