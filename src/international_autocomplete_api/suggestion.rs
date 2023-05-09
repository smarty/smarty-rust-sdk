use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SuggestionListing {
    #[serde(rename = "candidates")]
    pub suggestions: Vec<Suggestion>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Suggestion {
    pub street: String,
    pub locality: String,
    pub administrative_area: String,
    pub postal_code: String,
    pub country_iso3: String
}