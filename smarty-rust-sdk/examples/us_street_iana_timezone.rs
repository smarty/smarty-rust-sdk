extern crate smarty_rust_sdk;
extern crate tokio;

use smarty_rust_sdk::sdk::authentication::BasicAuthCredential;
use smarty_rust_sdk::sdk::batch::Batch;
use smarty_rust_sdk::sdk::options::OptionsBuilder;
use smarty_rust_sdk::us_street_api::client::USStreetAddressClient;
use smarty_rust_sdk::us_street_api::lookup::Lookup;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let lookup = Lookup {
        street: "1 Rosedale".to_string(),
        last_line: "Baltimore, MD 21229".to_string(),
        max_candidates: 10,
        ..Default::default()
    };

    let mut batch = Batch::default();
    batch.push(lookup)?;

    let authentication = BasicAuthCredential::new(
        std::env::var("SMARTY_AUTH_ID").expect("Missing SMARTY_AUTH_ID env variable"),
        std::env::var("SMARTY_AUTH_TOKEN").expect("Missing SMARTY_AUTH_TOKEN env variable"),
    );

    let options = OptionsBuilder::new(Some(authentication))
        .with_iana_time_zone()
        .build();

    let client = USStreetAddressClient::new(options)?;

    client.send(&mut batch).await?;

    for record in batch.records() {
        if !record.results.is_empty() {
            let metadata = &record.results[0].metadata;
            println!("Timezone: {}", metadata.time_zone);
            println!("UTC Offset: {}", metadata.utc_offset);
            println!("DST: {}", metadata.dst);
            println!("IANA Timezone: {}", metadata.iana_time_zone);
            println!("IANA UTC Offset: {}", metadata.iana_utc_offset);
            println!("IANA DST: {}", metadata.iana_dst);
        }
    }

    Ok(())
}
