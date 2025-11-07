pub mod candidate;
pub mod client;
pub mod lookup;

#[cfg(test)]
mod tests {
    use crate::international_postal_code_api::candidate::Candidate;
    use crate::international_postal_code_api::client::InternationalPostalCodeClient;
    use crate::international_postal_code_api::lookup::Lookup;
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
            ..Default::default()
        };

        let expected_params = vec![
            ("input_id".to_string(), "ID-8675309".to_string()),
            ("country".to_string(), "Brazil".to_string()),
            ("locality".to_string(), "Sao Paulo".to_string()),
            ("administrative_area".to_string(), "SP".to_string()),
            ("postal_code".to_string(), "02516".to_string()),
        ];

        assert_eq!(lookup.into_param_array(), expected_params);
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
        assert_eq!(candidate.administrative_area, "SP");
        assert_eq!(candidate.sub_administrative_area, "Greater Sao Paulo");
        assert_eq!(candidate.super_administrative_area, "Southeast");
        assert_eq!(candidate.postal_code_short, "02516");
        assert_eq!(candidate.postal_code_extra, "050");
    }
}
