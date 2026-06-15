pub mod client;
pub mod lookup;
pub mod suggestion;

#[cfg(test)]
mod tests {
    use crate::sdk::options::OptionsBuilder;
    use crate::us_autocomplete_api::client::USAutocompleteClient;
    use crate::us_autocomplete_api::lookup::{Geolocation, Lookup};

    #[test]
    fn client_test() {
        let client = USAutocompleteClient::new(OptionsBuilder::new(None).build()).unwrap();

        assert_eq!(
            client.client.url.to_string(),
            "https://us-autocomplete.api.smarty.com/v2/lookup".to_string()
        )
    }

    #[test]
    fn lookup_test() {
        let lookup = Lookup {
            search: "1042 W Center".to_string(),
            max_results: 5,
            city_filter: vec!["Denver,CO".to_string(), "Orem,UT".to_string()],
            state_filter: vec!["CO".to_string(), "UT".to_string()],
            prefer_city: vec!["Denver".to_string()],
            prefer_state: vec!["CO".to_string()],
            prefer_ratio: 3,
            geolocation: Geolocation::GeolocateCity,
            source: "all".to_string(),
            selected: "selectedAddress".to_string(),
            exclude: "excludedAddress".to_string(),
            ..Default::default()
        };

        let expected_results = vec![
            ("search".to_string(), "1042 W Center".to_string()),
            ("source".to_string(), "all".to_string()),
            ("max_results".to_string(), "5".to_string()),
            (
                "include_only_cities".to_string(),
                "Denver,CO;Orem,UT".to_string(),
            ),
            ("include_only_states".to_string(), "CO;UT".to_string()),
            ("prefer_cities".to_string(), "Denver".to_string()),
            ("prefer_states".to_string(), "CO".to_string()),
            ("prefer_ratio".to_string(), "3".to_string()),
            ("prefer_geolocation".to_string(), "city".to_string()),
            ("selected".to_string(), "selectedAddress".to_string()),
            ("exclude".to_string(), "excludedAddress".to_string()),
        ];

        assert_eq!(lookup.into_param_array(), expected_results)
    }
}
