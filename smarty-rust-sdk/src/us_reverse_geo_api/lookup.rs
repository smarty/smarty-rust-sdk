use crate::sdk::has_param;
use crate::us_reverse_geo_api::address::Results;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct Lookup {
    pub latitude: f64,
    pub longitude: f64,
    pub source: Source,
    pub results: Results,
}

impl Default for Lookup {
    fn default() -> Self {
        Lookup {
            latitude: 0.0,
            longitude: 0.0,
            source: Source::default(),
            results: Results { results: vec![] },
        }
    }
}

impl Lookup {
    pub(crate) fn into_param_array(self) -> Vec<(String, String)> {
        let mut result = vec![
            ("latitude".to_string(), self.latitude.to_string()),
            ("longitude".to_string(), self.longitude.to_string()),
        ];

        if let Some(source_param) = has_param("source".to_string(), self.source) {
            result.push(source_param);
        }

        result
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum Source {
    #[default]
    NotSpecified,
    All,
    Postal,
}

impl Display for Source {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Source::NotSpecified => write!(f, ""),
            Source::All => write!(f, "all"),
            Source::Postal => write!(f, "postal"),
        }
    }
}
