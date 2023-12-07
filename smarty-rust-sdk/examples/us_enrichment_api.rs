use smarty_rust_sdk::sdk::authentication::SecretKeyCredential;
use smarty_rust_sdk::sdk::options::OptionsBuilder;
use smarty_rust_sdk::us_enrichment_api::client::USEnrichmentClient;
use smarty_rust_sdk::us_enrichment_api::lookup::EnrichmentLookup;
use smarty_rust_sdk::us_enrichment_api::results::PrincipalResponse;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut lookup = EnrichmentLookup::<PrincipalResponse>::new(7);

    let authentication = SecretKeyCredential::new(
        std::env::var("SMARTY_AUTH_ID").expect("Missing SMARTY_AUTH_ID env variable"),
        std::env::var("SMARTY_AUTH_TOKEN").expect("Missing SMARTY_AUTH_TOKEN env variable"),
    );

    let options = OptionsBuilder::new()
        .with_license("us-property-data-principal-cloud")
        .with_logging()
        .authenticate(authentication)
        .build()
        .unwrap();

    let client = USEnrichmentClient::new(options)?;

    client.send(&mut lookup).await?;

    println!("{}", serde_json::to_string_pretty(&lookup.results)?);
    return Ok(());
}
