pub mod lookup;
pub mod client;
pub mod suggestion;

#[cfg(test)]
mod tests {
    use crate::sdk::options::Options;
    use crate::us_autocomplete_api::client::USAutocompleteClient;
    use crate::us_autocomplete_api::lookup::Lookup;

    #[test]
    fn client_test() {
        let client = USAutocompleteClient::new(Options::default()).unwrap();

        assert_eq!(client.client.url.to_string(), "https://us-autocomplete.api.smartystreets.com/suggest".to_string());
    }

    #[test]
    fn lookup_test() {
        let lookup = Lookup {
            prefix: "ST".to_string(),
            max_suggestions: 5,
            ..Default::default()
        };

        let expected_results = vec![
            ("prefix".to_string(), "ST".to_string()),
            ("max_suggestions".to_string(), "5".to_string()),
            ("geolocate".to_string(), "false".to_string()),
        ];

        assert_eq!(lookup.into_param_array(), expected_results);
    }
}