# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Allowed

- Edit(*)
- Write(*)
- Bash(rm *)
- Bash(git *)
- Bash(make *)
- Bash(cargo *)

## Build Commands

```bash
make build                  # Build the project
make test                   # Run all tests
make clippy                 # Run linter
make clean                  # Clean build artifacts

# Run individual examples (requires SMARTY_AUTH_ID and SMARTY_AUTH_TOKEN env vars)
make us_street_api
make us_enrichment_api
make us_enrichment_address_search_api
make us_zipcode_api
make us_extract_api
make us_autocomplete_pro_api
make us_reverse_geo_api
make international_street_api
make international_autocomplete_api
make international_postal_code_api
make logger
make examples               # Run all examples
```

Run a single test: `cargo test <test_name>`

## Architecture

This is the official Smarty Rust SDK for address verification APIs. It uses a workspace with two crates:

- **smarty-rust-sdk**: Main SDK library with API clients (edition 2021, rust-version 1.63.0)
- **smarty-rust-proc-macro**: Procedural macro that generates API client boilerplate

### API Client Pattern

Each API lives in its own module under `smarty-rust-sdk/src/` and follows a consistent structure:
- `client.rs`: Client struct generated via `#[smarty_api]` macro
- `lookup.rs`: Request struct with `into_param_array()` method for query params
- `mod.rs`: Module declarations and inline tests
- Response file (e.g., `candidate.rs`, `suggestion.rs`): Response structs

### The `#[smarty_api]` Proc Macro

Defined in `smarty-rust-proc-macro/src/lib.rs`. Generates `new(options)` constructor and `send()` methods. Key attributes:

```rust
#[smarty_api(
    default_url = "https://us-street.api.smarty.com/",
    api_path = "street-address",
    lookup_style(batch),       // batch = POST (multi) / GET (single), lookup = GET only
    lookup_type = "Lookup",
    result_type = "Candidates",
    result_handler(batch),     // delegates to self.handle_batch_results() or handle_lookup_results()
    // custom_send = true,     // skips send() generation entirely; used when the API needs
                               // non-standard request construction (USExtract, USEnrichment,
                               // InternationalAutocomplete)
)]
pub struct USStreetAddressClient;
```

Three `custom_send` clients implement their own `send()`:
- **USExtractClient**: POST with `text/plain` body instead of JSON
- **USEnrichmentClient**: Dynamic URL path (`/lookup/{key}/{type}`) and ETag header handling
- **InternationalAutocompleteClient**: Optional address_id path suffix appended to URL

### Core SDK Components (in `sdk/`)

- **client.rs**: Base `Client` struct wrapping `reqwest_middleware::ClientWithMiddleware`. Constructs the middleware chain: retry middleware (always) → logging middleware (if enabled). `build_request` handles auth, license params, custom headers, and User-Agent (`"smarty (sdk:rust@{VERSION})"`)
- **authentication.rs**: Three credential types implementing `Authenticate` trait: `SecretKeyCredential` (query params), `WebsiteKeyCredential` (query param + Referer header), `BasicAuthCredential` (HTTP Authorization header)
- **batch.rs**: `Batch<T>` — generic Vec wrapper capped at `MAX_BATCH_SIZE` (100)
- **options.rs**: `OptionsBuilder` for client configuration (retries, logging, proxy, custom headers, API feature flags like `iana-timezone`)
- **retry_strategy.rs**: `SmartyRetryMiddleware` — exponential backoff (max 10s), retries on 408/429/5xx and transient connection errors, fatal on all others. Default max 10 retries
- **logging.rs**: `LoggingMiddleware` — `log::trace!` before/after requests. Enable with `RUST_LOG=trace`
- **error.rs**: `SmartyError` enum (RequestProcess, Middleware, Parse, HttpError, ValidationError)
- **mod.rs**: Shared helpers (`send_request`, `parse_response_json`, `has_param`, `is_zero`), `CoordinateLicense` enum, `VERSION` constant

### Key Conventions

- Lookup structs use `#[serde(skip_serializing_if)]` to exclude empty/default values
- Response structs derive `Clone, Debug, Serialize, Deserialize` with `#[serde(default)]`
- Batch responses populate `lookup.results` field for each item
- Single-item batches automatically use GET; multi-item use POST
- All client methods are async (tokio runtime, async-trait)
- API feature flags (e.g., `iana-timezone`) are set via `OptionsBuilder` methods and sent as query params — not Rust crate features

### Version Management

Workspace version is "0.0.0" in Cargo.toml — CI/CD replaces with actual version on tag pushes via GitHub Actions (`publish.yml`). Publish order: proc-macro crate first, then SDK crate.
