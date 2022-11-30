use reqwest::{Client, Request};
use url::Url;

pub trait SDKClient {
    fn send(request: Request);
}

pub struct SDKBaseClient {
    pub reqwest_client: Client,
    pub base_url: Url
}

impl SDKBaseClient {
    pub fn send(request: Request) {

    }
}
