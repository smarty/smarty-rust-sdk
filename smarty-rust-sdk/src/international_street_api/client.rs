use crate::international_street_api::candidate::Candidate;
use crate::international_street_api::lookup::Lookup;
use crate::sdk::client::Client;
use crate::sdk::error::SDKError;
use crate::sdk::options::Options;
use crate::sdk::send_request;
use reqwest::Method;
use smarty_rust_proc_macro::smarty_api;
use url::{ParseError, Url};

#[smarty_api(
    api_path = "verify",
    default_url = "https://international-street.api.smartystreets.me/",
    lookup_style(lookup),
    lookup_type = "Lookup",
    result_type = "Vec<Candidate>"
)]
pub struct InternationalStreetClient;
