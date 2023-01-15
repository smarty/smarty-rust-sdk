pub mod client;
pub mod lookup;
pub mod address;

#[cfg(test)]
mod tests {
    use crate::sdk::options::Options;
    use crate::us_reverse_geo_api::client::USReverseGeoClient;
    use crate::us_reverse_geo_api::lookup::Lookup;

    #[test]
    fn client_test() {
        let client = USReverseGeoClient::new(Options::new()).unwrap();

        assert_eq!(client.client.url.to_string(), "https://us-reverse-geo.api.smartystreets.com/lookup".to_string())
    }

    #[test]
    fn lookup_test() {
        let lookup = Lookup {
            latitude: 37.42251134855708,
            longitude: -122.08412869140541,
            ..Default::default()
        };

        let expected_result = vec! [
            ("latitude".to_string(), "37.42251134855708".to_string()),
            ("longitude".to_string(), "-122.08412869140541".to_string()),
        ];

        assert_eq!(lookup.to_param_array(), expected_result)
    }
}