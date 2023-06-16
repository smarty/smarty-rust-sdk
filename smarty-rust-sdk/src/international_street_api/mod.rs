pub mod candidate;
pub mod client;
pub mod lookup;

#[cfg(test)]
mod tests {
    use crate::international_street_api::client::InternationalStreetClient;
    use crate::international_street_api::lookup::Lookup;
    use crate::sdk::authentication::SecretKeyCredential;
    use crate::sdk::options::OptionsBuilder;

    #[test]
    fn client_test() {
        let client = InternationalStreetClient::new(
            OptionsBuilder::new()
                .authenticate(SecretKeyCredential::new("".to_string(), "".to_string()))
                .build()
                .unwrap(),
        )
        .unwrap();

        assert_eq!(
            client.client.url.to_string(),
            "https://international-street.api.smartystreets.com/verify".to_string()
        )
    }

    #[test]
    fn lookup_test() {
        let lookup = Lookup {
            geocode: false,
            organization: "John Doe".to_string(),
            address1: "Rua Padre Antonio D'Angelo 121".to_string(),
            address2: "Casa Verde".to_string(),
            locality: "Sao Paulo".to_string(),
            administrative_area: "SP".to_string(),
            country: "Brazil".to_string(),
            postal_code: "02516-050".to_string(),
            ..Default::default()
        };

        let expected_results = vec![
            ("country".to_string(), "Brazil".to_string()),
            ("geocode".to_string(), "false".to_string()),
            ("language".to_string(), "native".to_string()),
            (
                "address1".to_string(),
                "Rua Padre Antonio D'Angelo 121".to_string(),
            ),
            ("address2".to_string(), "Casa Verde".to_string()),
            ("organization".to_string(), "John Doe".to_string()),
            ("locality".to_string(), "Sao Paulo".to_string()),
            ("administrative_area".to_string(), "SP".to_string()),
            ("postal_code".to_string(), "02516-050".to_string()),
        ];

        assert_eq!(lookup.into_param_array(), expected_results);
    }
}
