use serde::de::DeserializeOwned;
use url::Url;

use crate::sdk::error::SmartyError;

pub trait EnrichmentRequest {
    type Response: DeserializeOwned + Default + Clone;

    fn validate(&self) -> Result<(), SmartyError>;

    fn build_url(&self, base: &Url) -> Result<Url, SmartyError>;

    fn etag(&self) -> &str;

    fn set_etag(&mut self, etag: String);

    fn params(&self) -> Vec<(String, String)>;

    fn apply_results(&mut self, results: Vec<Self::Response>) -> Result<(), SmartyError>;
}
