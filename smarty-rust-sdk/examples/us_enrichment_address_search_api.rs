use smarty_rust_sdk::sdk::authentication::BasicAuthCredential;
use smarty_rust_sdk::sdk::options::OptionsBuilder;

use smarty_rust_sdk::us_enrichment_api::client::*;
use smarty_rust_sdk::us_enrichment_api::lookup::EnrichmentLookup;
use smarty_rust_sdk::us_enrichment_api::principal::*;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Example using structured address fields
    structured_address_search().await?;

    // Example using freeform address
    freeform_address_search().await?;

    Ok(())
}

async fn structured_address_search() -> Result<(), Box<dyn Error>> {
    println!("=== Structured Address Search ===");

    let mut lookup = EnrichmentLookup::<PrincipalResponse> {
        street: "56 Union Ave".to_string(),
        city: "Somerville".to_string(),
        state: "NJ".to_string(),
        zipcode: "08876".to_string(),
        ..Default::default()
    };

    let authentication = BasicAuthCredential::new(
        std::env::var("SMARTY_AUTH_ID").expect("Missing SMARTY_AUTH_ID env variable"),
        std::env::var("SMARTY_AUTH_TOKEN").expect("Missing SMARTY_AUTH_TOKEN env variable"),
    );

    let options = OptionsBuilder::new(Some(authentication))
        .with_logging()
        .build();

    let client = USEnrichmentClient::new(options)?;

    client.send(&mut lookup).await?;

    println!("{}", serde_json::to_string_pretty(&lookup.results)?);

    Ok(())
}

async fn freeform_address_search() -> Result<(), Box<dyn Error>> {
    println!("\n=== Freeform Address Search ===");

    let mut lookup = EnrichmentLookup::<PrincipalResponse> {
        freeform: "56 Union Ave, Somerville, NJ 08876".to_string(),
        ..Default::default()
    };

    let authentication = BasicAuthCredential::new(
        std::env::var("SMARTY_AUTH_ID").expect("Missing SMARTY_AUTH_ID env variable"),
        std::env::var("SMARTY_AUTH_TOKEN").expect("Missing SMARTY_AUTH_TOKEN env variable"),
    );

    let options = OptionsBuilder::new(Some(authentication))
        .with_logging()
        .build();

    let client = USEnrichmentClient::new(options)?;

    client.send(&mut lookup).await?;

    println!("{}", serde_json::to_string_pretty(&lookup.results)?);

    Ok(())
}
