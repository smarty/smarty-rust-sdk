extern crate env_logger;
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
    // Create a couple simple lookups
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

    // Build a simple batch
    let mut batch = Batch::default();
    batch.push(lookup)?;
    batch.push(lookup2)?;

    // Build a secret key auth id from environment variables
    let authentication = SecretKeyCredential::new(
        std::env::var("SMARTY_AUTH_ID").expect("Missing SMARTY_AUTH_ID env variable"),
        std::env::var("SMARTY_AUTH_TOKEN").expect("Missing SMARTY_AUTH_TOKEN env variable"),
    );

    // Create the options from it's builder pattern
    let options = OptionsBuilder::new(authentication)
        .with_license("us-core-cloud")
        .with_logging()
        .with_retries(2)
        .build();

    // Setup the logger
    // To better learn, look at (https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/log.html)
    env_logger::init();

    let client = USStreetAddressClient::new(options)?;

    // Send the Request to the server.
    client.send(&mut batch).await?;

    // Print out the results as a pretty json string, but only the first index of them.
    println!(
        "{}",
        serde_json::to_string_pretty(&batch.records()[0].results)?
    );

    Ok(())
}
