use crate::sdk::{Geolocation, has_i32_param, has_param, has_vec_param};
use crate::us_autocomplete_pro::suggestion::{SuggestionListing};

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
    pub(crate) fn to_param_array(self) -> Vec<(String, String)> {
        vec![
            has_param("search".to_string(), self.search),
            has_param("source".to_string(), self.source),
            has_i32_param("max_results".to_string(), self.max_results, 0),
            has_vec_param("city_filter".to_string(), self.city_filter)
        ].iter()
            .filter_map(Option::clone)
            .collect::<Vec<_>>()
    }
}