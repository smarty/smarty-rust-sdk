use serde::Serialize;
use crate::sdk::{has_i32_param, has_param, has_vec_param};
use crate::us_autocomplete_pro_api::suggestion::{SuggestionListing};

#[derive(Clone)]
pub struct Lookup {
    pub search: String,
    pub source: String,
    pub max_results: i32,
    pub city_filter: Vec<String>,
    pub state_filter: Vec<String>,
    pub zip_filter: Vec<String>,
    pub exclude_states: Vec<String>,
    pub prefer_city: Vec<String>,
    pub prefer_state: Vec<String>,
    pub prefer_zip: Vec<String>,
    pub prefer_ratio: i32,
    pub geolocation: Geolocation,

    pub results: SuggestionListing
}

impl Default for Lookup {
    fn default() -> Self {
        Lookup {
            search: String::default(),
            source: String::default(),
            max_results: 0,
            city_filter: vec![],
            state_filter: vec![],
            zip_filter: vec![],
            exclude_states: vec![],
            prefer_city: vec![],
            prefer_state: vec![],
            prefer_zip: vec![],
            prefer_ratio: 0,
            geolocation: Geolocation::default(),

            results: SuggestionListing { suggestions: vec![] }
        }
    }
}

impl Lookup {
    pub(crate) fn into_param_array(self) -> Vec<(String, String)> {
        let geolocation_self = self.clone();
        vec![
            has_param("search".to_string(), self.search),
            has_param("source".to_string(), self.source),
            has_i32_param("max_results".to_string(), self.max_results, 0),
            has_vec_param("city_filter".to_string(), self.city_filter),
            has_vec_param("state_filter".to_string(), self.state_filter),
            has_vec_param("zip_filter".to_string(), self.zip_filter),
            has_vec_param("exclude_states".to_string(), self.exclude_states),
            has_vec_param("prefer_state".to_string(), self.prefer_state),
            has_vec_param("prefer_zip".to_string(), self.prefer_zip),
            has_i32_param("prefer_ratio".to_string(), self.prefer_ratio, 0),
            geolocation_self.geolocation_param()
        ].iter()
            .filter_map(Option::clone)
            .collect::<Vec<_>>()
    }

    fn geolocation_param(self) -> Option<(String, String)> {
        if !self.zip_filter.is_empty() || !self.prefer_zip.is_empty() {
            return Some(("prefer_geolocation".to_string(), "none".to_string()))
        }

        match self.geolocation {
            Geolocation::GeolocateCity => { Some(("prefer_geolocation".to_string(), "city".to_string())) }
            Geolocation::GeolocateNone => { Some(("prefer_geolocation".to_string(), "none".to_string())) }
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub enum Geolocation {
    #[default]
    #[serde(rename = "none")]
    GeolocateNone,
    #[serde(rename = "city")]
    GeolocateCity
}