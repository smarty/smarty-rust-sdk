use crate::us_reverse_geo_api::address::Results;

#[derive(Debug, Clone, PartialEq)]
pub struct Lookup {
    pub latitude: f64,
    pub longitude: f64,
    pub results: Results,
}

impl Default for Lookup {
    fn default() -> Self {
        Lookup {
            latitude: 0.0,
            longitude: 0.0,
            results: Results { results: vec![] },
        }
    }
}

impl Lookup {
    pub(crate) fn into_param_array(self) -> Vec<(String, String)> {
        vec![
            ("latitude".to_string(), self.latitude.to_string()),
            ("longitude".to_string(), self.longitude.to_string()),
        ]
    }
}
