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
    custom_send = true
)]
pub struct InternationalStreetClient;

impl InternationalStreetClient {
    pub async fn send(&self, lookup: &mut Lookup) -> Result<(), SmartyError> {
        ensure_enough_info(lookup)?;

        let mut req = self
            .client
            .reqwest_client
            .request(Method::GET, self.client.url.clone());
        req = self.client.build_request(req);
        req = req.query(&lookup.clone().into_param_array());

        let candidates = send_request::<Vec<Candidate>>(req).await?;

        lookup.results = candidates;

        Ok(())
    }
}

pub(crate) fn ensure_enough_info(lookup: &Lookup) -> Result<(), SmartyError> {
    if lookup.country.is_empty() {
        return Err(SmartyError::ValidationError("country field is required".to_string()));
    }
    if lookup.freeform.is_empty() && lookup.address1.is_empty() {
        return Err(SmartyError::ValidationError("either freeform or address1 is required".to_string()));
    }
    Ok(())
}
