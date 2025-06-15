use serde::{Deserialize, Serialize};

use super::response::EnrichmentResponse;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SecondaryResponse {
    pub smarty_key: String,
    pub root_address: RootAddress,
    pub aliases: Vec<Alias>,
    pub secondaries: Vec<Secondary>,
}

impl EnrichmentResponse for SecondaryResponse {
    fn lookup_type() -> &'static str {
        "secondary"
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RootAddress {
    pub secondary_count: i64,
    pub smarty_key: String,
    pub primary_number: String,
    pub street_name: String,
    pub street_suffix: String,
    pub street_postdirection: String,
    pub city_name: String,
    pub state_abbreviation: String,
    pub zipcode: String,
    pub plus4_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Alias {
    pub smarty_key: String,
    pub primary_number: String,
    pub street_name: String,
    pub street_suffix: String,
    pub street_postdirection: String,
    pub city_name: String,
    pub state_abbreviation: String,
    pub zipcode: String,
    pub plus4_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Secondary {
    pub smarty_key: String,
    pub secondary_designator: String,
    pub secondary_number: String,
    pub plus4_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SecondaryCountResponse {
    pub smarty_key: String,
    pub count: i64,
}

impl EnrichmentResponse for SecondaryCountResponse {
    fn lookup_type() -> &'static str {
        "secondary/count"
    }
}
