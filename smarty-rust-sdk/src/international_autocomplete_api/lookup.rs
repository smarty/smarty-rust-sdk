use crate::international_autocomplete_api::suggestion::SuggestionListing;
use crate::sdk::has_param;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub struct Lookup {
    pub country: String,
    pub search: String,
    pub address_id: String,
    pub max_results: i32,
    pub max_group_results: i32,
    pub geolocation: bool,
    pub include_only_locality: String,
    pub include_only_postal_code: String,
    pub language: Option<Language>,
    pub results: SuggestionListing,
}

impl Default for Lookup {
    fn default() -> Self {
        Lookup {
            country: String::default(),
            search: String::default(),
            address_id: String::default(),
            max_results: 5,
            max_group_results: 100,
            geolocation: false,
            include_only_locality: "".to_string(),
            include_only_postal_code: "".to_string(),
            language: None,

            results: SuggestionListing {
                suggestions: vec![],
            },
        }
    }
}

impl Lookup {
    pub(crate) fn into_param_array(self) -> Vec<(String, String)> {
        [
            has_param("country".to_string(), self.country),
            has_param("search".to_string(), self.search),
            has_param("address_id".to_string(), self.address_id),
            has_param("max_results".to_string(), self.max_results.to_string()),
            has_param(
                "max_group_results".to_string(),
                self.max_group_results.to_string(),
            ),
            if self.geolocation {
                Some(("geolocation".to_string(), "on".to_string()))
            } else {
                None
            },
            has_param(
                "include_only_locality".to_string(),
                self.include_only_locality,
            ),
            has_param(
                "include_only_postal_code".to_string(),
                self.include_only_postal_code,
            ),
            self.language
                .map(|language| ("language".to_string(), language.to_string())),
        ]
        .iter()
        .filter_map(Option::clone)
        .collect::<Vec<_>>()
    }
}

/// The language of the returned suggestions, sent as the `language` query parameter.
///
/// A [`Lookup`] whose `language` is `None` omits the parameter entirely, and the output
/// language matches the default for the country.
#[derive(Clone, Debug, PartialEq)]
pub enum Language {
    /// Results are in the language of the output country whenever possible.
    ///
    /// Required to get French diacritics in Canada.
    Native,
    /// Results use the Latin character set, with accents and other diacritics removed.
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
