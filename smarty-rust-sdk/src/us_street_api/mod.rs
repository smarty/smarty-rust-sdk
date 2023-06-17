pub mod client;
pub mod lookup;

pub mod candidate;

#[cfg(test)]
mod tests {
    use crate::sdk::authentication::SecretKeyCredential;
    use crate::sdk::batch::Batch;
    use crate::sdk::options::OptionsBuilder;
    use crate::us_street_api::client::USStreetAddressClient;
    use crate::us_street_api::lookup::{Lookup, MatchStrategy};

    #[test]
    fn client_test() {
        let client = USStreetAddressClient::new(
            OptionsBuilder::new()
                .authenticate(SecretKeyCredential::new("".to_string(), "".to_string()))
                .build()
                .unwrap(),
        )
        .unwrap();

        assert_eq!(
            client.client.url.to_string(),
            "https://us-street.api.smartystreets.com/street-address".to_string()
        );
    }

    #[test]
    fn lookup_test() {
        let lookup = Lookup {
            street: "1600 Amphitheatre Pkwy".to_string(),
            last_line: "Mountain View, CA".to_string(),
            max_candidates: 10,
            match_strategy: MatchStrategy::Enhanced,
            ..Default::default()
        };

        let expected_result = vec![
            ("street".to_string(), "1600 Amphitheatre Pkwy".to_string()),
            ("lastline".to_string(), "Mountain View, CA".to_string()),
            ("candidates".to_string(), 5.to_string()),
            ("match".to_string(), "enhanced".to_string()),
        ];

        assert_eq!(lookup.into_param_array(), expected_result);
    }

    #[test]
    fn batch_test() {
        let lookup = Lookup {
            street: "1600 Amphitheatre Pkwy".to_string(),
            last_line: "Mountain View, CA".to_string(),
            max_candidates: 10,
            match_strategy: MatchStrategy::Enhanced,
            ..Default::default()
        };

        let expected_result = vec![
            ("street".to_string(), "1600 Amphitheatre Pkwy".to_string()),
            ("lastline".to_string(), "Mountain View, CA".to_string()),
            ("candidates".to_string(), 5.to_string()),
            ("match".to_string(), "enhanced".to_string()),
        ];

        let mut batch = Batch::default();
        batch.push(lookup.clone()).unwrap();
        batch.push(lookup.clone()).unwrap();
        batch.push(lookup.clone()).unwrap();
        batch.push(lookup).unwrap();

        assert_eq!(
            batch.records()[0].clone().into_param_array(),
            expected_result
        );
        assert_eq!(batch.records().len(), 4);
    }
}
