use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SuggestionListing {
    #[serde(rename = "candidates")]
    pub suggestions: Vec<Suggestion>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Suggestion {
    pub street: String,
    pub locality: String,
    pub administrative_area: String,
    pub administrative_area_short: String,
    pub administrative_area_long: String,
    pub postal_code: String,
    pub country_iso3: String,

    pub entries: i32,
    pub address_text: String,
    pub address_id: String,
}
