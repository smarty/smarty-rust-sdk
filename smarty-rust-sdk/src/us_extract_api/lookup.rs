use crate::sdk::{has_bool_param, has_i32_param, has_param};
use crate::us_extract_api::extraction::ExtractionResult;
use crate::us_street_api::lookup::MatchStrategy;
use serde::Serialize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Lookup {
    pub text: String,
    pub html: HTMLPayload,
    pub aggressive: bool,
    #[serde(rename = "addr_line_breaks")]
    pub addresses_with_line_breaks: bool, // addr_line_breaks
    #[serde(rename = "addr_per_line")]
    pub addresses_per_line: i32, //addr_per_line
    #[serde(rename = "match")]
    pub match_strategy: MatchStrategy,
    #[serde(skip_serializing)]
    pub result: ExtractionResult,
}

impl Default for Lookup {
    fn default() -> Self {
        Lookup {
            text: String::default(),
            html: HTMLPayload::HTMLUnspecified,
            aggressive: false,
            addresses_with_line_breaks: false,
            addresses_per_line: 1,
            match_strategy: MatchStrategy::Strict,
            result: ExtractionResult::default(),
        }
    }
}

impl Lookup {
    pub(crate) fn into_param_array(self) -> Vec<(String, String)> {
        vec![
            has_param("html".to_string(), self.html.to_string()),
            has_bool_param("aggressive".to_string(), self.aggressive, false),
            has_bool_param(
                "addr_line_breaks".to_string(),
                self.addresses_with_line_breaks,
                false,
            ),
            has_i32_param("addr_per_line".to_string(), self.addresses_per_line, 0),
            has_param("match".to_string(), self.match_strategy.to_string()),
        ]
        .iter()
        .filter_map(Option::clone)
        .collect::<Vec<_>>()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum HTMLPayload {
    #[serde(rename = "")]
    HTMLUnspecified,
    #[serde(rename = "true")]
    HTMLYes,
    #[serde(rename = "false")]
    HTMLNo,
}
impl Display for HTMLPayload {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HTMLPayload::HTMLUnspecified => {
                write!(f, "")
            }
            HTMLPayload::HTMLYes => {
                write!(f, "true")
            }
            HTMLPayload::HTMLNo => {
                write!(f, "false")
            }
        }
    }
}
