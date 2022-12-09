use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use url::{ParseError, Url};
use crate::sdk::options::Options;

pub struct Client {
    pub reqwest_client: ClientWithMiddleware,
    pub url: Url,
}

impl Client {
    pub fn new(base_url: Url, options: Options, api_path: &str) -> Result<Client, ParseError> {
        let url = Url::parse((base_url.as_str().to_string() + api_path + "?" + options.clone().to_param_array().as_str()).as_str())?;

        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(options.num_retries);
        let client = ClientBuilder::new(reqwest::Client::new())
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();

        let client = Client {
            reqwest_client: client,
            url: url.clone()
        };

        Ok(client)
    }
}