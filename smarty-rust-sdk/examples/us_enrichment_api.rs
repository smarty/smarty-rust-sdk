use smarty_rust_sdk::sdk::authentication::SecretKeyCredential;
use smarty_rust_sdk::sdk::options::OptionsBuilder;
use smarty_rust_sdk::us_enrichment_api::client::USEnrichmentClient;
use smarty_rust_sdk::us_enrichment_api::lookup::EnrichmentLookup;
use smarty_rust_sdk::us_enrichment_api::results::{
    EnrichmentResponse, FinancialResponse, PrincipalResponse,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let key = 7;

    lookup::<FinancialResponse>(key).await?;
    lookup::<PrincipalResponse>(key).await?;

    Ok(())
}

async fn lookup<R: EnrichmentResponse>(key: u32) -> Result<(), Box<dyn Error>> {
    let mut lookup = EnrichmentLookup::<R>::new(key);

    let authentication = SecretKeyCredential::new(
        std::env::var("SMARTY_AUTH_ID").expect("Missing SMARTY_AUTH_ID env variable"),
        std::env::var("SMARTY_AUTH_TOKEN").expect("Missing SMARTY_AUTH_TOKEN env variable"),
    );

    let options = OptionsBuilder::new(Some(authentication))
        // The appropriate license values to be used for your subscriptions
        // can be found on the Subscriptions page of the account dashboard.
        // https://www.smartystreets.com/docs/cloud/licensing
        .with_license(&format!("us-property-data-{}-cloud", R::lookup_type()))
        .with_logging()
        .build();

    let client = USEnrichmentClient::new(options)?;

    client.send(&mut lookup).await?;

    println!("{}", serde_json::to_string_pretty(&lookup.results)?);

    Ok(())
}
