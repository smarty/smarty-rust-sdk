use std::collections::HashMap;

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

        // Collect appended header values by key, then join with separator
        let mut appended: HashMap<String, Vec<String>> = HashMap::new();
        for (header_key, header_value) in self.options.headers.clone() {
            if self.options.append_headers.contains_key(&header_key) {
                appended.entry(header_key).or_default().push(header_value);
            } else {
                builder = builder.header(header_key, header_value);
            }
        }

        let ua_key = USER_AGENT.to_string();
        let base_ua = format!("smarty (sdk:rust@{})", VERSION);
        if let Some(ua_values) = appended.remove(&ua_key) {
            let separator = &self.options.append_headers[&ua_key];
            let mut all_values = vec![base_ua];
            all_values.extend(ua_values);
            builder = builder.header(&ua_key, all_values.join(separator));
        } else {
            builder = builder.header(&ua_key, base_ua);
        }

        for (key, values) in appended {
            let separator = &self.options.append_headers[&key];
            builder = builder.header(key, values.join(separator));
        }

        if let Some(queries) = self.options.custom_queries.clone() {
            for (key, value) in queries {
                builder = builder.query(&[(key, value)]);
            }
        }

        builder
    }
}
