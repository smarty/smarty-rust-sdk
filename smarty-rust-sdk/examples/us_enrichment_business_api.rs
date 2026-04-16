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

    // Demo SmartyKey — substitute your own in production code.
    let smarty_key = 1962995076;

    let mut summary_lookup = EnrichmentLookup::<BusinessSummaryResponse> {
        smarty_key,
        ..Default::default()
    };

    client.send(&mut summary_lookup).await?;

    let summary = match summary_lookup.results.first() {
        Some(summary) => summary,
        None => {
            println!("No response returned for SmartyKey {}", smarty_key);
            return Ok(());
        }
    };

    if summary.businesses.is_empty() {
        println!("SmartyKey {} has no business tenants", smarty_key);
        return Ok(());
    }

    println!("Summary results for SmartyKey: {}", smarty_key);
    for biz in &summary.businesses {
        println!("  - {} (ID: {})", biz.company_name, biz.business_id);
    }

    let first = &summary.businesses[0];
    println!(
        "\nFetching details for business: {} (ID: {})",
        first.company_name, first.business_id
    );

    let mut detail_lookup = BusinessDetailLookup {
        business_id: first.business_id.clone(),
        ..Default::default()
    };

    client.send(&mut detail_lookup).await?;

    match &detail_lookup.result {
        Some(response) => {
            let json_response = serde_json::to_string_pretty(response)?;
            println!("\nDetail results:\n{}", json_response);
        }
        None => println!("\nNo detail result returned"),
    }

    println!("OK");
    Ok(())
}
