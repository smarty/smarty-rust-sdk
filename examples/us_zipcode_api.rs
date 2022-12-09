extern crate smarty_rust_sdk;
extern crate tokio;
extern crate serde_json;

use std::error::Error;
use smarty_rust_sdk::sdk::authentication::Authentication;
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

    let authentication = Authentication::new("SMARTY_AUTH_ID", "SMARTY_AUTH_TOKEN")?;

    let mut options = Options::new();
    options.auth_id = authentication.auth_id.to_string();
    options.auth_token = authentication.auth_token.to_string();

    let client = USZipcodeClient::new("https://us-zipcode.api.smartystreets.me/".parse()?, options)?;

    client.send(batch).await?;

    println!("{}", serde_json::to_string_pretty(&batch.records()[0].result.clone())?);
    println!("{}", serde_json::to_string_pretty(&batch.records()[1].result.clone())?);

    Ok(())
}