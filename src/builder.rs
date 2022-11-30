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
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }
    pub fn proxy(&self) -> &Url {
        &self.proxy
    }
    pub fn retries(&self) -> i32 {
        self.retries
    }
    pub fn timeout(&self) -> Duration {
        self.timeout
    }
    pub fn debug(&self) -> bool {
        self.debug
    }
    pub fn close(&self) -> bool {
        self.close
    }
    pub fn trace(&self) -> bool {
        self.trace
    }
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }
    pub fn idle_conns(&self) -> i32 {
        self.idle_conns
    }
    pub fn http2disabled(&self) -> bool {
        self.http2disabled
    }
    pub fn client(&self) -> &Client {
        &self.client
    }
    pub fn licenses(&self) -> &Vec<String> {
        &self.licenses
    }
    pub fn set_base_url(&mut self, base_url: Url) {
        self.base_url = base_url;
    }
    pub fn set_proxy(&mut self, proxy: Url) {
        self.proxy = proxy;
    }
    pub fn set_retries(&mut self, retries: i32) {
        self.retries = retries;
    }
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }
    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }
    pub fn set_close(&mut self, close: bool) {
        self.close = close;
    }
    pub fn set_trace(&mut self, trace: bool) {
        self.trace = trace;
    }
    pub fn set_headers(&mut self, headers: HeaderMap) {
        self.headers = headers;
    }
    pub fn set_idle_conns(&mut self, idle_conns: i32) {
        self.idle_conns = idle_conns;
    }
    pub fn set_http2disabled(&mut self, http2disabled: bool) {
        self.http2disabled = http2disabled;
    }
    pub fn set_client(&mut self, client: Client) {
        self.client = client;
    }
    pub fn set_licenses(&mut self, licenses: Vec<String>) {
        self.licenses = licenses;
    }
}

pub trait HTTPClient {
    fn call(&mut self) {

    }
}