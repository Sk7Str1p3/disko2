//! CLI

use clap::{Parser, ValueEnum};
use clap::builder::Styles;
use owo_colors::OwoColorize as _;

/// CLI parameters
#[derive(Parser, Clone, Debug, PartialEq, Eq)]
#[command(
    version, 
    about = format!("{} v{}\n{}", 
        "Disko".blue(), 
        env!("CARGO_PKG_VERSION").green(), 
        "Declarative disk partitioning".bold().underline()
    ), 
    styles = get_styles(),
    override_usage = format!(
        "{disko} [{options}] disk-config.nix\n       {disko} [{options}] --flake github:user/repo#disk-config",
        disko = "disko".green(),
        options = "OPTIONS".cyan(),
    )
)]
pub struct Cli {
    #[arg(value_name = "mode", long, short, help = format!(
        r#"Set the mode. One of "{destroy}", "{format}", "{mount}", "{unmount}", "{format_mount}", "{destroy_format_mount}".
    {destroy}: unmount filesystems and destroy partition tables of the selected disks
    {format}: create partition tables, zpools, lvms, raids and filesystems if they don't exist yet
    {mount}: mount the partitions at the specified root-mountpoint
    {format_mount}: run format and mount in sequence
    {destroy_format_mount}: run all three modes in sequence. Previously known as --mode disko
        "#, 
        destroy = "destroy".blue(),
        format = "format".blue(),
        mount = "mount".blue(),
        unmount = "unmount".blue(),
        format_mount = "format,mount".blue(),
        destroy_format_mount = "destroy,format,mount".blue(),
    ),
    hide_possible_values = true)]
    pub mode: Mode,
    #[arg(long, short, help = "")]
    pub flake: String,
}

#[derive(Clone, ValueEnum, PartialEq, Eq, Debug)]
pub enum Mode {
    Destroy,
    Format,
    Mount,
    Unmount,
    #[value(name = "format,mount")]
    FormatMount,
    #[value(name = "destroy,format,mount")]
    DestroyFormatMount
}

/// Set clap styles
fn get_styles() -> Styles {
    use anstyle::{AnsiColor, Color::Ansi, Style};

    Styles::styled()
        .header(Style::new().bold().fg_color(Some(Ansi(AnsiColor::Blue))))
        .literal(Style::new().bold().fg_color(Some(Ansi(AnsiColor::Green))))
        .invalid(Style::new().bold().fg_color(Some(Ansi(AnsiColor::Red))))
        .error(Style::new().bold().fg_color(Some(Ansi(AnsiColor::Red))))
        .valid(Style::new().bold().fg_color(Some(Ansi(AnsiColor::Cyan))))
        .placeholder(Style::new().fg_color(Some(Ansi(AnsiColor::Cyan))))
}

const HELP_TEMPLATE: &'static str = r#""#;