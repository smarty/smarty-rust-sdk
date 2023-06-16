pub mod address;
pub mod client;
pub mod lookup;

#[cfg(test)]
mod tests {
    use crate::sdk::authentication::SecretKeyCredential;
    use crate::sdk::options::OptionsBuilder;
    use crate::us_reverse_geo_api::client::USReverseGeoClient;
    use crate::us_reverse_geo_api::lookup::Lookup;

    #[test]
    fn client_test() {
        let client = USReverseGeoClient::new(
            OptionsBuilder::new()
                .authenticate(SecretKeyCredential::new("".to_string(), "".to_string()))
                .build()
                .unwrap(),
        )
        .unwrap();

        assert_eq!(
            client.client.url.to_string(),
            "https://us-reverse-geo.api.smartystreets.com/lookup".to_string()
        )
    }

    #[test]
    fn lookup_test() {
        let lookup = Lookup {
            latitude: 37.42251134855708,
            longitude: -122.08412869140541,
            ..Default::default()
        };

        let expected_result = vec![
            ("latitude".to_string(), "37.42251134855708".to_string()),
            ("longitude".to_string(), "-122.08412869140541".to_string()),
        ];

        assert_eq!(lookup.into_param_array(), expected_result)
    }
}
