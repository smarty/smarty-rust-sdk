use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Candidate {
    pub input_id: String,
    #[serde(flatten)]
    pub root_level: RootLevel,
    pub components: Components,
    pub metadata: Metadata,
    pub analysis: Analysis
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RootLevel {
    pub organization: String,
    pub address1: String,
    pub address2: String,
    pub address3: String,
    pub address4: String,
    pub address5: String,
    pub address6: String,
    pub address7: String,
    pub address8: String,
    pub address9: String,
    pub address10: String,
    pub address11: String,
    pub address12: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Components {
    pub super_administrative_area: String,
    pub administrative_area: String,
    pub administrative_area_short: String,
    pub administrative_area_long: String,
    pub sub_administrative_area: String,
    pub building: String,
    pub dependent_locality: String,
    pub dependent_locality_name: String,
    pub double_dependent_locality: String,
    pub country_iso3: String,
    pub locality: String,
    pub postal_code: String,
    pub postal_code_short: String,
    pub postal_code_extra: String,
    pub premise: String,
    pub premise_extra: String,
    pub premise_number: String,
    pub premise_type: String,
    pub premise_prefix_number: String,
    pub thoroughfare: String,
    pub thoroughfare_predirection: String,
    pub thoroughfare_postdirection: String,
    pub thoroughfare_name: String,
    pub thoroughfare_trailing_type: String,
    pub thoroughfare_type: String,
    pub dependent_thoroughfare: String,
    pub dependent_thoroughfare_predirection: String,
    pub dependent_thoroughfare_postdirection: String,
    pub dependent_thoroughfare_name: String,
    pub dependent_thoroughfare_trailing_type: String,
    pub dependent_thoroughfare_type: String,
    pub building_leading_type: String,
    pub building_name: String,
    pub building_trailing_type: String,
    pub sub_building_type: String,
    pub sub_building_number: String,
    pub sub_building_name: String,
    pub sub_building: String,
    pub level_type: String,
    pub level_number: String,
    pub post_box: String,
    pub post_box_type: String,
    pub post_box_number: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Metadata {
    pub latitude: f64,
    pub longitude: f64,
    pub geocode_precision: String,
    pub max_geocode_precision: String,
    pub address_format: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Analysis {
    pub verification_status: String,
    pub address_precision: String,
    pub max_address_precision: String,

    pub changes: Changes
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Changes {
    #[serde(flatten)]
    pub root_level: RootLevel,

    pub components: Components
}
