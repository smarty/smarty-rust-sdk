use serde::{Deserialize, Serialize};
use crate::candidate::{Candidate, Candidates};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Lookup {
    pub street: String,
    pub street2: String,
    pub secondary: String,
    pub city: String,
    pub state: String,
    pub zipcode: String,
    #[serde(rename = "lastline")]
    pub last_line: String,
    pub adressee: String,
    pub urbanization: String,
    pub input_id: String,
    #[serde(rename = "candidates")]
    pub max_candidates: i64, // Default Value: 1

    #[serde(rename = "match")]
    pub match_strategy: MatchStrategy,

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
            adressee: String::default(),
            urbanization: String::default(),
            input_id: String::default(),
            max_candidates: 1,

            match_strategy: Default::default(),
            results: vec![]
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MatchStrategy {
    #[default]
    Strict,
    Invalid,
    Enhanced
}