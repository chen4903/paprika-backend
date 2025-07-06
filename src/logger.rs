use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::constants::DEFAULT_LOG_LEVEL;

/// Initialize the logger
pub fn init() {
    let default_filter = format!(
        "{},heimdall_cfg::core=off,actix_server=off,heimdall_disassembler=off",
        DEFAULT_LOG_LEVEL.to_string()
    );

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| default_filter.into());

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
}
