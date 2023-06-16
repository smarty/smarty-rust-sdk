pub mod client;
pub mod lookup;
pub mod suggestion;

#[cfg(test)]
mod tests {
    use crate::international_autocomplete_api::client::InternationalAutocompleteClient;
    use crate::international_autocomplete_api::lookup::Lookup;
    use crate::sdk::authentication::SecretKeyCredential;
    use crate::sdk::options::OptionsBuilder;

    #[test]
    fn client_test() {
        let client = InternationalAutocompleteClient::new(
            OptionsBuilder::new()
                .authenticate(SecretKeyCredential::new("".to_string(), "".to_string()))
                .build()
                .unwrap(),
        )
        .unwrap();

        assert_eq!(
            client.client.url.to_string(),
            "https://international-autocomplete.api.smartystreets.com/lookup".to_string()
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
            ("distance".to_string(), "5".to_string()),
            ("include_only_locality".to_string(), "Paris".to_string()),
        ];

        assert_eq!(lookup.into_param_array(), expected_results)
    }
}
