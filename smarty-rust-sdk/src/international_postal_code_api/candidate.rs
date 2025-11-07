use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Candidate {
    pub input_id: String,
    pub administrative_area: String,
    pub sub_administrative_area: String,
    pub super_administrative_area: String,
    pub country_iso_3: String,
    pub locality: String,
    pub dependent_locality: String,
    pub dependent_locality_name: String,
    pub double_dependent_locality: String,
    #[serde(rename = "postal_code")]
    pub postal_code_short: String,
    pub postal_code_extra: String,
}
