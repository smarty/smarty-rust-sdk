extern crate smarty_rust_sdk;
extern crate tokio;

use smarty_rust_sdk::sdk::authentication::BasicAuthCredential;
use smarty_rust_sdk::sdk::batch::Batch;
use smarty_rust_sdk::sdk::options::OptionsBuilder;
use smarty_rust_sdk::us_street_api::client::USStreetAddressClient;
use smarty_rust_sdk::us_street_api::lookup::{Lookup, MatchStrategy};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Each address is run through all three match strategies so you can compare how
    // 'strict', 'enhanced', and 'invalid' each handle a valid, an invalid, and an
    // ambiguous address.
    //   - strict:   only returns candidates that are valid, mailable addresses.
    //   - enhanced: returns a more comprehensive dataset (requires a US Core or Rooftop license).
    //   - invalid:  most permissive; always returns at least one candidate (a best-guess standardization).
    // Documentation for input fields: https://smartystreets.com/docs/us-street-api#input-fields
    let addresses = [
        ("valid (real, deliverable)", "1600 Amphitheatre Pkwy", "Mountain View", "CA", "94043"),
        ("invalid (no such address)", "9999 W 1150 S", "Provo", "UT", "84601"),
        ("ambiguous (missing ZIP/unit)", "1 Rosedale St", "Baltimore", "MD", ""),
    ];
    let strategies = [
        MatchStrategy::Strict,
        MatchStrategy::Enhanced,
        MatchStrategy::Invalid,
    ];

    let mut batch = Batch::default();
    let mut cases = Vec::new(); // parallel metadata for each lookup, in the order they are pushed

    for (label, street, city, state, zip) in addresses {
        for strategy in &strategies {
            let lookup = Lookup {
                street: street.to_string(),
                city: city.to_string(),
                state: state.to_string(),
                zipcode: zip.to_string(),
                match_strategy: strategy.clone(),
                max_candidates: 10, // allow ambiguous addresses to return more than one match
                ..Default::default()
            };
            batch.push(lookup)?;
            cases.push((label, format!("{street}, {city}, {state}"), strategy.clone()));
        }
    }

    let authentication = BasicAuthCredential::new(
        std::env::var("SMARTY_AUTH_ID").expect("Missing SMARTY_AUTH_ID env variable"),
        std::env::var("SMARTY_AUTH_TOKEN").expect("Missing SMARTY_AUTH_TOKEN env variable"),
    );

    let options = OptionsBuilder::new(Some(authentication)).build();
    let client = USStreetAddressClient::new(options)?;

    client.send(&mut batch).await?;

    let separator = "=".repeat(70);
    let mut last_address = String::new();

    for (i, record) in batch.records().iter().enumerate() {
        let (label, address, strategy) = &cases[i];

        if *address != last_address {
            println!("\n{separator}");
            println!(" Address: {address}  [{label}]");
            println!("{separator}");
            last_address = address.clone();
        }

        println!("\n--- '{strategy}' strategy ---");

        if record.results.is_empty() {
            println!("  0 candidates - no match returned under this strategy.");
            continue;
        }

        println!("  {} candidate(s):", record.results.len());
        for candidate in &record.results {
            println!(
                "    [{}] {}  {}",
                candidate.candidate_index, candidate.delivery_line_1, candidate.last_line
            );
        }
    }

    Ok(())
}
