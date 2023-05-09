use serde::Serialize;
use crate::sdk::has_param;
use crate::us_zipcode_api::candidate::ZipcodeResult;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(default)]
pub struct Lookup {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub city: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub state: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub zipcode: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub input_id: String,

    #[serde(skip_serializing)]
    pub result: ZipcodeResult
}

impl Default for Lookup {
    fn default() -> Self {
        Lookup {
            city: String::default(),
            state: String::default(),
            zipcode: String::default(),
            input_id: String::default(),
            result: ZipcodeResult::default()
        }
    }
}

impl Lookup {
    pub(crate) fn to_param_array(&self) -> Vec<(String, String)> {
        vec![
            has_param("city".to_string(), self.city.clone()),
            has_param("state".to_string(), self.state.clone()),
            has_param("zipcode".to_string(), self.zipcode.clone()),
            has_param("input_id".to_string(), self.input_id.clone()),
        ].iter()
            .filter_map(Option::clone)
            .collect::<Vec<_>>()
    }
}