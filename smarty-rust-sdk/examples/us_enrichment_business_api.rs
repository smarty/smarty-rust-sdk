use smarty_rust_sdk::sdk::authentication::BasicAuthCredential;
use smarty_rust_sdk::sdk::options::OptionsBuilder;

use smarty_rust_sdk::us_enrichment_api::business::BusinessSummaryResponse;
use smarty_rust_sdk::us_enrichment_api::client::USEnrichmentClient;
use smarty_rust_sdk::us_enrichment_api::lookup::{BusinessDetailLookup, EnrichmentLookup};

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let authentication = BasicAuthCredential::new(
        std::env::var("SMARTY_AUTH_ID").expect("Missing SMARTY_AUTH_ID env variable"),
        std::env::var("SMARTY_AUTH_TOKEN").expect("Missing SMARTY_AUTH_TOKEN env variable"),
    );

    let options = OptionsBuilder::new(Some(authentication))
        .with_logging()
        .build();

    let client = USEnrichmentClient::new(options)?;

    let smarty_key = 1962995076;

    // Step 1: Send a business summary lookup to get the list of businesses at this address
    let mut summary_lookup = EnrichmentLookup::<BusinessSummaryResponse> {
        smarty_key,
        ..Default::default()
    };

    client.send(&mut summary_lookup).await?;

    if summary_lookup.results.is_empty() || summary_lookup.results[0].businesses.is_empty() {
        println!("No businesses found for this SmartyKey");
        return Ok(());
    }

    println!("Summary results for SmartyKey: {}", smarty_key);
    for biz in &summary_lookup.results[0].businesses {
        println!("  - {} (ID: {})", biz.company_name, biz.business_id);
    }

    // Step 2: Use the first business ID to get detailed information
    let business_id = &summary_lookup.results[0].businesses[0].business_id;
    println!(
        "\nFetching details for business: {} (ID: {})",
        summary_lookup.results[0].businesses[0].company_name, business_id
    );

    let mut detail_lookup = BusinessDetailLookup {
        business_id: business_id.clone(),
        ..Default::default()
    };

    client.send_business_detail(&mut detail_lookup).await?;

    println!("\nDetail results:");
    for (i, response) in detail_lookup.results.iter().enumerate() {
        let json_response = serde_json::to_string_pretty(response)?;
        println!("#{}: {}", i, json_response);
    }

    println!("OK");
    Ok(())
}
