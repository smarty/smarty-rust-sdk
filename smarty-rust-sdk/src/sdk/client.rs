use crate::sdk::logging::LoggingMiddleware;
use crate::sdk::options::Options;
use crate::sdk::VERSION;
use reqwest::header::USER_AGENT;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, RequestBuilder};
use url::Url;

use super::{error::SmartyError, retry_strategy::SmartyRetryMiddleware};

/// The base client for all of Smarty's rust sdk
pub(crate) struct Client {
    pub(crate) reqwest_client: ClientWithMiddleware,
    pub(crate) url: Url,
    pub(crate) options: Options,
}

impl Client {
    pub(crate) fn new(
        base_url: Url,
        options: Options,
        api_path: &str,
    ) -> Result<Client, SmartyError> {
        let url = &mut base_url.join(api_path)?;

        let mut reqwest_client_builder = reqwest::ClientBuilder::new();

        if let Some(proxy) = options.proxy.clone() {
            reqwest_client_builder = reqwest_client_builder.proxy(proxy);
        }

        let mut client_builder = ClientBuilder::new(
            reqwest_client_builder
                .build()
                .map_err(SmartyError::RequestProcess)?,
        )
        .with(SmartyRetryMiddleware::new(options.num_retries));

        if options.logging_enabled {
            client_builder = client_builder.with(LoggingMiddleware);
        }

        let client = client_builder.build();

        let client = Client {
            reqwest_client: client,
            url: url.clone(),
            options,
        };

        Ok(client)
    }

    pub(crate) fn build_request(&self, mut builder: RequestBuilder) -> RequestBuilder {
        if let Some(auth) = &self.options.authentication {
            builder = auth.authenticate(builder);
        }

        if !self.options.license.is_empty() {
            builder = builder.query(&[("license".to_string(), self.options.license.clone())]);
        }

        for (header_key, header_value) in self.options.headers.clone() {
            builder = builder.header(header_key, header_value);
        }

        builder = builder.header(
            USER_AGENT.to_string(),
            format!("smarty (sdk:rust@{})", VERSION),
        );

        if let Some(queries) = self.options.custom_queries.clone() {
            for (key, value) in queries {
                builder = builder.query(&[(key, value)]);
            }
        }

        builder
    }
}
