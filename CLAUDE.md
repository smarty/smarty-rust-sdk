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
make us_zipcode_api
make us_extract_api
make us_autocomplete_pro_api
make us_reverse_geo_api
make international_street_api
make international_autocomplete_api
make international_postal_code_api
make examples               # Run all examples
```

Run a single test: `cargo test <test_name>`

## Architecture

This is the official Smarty Rust SDK for address verification APIs. It uses a workspace with two crates:

- **smarty-rust-sdk**: Main SDK library with API clients
- **smarty-rust-proc-macro**: Procedural macro that generates API client boilerplate

### API Client Pattern

Each API follows a consistent module structure:
- `client.rs`: Client struct generated via `#[smarty_api]` macro
- `lookup.rs`: Request struct with `into_param_array()` method for query params
- Response file (e.g., `candidate.rs`, `suggestion.rs`): Response structs

The `#[smarty_api]` macro in `smarty-rust-proc-macro/src/lib.rs` generates client implementations:

```rust
#[smarty_api(
    default_url = "https://us-street.api.smarty.com/",
    api_path = "street-address",
    lookup_style(batch),       // batch = POST, lookup = GET
    lookup_type = "Lookup",
    result_type = "Candidates",
    result_handler(batch)
)]
pub struct USStreetAddressClient;
```

### Core SDK Components (in `sdk/`)

- **authentication.rs**: `SecretKeyCredential` (auth-id + auth-token) and `WebsiteKeyCredential`
- **batch.rs**: `Batch<T>` for batching lookups (max 100 items, defined in `mod.rs`)
- **client.rs**: Base HTTP client with middleware stack
- **options.rs**: `OptionsBuilder` for client configuration (retries, logging, proxy, custom headers)
- **retry_strategy.rs**: `SmartyRetryMiddleware` with exponential backoff
- **logging.rs**: Trace-level request/response logging middleware
- **error.rs**: `SmartyError` enum for error handling

### Key Conventions

- Lookup structs use `#[serde(skip_serializing_if)]` to exclude empty/default values
- Response structs derive `Clone, Debug, Serialize, Deserialize` with `#[serde(default)]`
- Batch responses populate `lookup.results` field for each item
- Single-item batches automatically use GET; multi-item use POST
- All client methods are async (tokio runtime, async-trait)

### Version Management

Workspace version is "0.0.0" in Cargo.toml - CI/CD replaces with actual version on tag pushes via GitHub Actions.
