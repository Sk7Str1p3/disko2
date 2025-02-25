/// CLI library
use clap::{error::Result, ArgAction, Parser, Subcommand};
use std::path::PathBuf;

/// Default log level
///
/// 2 corresponds to the level INFO
const DEFAULT_LOG_LEVEL: usize = 2;

/// CLI Interface
#[derive(Parser)]
pub struct CLI {
    /// Remove any output
    #[arg(short, long)]
    quiet: bool,
    /// Increase verbosity of output
    #[arg(short, long, action = ArgAction::Count)]
    verbose: u8,
    #[clap(subcommand)]
    commands: Commands,
}
#[derive(Subcommand)]
enum Commands {
    Device,
    Configuration,
}
#[derive(Parser)]
struct SubcommandFlags {
    /// Flake URI
    #[arg(long, short)]
    flake: PathBuf,
    /// Config file
    file: PathBuf,
    /// Print what program WOULD do without actually doing this.
    dry_run: bool,
}

/// Device interaction parameters
#[derive(Parser)]
struct Device {
    #[clap(subcommand)]
    commands: DeviceSubcommand,
}
#[derive(Subcommand)]
enum DeviceSubcommand {
    /// Recreate partitions and tables from scratch
    Destroy(SubcommandFlags),
    /// Change disk configuration less destructively
    Adjust(SubcommandFlags),
    /// Mount partitions
    Mount(SubcommandFlags),
}

/// Configuration parameters
#[derive(Parser)]
struct Configuration {
    #[clap(subcommand)]
    commands: ConfigurationSubcommand,
}
#[derive(Subcommand)]
enum ConfigurationSubcommand {
    /// Validate existing config
    Validate(SubcommandFlags),
    /// Generate configuration from existing file
    Generate(SubcommandFlags),
}

// Pass arguments to fns
impl CLI {
    pub fn call(self, module: &str) {
        stderrlog::new()
            .module(module)
            .show_level(false)
            .quiet(self.quiet)
            .verbosity(DEFAULT_LOG_LEVEL + usize::from(self.verbose))
            .init()
            .expect("Failed to setup logger!");

        /*if let Err(e) = self.commands.call() {
            log::error!("{e:#}");
            std::process::exit(1);
        };*/
    }
}
impl DeviceSubcommand {
    pub fn call(self) -> Result<()> {
        match self {
            DeviceSubcommand::Destroy(args) => destroy(args),
            DeviceSubcommand::Adjust(args) => adjust(args),
            DeviceSubcommand::Mount(args) => mount(args),
        }
    }
}
impl ConfigurationSubcommand {
    pub fn call(self) -> Result<()>{
        match self {
            ConfigurationSubcommand::Generate(args) => generate(args),
            ConfigurationSubcommand::Validate(args) => validate(args),
        }
    }
}
