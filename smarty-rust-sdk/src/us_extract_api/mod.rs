pub mod client;
pub mod extraction;
pub mod lookup;

#[cfg(test)]
mod tests {
    use crate::sdk::authentication::SecretKeyCredential;
    use crate::sdk::options::OptionsBuilder;
    use crate::us_extract_api::client::USExtractClient;
    use crate::us_extract_api::lookup::Lookup;

    #[test]
    fn client_test() {
        let client = USExtractClient::new(
            OptionsBuilder::new()
                .authenticate(SecretKeyCredential::new("".to_string(), "".to_string()))
                .build()
                .unwrap(),
        )
        .unwrap();

        assert_eq!(
            client.client.url.to_string(),
            "https://us-extract.api.smartystreets.com/".to_string()
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

        assert_eq!(
            serde_json::to_string_pretty(&lookup).unwrap(),
            "{\n  \"text\": \"Meet me at 3214 N University Ave Provo UT 84604 just after 3pm.\",\n  \"html\": \"\",\n  \"aggressive\": true,\n  \"addr_line_breaks\": false,\n  \"addr_per_line\": 1\n}"
        );
    }
}
