pub mod client;
pub mod lookup;

pub mod candidate;

#[cfg(test)]
mod tests {
    use crate::sdk::batch::Batch;
    use crate::sdk::options::OptionsBuilder;
    use crate::us_street_api::client::USStreetAddressClient;
    use crate::us_street_api::lookup::{Lookup, MatchStrategy};
    use serde_json::json;
    use crate::us_street_api::candidate::Candidate;

    #[test]
    fn client_test() {
        let client = USStreetAddressClient::new(OptionsBuilder::new(None).build()).unwrap();

        assert_eq!(
            client.client.url.to_string(),
            "https://us-street.api.smarty.com/street-address".to_string()
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
            ("format".to_string(), "default".to_string()),
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
            ("format".to_string(), "default".to_string()),
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

    #[test]
    fn full_candidate_test_with_top_level_fields() {
        let data = json!({
            "input_id": "1234",
            "candidate_index": 0,
            "delivery_line_1": "1600 Amphitheatre Pkwy",
            "delivery_line_2": "Ste 100",
            "last_line": "Mountain View CA 94043-1351",
            "delivery_point_barcode": "940431351000",
            "components": {
                "urbanization": "URB",
                "primary_number": "1600",
                "street_name": "Amphitheatre",
                "street_predirection": "N",
                "street_postdirection": "W",
                "street_suffix": "Pkwy",
                "secondary_number": "100",
                "secondary_designator": "Ste",
                "extra_secondary_number": "200",
                "extra_secondary_designator": "Apt",
                "pmb_designator": "PMB",
                "pmb_number": "300",
                "city_name": "Mountain View",
                "default_city_name": "Mountain View",
                "state_abbreviation": "CA",
                "zipcode": "94043",
                "plus4_code": "1351",
                "delivery_point": "00",
                "delivery_point_check_digit": "1"
            },
            "metadata": {
                "record_type": "S",
                "zip_type": "Standard",
                "county_fips": "06085",
                "county_name": "Santa Clara",
                "carrier_route": "C001",
                "congressional_district": "18",
                "building_default_indicator": "Y",
                "rdi": "Residential",
                "elot_sequence": "0056",
                "elot_sort": "A",
                "latitude": 37.422,
                "longitude": -122.084,
                "precision": "Zip9",
                "time_zone": "Pacific",
                "utc_offset": -8.0,
                "dst": true,
                "ews_match": false
            },
            "analysis": {
                "dpv_match_code": "Y",
                "dpv_footnotes": "AABB",
                "dpv_cmra": "N",
                "dpv_vacant": "N",
                "dpv_no_stat": "N",
                "active": "Y",
                "ews_match": false,
                "footnotes": "N#",
                "lacslink_code": "L",
                "lacslink_indicator": "Y",
                "suitelink_match": true,
                "enhanced_match": "Y",
                "components": {
                    "primary_number": {
                        "status": "confirmed",
                        "change": ["added"]
                    },
                    "street_predirection": {
                        "status": "unconfirmed",
                        "change": ["spelling"]
                    },
                    "street_name": {
                        "status": "confirmed",
                        "change": ["replaced"]
                    },
                    "street_postdirection": {
                        "status": "confirmed",
                        "change": []
                    },
                    "street_suffix": {
                        "status": "confirmed",
                        "change": ["spelling"]
                    },
                    "secondary_number": {
                        "status": "unconfirmed",
                        "change": ["added"]
                    },
                    "secondary_designator": {
                        "status": "confirmed",
                        "change": ["replaced"]
                    },
                    "extra_secondary_number": {
                        "status": "confirmed",
                        "change": ["spelling"]
                    },
                    "extra_secondary_designator": {
                        "status": "confirmed",
                        "change": ["added"]
                    },
                    "city_name": {
                        "status": "unconfirmed",
                        "change": ["replaced"]
                    },
                    "state_abbreviation": {
                        "status": "confirmed",
                        "change": []
                    },
                    "zipcode": {
                        "status": "confirmed",
                        "change": ["spelling"]
                    },
                    "plus4_code": {
                        "status": "confirmed",
                        "change": ["added"]
                    },
                    "urbanization": {
                        "status": "unconfirmed",
                        "change": []
                    }
                }
            }
        });

        let candidate: Candidate = serde_json::from_value(data).unwrap();

        // Top-level
        assert_eq!(candidate.input_id, "1234");
        assert_eq!(candidate.candidate_index, 0);
        assert_eq!(candidate.delivery_line_1, "1600 Amphitheatre Pkwy");
        assert_eq!(candidate.delivery_line_2, "Ste 100");
        assert_eq!(candidate.last_line, "Mountain View CA 94043-1351");
        assert_eq!(candidate.delivery_point_barcode, "940431351000");

        // Components
        let c = &candidate.components;
        assert_eq!(c.urbanization, "URB");
        assert_eq!(c.primary_number, "1600");
        assert_eq!(c.street_name, "Amphitheatre");
        assert_eq!(c.street_predirection, "N");
        assert_eq!(c.street_postdirection, "W");
        assert_eq!(c.street_suffix, "Pkwy");
        assert_eq!(c.secondary_number, "100");
        assert_eq!(c.secondary_designator, "Ste");
        assert_eq!(c.extra_secondary_number, "200");
        assert_eq!(c.extra_secondary_designator, "Apt");
        assert_eq!(c.pmb_designator, "PMB");
        assert_eq!(c.pmb_number, "300");
        assert_eq!(c.city_name, "Mountain View");
        assert_eq!(c.default_city_name, "Mountain View");
        assert_eq!(c.state_abbreviation, "CA");
        assert_eq!(c.zipcode, "94043");
        assert_eq!(c.plus4_code, "1351");
        assert_eq!(c.delivery_point, "00");
        assert_eq!(c.delivery_point_check_digit, "1");

        // Metadata
        let m = &candidate.metadata;
        assert_eq!(m.record_type, "S");
        assert_eq!(m.zip_type, "Standard");
        assert_eq!(m.county_fips, "06085");
        assert_eq!(m.county_name, "Santa Clara");
        assert_eq!(m.carrier_route, "C001");
        assert_eq!(m.congressional_district, "18");
        assert_eq!(m.building_default_indicator, "Y");
        assert_eq!(m.rdi, "Residential");
        assert_eq!(m.elot_sequence, "0056");
        assert_eq!(m.elot_sort, "A");
        assert_eq!(m.latitude, 37.422);
        assert_eq!(m.longitude, -122.084);
        assert_eq!(m.precision, "Zip9");
        assert_eq!(m.time_zone, "Pacific");
        assert_eq!(m.utc_offset, -8.0);
        assert!(m.dst);
        assert!(!m.ews_match);

        // Analysis
        let a = &candidate.analysis;
        assert_eq!(a.dpv_match_code, "Y");
        assert_eq!(a.dpv_footnotes, "AABB");
        assert_eq!(a.dpv_cmra, "N");
        assert_eq!(a.dpv_vacant, "N");
        assert_eq!(a.active, "Y");
        assert!(!a.ews_match);
        assert_eq!(a.footnotes, "N#");
        assert_eq!(a.lacslink_code, "L");
        assert_eq!(a.lacslink_indicator, "Y");
        assert!(a.suitelink_match);
        assert_eq!(a.dpv_no_stat, "N");
        assert_eq!(a.enhanced_match, "Y");

        // Component Analysis (nested)
        let ca = &a.components;
        assert_eq!(ca.primary_number.status, "confirmed");
        assert_eq!(ca.primary_number.change, vec!["added"]);

        assert_eq!(ca.street_predirection.status, "unconfirmed");
        assert_eq!(ca.street_predirection.change, vec!["spelling"]);

        assert_eq!(ca.street_name.status, "confirmed");
        assert_eq!(ca.street_name.change, vec!["replaced"]);

        assert_eq!(ca.street_postdirection.status, "confirmed");
        assert!(ca.street_postdirection.change.is_empty());

        assert_eq!(ca.street_suffix.status, "confirmed");
        assert_eq!(ca.street_suffix.change, vec!["spelling"]);

        assert_eq!(ca.secondary_number.status, "unconfirmed");
        assert_eq!(ca.secondary_number.change, vec!["added"]);

        assert_eq!(ca.secondary_designator.status, "confirmed");
        assert_eq!(ca.secondary_designator.change, vec!["replaced"]);

        assert_eq!(ca.extra_secondary_number.status, "confirmed");
        assert_eq!(ca.extra_secondary_number.change, vec!["spelling"]);

        assert_eq!(ca.extra_secondary_designator.status, "confirmed");
        assert_eq!(ca.extra_secondary_designator.change, vec!["added"]);

        assert_eq!(ca.city_name.status, "unconfirmed");
        assert_eq!(ca.city_name.change, vec!["replaced"]);

        assert_eq!(ca.state_abbreviation.status, "confirmed");
        assert!(ca.state_abbreviation.change.is_empty());

        assert_eq!(ca.zipcode.status, "confirmed");
        assert_eq!(ca.zipcode.change, vec!["spelling"]);

        assert_eq!(ca.plus4_code.status, "confirmed");
        assert_eq!(ca.plus4_code.change, vec!["added"]);

        assert_eq!(ca.urbanization.status, "unconfirmed");
        assert!(ca.urbanization.change.is_empty());
    }
}
