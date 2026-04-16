pub mod client;
pub mod lookup;
pub mod request;

pub mod response;

pub mod business;
pub mod geo;
pub mod principal;
pub mod secondary;
pub mod risk;

#[cfg(test)]
mod tests {
    use crate::sdk::error::SmartyError;
    use crate::sdk::options::OptionsBuilder;
    use crate::us_enrichment_api::business::{BusinessDetailResponse, BusinessSummaryResponse};
    use crate::us_enrichment_api::client::{extract_etag, USEnrichmentClient};
    use crate::us_enrichment_api::lookup::{BusinessDetailLookup, EnrichmentLookup};
    use crate::us_enrichment_api::principal::PrincipalResponse;
    use crate::us_enrichment_api::request::EnrichmentRequest;
    use reqwest::header::{HeaderMap, HeaderValue};

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

        assert_eq!(lookup.params(), expected_params);
    }

    #[test]
    fn lookup_empty_params_test() {
        let lookup: EnrichmentLookup<PrincipalResponse> = EnrichmentLookup {
            smarty_key: 123456789,
            ..Default::default()
        };

        let expected_params: Vec<(String, String)> = vec![];

        assert_eq!(lookup.params(), expected_params);
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

        assert_eq!(lookup.params(), expected_params);
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

        assert_eq!(lookup.params(), expected_params);
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

    #[test]
    fn business_detail_lookup_default() {
        let lookup = BusinessDetailLookup::default();

        assert!(lookup.business_id.is_empty());
        assert!(lookup.etag.is_empty());
        assert!(lookup.result.is_none());
    }

    #[test]
    fn business_detail_lookup_params() {
        let lookup = BusinessDetailLookup {
            business_id: "ABC123".to_string(),
            include: "company_name,city_name".to_string(),
            exclude: "latitude".to_string(),
            ..Default::default()
        };

        let expected_params = vec![
            ("include".to_string(), "company_name,city_name".to_string()),
            ("exclude".to_string(), "latitude".to_string()),
        ];

        assert_eq!(lookup.params(), expected_params);
    }

    #[test]
    fn business_summary_address_search() {
        let lookup: EnrichmentLookup<BusinessSummaryResponse> = EnrichmentLookup {
            freeform: "123 Main St, Denver CO".to_string(),
            ..Default::default()
        };

        assert!(lookup.is_address_search());
        assert!(lookup.has_address_fields());

        let expected_params = vec![
            ("freeform".to_string(), "123 Main St, Denver CO".to_string()),
        ];

        assert_eq!(lookup.params(), expected_params);
    }

    #[test]
    fn business_summary_response_deserialize() {
        let json = r#"[{
            "smarty_key": "123",
            "data_set_name": "business",
            "businesses": [
                {"company_name": "Acme Corp", "business_id": "ABC123"},
                {"company_name": "Test Inc", "business_id": "DEF456"}
            ]
        }]"#;

        let results: Vec<BusinessSummaryResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].smarty_key, "123");
        assert_eq!(results[0].businesses.len(), 2);
        assert_eq!(results[0].businesses[0].company_name, "Acme Corp");
        assert_eq!(results[0].businesses[0].business_id, "ABC123");
    }

    #[test]
    fn business_detail_response_deserialize() {
        // Exercises a broader cross-section of BusinessDetailAttributes fields
        // so a future type change on any of these breaks loudly rather than
        // silently deserializing an empty string.
        let json = r#"[{
            "smarty_key": "7",
            "data_set_name": "business",
            "business_id": "7",
            "attributes": {
                "company_name": "Acme Corp",
                "company_name_secondary": "Acme Holdings",
                "city_name": "Denver",
                "state_abbreviation": "CO",
                "zipcode": "80202",
                "latitude": "39.7392",
                "longitude": "-104.9903",
                "ein": "12-3456789",
                "phone": "3035551212",
                "year_established": "1998",
                "number_of_years_in_business": "27",
                "location_employee_count": "42",
                "fortune_1000_indicator": "N",
                "minority_owned_indicator": "Y",
                "female_owned_indicator": "N",
                "primary_sic_code": "5812",
                "naics_01_code": "722511",
                "url": "https://example.com"
            }
        }]"#;

        let results: Vec<BusinessDetailResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(results.len(), 1);
        let r = &results[0];
        assert_eq!(r.smarty_key, "7");
        assert_eq!(r.business_id, "7");
        let a = &r.attributes;
        assert_eq!(a.company_name, "Acme Corp");
        assert_eq!(a.company_name_secondary, "Acme Holdings");
        assert_eq!(a.city_name, "Denver");
        assert_eq!(a.state_abbreviation, "CO");
        assert_eq!(a.zipcode, "80202");
        assert_eq!(a.latitude, "39.7392");
        assert_eq!(a.longitude, "-104.9903");
        assert_eq!(a.ein, "12-3456789");
        assert_eq!(a.phone, "3035551212");
        assert_eq!(a.year_established, "1998");
        assert_eq!(a.number_of_years_in_business, "27");
        assert_eq!(a.location_employee_count, "42");
        assert_eq!(a.fortune_1000_indicator, "N");
        assert_eq!(a.minority_owned_indicator, "Y");
        assert_eq!(a.female_owned_indicator, "N");
        assert_eq!(a.primary_sic_code, "5812");
        assert_eq!(a.naics_01_code, "722511");
        assert_eq!(a.url, "https://example.com");
    }

    #[test]
    fn business_detail_build_url_encodes_reserved_chars() {
        let base = "https://us-enrichment.api.smarty.com/".parse().unwrap();

        let lookup = BusinessDetailLookup {
            business_id: "a/b?c#d".to_string(),
            ..Default::default()
        };
        let url = lookup.build_url(&base).unwrap();
        assert_eq!(
            url.as_str(),
            "https://us-enrichment.api.smarty.com/lookup/business/a%2Fb%3Fc%23d"
        );

        let plain = BusinessDetailLookup {
            business_id: "GEYTCMZSGU2TCMBZHE3DIOI".to_string(),
            ..Default::default()
        };
        let plain_url = plain.build_url(&base).unwrap();
        assert_eq!(
            plain_url.as_str(),
            "https://us-enrichment.api.smarty.com/lookup/business/GEYTCMZSGU2TCMBZHE3DIOI"
        );
    }

    #[tokio::test]
    async fn send_business_detail_rejects_empty_business_id() {
        let options = OptionsBuilder::new(None).build();
        let client = USEnrichmentClient::new(options).unwrap();

        let mut lookup = BusinessDetailLookup::default();
        let err = client.send(&mut lookup).await.unwrap_err();
        assert!(matches!(err, SmartyError::ValidationError(_)));
    }

    #[tokio::test]
    async fn send_business_detail_rejects_whitespace_business_id() {
        let options = OptionsBuilder::new(None).build();
        let client = USEnrichmentClient::new(options).unwrap();

        let mut lookup = BusinessDetailLookup {
            business_id: "   ".to_string(),
            ..Default::default()
        };
        let err = client.send(&mut lookup).await.unwrap_err();
        assert!(matches!(err, SmartyError::ValidationError(_)));
    }

    #[test]
    fn business_detail_apply_results_rejects_multiple() {
        let mut lookup = BusinessDetailLookup {
            business_id: "ABC".to_string(),
            ..Default::default()
        };

        let too_many = vec![
            BusinessDetailResponse::default(),
            BusinessDetailResponse::default(),
        ];
        let err = lookup.apply_results(too_many).unwrap_err();
        assert!(matches!(err, SmartyError::ValidationError(_)));
        assert!(lookup.result.is_none());
    }

    #[test]
    fn extract_etag_handles_missing_header() {
        let headers = HeaderMap::new();
        assert_eq!(extract_etag(&headers), "");
    }

    #[test]
    fn extract_etag_returns_ascii_value() {
        let mut headers = HeaderMap::new();
        headers.insert("ETag", HeaderValue::from_static("\"abc-123\""));
        assert_eq!(extract_etag(&headers), "\"abc-123\"");
    }

    #[test]
    fn extract_etag_returns_empty_on_non_utf8_bytes() {
        // Bytes 0xFF / 0xFE are valid in a HeaderValue (any byte >= 0x80 is
        // legal) but HeaderValue::to_str rejects non-UTF-8. The prior code
        // used .expect() here and would panic; extract_etag must not.
        let bad = HeaderValue::from_bytes(&[0xff, 0xfe, 0xfd]).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert("ETag", bad);
        assert_eq!(extract_etag(&headers), "");
    }

    #[test]
    fn business_detail_apply_results_accepts_empty_and_single() {
        let mut lookup = BusinessDetailLookup {
            business_id: "ABC".to_string(),
            ..Default::default()
        };

        lookup.apply_results(vec![]).unwrap();
        assert!(lookup.result.is_none());

        let one = BusinessDetailResponse {
            business_id: "ABC".to_string(),
            ..Default::default()
        };
        lookup.apply_results(vec![one]).unwrap();
        assert_eq!(lookup.result.as_ref().unwrap().business_id, "ABC");
    }
}
