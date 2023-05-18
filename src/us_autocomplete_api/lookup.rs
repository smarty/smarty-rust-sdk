use serde::Serialize;
use crate::sdk::{has_f64_param, has_i32_param, has_param, has_vec_param};
use crate::us_autocomplete_api::suggestion::SuggestionListing;

#[derive(Clone)]
pub struct Lookup {
    pub prefix: String,
    pub max_suggestions: i32,
    pub city_filter: Vec<String>,
    pub state_filter: Vec<String>,
    pub preferences: Vec<String>,
    pub geolocation: Geolocation,
    pub prefer_ratio: f64,

    pub results: SuggestionListing
}

impl Default for Lookup {
    fn default() -> Self {
        Lookup {
            prefix: String::default(),
            max_suggestions: 0,
            city_filter: vec![],
            state_filter: vec![],
            prefer_ratio: 0.0,
            geolocation: Geolocation::default(),

            results: SuggestionListing { suggestions: vec![] },
            preferences: vec![]
        }
    }
}

impl Lookup {
    pub(crate) fn into_param_array(self) -> Vec<(String, String)> {
        vec![
            has_param("prefix".to_string(), self.prefix),
            has_i32_param("max_suggestions".to_string(), self.max_suggestions, 0),
            has_vec_param("city_filter".to_string(), self.city_filter),
            has_vec_param("state_filter".to_string(), self.state_filter),
            has_vec_param("preferences".to_string(), self.preferences),
            self.geolocation.geolocation_to_param(),
            has_f64_param("prefer_ratio".to_string(), self.prefer_ratio, 0.0)
        ].iter()
            .filter_map(Option::clone)
            .collect::<Vec<_>>()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub enum Geolocation {
    #[default]
    #[serde(rename = "none")]
    GeolocateNone,
    #[serde(rename = "city")]
    GeolocateCity,
    #[serde(rename = "state")]
    GeolocateState
}

impl Geolocation {
    fn geolocation_to_param(self) -> Option<(String, String)> {
        match self {
            Geolocation::GeolocateNone => Some(("geolocate".to_string(), "false".to_string())),
            Geolocation::GeolocateCity => None,
            Geolocation::GeolocateState => Some(("geolocate_precision".to_string(), "state".to_string())),
        }
    }
}