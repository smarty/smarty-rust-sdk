use std::fmt::{Display, Formatter};
use serde::{Serialize};
use crate::us_street_api::candidate::Candidates;
use crate::sdk::{has_param, is_zero};

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(default)]
pub struct Lookup {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub street: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub street2: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub secondary: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub city: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub state: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub zipcode: String,
    #[serde(rename = "lastline")]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub last_line: String, // "lastline" in json
    #[serde(skip_serializing_if = "String::is_empty")]
    pub addressee: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub urbanization: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub input_id: String,

    #[serde(rename = "candidates")]
    #[serde(skip_serializing_if = "is_zero")]
    pub max_candidates: i64, // Default Value: 1 // candidates in json

    #[serde(rename = "match")]
    pub match_strategy: MatchStrategy, // "match" in json

    #[serde(skip_serializing)]
    pub results: Candidates
}

impl Default for Lookup {
    fn default() -> Self {
        Lookup {
            street: String::default(),
            street2: String::default(),
            secondary: String::default(),
            city: String::default(),
            state: String::default(),
            zipcode: String::default(),
            last_line: String::default(),
            addressee: String::default(),
            urbanization: String::default(),
            input_id: String::default(),
            max_candidates: 1,

            match_strategy: Default::default(),
            results: vec![]
        }
    }
}

impl Lookup {
    pub(crate) fn into_param_array(self) -> Vec<(String, String)> {
        let mut max_candidates_string = self.max_candidates.to_string();

        if self.max_candidates <= 0 {
            max_candidates_string = String::default();
        }

        if self.match_strategy == MatchStrategy::Enhanced {
            max_candidates_string = 5.to_string();
        }

        vec![
            has_param("street".to_string(), self.street),
            has_param("street2".to_string(), self.street2),
            has_param("secondary".to_string(), self.secondary),
            has_param("city".to_string(), self.city),
            has_param("state".to_string(), self.state),
            has_param("zipcode".to_string(), self.zipcode),
            has_param("lastline".to_string(), self.last_line),
            has_param("addressee".to_string(), self.addressee),
            has_param("urbanization".to_string(), self.urbanization),
            has_param("input_id".to_string(), self.input_id),
            has_param("candidates".to_string(), max_candidates_string),
            has_param("match".to_string(), self.match_strategy.to_string()),
        ].iter()
            .filter_map(Option::clone)
            .collect::<Vec<_>>()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum MatchStrategy {
    #[default]
    Strict,
    Invalid,
    Enhanced
}

impl Display for MatchStrategy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MatchStrategy::Strict => { write!(f, "strict") }
            MatchStrategy::Invalid => { write!(f, "invalid") }
            MatchStrategy::Enhanced => { write!(f, "enhanced") }
        }
    }
}