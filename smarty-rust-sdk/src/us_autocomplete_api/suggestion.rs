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
    pub urbanization: String,
    pub street_line: String,
    pub secondary: String,
    pub city: String,
    pub state: String,
    pub zipcode: String,
    pub entries: i32,
    pub source: String,
}

#[cfg(test)]
mod tests {
    use super::Suggestion;

    #[test]
    fn urbanization_deserializes_when_present() {
        let suggestion: Suggestion = serde_json::from_str(r#"{"urbanization":"urb"}"#).unwrap();
        assert_eq!(suggestion.urbanization, "urb");
    }

    #[test]
    fn urbanization_defaults_when_absent() {
        let suggestion: Suggestion = serde_json::from_str(r#"{}"#).unwrap();
        assert_eq!(suggestion.urbanization, "");
    }
}
