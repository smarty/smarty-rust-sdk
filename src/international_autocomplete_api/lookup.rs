use crate::international_autocomplete_api::suggestion::SuggestionListing;
use crate::sdk::has_param;

#[derive(Clone)]
pub struct Lookup {
    pub country: String,
    pub search: String,
    pub administrative_area: String,
    pub locality: String,
    pub postal_code: String,
    pub results: SuggestionListing
}

impl Default for Lookup {
    fn default() -> Self {
        Lookup {
            country: String::default(),
            search: String::default(),
            administrative_area: String::default(),
            locality: String::default(),
            postal_code: String::default(),
            results: SuggestionListing { suggestions: vec![] }
        }
    }
}

impl Lookup {
    pub(crate) fn to_param_array(self) -> Vec<(String, String)> {
        vec![
            has_param("country".to_string(), self.country),
            has_param("search".to_string(), self.search),
            has_param("include_only_administrative_area".to_string(), self.administrative_area),
            has_param("include_only_locality".to_string(), self.locality),
            has_param("include_only_postal_code".to_string(), self.postal_code),
        ].iter()
            .filter_map(Option::clone)
            .collect::<Vec<_>>()
    }
}