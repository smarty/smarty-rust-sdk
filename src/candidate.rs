//TODO: Add Json Functionality to structs

// Candidate contains all output fields defined here:
// https://smartystreets.com/docs/us-street-api#http-response-output
pub struct Candidate {
    input_id: String,
    input_index: i32,
    candidate_index: i32,
    addressee: String,
    delivery_line1: String,
    delivery_line2: String,
    last_line: String,
    delivery_point_barcode: String,
    components: Components,
    metadata: Metadata,
    analysis: Analysis,
}

// Components contains all output fields defined here:
// https://smartystreets.com/docs/us-street-api#components
pub struct Components {
    primary_number: String,
    street_predirection: String,
    street_name: String,
    street_postdirection: String,
    street_suffix: String,
    secondary_number: String,
    secondary_designator: String,
    extra_secondary_number: String,
    extra_secondary_designator: String,
    pmb_number: String,
    pmb_designator: String,
    city_name: String,
    default_city_name: String,
    state_abbreviation: String,
    zipcode: String,
    plus4code: String,
    delivery_point: String,
    delivery_point_check_digit: String,
    urbanization: String,
}

// Metadata contains all output fields defined here:
// https://smartystreets.com/docs/us-street-api#metadata
pub struct Metadata {
    record_type: String,
    zip_type: String,
    county_fips: String,
    county_name: String,
    carrier_route: String,
    congressional_district: String,
    building_default_indicator: String,
    rdi: String,
    elot_sequence: String,
    elot_sort: String,
    latitude: f64,
    longitude: f64,
    coordinate_license: u8,
    precision: String,
    time_zone: String,
    utcoffset: f32,
    dst: bool,
    ews_match: bool,
}

// Analysis contains all output fields defined here:
// https://smartystreets.com/docs/us-street-api#analysis
#[derive(Deserialize)]
pub struct Analysis {
    dpv_match_code: String,
    dpv_footnotes: String,
    dpv_cmracode: String,
    dpv_vacant_code: String,
    dpv_no_stat: String,
    active: String,
    footnotes: String,
    lacs_link_code: String,
    lacs_link_indicator: String,
    suite_link_match: bool,
    ews_match: bool,
    enhanced_match: String,
}