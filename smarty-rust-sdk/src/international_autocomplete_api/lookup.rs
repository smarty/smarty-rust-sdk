use crate::international_autocomplete_api::suggestion::SuggestionListing;
use crate::sdk::has_param;

#[derive(Clone, Debug, PartialEq)]
pub struct Lookup {
    pub country: String,
    pub search: String,
    pub address_id: String,
    pub max_results: i32,
    pub include_only_locality: String,
    pub include_only_postal_code: String,
    pub results: SuggestionListing,
}

impl Default for Lookup {
    fn default() -> Self {
        Lookup {
            country: String::default(),
            search: String::default(),
            address_id: String::default(),
            max_results: 5,
            include_only_locality: "".to_string(),
            include_only_postal_code: "".to_string(),

            results: SuggestionListing {
                suggestions: vec![],
            },
        }
    }
}

impl Lookup {
    pub(crate) fn into_param_array(self) -> Vec<(String, String)> {
        vec![
            has_param("country".to_string(), self.country),
            has_param("search".to_string(), self.search),
            has_param("address_id".to_string(), self.address_id),
            has_param("max_results".to_string(), self.max_results.to_string()),
            has_param(
                "include_only_locality".to_string(),
                self.include_only_locality,
            ),
            has_param(
                "include_only_postal_code".to_string(),
                self.include_only_postal_code,
            ),
        ]
        .iter()
        .filter_map(Option::clone)
        .collect::<Vec<_>>()
    }
}
