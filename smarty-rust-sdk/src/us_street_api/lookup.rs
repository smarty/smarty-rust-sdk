use crate::sdk::has_param;
use crate::us_street_api::candidate::Candidates;
use serde::ser::SerializeMap;
use serde::Serialize;
use std::fmt::{Display, Formatter};

const DEFAULT_ENHANCED_CANDIDATES: i64 = 5;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Lookup {
    pub street: String,
    pub street2: String,
    pub secondary: String,
    pub city: String,
    pub state: String,
    pub zipcode: String,
    pub last_line: String,
    pub addressee: String,
    pub urbanization: String,
    pub input_id: String,
    pub max_candidates: i64,
    pub match_strategy: MatchStrategy,
    pub format_output: OutputFormat,
    pub county_source: Option<CountySource>,
    pub results: Candidates,
}

impl Serialize for Lookup {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;

        if !self.street.is_empty() {
            map.serialize_entry("street", &self.street)?;
        }
        if !self.street2.is_empty() {
            map.serialize_entry("street2", &self.street2)?;
        }
        if !self.secondary.is_empty() {
            map.serialize_entry("secondary", &self.secondary)?;
        }
        if !self.city.is_empty() {
            map.serialize_entry("city", &self.city)?;
        }
        if !self.state.is_empty() {
            map.serialize_entry("state", &self.state)?;
        }
        if !self.zipcode.is_empty() {
            map.serialize_entry("zipcode", &self.zipcode)?;
        }
        if !self.last_line.is_empty() {
            map.serialize_entry("lastline", &self.last_line)?;
        }
        if !self.addressee.is_empty() {
            map.serialize_entry("addressee", &self.addressee)?;
        }
        if !self.urbanization.is_empty() {
            map.serialize_entry("urbanization", &self.urbanization)?;
        }
        if !self.input_id.is_empty() {
            map.serialize_entry("input_id", &self.input_id)?;
        }

        let candidates = self.effective_candidates();
        if candidates > 0 {
            map.serialize_entry("candidates", &candidates)?;
        }

        map.serialize_entry("match", &self.match_strategy)?;

        let format_str = self.format_output.to_string();
        if !format_str.is_empty() {
            map.serialize_entry("format", &format_str)?;
        }

        if let Some(ref source) = self.county_source {
            map.serialize_entry("county_source", source)?;
        }

        map.end()
    }
}

impl Lookup {
    fn effective_candidates(&self) -> i64 {
        if self.max_candidates > 0 {
            self.max_candidates
        } else if self.match_strategy == MatchStrategy::Enhanced {
            DEFAULT_ENHANCED_CANDIDATES
        } else {
            0
        }
    }

    pub(crate) fn into_param_array(self) -> Vec<(String, String)> {
        let candidates = self.effective_candidates();
        let candidates_string = if candidates > 0 {
            candidates.to_string()
        } else {
            String::default()
        };

        let match_string = self.match_strategy.to_string();

        let mut res = vec![
            has_param("street".to_string(), self.street),
            has_param("street2".to_string(), self.street2),
            has_param("secondary".to_string(), self.secondary),
            has_param("city".to_string(), self.city),
            has_param("state".to_string(), self.state),
            has_param("zipcode".to_string(), self.zipcode),
            has_param("lastline".to_string(), self.last_line),
            has_param("addressee".to_string(), self.addressee),
            has_param("urbanization".to_string(), self.urbanization),
            has_param("input_id".to_string(), self.input_id),
            has_param("candidates".to_string(), candidates_string),
            has_param("match".to_string(), match_string),
            has_param("format".to_string(), self.format_output.to_string()),
        ];

        if let Some(source) = self.county_source {
            res.push(Some(("country_source".to_string(), source.to_string())));
        }

        res.iter().filter_map(Option::clone).collect::<Vec<_>>()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MatchStrategy {
    Strict,
    Invalid,
    #[default]
    Enhanced,
}

impl Display for MatchStrategy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MatchStrategy::Strict => {
                write!(f, "strict")
            }
            MatchStrategy::Invalid => {
                write!(f, "invalid")
            }
            MatchStrategy::Enhanced => {
                write!(f, "enhanced")
            }
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub enum OutputFormat {
    #[default]
    FormatDefault,
    ProjectUsa,
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::FormatDefault => {
                write!(f, "")
            }
            OutputFormat::ProjectUsa => {
                write!(f, "project-usa")
            }
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CountySource {
    #[default]
    Postal,
    Geographic,
}

impl Display for CountySource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CountySource::Postal => {
                write!(f, "postal")
            }
            CountySource::Geographic => {
                write!(f, "geographic")
            }
        }
    }
}
