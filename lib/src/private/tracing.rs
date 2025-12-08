//! ## Tracing
//!
//! Module provides custom log format for [`tracing`]

use color_eyre::Result;
use color_eyre::eyre::Context as _;

mod time {
    //! ## Time
    //!
    //! Implements time formatting in logs

    use std::fmt::Result;
    use std::time::{
        SystemTime,
        UNIX_EPOCH
    };

    use owo_colors::OwoColorize as _;
    use tracing_subscriber::fmt::format::Writer;
    use tracing_subscriber::fmt::time::FormatTime;

    /// A type representing time format in logs
    pub(super) struct TimeFormatter;

    impl FormatTime for TimeFormatter {
        fn format_time(
            &self,
            w: &mut Writer<'_>
        ) -> Result {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("SystemTime before UNIX EPOCH!");
            let secs = now.as_secs() % (60 * 60 * 24);

            let hours = secs / (60 * 60);
            let mins = (secs % 3600) / 60;
            let secs = secs & 60;
            let millis = now.subsec_millis();
            write!(
                w,
                "{}",
                format!("[{hours:02}:{mins:02}:{secs:02}.{millis:03}]")
                    .blue()
                    .dimmed()
            )
        }
    }
}

/// Install trace dispatcher
pub fn install() -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_timer(time::TimeFormatter)
        .finish();

    tracing::subscriber::set_global_default(subscriber).context("Failed to install log")
}
