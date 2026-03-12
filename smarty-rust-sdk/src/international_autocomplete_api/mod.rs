pub mod client;
pub mod lookup;
pub mod suggestion;

#[cfg(test)]
mod tests {
    use crate::international_autocomplete_api::client::InternationalAutocompleteClient;
    use crate::international_autocomplete_api::lookup::Lookup;
    use crate::sdk::options::OptionsBuilder;

    #[test]
    fn client_test() {
        let client =
            InternationalAutocompleteClient::new(OptionsBuilder::new(None).build()).unwrap();

        assert_eq!(
            client.client.url.to_string(),
            "https://international-autocomplete.api.smarty.com/v2/lookup/".to_string()
        )
    }

    #[test]
    fn lookup_test() {
        let lookup = Lookup {
            country: "FRA".to_string(),
            search: "Louis".to_string(),
            include_only_locality: "Paris".to_string(),
            ..Default::default()
        };

        let expected_results = vec![
            ("country".to_string(), "FRA".to_string()),
            ("search".to_string(), "Louis".to_string()),
            ("max_results".to_string(), "5".to_string()),
            ("max_group_results".to_string(), "100".to_string()),
            ("include_only_locality".to_string(), "Paris".to_string()),
        ];

        assert_eq!(lookup.into_param_array(), expected_results)
    }

    #[test]
    fn lookup_with_geolocation() {
        let lookup = Lookup {
            country: "FRA".to_string(),
            search: "Louis".to_string(),
            geolocation: true,
            ..Default::default()
        };

        let params = lookup.into_param_array();

        assert!(params.contains(&("geolocation".to_string(), "on".to_string())));
    }

    #[test]
    fn lookup_without_geolocation() {
        let lookup = Lookup {
            country: "FRA".to_string(),
            search: "Louis".to_string(),
            geolocation: false,
            ..Default::default()
        };

        let params = lookup.into_param_array();

        assert!(!params.iter().any(|(k, _)| k == "geolocation"));
    }

    #[test]
    fn lookup_with_custom_max_group_results() {
        let lookup = Lookup {
            country: "FRA".to_string(),
            search: "Louis".to_string(),
            max_group_results: 50,
            ..Default::default()
        };

        let params = lookup.into_param_array();

        assert!(params.contains(&("max_group_results".to_string(), "50".to_string())));
    }

    #[test]
    fn lookup_defaults() {
        let lookup = Lookup::default();

        assert_eq!(lookup.max_results, 5);
        assert_eq!(lookup.max_group_results, 100);
        assert_eq!(lookup.geolocation, false);
    }
}
