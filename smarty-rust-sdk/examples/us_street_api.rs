extern crate serde_json;
extern crate smarty_rust_sdk;
extern crate tokio;

use smarty_rust_sdk::us_street_api::lookup::{Lookup, MatchStrategy};

use smarty_rust_sdk::sdk::authentication::SecretKeyCredential;
use smarty_rust_sdk::sdk::batch::Batch;
use smarty_rust_sdk::sdk::options::OptionsBuilder;
use smarty_rust_sdk::us_street_api::client::USStreetAddressClient;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let lookup = Lookup {
        street: "1600 Amphitheatre Pkwy".to_string(),
        last_line: "Mountain View, CA".to_string(),
        max_candidates: 10,
        match_strategy: MatchStrategy::Enhanced,
        ..Default::default()
    };

    let lookup2 = Lookup {
        street: "1 Rosedale Street, Baltimore, MD".to_string(),
        max_candidates: 8,
        match_strategy: MatchStrategy::Enhanced,
        ..Default::default()
    };

    let mut batch = Batch::default();
    batch.push(lookup)?;
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

    let client = USStreetAddressClient::new(options)?;

    client.send(&mut batch).await?;

    println!(
        "{}",
        serde_json::to_string_pretty(&batch.records()[0].results)?
    );

    Ok(())
}
