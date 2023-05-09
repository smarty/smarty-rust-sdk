use serde::Deserialize;
use serde::Serialize;
use crate::sdk::CoordinateLicense;

pub type Candidates = Vec<Candidate>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Candidate {
    pub input_id: String,
    pub input_index: i64,
    pub candidate_index: i64,
    pub adressee: String,
    pub delivery_line_1: String,
    pub delivery_line_2: String,
    pub last_line: String,
    pub delivery_point_barcode: String,
    pub components: Components,
    pub metadata: Metadata,
    pub analysis: Analysis,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Components {
    pub primary_number: String,
    pub street_prediction: String,
    pub street_name: String,
    pub street_postdirection: String,
    pub street_suffix: String,
    pub secondary_number: String,
    pub secondary_designator: String,
    pub extra_secondary_number: String,
    pub extra_secondary_designator: String,
    pub pmb_number: String,
    pub pmb_designator: String,
    pub city_name: String,
    pub default_city_name: String,
    pub state_abbreviation: String,
    pub zipcode: String,
    pub plus4_code: String,
    pub delivery_point: String,
    pub delivery_point_check_digit: String,
    pub urbanization: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Metadata {
    pub record_type: String,
    pub zip_type: String,
    pub county_fips: String,
    pub county_name: String,
    pub carrier_route: String,
    pub congressional_district: String,
    pub building_default_indicator: String,
    pub rdi: String,
    pub elot_sequence: String,
    pub elot_sort: String,
    pub latitude: f64,
    pub longitude: f64,
    pub coordinate_license: CoordinateLicense,
    pub precision: String,
    pub time_zone: String,
    pub utc_offset: f32,
    pub dst: bool,
    pub ews_match: bool
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Analysis {
    pub dpv_match_code: String,
    pub dpv_footnotes: String,
    pub dpv_cmra: String,
    pub dpv_vacant: String,
    pub dpv_no_stat: String,
    pub active: String,
    pub footnotes: String,
    pub lacslink_code: String,
    pub lacslink_indicator: String,
    pub suitelink_match: bool,
    pub ews_match: bool,
    pub enhanced_match: String,
}
