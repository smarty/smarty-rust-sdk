use serde::{Deserialize, Serialize};
use crate::us_street_api::candidate::Candidates;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ExtractionResult {
    #[serde(rename = "meta")]
    metadata: Metadata,
    addresses: Vec<ExtractedAddress>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Metadata {
    lines: i32,
    characters: i32,
    bytes: i32,
    addresses: i32,
    #[serde(rename = "verified_count")]
    verified_addresses: i32,
    #[serde(rename = "unicode")]
    contains_non_ascii_unicode: bool
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ExtractedAddress {
    text: String,
    verified: bool,
    line: i32,
    start: i32,
    end: i32,
    api_output: Candidates
}