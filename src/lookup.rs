use std::fmt::{Display, Error, Formatter};
use std::future::Future;
use reqwest::{Client, Method, Request, RequestBuilder, Response};
use serde::{Deserialize, Serialize};
use url::Url;
use crate::candidate::{Candidate, Candidates};

#[derive(Debug, Clone, PartialEq)]
pub struct Lookup {
    pub street: String,
    pub street2: String,
    pub secondary: String,
    pub city: String,
    pub state: String,
    pub zipcode: String,
    pub last_line: String, // lastline in json
    pub adressee: String,
    pub urbanization: String,
    pub input_id: String,
    pub max_candidates: i64, // Default Value: 1 // candidates in json

    pub match_strategy: MatchStrategy, // match in json

    pub results: Candidates
}

impl Default for Lookup {
    fn default() -> Self {
        Lookup {
            street: String::default(),
            street2: String::default(),
            secondary: String::default(),
            city: String::default(),
            state: String::default(),
            zipcode: String::default(),
            last_line: String::default(),
            adressee: String::default(),
            urbanization: String::default(),
            input_id: String::default(),
            max_candidates: 1,

            match_strategy: Default::default(),
            results: vec![]
        }
    }
}

impl Lookup {
    pub fn to_param_array(self) -> Vec<(String, String)> {
        vec![
            has_param("street".to_string(), self.street),
            has_param("street2".to_string(), self.street2),
            has_param("secondary".to_string(), self.secondary),
            has_param("city".to_string(), self.city),
            has_param("state".to_string(), self.state),
            has_param("zipcode".to_string(), self.zipcode),
            has_param("lastline".to_string(), self.last_line),
            has_param("adressee".to_string(), self.adressee),
            has_param("urbanization".to_string(), self.urbanization),
            has_param("input_id".to_string(), self.input_id),
            has_param("candidates".to_string(), self.max_candidates.to_string()),
            has_param("match".to_string(), self.match_strategy.to_string()),
        ].iter()
            .filter_map(Option::clone)
            .collect::<Vec<_>>()
    }

    pub async fn send(&self, client: Client, url: Url) -> Result<Candidates, reqwest::Error> {
        let req = client.request(Method::GET, url.as_str()).query(&self.clone().to_param_array());
        req.send().await?.json::<Candidates>().await
    }
}

fn has_param(name: String, param: String) -> Option<(String, String)> {
    if param != String::default() {
        Some((name, param))
    } else {
        None
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum MatchStrategy {
    #[default]
    Strict,
    Invalid,
    Enhanced
}

impl Display for MatchStrategy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MatchStrategy::Strict => { write!(f, "strict") }
            MatchStrategy::Invalid => { write!(f, "invalid") }
            MatchStrategy::Enhanced => { write!(f, "enhanced") }
        }
    }
}