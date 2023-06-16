use crate::sdk::batch::Batch;
use crate::sdk::client::Client;
use crate::sdk::error::SDKError;
use crate::sdk::options::Options;
use crate::sdk::send_request;
use crate::us_zipcode_api::candidate::ZipcodeResult;
use crate::us_zipcode_api::lookup::Lookup;
use reqwest::Method;
use smarty_rust_proc_macro::smarty_api;
use url::{ParseError, Url};

#[smarty_api(
    default_url = "https://us-zipcode.api.smartystreets.me/",
    api_path = "lookup",
    lookup_style(batch),
    result_type = "ZipcodeResult",
    lookup_type = "Lookup",
    multi_result_type = "Vec<ZipcodeResult>"
)]
pub struct USZipcodeClient;
