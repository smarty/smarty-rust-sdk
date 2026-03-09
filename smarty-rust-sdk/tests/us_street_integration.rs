extern crate smarty_rust_sdk;
extern crate tokio;

use smarty_rust_sdk::sdk::authentication::BasicAuthCredential;
use smarty_rust_sdk::sdk::batch::Batch;
use smarty_rust_sdk::sdk::options::OptionsBuilder;
use smarty_rust_sdk::us_street_api::client::USStreetAddressClient;
use smarty_rust_sdk::us_street_api::lookup::{Lookup, MatchStrategy};

fn new_client() -> USStreetAddressClient {
    let authentication = BasicAuthCredential::new(
        std::env::var("SMARTY_AUTH_ID").expect("Missing SMARTY_AUTH_ID env variable"),
        std::env::var("SMARTY_AUTH_TOKEN").expect("Missing SMARTY_AUTH_TOKEN env variable"),
    );

    let options = OptionsBuilder::new(Some(authentication)).build();
    USStreetAddressClient::new(options).expect("Failed to create client")
}

#[tokio::test]
#[ignore]
async fn single_lookup_default_enhanced() {
    let client = new_client();
    let mut batch = Batch::default();
    batch
        .push(Lookup {
            street: "1600 Amphitheatre Pkwy".to_string(),
            last_line: "Mountain View, CA".to_string(),
            ..Default::default()
        })
        .unwrap();

    client.send(&mut batch).await.unwrap();

    let results = &batch.records()[0].results;
    assert!(!results.is_empty(), "Expected results for default enhanced lookup");
}

#[tokio::test]
#[ignore]
async fn single_lookup_strict() {
    let client = new_client();
    let mut batch = Batch::default();
    batch
        .push(Lookup {
            street: "1600 Amphitheatre Pkwy".to_string(),
            last_line: "Mountain View, CA".to_string(),
            match_strategy: MatchStrategy::Strict,
            ..Default::default()
        })
        .unwrap();

    client.send(&mut batch).await.unwrap();

    let results = &batch.records()[0].results;
    assert!(!results.is_empty(), "Expected results for strict lookup");
}

#[tokio::test]
#[ignore]
async fn batch_default_enhanced() {
    let client = new_client();
    let mut batch = Batch::default();
    batch
        .push(Lookup {
            street: "1600 Amphitheatre Pkwy".to_string(),
            last_line: "Mountain View, CA".to_string(),
            ..Default::default()
        })
        .unwrap();
    batch
        .push(Lookup {
            street: "1 Rosedale Street".to_string(),
            last_line: "Baltimore, MD".to_string(),
            ..Default::default()
        })
        .unwrap();

    client.send(&mut batch).await.unwrap();

    for (i, record) in batch.records().iter().enumerate() {
        assert!(!record.results.is_empty(), "Expected results for batch record {i}");
    }
}

#[tokio::test]
#[ignore]
async fn batch_strict() {
    let client = new_client();
    let mut batch = Batch::default();
    batch
        .push(Lookup {
            street: "1600 Amphitheatre Pkwy".to_string(),
            last_line: "Mountain View, CA".to_string(),
            match_strategy: MatchStrategy::Strict,
            ..Default::default()
        })
        .unwrap();
    batch
        .push(Lookup {
            street: "1 Rosedale Street".to_string(),
            last_line: "Baltimore, MD".to_string(),
            match_strategy: MatchStrategy::Strict,
            ..Default::default()
        })
        .unwrap();

    client.send(&mut batch).await.unwrap();

    for (i, record) in batch.records().iter().enumerate() {
        assert!(!record.results.is_empty(), "Expected results for strict batch record {i}");
    }
}

#[tokio::test]
#[ignore]
async fn batch_mixed_strategies() {
    let client = new_client();
    let mut batch = Batch::default();
    batch
        .push(Lookup {
            street: "1600 Amphitheatre Pkwy".to_string(),
            last_line: "Mountain View, CA".to_string(),
            ..Default::default()
        })
        .unwrap();
    batch
        .push(Lookup {
            street: "1600 Amphitheatre Pkwy".to_string(),
            last_line: "Mountain View, CA".to_string(),
            match_strategy: MatchStrategy::Strict,
            ..Default::default()
        })
        .unwrap();

    client.send(&mut batch).await.unwrap();

    for (i, record) in batch.records().iter().enumerate() {
        assert!(!record.results.is_empty(), "Expected results for mixed batch record {i}");
    }
}
