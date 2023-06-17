use crate::international_autocomplete_api::suggestion::SuggestionListing;
use crate::sdk::has_param;

#[derive(Clone)]
pub struct Lookup {
    pub country: String,
    pub search: String,
    pub max_results: i32,
    pub distance: i32,
    pub geolocation: String,
    pub include_only_administrative_area: String,
    pub include_only_locality: String,
    pub include_only_postal_code: String,
    pub latitude: String,
    pub longitude: String,
    pub results: SuggestionListing
}

impl Default for Lookup {
    fn default() -> Self {
        Lookup {
            country: String::default(),
            search: String::default(),
            max_results: 5,
            distance: 5,
            geolocation: "".to_string(),
            include_only_administrative_area: "".to_string(),
            include_only_locality: "".to_string(),
            include_only_postal_code: "".to_string(),
            latitude: "".to_string(),
            longitude: "".to_string(),

            results: SuggestionListing { suggestions: vec![] }
        }
    }
}

impl Lookup {
    pub(crate) fn into_param_array(self) -> Vec<(String, String)> {
        vec![
            has_param("country".to_string(), self.country),
            has_param("search".to_string(), self.search),
            has_param("max_results".to_string(), self.max_results.to_string()),
            has_param("distance".to_string(), self.distance.to_string()),
            has_param("geolocation".to_string(), self.geolocation),
            has_param("include_only_administrative_area".to_string(), self.include_only_administrative_area),
            has_param("include_only_locality".to_string(), self.include_only_locality),
            has_param("include_only_postal_code".to_string(), self.include_only_postal_code),
            has_param("latitude".to_string(), self.latitude),
            has_param("longitude".to_string(), self.longitude),
        ].iter()
            .filter_map(Option::clone)
            .collect::<Vec<_>>()
    }
}