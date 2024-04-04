pub mod client;
pub mod extraction;
pub mod lookup;

#[cfg(test)]
mod tests {
    use crate::sdk::options::OptionsBuilder;
    use crate::us_extract_api::client::USExtractClient;
    use crate::us_extract_api::lookup::Lookup;

    #[test]
    fn client_test() {
        let client = USExtractClient::new(OptionsBuilder::new(None).build()).unwrap();

        assert_eq!(
            client.client.url.to_string(),
            "https://us-extract.api.smarty.com/".to_string()
        )
    }

    #[test]
    fn lookup_test() {
        let lookup = Lookup {
            text: "Meet me at 3214 N University Ave Provo UT 84604 just after 3pm.".to_string(),
            aggressive: true,
            addresses_with_line_breaks: false,
            addresses_per_line: 1,
            ..Default::default()
        };

        let expected_result = vec![
            ("aggressive".to_string(), "true".to_string()),
            ("addr_per_line".to_string(), "1".to_string()),
            ("match".to_string(), "strict".to_string()),
        ];

        assert_eq!(lookup.into_param_array(), expected_result);
    }
}
