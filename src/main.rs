use std::error::Error;
use reqwest::{Request};
use tokio;
use std::env;
use std::env::VarError;
use url::Url;
use crate::batch::Batch;
use crate::lookup::Lookup;
use crate::smarty_client::SmartyClient;

mod smarty_client;
mod candidate;
mod lookup;
mod batch;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let lookup = Lookup {
        street: "1600 Amphitheatre Pkwy".to_string(),
        last_line: "Mountain View, CA".to_string(),
        max_candidates: 10,
        ..Default::default()
    };

    let lookup2 = Lookup {
        street: "2758 W 530".to_string(),
        last_line: "Provo, UT".to_string(),
        max_candidates: 8,
        ..Default::default()
    };

    let batch = &mut Batch::new();
    batch.push(lookup);
    batch.push(lookup2);

    let auth = Authentication::new("SMARTY_AUTH_ID", "SMARTY_AUTH_TOKEN")?;

    let url = format!("https://us-street.api.smartystreets.me/street-address?auth-id={auth_id}&auth-token={auth_token}&license={license}",
                      auth_id = auth.auth_token,
                      auth_token = auth.auth_id,
                      license = "us-core-cloud");

    let client = SmartyClient::new(Url::parse(url.as_str())?);

    client.send_batch(batch).await?;

    println!("result: {:?}", batch.records().clone()[0].results);

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
