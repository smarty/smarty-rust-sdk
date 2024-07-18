use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;
use serde::de::DeserializeOwned;

pub trait EnrichmentResponse: Clone + Serialize + DeserializeOwned {
    fn lookup_type() -> &'static str;
    fn lookup_subtype() -> &'static str;
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrincipalResponse {
    pub smarty_key: String,
    pub data_set_name: String,
    pub data_subset_name: String,
    pub attributes: PrincipalAttributes,
}

impl EnrichmentResponse for PrincipalResponse {
    fn lookup_type() -> &'static str {
        "property"
    }
    fn lookup_subtype() -> &'static str {
        "principal"
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PrincipalAttributes {
    #[serde(rename = "1st_floor_sqft")]
    pub first_floor_sqft: String,
    #[serde(rename = "2nd_floor_sqft")]
    pub second_floor_sqft: String,
    pub acres: String,
    pub air_conditioner: String,
    pub arbor_pergola: String,
    pub assessed_improvement_percent: String,
    pub assessed_improvement_value: String,
    pub assessed_land_value: String,
    pub assessed_value: String,
    pub assessor_last_update: String,
    pub assessor_taxroll_update: String,
    pub attic_area: String,
    pub attic_flag: String,
    pub balcony: String,
    pub balcony_area: String,
    pub basement_sqft: String,
    pub basement_sqft_finished: String,
    pub basement_sqft_unfinished: String,
    pub bath_house: String,
    pub bath_house_sqft: String,
    pub bathrooms_partial: String,
    pub bathrooms_total: String,
    pub bedrooms: String,
    pub block1: String,
    pub block2: String,
    pub boat_access: String,
    pub boat_house: String,
    pub boat_house_sqft: String,
    pub boat_lift: String,
    pub bonus_room: String,
    pub breakfast_nook: String,
    pub breezeway: String,
    pub building_definition_code: String,
    pub building_sqft: String,
    pub cabin: String,
    pub cabin_sqft: String,
    pub canopy: String,
    pub canopy_sqft: String,
    pub carport: String,
    pub carport_sqft: String,
    pub cbsa_code: String,
    pub cbsa_name: String,
    pub cellar: String,
    pub census_block: String,
    pub census_block_group: String,
    pub census_fips_place_code: String,
    pub census_tract: String,
    pub central_vacuum: String,
    pub code_title_company: String,
    pub combined_statistical_area: String,
    pub community_rec: String,
    pub company_flag: String,
    pub congressional_district: String,
    pub construction_type: String,
    pub contact_city: String,
    pub contact_crrt: String,
    pub contact_full_address: String,
    pub contact_house_number: String,
    pub contact_mail_info_format: String,
    pub contact_mail_info_privacy: String,
    pub contact_mailing_county: String,
    pub contact_mailing_fips: String,
    pub contact_post_direction: String,
    pub contact_pre_direction: String,
    pub contact_state: String,
    pub contact_street_name: String,
    pub contact_suffix: String,
    pub contact_unit_designator: String,
    pub contact_value: String,
    pub contact_zip: String,
    pub contact_zip4: String,
    pub courtyard: String,
    pub courtyard_area: String,
    pub deck: String,
    pub deck_area: String,
    pub deed_document_page: String,
    pub deed_document_book: String,
    pub deed_document_number: String,
    pub deed_owner_first_name: String,
    pub deed_owner_first_name2: String,
    pub deed_owner_first_name3: String,
    pub deed_owner_first_name4: String,
    pub deed_owner_full_name: String,
    pub deed_owner_full_name2: String,
    pub deed_owner_full_name3: String,
    pub deed_owner_full_name4: String,
    pub deed_owner_last_name: String,
    pub deed_owner_last_name2: String,
    pub deed_owner_last_name3: String,
    pub deed_owner_last_name4: String,
    pub deed_owner_middle_name: String,
    pub deed_owner_middle_name2: String,
    pub deed_owner_middle_name3: String,
    pub deed_owner_middle_name4: String,
    pub deed_owner_suffix: String,
    pub deed_owner_suffix2: String,
    pub deed_owner_suffix3: String,
    pub deed_owner_suffix4: String,
    pub deed_sale_date: String,
    pub deed_sale_price: String,
    pub deed_transaction_id: String,
    pub depth_linear_footage: String,
    pub disabled_tax_exemption: String,
    pub document_type_description: String,
    pub driveway_sqft: String,
    pub driveway_type: String,
    pub effective_year_built: String,
    pub elevation_feet: String,
    pub elevator: String,
    pub equestrian_arena: String,
    pub escalator: String,
    pub exercise_room: String,
    pub exterior_walls: String,
    pub family_room: String,
    pub fence: String,
    pub fence_area: String,
    pub fips_code: String,
    pub fire_resistance_code: String,
    pub fire_sprinklers_flag: String,
    pub fireplace: String,
    pub fireplace_number: String,
    pub first_name: String,
    pub first_name_2: String,
    pub first_name_3: String,
    pub first_name_4: String,
    pub flooring: String,
    pub foundation: String,
    pub game_room: String,
    pub garage: String,
    pub garage_sqft: String,
    pub gazebo: String,
    pub gazebo_sqft: String,
    pub golf_course: String,
    pub grainery: String,
    pub grainery_sqft: String,
    pub great_room: String,
    pub greenhouse: String,
    pub greenhouse_sqft: String,
    pub gross_sqft: String,
    pub guesthouse: String,
    pub guesthouse_sqft: String,
    pub handicap_accessibility: String,
    pub heat: String,
    pub heat_fuel_type: String,
    pub hobby_room: String,
    pub homeowner_tax_exemption: String,
    pub instrument_date: String,
    pub intercom_system: String,
    pub interest_rate_type_2: String,
    pub interior_structure: String,
    pub kennel: String,
    pub kennel_sqft: String,
    pub land_use_code: String,
    pub land_use_group: String,
    pub land_use_standard: String,
    pub last_name: String,
    pub last_name_2: String,
    pub last_name_3: String,
    pub last_name_4: String,
    pub latitude: String,
    pub laundry: String,
    pub lean_to: String,
    pub lean_to_sqft: String,
    pub legal_description: String,
    pub legal_unit: String,
    pub lender_address: String,
    pub lender_address_2: String,
    pub lender_city: String,
    pub lender_city_2: String,
    pub lender_code_2: String,
    pub lender_first_name: String,
    pub lender_first_name_2: String,
    pub lender_last_name: String,
    pub lender_last_name_2: String,
    pub lender_name: String,
    pub lender_name_2: String,
    pub lender_seller_carry_back: String,
    pub lender_seller_carry_back_2: String,
    pub lender_state: String,
    pub lender_state_2: String,
    pub lender_zip: String,
    pub lender_zip_2: String,
    pub lender_zip_extended: String,
    pub lender_zip_extended_2: String,
    pub loading_platform: String,
    pub loading_platform_sqft: String,
    pub longitude: String,
    pub lot_1: String,
    pub lot_2: String,
    pub lot_3: String,
    pub lot_sqft: String,
    pub market_improvement_percent: String,
    pub market_improvement_value: String,
    pub market_land_value: String,
    pub market_value_year: String,
    pub match_type: String,
    pub media_room: String,
    pub metro_division: String,
    pub middle_name: String,
    pub middle_name_2: String,
    pub middle_name_3: String,
    pub middle_name_4: String,
    pub milkhouse: String,
    pub milkhouse_sqft: String,
    pub minor_civil_division_code: String,
    pub minor_civil_division_name: String,
    pub mobile_home_hookup: String,
    pub mortgage_amount: String,
    pub mortgage_amount_2: String,
    pub mortgage_due_date: String,
    pub mortgage_due_date_2: String,
    pub mortgage_interest_rate: String,
    pub mortgage_interest_rate_type: String,
    pub mortgage_lender_code: String,
    pub mortgage_rate_2: String,
    pub mortgage_recording_date: String,
    pub mortgage_recording_date_2: String,
    pub mortgage_term: String,
    pub mortgage_term_2: String,
    pub mortgage_term_type: String,
    pub mortgage_term_type_2: String,
    pub mortgage_type: String,
    pub mortgage_type_2: String,
    pub msa_code: String,
    pub msa_name: String,
    pub mud_room: String,
    pub multi_parcel_flag: String,
    pub name_title_company: String,
    pub neighborhood_code: String,
    pub number_of_buildings: String,
    pub office: String,
    pub office_sqft: String,
    pub other_tax_exemption: String,
    pub outdoor_kitchen_fireplace: String,
    pub overhead_door: String,
    pub owner_full_name: String,
    pub owner_full_name_2: String,
    pub owner_full_name_3: String,
    pub owner_full_name_4: String,
    pub owner_occupancy_status: String,
    pub ownership_transfer_date: String,
    pub ownership_transfer_doc_number: String,
    pub ownership_transfer_transaction_id: String,
    pub ownership_type: String,
    pub ownership_type_2: String,
    pub ownership_vesting_relation_code: String,
    pub parcel_account_number: String,
    pub parcel_map_book: String,
    pub parcel_map_page: String,
    pub parcel_number_alternate: String,
    pub parcel_number_formatted: String,
    pub parcel_number_previous: String,
    pub parcel_number_year_added: String,
    pub parcel_number_year_change: String,
    pub parcel_raw_number: String,
    pub parcel_shell_record: String,
    pub parking_spaces: String,
    pub patio_area: String,
    pub phase_name: String,
    pub plumbing_fixtures_count: String,
    pub pole_struct: String,
    pub pole_struct_sqft: String,
    pub pond: String,
    pub pool: String,
    pub pool_area: String,
    pub poolhouse: String,
    pub poolhouse_sqft: String,
    pub porch: String,
    pub porch_area: String,
    pub poultry_house: String,
    pub poultry_house_sqft: String,
    pub previous_assessed_value: String,
    pub prior_sale_amount: String,
    pub prior_sale_date: String,
    pub property_address_carrier_route_code: String,
    pub property_address_city: String,
    pub property_address_full: String,
    pub property_address_house_number: String,
    pub property_address_post_direction: String,
    pub property_address_pre_direction: String,
    pub property_address_state: String,
    pub property_address_street_name: String,
    pub property_address_street_suffix: String,
    pub property_address_unit_designator: String,
    pub property_address_unit_value: String,
    pub property_address_zip_4: String,
    pub property_address_zipcode: String,
    pub publication_date: String,
    pub quarter: String,
    pub quarter_quarter: String,
    pub quonset: String,
    pub quonset_sqft: String,
    pub range: String,
    pub recording_date: String,
    pub roof_cover: String,
    pub roof_frame: String,
    pub rooms: String,
    pub rv_parking: String,
    pub safe_room: String,
    pub sale_amount: String,
    pub sale_date: String,
    pub sauna: String,
    pub section: String,
    pub security_alarm: String,
    pub senior_tax_exemption: String,
    pub sewer_type: String,
    pub shed: String,
    pub shed_sqft: String,
    pub silo: String,
    pub silo_sqft: String,
    pub sitting_room: String,
    pub situs_county: String,
    pub situs_state: String,
    pub sound_system: String,
    pub sports_court: String,
    pub sprinklers: String,
    pub stable: String,
    pub stable_sqft: String,
    pub storage_building: String,
    pub storage_building_sqft: String,
    pub stories_number: String,
    pub storm_shelter: String,
    pub storm_shutter: String,
    pub structure_style: String,
    pub study: String,
    pub subdivision: String,
    pub suffix: String,
    pub suffix_2: String,
    pub suffix_3: String,
    pub suffix_4: String,
    pub sunroom: String,
    pub tax_assess_year: String,
    pub tax_billed_amount: String,
    pub tax_delinquent_year: String,
    pub tax_fiscal_year: String,
    pub tax_jurisdiction: String,
    pub tax_rate_area: String,
    pub tennis_court: String,
    pub topography_code: String,
    pub total_market_value: String,
    pub township: String,
    pub tract_number: String,
    pub transfer_amount: String,
    pub trust_description: String,
    pub unit_count: String,
    pub upper_floors_sqft: String,
    pub utility: String,
    pub utility_building: String,
    pub utility_building_sqft: String,
    pub utility_sqft: String,
    pub veteran_tax_exemption: String,
    pub view_description: String,
    pub water_feature: String,
    pub water_service_type: String,
    pub wet_bar: String,
    pub widow_tax_exemption: String,
    pub width_linear_footage: String,
    pub wine_cellar: String,
    pub year_built: String,
    pub zoning: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FinancialAttributes {
    pub assessed_improvement_percent: String,
    pub assessed_improvement_value: String,
    pub assessed_land_value: String,
    pub assessed_value: String,
    pub assessor_last_update: String,
    pub assessor_taxroll_update: String,
    pub contact_city: String,
    pub contact_crrt: String,
    pub contact_full_address: String,
    pub contact_house_number: String,
    pub contact_mail_info_format: String,
    pub contact_mail_info_privacy: String,
    pub contact_mailing_county: String,
    pub contact_mailing_fips: String,
    pub contact_post_direction: String,
    pub contact_pre_direction: String,
    pub contact_state: String,
    pub contact_street_name: String,
    pub contact_suffix: String,
    pub contact_unit_designator: String,
    pub contact_value: String,
    pub contact_zip: String,
    pub contact_zip4: String,
    pub deed_document_page: String,
    pub deed_document_book: String,
    pub deed_document_number: String,
    pub deed_owner_first_name: String,
    pub deed_owner_first_name2: String,
    pub deed_owner_first_name3: String,
    pub deed_owner_first_name4: String,
    pub deed_owner_full_name: String,
    pub deed_owner_full_name2: String,
    pub deed_owner_full_name3: String,
    pub deed_owner_full_name4: String,
    pub deed_owner_last_name: String,
    pub deed_owner_last_name2: String,
    pub deed_owner_last_name3: String,
    pub deed_owner_last_name4: String,
    pub deed_owner_middle_name: String,
    pub deed_owner_middle_name2: String,
    pub deed_owner_middle_name3: String,
    pub deed_owner_middle_name4: String,
    pub deed_owner_suffix: String,
    pub deed_owner_suffix2: String,
    pub deed_owner_suffix3: String,
    pub deed_owner_suffix4: String,
    pub deed_sale_date: String,
    pub deed_sale_price: String,
    pub deed_transaction_id: String,
    pub disabled_tax_exemption: String,
    pub financial_history: Vec<FinancialHistory>,
    pub first_name: String,
    pub first_name_2: String,
    pub first_name_3: String,
    pub first_name_4: String,
    pub homeowner_tax_exemption: String,
    pub last_name: String,
    pub last_name_2: String,
    pub last_name_3: String,
    pub last_name_4: String,
    pub market_improvement_percent: String,
    pub market_improvement_value: String,
    pub market_land_value: String,
    pub market_value_year: String,
    pub match_type: String,
    pub middle_name: String,
    pub middle_name_2: String,
    pub middle_name_3: String,
    pub middle_name_4: String,
    pub other_tax_exemption: String,
    pub owner_full_name: String,
    pub owner_full_name_2: String,
    pub owner_full_name_3: String,
    pub owner_full_name_4: String,
    pub ownership_transfer_date: String,
    pub ownership_transfer_doc_number: String,
    pub ownership_transfer_transaction_id: String,
    pub ownership_type: String,
    pub ownership_type_2: String,
    pub previous_assessed_value: String,
    pub prior_sale_amount: String,
    pub prior_sale_date: String,
    pub sale_amount: String,
    pub sale_date: String,
    pub senior_tax_exemption: String,
    pub suffix: String,
    pub suffix_2: String,
    pub suffix_3: String,
    pub suffix_4: String,
    pub tax_assess_year: String,
    pub tax_billed_amount: String,
    pub tax_delinquent_year: String,
    pub tax_fiscal_year: String,
    pub tax_rate_area: String,
    pub total_market_value: String,
    pub trust_description: String,
    pub veteran_tax_exemption: String,
    pub widow_tax_exemption: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FinancialHistory {
    pub code_title_company: String,
    pub document_type_description: String,
    pub instrument_date: String,
    pub interest_rate_type_2: String,
    pub lender_address: String,
    pub lender_address_2: String,
    pub lender_city: String,
    pub lender_city_2: String,
    pub lender_code_2: String,
    pub lender_first_name: String,
    pub lender_first_name_2: String,
    pub lender_last_name: String,
    pub lender_last_name_2: String,
    pub lender_name: String,
    pub lender_name_2: String,
    pub lender_seller_carry_back: String,
    pub lender_seller_carry_back_2: String,
    pub lender_state: String,
    pub lender_state_2: String,
    pub lender_zip: String,
    pub lender_zip_2: String,
    pub lender_zip_extended: String,
    pub lender_zip_extended_2: String,
    pub mortgage_amount: String,
    pub mortgage_amount_2: String,
    pub mortgage_due_date: String,
    pub mortgage_due_date_2: String,
    pub mortgage_interest_rate: String,
    pub mortgage_interest_rate_type: String,
    pub mortgage_lender_code: String,
    pub mortgage_rate_2: String,
    pub mortgage_recording_date: String,
    pub mortgage_recording_date_2: String,
    pub mortgage_term: String,
    pub mortgage_term_2: String,
    pub mortgage_term_type: String,
    pub mortgage_term_type_2: String,
    pub mortgage_type: String,
    pub mortgage_type_2: String,
    pub multi_parcel_flag: String,
    pub name_title_company: String,
    pub recording_date: String,
    pub transfer_amount: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialResponse {
    pub smarty_key: String,
    pub data_set_name: String,
    pub data_subset_name: String,
    pub attributes: FinancialAttributes,
}

impl EnrichmentResponse for FinancialResponse {
    fn lookup_type() -> &'static str {
        "property"
    }
    fn lookup_subtype() -> &'static str {
        "financial"
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecondaryResponse {
    pub smarty_key: String,
    pub root_address: RootAddress,
    pub aliases: Vec<Alias>,
    pub secondaries: Vec<Secondary>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RootAddress {
    pub secondary_count: i64,
    pub smarty_key: String,
    pub primary_number: String,
    pub street_name: String,
    pub street_suffix: String,
    pub street_postdirection: String,
    pub city_name: String,
    pub state_abbreviation: String,
    pub zipcode: String,
    pub plus4_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Alias {
    pub smarty_key: String,
    pub primary_number: String,
    pub street_name: String,
    pub street_suffix: String,
    pub street_postdirection: String,
    pub city_name: String,
    pub state_abbreviation: String,
    pub zipcode: String,
    pub plus4_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Secondary {
    pub smarty_key: String,
    pub secondary_designator: String,
    pub secondary_number: String,
    pub plus4_code: String,
}

impl EnrichmentResponse for SecondaryResponse {
    fn lookup_type() -> &'static str {
        "secondary"
    }
    fn lookup_subtype() ->&'static str {
        ""
    }
}
