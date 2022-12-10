extern crate smarty_rust_sdk;
extern crate tokio;
extern crate serde_json;

use std::error::Error;
use smarty_rust_sdk::sdk::authentication::SecretKeyCredential;
use smarty_rust_sdk::sdk::batch::Batch;
use smarty_rust_sdk::sdk::options::Options;
use smarty_rust_sdk::us_zipcode_api::client::USZipcodeClient;
use smarty_rust_sdk::us_zipcode_api::lookup::Lookup;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let lookup1 = Lookup {
        city: "PROVO".to_string(),
        state: "UT".to_string(),
        zipcode: "84604".to_string(),
        ..Default::default()
    };

    let lookup2 = Lookup {
        zipcode: "90210".to_string(),
        ..Default::default()
    };

    let batch = &mut Batch::new();
    batch.push(lookup1)?;
    batch.push(lookup2)?;

    let authentication = SecretKeyCredential::new(std::env::var("SMARTY_AUTH_ID")?, std::env::var("SMARTY_AUTH_TOKEN")?);

    let mut options = Options::new();
    options.license = "us-core-cloud".to_string();

    options.authentication = authentication;

    let client = USZipcodeClient::new(options)?;

    client.send(batch).await?;

    println!("{}", serde_json::to_string_pretty(&batch.records()[0].result.clone())?);
    println!("{}", serde_json::to_string_pretty(&batch.records()[1].result.clone())?);

    Ok(())
}