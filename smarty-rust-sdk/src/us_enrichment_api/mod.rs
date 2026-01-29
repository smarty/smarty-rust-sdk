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

    #[test]
    fn address_search_lookup_test() {
        let lookup: EnrichmentLookup<PrincipalResponse> = EnrichmentLookup {
            street: "123 Main St".to_string(),
            city: "Phoenix".to_string(),
            state: "AZ".to_string(),
            zipcode: "85001".to_string(),
            ..Default::default()
        };

        assert!(lookup.is_address_search());

        let expected_params = vec![
            ("street".to_string(), "123 Main St".to_string()),
            ("city".to_string(), "Phoenix".to_string()),
            ("state".to_string(), "AZ".to_string()),
            ("zipcode".to_string(), "85001".to_string()),
        ];

        assert_eq!(lookup.into_param_array(), expected_params);
    }

    #[test]
    fn freeform_address_search_test() {
        let lookup: EnrichmentLookup<PrincipalResponse> = EnrichmentLookup {
            freeform: "123 Main St, Phoenix, AZ 85001".to_string(),
            ..Default::default()
        };

        assert!(lookup.is_address_search());

        let expected_params = vec![
            ("freeform".to_string(), "123 Main St, Phoenix, AZ 85001".to_string()),
        ];

        assert_eq!(lookup.into_param_array(), expected_params);
    }

    #[test]
    fn smarty_key_lookup_is_not_address_search() {
        let lookup: EnrichmentLookup<PrincipalResponse> = EnrichmentLookup {
            smarty_key: 123456789,
            ..Default::default()
        };

        assert!(!lookup.is_address_search());
    }

    #[test]
    fn has_address_fields_returns_true_when_any_field_set() {
        let lookup_street: EnrichmentLookup<PrincipalResponse> = EnrichmentLookup {
            street: "123 Main St".to_string(),
            ..Default::default()
        };
        assert!(lookup_street.has_address_fields());

        let lookup_city: EnrichmentLookup<PrincipalResponse> = EnrichmentLookup {
            city: "Phoenix".to_string(),
            ..Default::default()
        };
        assert!(lookup_city.has_address_fields());

        let lookup_state: EnrichmentLookup<PrincipalResponse> = EnrichmentLookup {
            state: "AZ".to_string(),
            ..Default::default()
        };
        assert!(lookup_state.has_address_fields());

        let lookup_zipcode: EnrichmentLookup<PrincipalResponse> = EnrichmentLookup {
            zipcode: "85001".to_string(),
            ..Default::default()
        };
        assert!(lookup_zipcode.has_address_fields());

        let lookup_freeform: EnrichmentLookup<PrincipalResponse> = EnrichmentLookup {
            freeform: "123 Main St, Phoenix, AZ".to_string(),
            ..Default::default()
        };
        assert!(lookup_freeform.has_address_fields());
    }

    #[test]
    fn has_address_fields_returns_false_when_no_fields_set() {
        let lookup: EnrichmentLookup<PrincipalResponse> = EnrichmentLookup {
            smarty_key: 123456789,
            ..Default::default()
        };
        assert!(!lookup.has_address_fields());

        let empty_lookup: EnrichmentLookup<PrincipalResponse> = EnrichmentLookup::default();
        assert!(!empty_lookup.has_address_fields());
    }

    #[test]
    fn smarty_key_takes_precedence_over_address_fields() {
        // When both smarty_key and address fields are provided, smarty_key lookup is used
        let lookup: EnrichmentLookup<PrincipalResponse> = EnrichmentLookup {
            smarty_key: 123456789,
            street: "123 Main St".to_string(),
            city: "Phoenix".to_string(),
            ..Default::default()
        };

        assert!(!lookup.is_address_search());
        assert!(lookup.has_address_fields());
    }

}
