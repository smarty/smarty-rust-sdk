use serde::Serialize;
use crate::us_extract_api::extraction::ExtractionResult;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Lookup {
    pub text: String,
    pub html: HTMLPayload,
    pub aggressive: bool,
    #[serde(rename = "addr_line_breaks")]
    pub addresses_with_line_breaks: bool, // addr_line_breaks
    #[serde(rename = "addr_per_line")]
    pub addresses_per_line: i32, //addr_per_line
    #[serde(skip_serializing)]
    pub result: ExtractionResult
}

impl Default for Lookup {
    fn default() -> Self {
        Lookup {
            text: String::default(),
            html: HTMLPayload::HTMLUnspecified,
            aggressive: false,
            addresses_with_line_breaks: false,
            addresses_per_line: 1,
            result: ExtractionResult::default()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum HTMLPayload {
    #[serde(rename = "")]
    HTMLUnspecified,
    #[serde(rename = "true")]
    HTMLYes,
    #[serde(rename = "false")]
    HTMLNo
}