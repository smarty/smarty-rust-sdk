use crate::sdk::CoordinateLicense;
use serde::Deserialize;
use serde::Serialize;

pub type Candidates = Vec<Candidate>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Candidate {
    pub input_id: String,
    pub input_index: i64,
    pub candidate_index: i64,
    pub addressee: String,
    pub delivery_line_1: String,
    pub delivery_line_2: String,
    pub last_line: String,
    pub delivery_point_barcode: String,
    pub smarty_key: String,
    pub components: Components,
    pub metadata: Metadata,
    pub analysis: Analysis,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Components {
    pub primary_number: String,
    pub street_name: String,
    pub street_predirection: String,
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
    pub urbanization: String,
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
    pub ews_match: bool,
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
    pub components: ComponentAnalysis,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MatchInfo {
    pub status: String,
    pub change: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ComponentAnalysis {
    pub primary_number: MatchInfo,
    pub street_predirection: MatchInfo,
    pub street_name: MatchInfo,
    pub street_postdirection: MatchInfo,
    pub street_suffix: MatchInfo,
    pub secondary_number: MatchInfo,
    pub secondary_designator: MatchInfo,
    pub extra_secondary_number: MatchInfo,
    pub extra_secondary_designator: MatchInfo,
    pub city_name: MatchInfo,
    pub state_abbreviation: MatchInfo,
    pub zipcode: MatchInfo,
    pub plus4_code: MatchInfo,
    pub urbanization: MatchInfo,
}
