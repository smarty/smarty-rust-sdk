use reqwest_middleware::RequestBuilder;
use std::fmt::Debug;

pub trait AuthClone {
    fn clone_box(&self) -> Box<dyn Authenticate>;
}

pub trait Authenticate: Sync + Send + Debug + AuthClone {
    fn authenticate(&self, request: RequestBuilder) -> RequestBuilder;
}

#[derive(Clone, PartialEq, Debug)]
pub struct SecretKeyCredential {
    pub auth_id: String,
    pub auth_token: String,
}

impl SecretKeyCredential {
    pub fn new(auth_id: String, auth_token: String) -> Box<Self> {
        Box::new(SecretKeyCredential {
            auth_id,
            auth_token,
        })
    }
}

impl AuthClone for SecretKeyCredential {
    fn clone_box(&self) -> Box<dyn Authenticate> {
        Box::new(self.clone())
    }
}

impl Authenticate for SecretKeyCredential {
    fn authenticate(&self, request: RequestBuilder) -> RequestBuilder {
        request.query(&[
            ("auth-id".to_string(), self.auth_id.clone()),
            ("auth-token".to_string(), self.auth_token.clone()),
        ])
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct WebsiteKeyCredential {
    key: String,
    host: String,
}

impl WebsiteKeyCredential {
    pub fn new(key: &str, host: &str) -> Box<Self> {
        Box::new(Self {
            key: key.to_string(),
            host: host.to_string(),
        })
    }
}

impl AuthClone for WebsiteKeyCredential {
    fn clone_box(&self) -> Box<dyn Authenticate> {
        Box::new(self.clone())
    }
}

impl Authenticate for WebsiteKeyCredential {
    fn authenticate(&self, mut request: RequestBuilder) -> RequestBuilder {
        request = request.query(&[("key".to_string(), self.key.clone())]);
        request = request.header(reqwest::header::REFERER, self.host.clone());
        request
    }
}
