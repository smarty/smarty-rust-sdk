use crate::sdk::batch::Batch;
use crate::sdk::client::Client;
use reqwest::Method;
use smarty_rust_proc_macro::smarty_api;
use url::{ParseError, Url};

use crate::sdk::error::SDKError;
use crate::sdk::options::Options;
use crate::sdk::send_request;
use crate::us_street_api::candidate::Candidates;
use crate::us_street_api::lookup::Lookup;

#[smarty_api(
    default_url = "https://us-street.api.smartystreets.me/",
    api_path = "street-address",
    lookup_style(batch),
    lookup_type = "Lookup",
    result_type = "Candidates",
    result_handler(batch)
)]
pub struct USStreetAddressClient;

impl USStreetAddressClient {
    fn handle_batch_results(&self, batch: &mut Batch<Lookup>, results: Candidates) {
        let records = batch.records_mut();
        for result in results {
            records[result.input_index as usize].results.push(result);
        }
    }
}
