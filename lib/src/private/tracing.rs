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

mod format {
    //! ## Format
    //!
    //! Format final log message

    use std::fmt::Result;

    use console::{
        Term,
        measure_text_width,
        strip_ansi_codes
    };
    use owo_colors::OwoColorize as _;
    use tracing::{
        Event,
        Level,
        Subscriber
    };
    use tracing_subscriber::fmt::format::Writer;
    use tracing_subscriber::fmt::time::FormatTime;
    use tracing_subscriber::fmt::{
        FmtContext,
        FormatEvent,
        FormatFields,
        FormattedFields
    };
    use tracing_subscriber::registry::LookupSpan;

    pub(super) struct TracingFormatter;

    impl<S, F> FormatEvent<S, F> for TracingFormatter
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
        F: for<'a> FormatFields<'a> + 'static
    {
        fn format_event(
            &self,
            ctx: &FmtContext<'_, S, F>,
            mut wr: Writer<'_>,
            event: &Event<'_>
        ) -> Result {
            let meta = event.metadata();

            let left = {
                let mut buf = String::new();
                let mut wr = Writer::new(&mut buf);

                // Time
                super::time::TimeFormatter.format_time(&mut wr)?;

                // Log level
                let level = match *meta.level() {
                    Level::TRACE => "TRACE".purple().to_string(),
                    Level::DEBUG => "DEBUG".blue().to_string(),
                    Level::INFO => " INFO".green().to_string(),
                    Level::WARN => " WARN".yellow().bold().to_string(),
                    Level::ERROR => "ERROR".red().bold().to_string()
                };
                write!(wr, " {} ", level.dimmed())?;

                // Message
                ctx.format_fields(wr.by_ref(), event)?;

                buf
            };

            let right = {
                let mut buf = String::new();
                let mut wr = Writer::new(&mut buf);

                // Target
                write!(wr, "{}", meta.target().purple().dimmed())?;

                // Spans and their extensions
                if let Some(scope) = ctx.event_scope() {
                    write!(wr, "{}", "(".purple().dimmed())?;
                    let mut spans = Vec::new();
                    for span in scope.from_root() {
                        let mut span_info = String::new();
                        span_info.push_str(&span.metadata().name().dimmed().to_string());

                        if let Some(fields) = span.extensions().get::<FormattedFields<F>>()
                            && !fields.is_empty()
                        {
                            let fields = {
                                let mut f = Vec::new();
                                let pairs = fields.fields.split(' ').collect::<Vec<_>>();
                                for pair in pairs {
                                    let pair = strip_ansi_codes(pair);
                                    let (key, value) = pair.split_once('=').unwrap();
                                    let key = key.cyan();
                                    let value = value.cyan();
                                    let value = value.bold();

                                    f.push(format!("{key}: {value}").dimmed().to_string())
                                }
                                f
                            };
                            span_info.push_str(&format!(
                                "{}{}{}",
                                "(".dimmed(),
                                fields.join(&", ".dimmed().to_string()),
                                ")".dimmed()
                            ));
                        }
                        spans.push(span_info);
                    }
                    write!(
                        wr,
                        "{}{}",
                        spans.join(&", ".dimmed().to_string()),
                        ")".purple().dimmed()
                    )?;
                };

                // Location in source code
                write!(
                    wr,
                    " {} ",
                    format!(
                        "{}:{}",
                        meta.file().unwrap_or("<unknown>.rs").blue(),
                        meta.line()
                            .map(|l| l.to_string())
                            .unwrap_or("??".into())
                            .blue()
                    )
                    .dimmed()
                    .underline()
                )?;

                buf
            };

            let width = measure_text_width(&left) + measure_text_width(&right);
            let term_width = Term::stdout().size().1 as usize;
            let spaces = if width < term_width {
                term_width - width
            } else {
                1
            };
            writeln!(wr, "{left}{}{right}", " ".repeat(spaces))?;

            Ok(())
        }
    }
}

/// Install trace dispatcher
pub fn install() -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .event_format(format::TracingFormatter)
        .finish();

    tracing::subscriber::set_global_default(subscriber).context("Failed to install log")
}
