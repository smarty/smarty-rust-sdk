use reqwest::header::{USER_AGENT};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, RequestBuilder};
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use url::{ParseError, Url};
use crate::sdk::logging::LoggingMiddleware;
use crate::sdk::options::Options;
use crate::sdk::VERSION;

/// The base client for all of Smarty's rust sdk
pub struct Client {
    pub reqwest_client: ClientWithMiddleware,
    pub url: Url,
    pub options: Options,
}

impl Client {
    pub fn new(base_url: Url, options: Options, api_path: &str) -> Result<Client, ParseError> {
        let url = &mut base_url.join(api_path)?;

        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(options.num_retries);
        let mut client_builder = ClientBuilder::new(reqwest::Client::new())
            .with(RetryTransientMiddleware::new_with_policy(retry_policy));

        if options.logging_enabled {
            client_builder = client_builder.with(LoggingMiddleware);
        }

        let client = client_builder.build();

        let client = Client {
            reqwest_client: client,
            url: url.clone(),
            options
        };

        Ok(client)
    }

    pub(crate) fn build_request(&self, mut builder: RequestBuilder) -> RequestBuilder {

        builder = self.options.authentication.authenticate(builder);

        builder = builder.query(&[("license".to_string(), self.options.license.clone())]);

        for (header_key, header_value) in self.options.headers.clone() {
            builder = builder.header(header_key, header_value);
        }

        builder = builder.header(USER_AGENT.to_string(), format!("smarty (sdk:rust@{})", VERSION));

        builder
    }
}