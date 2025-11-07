use crate::international_postal_code_api::candidate::Candidate;
use crate::sdk::has_param;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Lookup {
    pub input_id: String,
    pub country: String,
    pub locality: String,
    pub administrative_area: String,
    pub postal_code: String,

    pub results: Vec<Candidate>,
}

impl Lookup {
    pub(crate) fn into_param_array(self) -> Vec<(String, String)> {
        vec![
            has_param("input_id".to_string(), self.input_id),
            has_param("country".to_string(), self.country),
            has_param("locality".to_string(), self.locality),
            has_param("administrative_area".to_string(), self.administrative_area),
            has_param("postal_code".to_string(), self.postal_code),
        ]
        .into_iter()
        .filter_map(std::convert::identity)
        .collect()
    }
}
