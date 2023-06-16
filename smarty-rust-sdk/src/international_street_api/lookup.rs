use std::fmt::{Display, Formatter};
use crate::international_street_api::candidate::{Candidate};
use crate::sdk::has_param;

#[derive(Clone)]
pub struct Lookup {
    pub input_id: String,
    pub country: String,
    pub geocode: bool,
    pub language: Language,
    pub freeform: String,
    pub address1: String,
    pub address2: String,
    pub address3: String,
    pub address4: String,
    pub organization: String,
    pub locality: String,
    pub administrative_area: String,
    pub postal_code: String,

    pub results: Vec<Candidate>
}

impl Default for Lookup {
    fn default() -> Self {
        Lookup {
            input_id: String::default(),
            country: String::default(),
            geocode: true,
            language: Language::Native,
            freeform: String::default(),
            address1: String::default(),
            address2: String::default(),
            address3: String::default(),
            address4: String::default(),
            organization: String::default(),
            locality: String::default(),
            administrative_area: String::default(),
            postal_code: String::default(),
            results: vec![]
        }
    }
}

impl Lookup {
    pub(crate) fn into_param_array(self) -> Vec<(String, String)> {
        vec![
            has_param("input_id".to_string(), self.input_id),
            has_param("country".to_string(), self.country),
            Some(("geocode".to_string(), self.geocode.to_string())),
            has_param("language".to_string(), self.language.to_string()),
            has_param("address1".to_string(), self.address1),
            has_param("address2".to_string(), self.address2),
            has_param("address3".to_string(), self.address3),
            has_param("address4".to_string(), self.address4),
            has_param("organization".to_string(), self.organization),
            has_param("locality".to_string(), self.locality),
            has_param("administrative_area".to_string(), self.administrative_area),
            has_param("postal_code".to_string(), self.postal_code)
        ].iter()
            .filter_map(Option::clone)
            .collect::<Vec<_>>()
    }
}

#[derive(Clone)]
pub enum Language {
    Native,
    Latin
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Native => write!(f, "native"),
            Language::Latin => write!(f, "latin")
        }
    }
}