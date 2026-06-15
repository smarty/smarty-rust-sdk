use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SuggestionListing {
    pub suggestions: Vec<Suggestion>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Suggestion {
    pub smarty_key: String,
    pub entry_id: String,
    pub street_line: String,
    pub secondary: String,
    pub city: String,
    pub state: String,
    pub zipcode: String,
    pub entries: i32,
}
