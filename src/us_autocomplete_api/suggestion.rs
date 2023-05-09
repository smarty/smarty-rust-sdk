use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SuggestionListing {
    pub(crate) suggestions: Vec<Suggestion>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Suggestion {
    pub text: String,
    pub street_line: String,
    pub city: String,
    pub state: String
}