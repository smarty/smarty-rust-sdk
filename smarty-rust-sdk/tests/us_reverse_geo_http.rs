//! HTTP-layer tests for the US Reverse Geo client, driven by wiremock.
//!
//! The SDK does no client-side latitude/longitude validation, so an
//! out-of-range coordinate is sent to the API, which answers 422. These tests
//! pin how that error is surfaced: captured as `SmartyError::HttpError` with
//! the status code and response body preserved, and rendered into the Display
//! message. They run without credentials, so they're part of the default
//! `cargo test` run.

use smarty_rust_sdk::sdk::error::SmartyError;
use smarty_rust_sdk::sdk::options::OptionsBuilder;
use smarty_rust_sdk::us_reverse_geo_api::client::USReverseGeoClient;
use smarty_rust_sdk::us_reverse_geo_api::lookup::Lookup;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn client_for(server: &MockServer) -> USReverseGeoClient {
    let options = OptionsBuilder::new(None)
        .with_base_url(&server.uri())
        .build();
    USReverseGeoClient::new(options).unwrap()
}

#[tokio::test]
async fn out_of_range_latitude_surfaces_http_error_with_code_and_body() {
    let server = MockServer::start().await;

    let body = "{\"errors\":[{\"message\":\"latitude is out of range\"}]}";

    Mock::given(method("GET"))
        .and(path("/lookup"))
        .respond_with(ResponseTemplate::new(422).set_body_string(body))
        .mount(&server)
        .await;

    let client = client_for(&server);
    let mut lookup = Lookup {
        latitude: 99_937.422_511_348_56,
        longitude: -122.084_128_691_405_41,
        ..Default::default()
    };

    let err = client
        .send(&mut lookup)
        .await
        .expect_err("422 should be an error");

    match err {
        SmartyError::HttpError {
            code,
            message,
            body: raw_body,
        } => {
            assert_eq!(code.as_u16(), 422);
            assert_eq!(raw_body, body, "response body should be preserved verbatim");
            assert_eq!(
                message, "latitude is out of range",
                "message should be parsed from errors[].message"
            );
        }
        other => panic!("expected HttpError, got {other:?}"),
    }

    // results are left untouched on error
    assert!(lookup.results.results.is_empty());
}

#[tokio::test]
async fn http_error_display_includes_status_and_detail() {
    let server = MockServer::start().await;

    let body = "{\"errors\":[{\"message\":\"latitude is out of range\"}]}";

    Mock::given(method("GET"))
        .and(path("/lookup"))
        .respond_with(ResponseTemplate::new(422).set_body_string(body))
        .mount(&server)
        .await;

    let client = client_for(&server);
    let mut lookup = Lookup {
        latitude: 99_937.422_511_348_56,
        longitude: -122.084_128_691_405_41,
        ..Default::default()
    };

    let err = client
        .send(&mut lookup)
        .await
        .expect_err("422 should be an error");

    // Display must carry the actionable info, not the old bare "http error".
    let rendered = err.to_string();
    assert!(
        rendered.contains("422") && rendered.contains("latitude is out of range"),
        "Display dropped status/detail: {rendered:?}"
    );
}

#[tokio::test]
async fn unusable_error_body_falls_back_to_standard_message() {
    let server = MockServer::start().await;

    let body = "latitude is out of range";

    Mock::given(method("GET"))
        .and(path("/lookup"))
        .respond_with(ResponseTemplate::new(422).set_body_string(body))
        .mount(&server)
        .await;

    let client = client_for(&server);
    let mut lookup = Lookup {
        latitude: 99_937.422_511_348_56,
        longitude: -122.084_128_691_405_41,
        ..Default::default()
    };

    let err = client
        .send(&mut lookup)
        .await
        .expect_err("422 should be an error");

    match err {
        SmartyError::HttpError {
            code,
            message,
            body: raw_body,
        } => {
            assert_eq!(code.as_u16(), 422);
            assert_eq!(raw_body, body, "response body should be preserved verbatim");
            assert_eq!(
                message, "GET request lacked required fields.",
                "non-JSON body should fall back to the standard message"
            );
        }
        other => panic!("expected HttpError, got {other:?}"),
    }
}
