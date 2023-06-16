use crate::sdk::client::Client;
use crate::sdk::error::SDKError;
use crate::sdk::options::Options;
use crate::sdk::send_request;
use crate::us_extract_api::lookup::Lookup;
use reqwest::Method;
use smarty_rust_proc_macro::smarty_api;
use url::{ParseError, Url};

#[smarty_api(
    api_path = "",
    default_url = "https://us-extract.api.smartystreets.me/",
    lookup_style(lookup),
    lookup_type = "Lookup",
    result_type = "ExtractionResult",
    custom_send
)]
pub struct USExtractClient;

impl USExtractClient {
    pub async fn send(&self, lookup: &mut Lookup) -> Result<(), SDKError> {
        let mut req = self
            .client
            .reqwest_client
            .request(Method::POST, self.client.url.clone());
        req = self.client.build_request(req);
        req = req.header("Content-Type", "text/plain");
        req = req.query(&lookup.clone().into_param_array());
        req = req.body(lookup.text.clone());

        let response = send_request(req).await?;

        lookup.result = response;

        Ok(())
    }
}
