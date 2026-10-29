pub mod candidate;
pub mod client;
pub mod lookup;

#[cfg(test)]
mod tests {
    use crate::international_postal_code_api::candidate::Candidate;
    use crate::international_postal_code_api::client::InternationalPostalCodeClient;
    use crate::international_postal_code_api::lookup::{Language, Lookup};
    use crate::sdk::options::OptionsBuilder;
    use serde_json::from_str;

    #[test]
    fn client_test() {
        let client = InternationalPostalCodeClient::new(OptionsBuilder::new(None).build()).unwrap();

        assert_eq!(
            client.client.url.to_string(),
            "https://international-postal-code.api.smarty.com/lookup".to_string()
        );
    }

    #[test]
    fn lookup_test() {
        let lookup = Lookup {
            input_id: "ID-8675309".to_string(),
            locality: "Sao Paulo".to_string(),
            administrative_area: "SP".to_string(),
            country: "Brazil".to_string(),
            postal_code: "02516".to_string(),
            language: Some(Language::Native),
            max_results: 25,
            ..Default::default()
        };

        let expected_params = vec![
            ("input_id".to_string(), "ID-8675309".to_string()),
            ("country".to_string(), "Brazil".to_string()),
            ("locality".to_string(), "Sao Paulo".to_string()),
            ("administrative_area".to_string(), "SP".to_string()),
            ("postal_code".to_string(), "02516".to_string()),
            ("language".to_string(), "native".to_string()),
            ("max_results".to_string(), "25".to_string()),
        ];

        assert_eq!(lookup.into_param_array(), expected_params);
    }

    #[test]
    fn lookup_with_native_language() {
        let lookup = Lookup {
            country: "Brazil".to_string(),
            language: Some(Language::Native),
            ..Default::default()
        };

        let params = lookup.into_param_array();

        assert!(params.contains(&("language".to_string(), "native".to_string())));
    }

    #[test]
    fn lookup_with_latin_language() {
        let lookup = Lookup {
            country: "Brazil".to_string(),
            language: Some(Language::Latin),
            ..Default::default()
        };

        let params = lookup.into_param_array();

        assert!(params.contains(&("language".to_string(), "latin".to_string())));
    }

    #[test]
    fn lookup_without_language() {
        let lookup = Lookup {
            country: "Brazil".to_string(),
            ..Default::default()
        };

        let params = lookup.into_param_array();

        assert!(!params.iter().any(|(k, _)| k == "language"));
    }

    #[test]
    fn lookup_with_max_results() {
        let lookup = Lookup {
            country: "Brazil".to_string(),
            max_results: 1000,
            ..Default::default()
        };

        let params = lookup.into_param_array();

        assert!(params.contains(&("max_results".to_string(), "1000".to_string())));
    }

    #[test]
    fn lookup_without_max_results() {
        let lookup = Lookup {
            country: "Brazil".to_string(),
            ..Default::default()
        };

        let params = lookup.into_param_array();

        assert!(!params.iter().any(|(k, _)| k == "max_results"));
    }

    #[test]
    fn candidate_test() {
        let payload = r#"[
            {
                "input_id": "ID-8675309",
                "administrative_area": "SP",
                "sub_administrative_area": "Greater Sao Paulo",
                "super_administrative_area": "Southeast",
                "country_iso_3": "BRA",
                "locality": "Sao Paulo",
                "dependent_locality": "Vila Guilherme",
                "dependent_locality_name": "Santana",
                "double_dependent_locality": "Zona Norte",
                "thoroughfare": "Rua Padre Antonio D'Angelo",
                "postal_code": "02516",
                "postal_code_extra": "050"
            }
        ]"#;

        let candidates: Vec<Candidate> = from_str(payload).expect("Failed to deserialize JSON");
        assert_eq!(candidates.len(), 1);

        let candidate = &candidates[0];
        assert_eq!(candidate.input_id, "ID-8675309");
        assert_eq!(candidate.country_iso_3, "BRA");
        assert_eq!(candidate.locality, "Sao Paulo");
        assert_eq!(candidate.dependent_locality, "Vila Guilherme");
        assert_eq!(candidate.dependent_locality_name, "Santana");
        assert_eq!(candidate.double_dependent_locality, "Zona Norte");
        assert_eq!(candidate.thoroughfare, "Rua Padre Antonio D'Angelo");
        assert_eq!(candidate.administrative_area, "SP");
        assert_eq!(candidate.sub_administrative_area, "Greater Sao Paulo");
        assert_eq!(candidate.super_administrative_area, "Southeast");
        assert_eq!(candidate.postal_code, "02516");
        assert_eq!(candidate.postal_code_extra, "050");
    }
}
