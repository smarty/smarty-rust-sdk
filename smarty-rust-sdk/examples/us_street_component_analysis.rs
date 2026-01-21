extern crate serde_json;
extern crate smarty_rust_sdk;
extern crate tokio;

use smarty_rust_sdk::us_street_api::lookup::{Lookup, MatchStrategy};

use smarty_rust_sdk::sdk::authentication::BasicAuthCredential;
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
        match_strategy: MatchStrategy::Enhanced, // Enhanced matching is required to return component analysis results.
        ..Default::default()
    };

    let mut batch = Batch::default();
    batch.push(lookup)?;

    let authentication = BasicAuthCredential::new(
        std::env::var("SMARTY_AUTH_ID").expect("Missing SMARTY_AUTH_ID env variable"),
        std::env::var("SMARTY_AUTH_TOKEN").expect("Missing SMARTY_AUTH_TOKEN env variable"),
    );

    let options = OptionsBuilder::new(Some(authentication))
        .with_component_analysis() // To add component analysis feature you need to specify when you create the options for the client.
        .build();

    let client = USStreetAddressClient::new(options)?;

    client.send(&mut batch).await?;

    // Here is an example of how to access component analysis
    for record in batch.records() {
        if !record.results.is_empty() {
            println!("Component Analysis Results:\n {}",serde_json::to_string_pretty(&record.results[0].analysis.components)?);
        }
    }

    Ok(())
}
