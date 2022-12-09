use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ZipcodeResult {
    input_id: String,
    input_index: i32,

    status: String,
    reason: String,
    city_states: Vec<CityState>,
    zipcodes: Vec<ZIPCode>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CityState {
    city: String,
    mailable_city: bool,
    state_abbreviation: String,
    state: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ZIPCode {
    zipcode: String,
    zipcode_type: String,
    default_city: String,
    latitude: f64,
    longitude: f64,
    precision: String,
    alternate_counties: Vec<County>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct County {
    county_fips: String,
    county_name: String,
    state_abbreviation: String,
    state: String
}