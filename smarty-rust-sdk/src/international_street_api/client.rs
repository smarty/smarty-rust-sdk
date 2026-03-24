use crate::international_street_api::candidate::Candidate;
use crate::international_street_api::lookup::Lookup;
use crate::sdk::client::Client;
use crate::sdk::error::SmartyError;
use crate::sdk::options::Options;
use crate::sdk::send_request;
use reqwest::Method;
use smarty_rust_proc_macro::smarty_api;

#[smarty_api(
    api_path = "verify",
    default_url = "https://international-street.api.smarty.com/",
    lookup_style(lookup),
    lookup_type = "Lookup",
    result_type = "Vec<Candidate>",
    verify_func = "ensure_enough_info"
)]
pub struct InternationalStreetClient;

pub(crate) fn ensure_enough_info(lookup: &Lookup) -> Result<(), SmartyError> {
    if lookup.country.is_empty() {
        return Err(SmartyError::ValidationError(
            "country field is required".to_string(),
        ));
    }
    if lookup.freeform.is_empty() && lookup.address1.is_empty() {
        return Err(SmartyError::ValidationError(
            "either freeform or address1 is required".to_string(),
        ));
    }
    Ok(())
}
