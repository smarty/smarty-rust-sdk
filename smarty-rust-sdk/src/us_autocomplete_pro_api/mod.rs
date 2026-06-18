pub mod client;
pub mod lookup;
pub mod suggestion;

#[cfg(test)]
mod tests {
    use crate::sdk::options::OptionsBuilder;
    use crate::us_autocomplete_pro_api::client::USAutocompleteProClient;
    use crate::us_autocomplete_pro_api::lookup::{Geolocation, Lookup, Source};

    #[test]
    fn client_test() {
        let client = USAutocompleteProClient::new(OptionsBuilder::new(None).build()).unwrap();

        assert_eq!(
            client.client.url.to_string(),
            "https://us-autocomplete-pro.api.smarty.com/lookup".to_string()
        )
    }

    #[test]
    fn lookup_test() {
        let lookup = Lookup {
            search: "1042 W Center".to_string(),
            max_results: 5,
            city_filter: vec!["Denver,CO".to_string(), "Orem,UT".to_string()],
            state_filter: vec!["CO".to_string(), "UT".to_string()],
            prefer_state: vec!["CO".to_string()],
            prefer_ratio: 3,
            geolocation: Geolocation::GeolocateCity,
            source: Some(Source::All),
            ..Default::default()
        };

        let expected_results = vec![
            ("search".to_string(), "1042 W Center".to_string()),
            ("max_results".to_string(), "5".to_string()),
            (
                "include_only_cities".to_string(),
                "Denver,CO;Orem,UT".to_string(),
            ),
            ("include_only_states".to_string(), "CO;UT".to_string()),
            ("prefer_states".to_string(), "CO".to_string()),
            ("prefer_ratio".to_string(), "3".to_string()),
            ("prefer_geolocation".to_string(), "city".to_string()),
            ("source".to_string(), "all".to_string()),
        ];

        assert_eq!(lookup.into_param_array(), expected_results)
    }

    #[test]
    fn lookup_excludes_source_when_not_specified() {
        let lookup = Lookup {
            search: "1042 W Center".to_string(),
            ..Default::default()
        };

        let params = lookup.into_param_array();
        assert!(!params.iter().any(|(k, _)| k == "source"));
    }

    #[test]
    fn lookup_includes_source_postal() {
        let lookup = Lookup {
            search: "1042 W Center".to_string(),
            source: Some(Source::Postal),
            ..Default::default()
        };

        let params = lookup.into_param_array();
        assert!(params.iter().any(|(k, v)| k == "source" && v == "postal"));
    }
}
