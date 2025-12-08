//! ## Tracing
//!
//! Module provides custom log format for [`tracing`]

use color_eyre::Result;
use color_eyre::eyre::Context as _;

/// Install trace dispatcher
pub fn install() -> Result<()> {
    let subscriber = tracing_subscriber::fmt().finish();

    tracing::subscriber::set_global_default(subscriber).context("Failed to install log")
}
