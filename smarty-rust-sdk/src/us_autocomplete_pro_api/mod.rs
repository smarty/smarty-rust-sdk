pub mod client;
pub mod lookup;
pub mod suggestion;

#[cfg(test)]
mod tests {
    use crate::sdk::authentication::SecretKeyCredential;
    use crate::sdk::options::OptionsBuilder;
    use crate::us_autocomplete_pro_api::client::USAutocompleteProClient;
    use crate::us_autocomplete_pro_api::lookup::{Geolocation, Lookup};

    #[test]
    fn client_test() {
        let client = USAutocompleteProClient::new(
            OptionsBuilder::new()
                .authenticate(SecretKeyCredential::new("".to_string(), "".to_string()))
                .build()
                .unwrap(),
        )
        .unwrap();

        assert_eq!(
            client.client.url.to_string(),
            "https://us-autocomplete-pro.api.smartystreets.com/lookup".to_string()
        )
    }

    #[test]
    fn lookup_test() {
        let lookup = Lookup {
            search: "1042 W Center".to_string(),
            max_results: 5,
            city_filter: vec!["Denver".to_string(), "Orem".to_string()],
            state_filter: vec!["CO".to_string(), "UT".to_string()],
            prefer_state: vec!["CO".to_string()],
            prefer_ratio: 3,
            geolocation: Geolocation::GeolocateCity,
            source: "all".to_string(),
            ..Default::default()
        };

        let expected_results = vec![
            ("search".to_string(), "1042 W Center".to_string()),
            ("source".to_string(), "all".to_string()),
            ("max_results".to_string(), "5".to_string()),
            ("city_filter".to_string(), "[Denver,Orem]".to_string()),
            ("state_filter".to_string(), "[CO,UT]".to_string()),
            ("prefer_state".to_string(), "[CO]".to_string()),
            ("prefer_ratio".to_string(), "3".to_string()),
            ("prefer_geolocation".to_string(), "city".to_string()),
        ];

        assert_eq!(lookup.into_param_array(), expected_results)
    }
}
