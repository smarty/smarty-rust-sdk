use crate::sdk::client::Client;
use crate::sdk::error::SmartyError;
use crate::sdk::options::Options;
use crate::sdk::send_request;
use crate::us_reverse_geo_api::address::Results;
use crate::us_reverse_geo_api::lookup::Lookup;
use reqwest::Method;
use smarty_rust_proc_macro::smarty_api;

#[smarty_api(
    api_path = "lookup",
    default_url = "https://us-reverse-geo.api.smarty.com/",
    lookup_style(lookup),
    lookup_type = "Lookup",
    result_type = "Results"
)]
pub struct USReverseGeoClient;
