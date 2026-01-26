pub mod client;
pub mod lookup;

pub mod response;

pub mod geo;
pub mod principal;
pub mod secondary;
pub mod risk;

#[cfg(test)]
mod tests {
    use crate::us_enrichment_api::client::USEnrichmentClient;
    use crate::us_enrichment_api::lookup::EnrichmentLookup;
    use crate::us_enrichment_api::principal::PrincipalResponse;
    use crate::sdk::options::OptionsBuilder;

    #[test]
    fn client_test() {
        let options = OptionsBuilder::new(None).build();
        let client = USEnrichmentClient::new(options).unwrap();

        assert_eq!(
            client.client.url.to_string(),
            "https://us-enrichment.api.smarty.com/lookup".to_string()
        )
    }

    #[test]
    fn lookup_test() {
        let lookup: EnrichmentLookup<PrincipalResponse> = EnrichmentLookup {
            smarty_key: 123456789,
            include: "group_structural,group_other".to_string(),
            exclude: "assessed_improvement_value".to_string(),
            features: "".to_string(),
            ..Default::default()
        };

        let expected_params = vec![
            ("include".to_string(), "group_structural,group_other".to_string()),
            ("exclude".to_string(), "assessed_improvement_value".to_string()),
        ];

        assert_eq!(lookup.into_param_array(), expected_params);
    }

    #[test]
    fn lookup_empty_params_test() {
        let lookup: EnrichmentLookup<PrincipalResponse> = EnrichmentLookup {
            smarty_key: 123456789,
            ..Default::default()
        };

        let expected_params: Vec<(String, String)> = vec![];

        assert_eq!(lookup.into_param_array(), expected_params);
    }
}
