use smarty_rust_sdk::sdk::authentication::BasicAuthCredential;
use smarty_rust_sdk::sdk::options::OptionsBuilder;
use smarty_rust_sdk::us_enrichment_api::business::BusinessSummaryResponse;
use smarty_rust_sdk::us_enrichment_api::client::USEnrichmentClient;
use smarty_rust_sdk::us_enrichment_api::lookup::EnrichmentLookup;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let authentication = BasicAuthCredential::new(
        std::env::var("SMARTY_AUTH_ID").expect("Missing SMARTY_AUTH_ID env variable"),
        std::env::var("SMARTY_AUTH_TOKEN").expect("Missing SMARTY_AUTH_TOKEN env variable"),
    );
    let client = USEnrichmentClient::new(OptionsBuilder::new(Some(authentication)).build())?;

    let smarty_key = 1962995076;

    let mut first = EnrichmentLookup::<BusinessSummaryResponse> {
        smarty_key,
        ..Default::default()
    };
    client.send(&mut first).await?;
    println!(
        "First call: {} result(s), Etag={}",
        first.results.len(),
        first.response_etag
    );

    let mut second = EnrichmentLookup::<BusinessSummaryResponse> {
        smarty_key,
        etag: first.response_etag.clone(),
        ..Default::default()
    };
    client.send(&mut second).await?;
    if second.results.is_empty() {
        println!("Second call: not modified, Etag={}", second.response_etag);
    } else {
        println!(
            "Second call: modified, {} result(s), Etag={}",
            second.results.len(),
            second.response_etag
        );
    }

    Ok(())
}
