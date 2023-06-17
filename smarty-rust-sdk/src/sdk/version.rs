#![allow(dead_code)]
const SMARTY_SDK_VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

pub(crate) fn get_version() -> &'static str {
    SMARTY_SDK_VERSION.unwrap_or("UNKNOWN")
}
