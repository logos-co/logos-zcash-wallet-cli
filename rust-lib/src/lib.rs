//! zcash_wallet_cli: a headless relay to zcash_wallet_backend, for logosctl.

pub mod relay;

#[cfg(feature = "logos_module")]
mod glue;
