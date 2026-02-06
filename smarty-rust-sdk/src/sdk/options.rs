use std::{collections::HashMap};
use reqwest::Proxy;
use url::Url;

use crate::sdk::authentication::Authenticate;

/// A builder for the options
///
/// Example:
/// ```ignore
/// let authentication = SecretKeyCredential::new("test".to_string(), "test".to_string());
///
/// OptionsBuilder::new(authentication)
///     .with_license("test_license")
///     .with_logging()
///     .build()
/// ```
pub struct OptionsBuilder {
    license: String,
    num_retries: u64,
    logging_enabled: bool,
    headers: Vec<(String, String)>,
    append_headers: HashMap<String, String>,
    authentication: Option<Box<dyn Authenticate>>,
    custom_queries: Option<HashMap<String, String>>,

    url: Option<Url>,

    proxy: Option<Proxy>,
}

// Allowing this because it is a builder pattern
#[allow(clippy::new_without_default)]
impl OptionsBuilder {
    /// Creates a new OptionsBuilder, taking in the authentication for the options.
    pub fn new(authentication: Option<Box<dyn Authenticate>>) -> Self {
        Self {
            license: "".to_string(),
            num_retries: 10,
            logging_enabled: false,
            headers: vec![],
            append_headers: HashMap::new(),
            authentication,
            custom_queries: None,

            url: None,

            proxy: None,
        }
    }

    /// Builds the builder into options with the parameters you set
    /// Returns an error if authentication is not set
    pub fn build(self) -> Options {
        Options {
            license: self.license,
            num_retries: self.num_retries,
            logging_enabled: self.logging_enabled,
            headers: self.headers,
            append_headers: self.append_headers,
            authentication: self.authentication,
            custom_queries: self.custom_queries,

            url: self.url,

            proxy: self.proxy,
        }
    }

    /// Adds a license string to the options
    pub fn with_license(mut self, license: &str) -> Self {
        self.license = license.to_string();
        self
    }

    /// Forces a maximum number of retries that a request will attempt to handle.
    pub fn with_retries(mut self, num_retries: u64) -> Self {
        self.num_retries = num_retries;
        self
    }

    /// Enables Logging
    pub fn with_logging(mut self) -> Self {
        self.logging_enabled = true;
        self
    }

    /// Adds a set of custom headers to your request.
    pub fn with_headers(mut self, headers: Vec<(String, String)>) -> Self {
        self.headers = headers;
        self
    }

    /// Appends the provided value to the existing header value using the specified separator,
    /// rather than adding a separate header value. This is useful for single-value headers like User-Agent.
    pub fn with_appended_header(mut self, key: &str, value: &str, separator: &str) -> Self {
        self.append_headers.insert(key.to_string(), separator.to_string());
        self.headers.push((key.to_string(), value.to_string()));
        self
    }

    /// Sets the base url that the request should use.
    pub fn with_url(mut self, url: Url) -> Self {
        self.url = Some(url);
        self
    }

    /// Adds a custom proxy for the request to point to.
    pub fn with_proxy(mut self, proxy: Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }

    /// Adds a custom query to the request.
    pub fn with_custom_query(mut self, key: &str, value: &str) -> Self {
        self.custom_queries.get_or_insert_with(HashMap::new).insert(key.to_string(), value.to_string());
        self
    }

    /// Appends a custom set of values to the existing query parameter.
    pub fn with_custom_comma_separated_query(mut self, key: &str, value: &str) -> Self {
        self.custom_queries
            .get_or_insert_with(HashMap::new)
            .entry(key.to_string())
            .and_modify(|existing| {
                if !existing.is_empty() {
                    existing.push(',');
                }
                existing.push_str(value);
            })
            .or_insert_with(|| value.to_string());
        self
    }

    /// Adds component analysis feature to the request.
    pub fn with_component_analysis(self) -> Self {
        self.with_custom_comma_separated_query("features", "component-analysis")
    }
}

/// Options that can be passed into a new client
/// <num_retries>: the number of retries that the client with run before giving up.
/// <logging_enabled>: whether we should send logging data
/// <headers>: Custom headers that you can pass in
/// <authentication>: A authentication for Smarty
pub struct Options {
    pub(crate) license: String,

    // Retry Sender
    pub(crate) num_retries: u64,

    // Logger
    pub(crate) logging_enabled: bool,

    // Custom Headers
    pub(crate) headers: Vec<(String, String)>,

    // Appended Headers (key -> separator)
    pub(crate) append_headers: HashMap<String, String>,

    // Authentication
    pub(crate) authentication: Option<Box<dyn Authenticate>>,

    // Custom Queries
    pub(crate) custom_queries: Option<HashMap<String, String>>,

    // Url
    pub(crate) url: Option<Url>,

    // Proxy
    pub(crate) proxy: Option<Proxy>,
}

impl Clone for Options {
    fn clone(&self) -> Self {
        Self {
            license: self.license.clone(),
            num_retries: self.num_retries,
            logging_enabled: self.logging_enabled,
            headers: self.headers.clone(),
            append_headers: self.append_headers.clone(),
            authentication: self.authentication.as_ref().map(|x| x.clone_box()),
            custom_queries: self.custom_queries.clone(),
            url: self.url.clone(),
            proxy: self.proxy.clone(),
        }
    }
}
