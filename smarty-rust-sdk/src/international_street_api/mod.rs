pub mod candidate;
pub mod client;
pub mod lookup;


#[cfg(test)]
mod tests {
    use serde_json::from_str;
    use crate::international_street_api::client::InternationalStreetClient;
    use crate::international_street_api::lookup::Lookup;
    use crate::sdk::options::OptionsBuilder;
    use crate::international_street_api::candidate::*;


    #[test]
    fn client_test() {
        let client = InternationalStreetClient::new(OptionsBuilder::new(None).build()).unwrap();

        assert_eq!(
            client.client.url.to_string(),
            "https://international-street.api.smarty.com/verify".to_string()
        )
    }

    #[test]
    fn lookup_test() {
        let lookup = Lookup {
            geocode: false,
            organization: "John Doe".to_string(),
            address1: "Rua Padre Antonio D'Angelo 121".to_string(),
            address2: "Casa Verde".to_string(),
            locality: "Sao Paulo".to_string(),
            administrative_area: "SP".to_string(),
            country: "Brazil".to_string(),
            postal_code: "02516-050".to_string(),
            ..Default::default()
        };

        let expected_results = vec![
            ("country".to_string(), "Brazil".to_string()),
            ("geocode".to_string(), "false".to_string()),
            ("language".to_string(), "native".to_string()),
            (
                "address1".to_string(),
                "Rua Padre Antonio D'Angelo 121".to_string(),
            ),
            ("address2".to_string(), "Casa Verde".to_string()),
            ("organization".to_string(), "John Doe".to_string()),
            ("locality".to_string(), "Sao Paulo".to_string()),
            ("administrative_area".to_string(), "SP".to_string()),
            ("postal_code".to_string(), "02516-050".to_string()),
        ];

        assert_eq!(lookup.into_param_array(), expected_results);
    }

