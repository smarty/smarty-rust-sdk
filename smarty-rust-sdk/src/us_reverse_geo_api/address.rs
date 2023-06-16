use serde::{Deserialize, Serialize};
use crate::sdk::CoordinateLicense;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Results {
    pub results: Vec<Result>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Result {
    pub coordinate: Coordinate,
    pub address: Address,
    pub distance: f64
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
    pub accuracy: String,
    pub license: CoordinateLicense
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub state_abbreviation: String,
    pub zipcode: String
}