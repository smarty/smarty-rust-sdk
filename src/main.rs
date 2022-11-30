use std::error::Error;
use reqwest::{Client, Method, Request};
use crate::builder::ClientBuilder;
use tokio;
use std::env;
use std::env::VarError;
use url::Url;
use crate::candidate::{Candidates};
use crate::lookup::Lookup;
use crate::sdk_client::SDKClient;

mod builder;
mod candidate;
mod lookup;
mod batch;

mod sdk_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let lookup = Lookup {
        street: "1600 Amphitheatre Pkwy".to_string(),
        last_line: "Mountain View, CA".to_string(),
        max_candidates: 10,
        ..Default::default()
    };

    let auth = Authentication::new("SMARTY_AUTH_ID", "SMARTY_AUTH_TOKEN")?;

    let url = format!("https://us-street.api.smartystreets.me/street-address?auth-id={auth_id}&auth-token={auth_token}&license={license}",
                      auth_id = auth.auth_token,
                      auth_token = auth.auth_id,
                      license = "us-core-cloud");

    let content = lookup.send(Client::new(), Url::parse(url.as_str())?).await?;

    println!("text: {:?}", content);

    // println!("{}", url);
    //
    // let mut client_builder = ClientBuilder::new(url.as_str())?;
    // client_builder.set_debug(true);
    // let client = client_builder.build_httpclient();
    //
    // let req = client.request(Method::GET, url.as_str()).query(&lookup.to_param_array());
    //
    // let content = req.send()
    //     .await?
    //     .json::<Candidates>()
    //     .await?;
    //
    // println!("text: {:?}", content);

    Ok(())
}

pub struct Authentication {
    auth_id: String,
    auth_token: String,
}

impl Authentication {
    pub fn new(auth_id_env: &str, auth_token_env: &str) -> Result<Authentication, VarError> {
        let auth_token = env::var(auth_id_env)?;
        let auth_id = env::var(auth_token_env)?;

        let authentication = Authentication { auth_id, auth_token };

        Ok(authentication)
    }
}
