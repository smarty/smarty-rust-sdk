//! HTTP-layer tests for the US Enrichment client, driven by wiremock.
//!
//! These exercise the wire contract that unit tests can't reach: the
//! If-None-Match request header, 304 Not Modified handling, ETag round-trips,
//! and server-contract violations. They run without credentials against a
//! local mock server, so they're part of the default `cargo test` run.

use smarty_rust_sdk::sdk::error::SmartyError;
use smarty_rust_sdk::sdk::options::OptionsBuilder;
use smarty_rust_sdk::us_enrichment_api::business::BusinessDetailResponse;
use smarty_rust_sdk::us_enrichment_api::client::USEnrichmentClient;
use smarty_rust_sdk::us_enrichment_api::lookup::{BusinessDetailLookup, EnrichmentLookup};
use smarty_rust_sdk::us_enrichment_api::principal::PrincipalResponse;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn client_for(server: &MockServer) -> USEnrichmentClient {
    let options = OptionsBuilder::new(None)
        .with_base_url(&server.uri())
        .build();
    USEnrichmentClient::new(options).unwrap()
}

fn principal_body() -> serde_json::Value {
    serde_json::json!([{
        "smarty_key": "100",
        "data_set_name": "property",
        "data_subset_name": "principal",
        "attributes": {}
    }])
}

fn business_detail_body(business_id: &str) -> serde_json::Value {
    serde_json::json!([{
        "smarty_key": "100",
        "data_set_name": "business",
        "business_id": business_id,
        "attributes": { "company_name": "Acme" }
    }])
}

#[tokio::test]
async fn send_uses_if_none_match_header_not_etag_request_header() {
    let server = MockServer::start().await;

    // The `if-none-match` matcher is what enforces the header name: any other
    // name (e.g. "ETag") falls through to wiremock's default 404 and send fails.
    Mock::given(method("GET"))
        .and(path("/lookup/100/property/principal"))
        .and(header("if-none-match", "prev-tag"))
        .respond_with(ResponseTemplate::new(200).set_body_json(principal_body()))
        .mount(&server)
        .await;

    let client = client_for(&server);
    let mut lookup: EnrichmentLookup<PrincipalResponse> = EnrichmentLookup {
        smarty_key: 100,
        etag: "prev-tag".to_string(),
        ..Default::default()
    };

    client.send(&mut lookup).await.expect("send should succeed");

    // The mock matcher proves If-None-Match was sent but does not rule out a
    // duplicate literal "ETag" header alongside it; this closes that gap.
    let received = server.received_requests().await.expect("recording on");
    let req = received.first().expect("one request");
    assert!(
        !req.headers
            .iter()
            .any(|(k, _)| k.as_str().eq_ignore_ascii_case("etag")),
        "unexpected ETag request header"
    );
}

#[tokio::test]
async fn not_modified_preserves_prior_results_and_refreshes_etag() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/lookup/100/property/principal"))
        .respond_with(ResponseTemplate::new(304).insert_header("ETag", "refreshed-tag"))
        .mount(&server)
        .await;

    let client = client_for(&server);

    let existing = PrincipalResponse {
        smarty_key: "prior".to_string(),
        ..Default::default()
    };

    let mut lookup: EnrichmentLookup<PrincipalResponse> = EnrichmentLookup {
        smarty_key: 100,
        etag: "old-tag".to_string(),
        results: vec![existing],
        ..Default::default()
    };

    client.send(&mut lookup).await.expect("304 is not an error");

    assert_eq!(lookup.etag, "refreshed-tag");
    assert_eq!(lookup.results.len(), 1);
    assert_eq!(lookup.results[0].smarty_key, "prior");
}

#[tokio::test]
async fn not_modified_preserves_prior_business_detail_result() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/lookup/business/ABC123"))
        .respond_with(ResponseTemplate::new(304).insert_header("ETag", "refreshed-tag"))
        .mount(&server)
        .await;

    let client = client_for(&server);

    let existing = BusinessDetailResponse {
        business_id: "ABC123".to_string(),
        ..Default::default()
    };

    let mut lookup = BusinessDetailLookup {
        business_id: "ABC123".to_string(),
        etag: "old-tag".to_string(),
        result: Some(existing),
        ..Default::default()
    };

    client.send(&mut lookup).await.expect("304 is not an error");

    assert_eq!(lookup.etag, "refreshed-tag");
    assert_eq!(lookup.result.as_ref().unwrap().business_id, "ABC123");
}

#[tokio::test]
async fn ok_response_refreshes_etag_and_replaces_results() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/lookup/100/property/principal"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("ETag", "server-tag")
                .set_body_json(principal_body()),
        )
        .mount(&server)
        .await;

    let client = client_for(&server);
    let mut lookup: EnrichmentLookup<PrincipalResponse> = EnrichmentLookup {
        smarty_key: 100,
        ..Default::default()
    };

    client.send(&mut lookup).await.expect("send should succeed");

    assert_eq!(lookup.etag, "server-tag");
    assert_eq!(lookup.results.len(), 1);
    assert_eq!(lookup.results[0].smarty_key, "100");
}

#[tokio::test]
async fn business_detail_rejects_multiple_results_from_server() {
    let server = MockServer::start().await;

    let body = serde_json::json!([
        {
            "smarty_key": "100",
            "data_set_name": "business",
            "business_id": "ABC123",
            "attributes": {"company_name": "Acme"}
        },
        {
            "smarty_key": "100",
            "data_set_name": "business",
            "business_id": "ABC123",
            "attributes": {"company_name": "Second"}
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/lookup/business/ABC123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .mount(&server)
        .await;

    let client = client_for(&server);
    let mut lookup = BusinessDetailLookup {
        business_id: "ABC123".to_string(),
        ..Default::default()
    };

    let err = client
        .send(&mut lookup)
        .await
        .expect_err("expected server-contract violation");
    assert!(
        matches!(err, SmartyError::ValidationError(_)),
        "unexpected error variant: {err:?}"
    );
    assert!(lookup.result.is_none(), "result should not be set on error");
}

#[tokio::test]
async fn business_detail_happy_path_populates_result() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/lookup/business/ABC123"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("ETag", "b-tag")
                .set_body_json(business_detail_body("ABC123")),
        )
        .mount(&server)
        .await;

    let client = client_for(&server);
    let mut lookup = BusinessDetailLookup {
        business_id: "ABC123".to_string(),
        ..Default::default()
    };

    client.send(&mut lookup).await.expect("send should succeed");

    assert_eq!(lookup.etag, "b-tag");
    let result = lookup.result.expect("result should be populated");
    assert_eq!(result.business_id, "ABC123");
    assert_eq!(result.attributes.company_name, "Acme");
}
