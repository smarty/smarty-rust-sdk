use std::time::Duration;
use reqwest::Client;
use reqwest::header::HeaderMap;
use url::{ParseError, Url};

//TODO: Fix Client Builder to return Address to client and not a clone.

pub struct ClientBuilder {
    base_url: Url,
    proxy: Url,
    retries: i32,
    timeout: Duration,
    debug: bool,
    close: bool,
    trace: bool,
    headers: HeaderMap,
    idle_conns: i32,
    http2disabled: bool,
    client: Client,
    licenses: Vec<String>
}

impl ClientBuilder {
    pub fn new(url_string: &str) -> Result<ClientBuilder, ParseError> {
        let url = Url::parse(url_string)?;
        let builder = ClientBuilder {
            base_url: url.clone(),
            proxy: url,
            retries: 0,
            timeout: Default::default(),
            debug: false,
            close: false,
            trace: false,
            headers: Default::default(),
            idle_conns: 0,
            http2disabled: false,
            client: Default::default(),
            licenses: vec!["us-core-cloud".to_string()]
        };

        Ok(builder)
    }

    pub fn build_httpclient(&mut self) -> &Client {
        self.client = Client::new();
        &self.client
    }
}

pub trait HTTPClient {
    fn call(&mut self) {

    }
}