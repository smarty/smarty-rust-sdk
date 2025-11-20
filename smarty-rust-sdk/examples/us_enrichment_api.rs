use smarty_rust_sdk::sdk::authentication::SecretKeyCredential;
use smarty_rust_sdk::sdk::options::OptionsBuilder;

use smarty_rust_sdk::us_enrichment_api::client::*;
use smarty_rust_sdk::us_enrichment_api::financial::*;
use smarty_rust_sdk::us_enrichment_api::geo::*;
use smarty_rust_sdk::us_enrichment_api::principal::*;

use smarty_rust_sdk::us_enrichment_api::lookup::EnrichmentLookup;
use smarty_rust_sdk::us_enrichment_api::response::EnrichmentResponse;
use smarty_rust_sdk::us_enrichment_api::secondary::SecondaryCountResponse;
use smarty_rust_sdk::us_enrichment_api::secondary::SecondaryResponse;
use smarty_rust_sdk::us_enrichment_api::risk::RiskResponse;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let key = 7;

    lookup::<FinancialResponse>(key).await?;
    lookup::<PrincipalResponse>(key).await?;
    lookup::<GeoReferenceResponse>(key).await?;
    lookup::<GeoReference2010Response>(key).await?;
    lookup::<GeoReference2020Response>(key).await?;
    lookup::<SecondaryResponse>(key).await?;
    lookup::<SecondaryCountResponse>(key).await?;
    lookup::<RiskResponse>(key).await?;

    Ok(())
}

async fn lookup<R: EnrichmentResponse>(key: u32) -> Result<(), Box<dyn Error>> {
    let mut lookup = EnrichmentLookup::<R> {
        smarty_key: key,
        include: "".to_string(), // optional: only include these attributes in the returned data. e.g. "group_structural,sale_date"
        exclude: "".to_string(), // optional: exclude attributes from the returned data
        etag: "".to_string(),
        features: "financial".to_string(),
        ..Default::default()
    };

    let authentication = SecretKeyCredential::new(
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
