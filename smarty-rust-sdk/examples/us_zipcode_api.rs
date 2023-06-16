extern crate serde_json;
extern crate smarty_rust_sdk;
extern crate tokio;

use smarty_rust_sdk::sdk::authentication::SecretKeyCredential;
use smarty_rust_sdk::sdk::batch::Batch;
use smarty_rust_sdk::sdk::options::OptionsBuilder;
use smarty_rust_sdk::us_zipcode_api::client::USZipcodeClient;
use smarty_rust_sdk::us_zipcode_api::lookup::Lookup;
use std::error::Error;

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

    let batch = &mut Batch::default();
    batch.push(lookup1)?;
    batch.push(lookup2)?;

    let authentication = SecretKeyCredential::new(
        std::env::var("SMARTY_AUTH_ID")?,
        std::env::var("SMARTY_AUTH_TOKEN")?,
    );

    let options = OptionsBuilder::new()
        .with_license("us-core-cloud")
        .authenticate(authentication)
        .build()
        .unwrap();

    let client = USZipcodeClient::new(options)?;

    client.send(batch).await?;

    println!(
        "{}",
        serde_json::to_string_pretty(&batch.records()[0].results.clone())?
    );
    println!(
        "{}",
        serde_json::to_string_pretty(&batch.records()[1].results.clone())?
    );

    Ok(())
}