    #[test]
    fn candidate_test(){
        let response_payload = r#"[{
            "input_id": "12345678",
            "organization": "1",
            "address1": "2", "address2": "3", "address3": "4", "address4": "5",
            "address5": "6", "address6": "7", "address7": "8", "address8": "9",
            "address9": "10", "address10": "11", "address11": "12", "address12": "13",
            "components": {
                "country_iso_3": "14", "super_administrative_area": "15",
                "administrative_area": "16", "administrative_area_iso2": "16.1",
                "administrative_area_short": "16.2", "administrative_area_long": "16.3",
                "sub_administrative_area": "17", "dependent_locality": "18",
                "dependent_locality_name": "19", "double_dependent_locality": "20",
                "locality": "21", "postal_code": "22", "postal_code_short": "23",
                "postal_code_extra": "24", "premise": "25", "premise_extra": "26",
                "premise_number": "27", "premise_prefix_number": "27.5", "premise_type": "28",
                "thoroughfare": "29", "thoroughfare_predirection": "30", "thoroughfare_postdirection": "31",
                "thoroughfare_name": "32", "thoroughfare_trailing_type": "33", "thoroughfare_type": "34",
                "dependent_thoroughfare": "35", "dependent_thoroughfare_predirection": "36",
                "dependent_thoroughfare_postdirection": "37", "dependent_thoroughfare_name": "38",
                "dependent_thoroughfare_trailing_type": "39", "dependent_thoroughfare_type": "40",
                "building": "41", "building_leading_type": "42", "building_name": "43",
                "building_trailing_type": "44", "sub_building_type": "45", "sub_building_number": "46",
                "sub_building_name": "47", "sub_building": "48", "level_type": "48.1", "level_number": "48.2",
                "post_box": "49", "post_box_type": "50", "post_box_number": "51", "use_indicator": "52"
            },
            "metadata": {
                "latitude": 52.0, "longitude": 53.0,
                "geocode_precision": "54", "max_geocode_precision": "55",
                "address_format": "56"
            },
            "analysis": {
                "verification_status": "57", "address_precision": "58",
                "max_address_precision": "59",
                "changes": {
                    "organization": "60", "address1": "61", "address2": "62", "address3": "63",
                    "address4": "64", "address5": "65", "address6": "66", "address7": "67",
                    "address8": "68", "address9": "69", "address10": "70", "address11": "71",
                    "address12": "72",
                    "components": {
                        "super_administrative_area": "73", "administrative_area": "74",
                        "administrative_area_short": "74.1", "administrative_area_long": "74.2",
                        "sub_administrative_area": "75", "building": "76",
                        "dependent_locality": "77", "dependent_locality_name": "78",
                        "double_dependent_locality": "79", "country_iso_3": "80", "locality": "81",
                        "postal_code": "82", "postal_code_short": "83", "postal_code_extra": "84",
                        "premise": "85", "premise_extra": "86", "premise_number": "87",
                        "premise_type": "88", "premise_prefix_number": "89", "thoroughfare": "90",
                        "thoroughfare_predirection": "91", "thoroughfare_postdirection": "92",
                        "thoroughfare_name": "93", "thoroughfare_trailing_type": "94", "thoroughfare_type": "95",
                        "dependent_thoroughfare": "96", "dependent_thoroughfare_predirection": "97",
                        "dependent_thoroughfare_postdirection": "98", "dependent_thoroughfare_name": "99",
                        "dependent_thoroughfare_trailing_type": "100", "dependent_thoroughfare_type": "101",
                        "building_leading_type": "102", "building_name": "103", "building_trailing_type": "104",
                        "sub_building_type": "105", "sub_building_number": "106", "sub_building_name": "107",
                        "sub_building": "108", "level_type": "108.1", "level_number": "108.2",
                        "post_box": "109", "post_box_type": "110", "post_box_number": "111",
                        "additional_content": "112", "delivery_installation": "113",
                        "delivery_installation_type": "114", "delivery_installation_qualifier_name": "115",
                        "route": "116", "route_number": "117", "route_type": "118", "use_indicator": "119"
                    }
                }
            }
        }]"#;
    
        let candidates: Vec<Candidate> = from_str(response_payload).expect("Failed to deserialize JSON");
        let candidate = &candidates[0];

        assert_eq!(candidate.input_id, "12345678");
        assert_eq!(candidate.root_level.organization, "1");
        assert_eq!(candidate.root_level.address1, "2");
        assert_eq!(candidate.root_level.address2, "3");
        assert_eq!(candidate.root_level.address3, "4");
        assert_eq!(candidate.root_level.address4, "5");
        assert_eq!(candidate.root_level.address5, "6");
        assert_eq!(candidate.root_level.address6, "7");
        assert_eq!(candidate.root_level.address7, "8");
        assert_eq!(candidate.root_level.address8, "9");
        assert_eq!(candidate.root_level.address9, "10");
        assert_eq!(candidate.root_level.address10, "11");
        assert_eq!(candidate.root_level.address11, "12");
        assert_eq!(candidate.root_level.address12, "13");
        assert_eq!(candidate.components.country_iso_3, "14");
        assert_eq!(candidate.components.super_administrative_area, "15");
        assert_eq!(candidate.components.administrative_area, "16");
        assert_eq!(candidate.components.administrative_area_iso2, "16.1");
        assert_eq!(candidate.components.administrative_area_short, "16.2");
        assert_eq!(candidate.components.administrative_area_long, "16.3");
        assert_eq!(candidate.components.sub_administrative_area, "17");
        assert_eq!(candidate.components.dependent_locality, "18");
        assert_eq!(candidate.components.dependent_locality_name, "19");
        assert_eq!(candidate.components.double_dependent_locality, "20");
        assert_eq!(candidate.components.locality, "21");
        assert_eq!(candidate.components.postal_code, "22");
        assert_eq!(candidate.components.postal_code_short, "23");
        assert_eq!(candidate.components.postal_code_extra, "24");
        assert_eq!(candidate.components.premise, "25");
        assert_eq!(candidate.components.premise_extra, "26");
        assert_eq!(candidate.components.premise_number, "27");
        assert_eq!(candidate.components.premise_prefix_number, "27.5");
        assert_eq!(candidate.components.premise_type, "28");
        assert_eq!(candidate.components.thoroughfare, "29");
        assert_eq!(candidate.components.thoroughfare_predirection, "30");
        assert_eq!(candidate.components.thoroughfare_postdirection, "31");
        assert_eq!(candidate.components.thoroughfare_name, "32");
        assert_eq!(candidate.components.thoroughfare_trailing_type, "33");
        assert_eq!(candidate.components.thoroughfare_type, "34");
        assert_eq!(candidate.components.dependent_thoroughfare, "35");
        assert_eq!(candidate.components.dependent_thoroughfare_predirection, "36");
        assert_eq!(candidate.components.dependent_thoroughfare_postdirection, "37");
        assert_eq!(candidate.components.dependent_thoroughfare_name, "38");
        assert_eq!(candidate.components.dependent_thoroughfare_trailing_type, "39");
        assert_eq!(candidate.components.dependent_thoroughfare_type, "40");
        assert_eq!(candidate.components.building, "41");
        assert_eq!(candidate.components.building_leading_type, "42");
        assert_eq!(candidate.components.building_name, "43");
        assert_eq!(candidate.components.building_trailing_type, "44");
        assert_eq!(candidate.components.sub_building_type, "45");
        assert_eq!(candidate.components.sub_building_number, "46");
        assert_eq!(candidate.components.sub_building_name, "47");
        assert_eq!(candidate.components.sub_building, "48");
        assert_eq!(candidate.components.level_type, "48.1");
        assert_eq!(candidate.components.level_number, "48.2");
        assert_eq!(candidate.components.post_box, "49");
        assert_eq!(candidate.components.post_box_type, "50");
        assert_eq!(candidate.components.post_box_number, "51");
        assert_eq!(candidate.components.use_indicator, "52");
        assert_eq!(candidate.metadata.latitude, 52.0);
        assert_eq!(candidate.metadata.longitude, 53.0);
        assert_eq!(candidate.metadata.geocode_precision, "54");
        assert_eq!(candidate.metadata.max_geocode_precision, "55");
        assert_eq!(candidate.metadata.address_format, "56");
        assert_eq!(candidate.analysis.verification_status, "57");
        assert_eq!(candidate.analysis.address_precision, "58");
        assert_eq!(candidate.analysis.max_address_precision, "59");
        assert_eq!(candidate.analysis.changes.root_level.organization, "60");
        assert_eq!(candidate.analysis.changes.root_level.address1, "61");
        assert_eq!(candidate.analysis.changes.root_level.address2, "62");
        assert_eq!(candidate.analysis.changes.root_level.address3, "63");
        assert_eq!(candidate.analysis.changes.root_level.address4, "64");
        assert_eq!(candidate.analysis.changes.root_level.address5, "65");
        assert_eq!(candidate.analysis.changes.root_level.address6, "66");
        assert_eq!(candidate.analysis.changes.root_level.address7, "67");
        assert_eq!(candidate.analysis.changes.root_level.address8, "68");
        assert_eq!(candidate.analysis.changes.root_level.address9, "69");
        assert_eq!(candidate.analysis.changes.root_level.address10, "70");
        assert_eq!(candidate.analysis.changes.root_level.address11, "71");
        assert_eq!(candidate.analysis.changes.root_level.address12, "72");
        assert_eq!(candidate.analysis.changes.components.super_administrative_area, "73");
        assert_eq!(candidate.analysis.changes.components.administrative_area, "74");
        assert_eq!(candidate.analysis.changes.components.administrative_area_short, "74.1");
        assert_eq!(candidate.analysis.changes.components.administrative_area_long, "74.2");
        assert_eq!(candidate.analysis.changes.components.sub_administrative_area, "75");
        assert_eq!(candidate.analysis.changes.components.building, "76");
        assert_eq!(candidate.analysis.changes.components.dependent_locality, "77");
        assert_eq!(candidate.analysis.changes.components.dependent_locality_name, "78");
        assert_eq!(candidate.analysis.changes.components.double_dependent_locality, "79");
        assert_eq!(candidate.analysis.changes.components.country_iso_3, "80");
        assert_eq!(candidate.analysis.changes.components.locality, "81");
        assert_eq!(candidate.analysis.changes.components.postal_code, "82");
        assert_eq!(candidate.analysis.changes.components.postal_code_short, "83");
        assert_eq!(candidate.analysis.changes.components.postal_code_extra, "84");
        assert_eq!(candidate.analysis.changes.components.premise, "85");
        assert_eq!(candidate.analysis.changes.components.premise_extra, "86");
        assert_eq!(candidate.analysis.changes.components.premise_number, "87");
        assert_eq!(candidate.analysis.changes.components.premise_type, "88");
        assert_eq!(candidate.analysis.changes.components.premise_prefix_number, "89");
        assert_eq!(candidate.analysis.changes.components.thoroughfare, "90");
        assert_eq!(candidate.analysis.changes.components.thoroughfare_predirection, "91");
        assert_eq!(candidate.analysis.changes.components.thoroughfare_postdirection, "92");
        assert_eq!(candidate.analysis.changes.components.thoroughfare_name, "93");
        assert_eq!(candidate.analysis.changes.components.thoroughfare_trailing_type, "94");
        assert_eq!(candidate.analysis.changes.components.thoroughfare_type, "95");
        assert_eq!(candidate.analysis.changes.components.dependent_thoroughfare, "96");
        assert_eq!(candidate.analysis.changes.components.dependent_thoroughfare_predirection, "97");
        assert_eq!(candidate.analysis.changes.components.dependent_thoroughfare_postdirection, "98");
        assert_eq!(candidate.analysis.changes.components.dependent_thoroughfare_name, "99");
        assert_eq!(candidate.analysis.changes.components.dependent_thoroughfare_trailing_type, "100");
        assert_eq!(candidate.analysis.changes.components.dependent_thoroughfare_type, "101");
        assert_eq!(candidate.analysis.changes.components.building_leading_type, "102");
        assert_eq!(candidate.analysis.changes.components.building_name, "103");
        assert_eq!(candidate.analysis.changes.components.building_trailing_type, "104");
        assert_eq!(candidate.analysis.changes.components.sub_building_type, "105");
        assert_eq!(candidate.analysis.changes.components.sub_building_number, "106");
        assert_eq!(candidate.analysis.changes.components.sub_building_name, "107");
        assert_eq!(candidate.analysis.changes.components.sub_building, "108");
        assert_eq!(candidate.analysis.changes.components.level_type, "108.1");
        assert_eq!(candidate.analysis.changes.components.level_number, "108.2");
        assert_eq!(candidate.analysis.changes.components.post_box, "109");
        assert_eq!(candidate.analysis.changes.components.post_box_type, "110");
        assert_eq!(candidate.analysis.changes.components.post_box_number, "111");
        assert_eq!(candidate.analysis.changes.components.additional_content, "112");
        assert_eq!(candidate.analysis.changes.components.delivery_installation, "113");
        assert_eq!(candidate.analysis.changes.components.delivery_installation_type, "114");
        assert_eq!(candidate.analysis.changes.components.delivery_installation_qualifier_name, "115");
        assert_eq!(candidate.analysis.changes.components.route, "116");
        assert_eq!(candidate.analysis.changes.components.route_number, "117");
        assert_eq!(candidate.analysis.changes.components.route_type, "118");
        assert_eq!(candidate.analysis.changes.components.use_indicator, "119");
    }
}
