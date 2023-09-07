use crate::sdk::has_param;
use crate::us_reverse_geo_api::address::Results;

#[derive(Debug, Clone, PartialEq)]
pub struct Lookup {
    pub latitude: f64,
    pub longitude: f64,
    pub source: String,
    pub results: Results,
}

impl Default for Lookup {
    fn default() -> Self {
        Lookup {
            latitude: 0.0,
            longitude: 0.0,
            source: String::default(),
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

        if let Some(source_string) = has_param("source".to_string(), self.source) {
            result.push(source_string)
        }

        result
    }
}
