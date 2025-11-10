use crate::international_postal_code_api::candidate::Candidate;
use crate::international_postal_code_api::lookup::Lookup;
use crate::sdk::client::Client;
use crate::sdk::error::SmartyError;
use crate::sdk::options::Options;
use crate::sdk::send_request;
use reqwest::Method;
use smarty_rust_proc_macro::smarty_api;

#[smarty_api(
    api_path = "lookup",
    default_url = "https://international-postal-code.api.smarty.com/",
    lookup_style(lookup),
    lookup_type = "Lookup",
    result_type = "Vec<Candidate>"
)]
pub struct InternationalPostalCodeClient;
