//! ## Error formatting
//!
//! Module provides human-friendly error messages and panic
//! reports with [`color_eyre`] and [`human_panic`] crates

/// Issue URL where user should submut an issue
const ISSUE_URL: &str = "https://github.com/nix-community/disko";

mod panic {
    //! ## Panic
    //! Human-friendly colorful panic report with crashdump.
    //! Heavily inspired by [`human_panic`]

    use color_eyre::section::PanicMessage;
    use console::strip_ansi_codes;
    use owo_colors::OwoColorize as _;

    use super::ISSUE_URL;

    /// Type representing panic message
    pub(super) struct PanicReport;

    impl PanicMessage for PanicReport {
        fn display(
            &self,
            pi: &std::panic::PanicHookInfo<'_>,
            f: &mut std::fmt::Formatter<'_>
        ) -> std::fmt::Result {
            writeln!(
                f,
                "\nDisko had unrecoverable error and {}.",
                "crashed".red().bold(),
            )?;
            writeln!(f, "Here's some info about error:")?;

            let message = pi.payload_as_str().unwrap_or("<not string>");
            writeln!(f, "    {}:  {}", "Message".red().bold(), message.blue())?;

            let thread = std::thread::current();
            let thread_name = thread.name().unwrap_or("<no name>");
            let thread_id = unsafe { std::mem::transmute::<_, u64>(thread.id()) };
            writeln!(
                f,
                "    {}:   {} (id: {})",
                "Thread".red().bold(),
                thread_name.yellow(),
                thread_id.yellow()
            )?;

            let location = if let Some(loc) = pi.location() {
                format!(
                    "{}, line {}, column {}",
                    loc.file().purple(),
                    loc.line().purple(),
                    loc.column().purple()
                )
            } else {
                "???".into()
            };
            writeln!(f, "    {}: {}", "Location".red().bold(), location)?;

            let report = human_panic::report::Report::new(
                env!("CARGO_PKG_VERSION"),
                env!("CARGO_PKG_VERSION"),
                human_panic::report::Method::Panic,
                format!("Panic occurred in file {}", strip_ansi_codes(&location)),
                message.into()
            );
            let dump = report.persist();
            if let Ok(path) = dump {
                writeln!(f, "\nMore info saved at {}.", path.display().blue())?;
                writeln!(
                    f,
                    "Please, submit an issue at {} and attach report.",
                    ISSUE_URL.blue()
                )?;
            } else {
                writeln!(
                    f,
                    "\nTried to safe crashdump but failed: {}",
                    dump.unwrap_err()
                )?;
                writeln!(f, "Please, submit an issue at {}.", ISSUE_URL.blue())?;
            }

            Ok(())
        }
    }
}

/// Install error and panic hooks
pub fn install() -> color_eyre::Result<()> {
    color_eyre::config::HookBuilder::blank()
        .panic_message(panic::PanicReport)
        .display_env_section(false)
        .install()
}
