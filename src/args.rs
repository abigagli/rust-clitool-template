use clap::{Args, Parser, Subcommand, ValueEnum, builder::{Styles, styling}, crate_authors};
use std::path::PathBuf;

{% if include_build_script %}
pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));

    use std::fmt::Write as _;

    /// Returns a detailed version string with build-time metadata.
    /// The returned `&'static str` is leaked once (called at startup only).
    pub fn long_version() -> &'static str {
        let mut s = String::new();
        writeln!(s, "{PKG_VERSION}").ok();
        writeln!(s, "target:       {TARGET}").ok();
        writeln!(s, "rustc:        {RUSTC_VERSION}").ok();

        if let (Some(version), Some(dirty), Some(hash), Some(head_ref)) =
            (GIT_VERSION, GIT_DIRTY, GIT_COMMIT_HASH_SHORT, GIT_HEAD_REF)
        {
            let dirty_marker = if dirty { " (dirty)" } else { "" };
            writeln!(s, "git:          {version}{dirty_marker}").ok();
            writeln!(s, "commit:       {hash}").ok();
            writeln!(s, "branch:       {head_ref}").ok();
        }

        writeln!(s, "built:        {BUILT_TIME_UTC}").ok();
        writeln!(s, "profile:      {PROFILE}").ok();
        write!(s, "features:     {FEATURES_STR}").ok();
        Box::leak(s.into_boxed_str())
    }
}
{% endif %}

fn get_styles() -> Styles {
    Styles::styled()
        .header(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .usage(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .literal(styling::AnsiColor::Blue.on_default() | styling::Effects::BOLD)
        .placeholder(styling::AnsiColor::Cyan.on_default())
}


// {% if include_build_script %}
// const VERSION_INFO: &str = concat!(
//     env!("CARGO_PKG_VERSION"),
//     "\n",
//     "commit:  ", env!("BUILD_GIT_HASH"), "\n",
//     "date:    ", env!("BUILD_GIT_DATE"), "\n",
//     "built:   ", env!("BUILD_TIMESTAMP"),
// );
// {% endif %}
#[derive(Debug, Parser)]
{%- if include_build_script %}
#[command( author=crate_authors!(),
    version,
    //long_version = VERSION_INFO,
    long_version = built_info::long_version(),
    about,
    styles=get_styles(),
    help_template = "\
{before-help}{name} {version}
Author: {author-with-newline}{about-with-newline}
{usage-heading} {usage}
{all-args}{after-help}"
)]
{%- else %}
#[command(
    author=crate_authors!(),
    version,
    about,
    styles=get_styles(),
    help_template = "\
{before-help}{name} {version}
Author: {author-with-newline}{about-with-newline}
{usage-heading} {usage}
{all-args}{after-help}"
)]
{%- endif %}
pub struct Cli {
    #[arg(
        short,
        long("secrets"),
        default_value = "local/secrets.json",
        help = "Path to secrets JSON file"
    )]
    pub secrets_file: PathBuf,

    #[arg(short, long, default_value_t = false, help = "Enable verbose output")]
    pub verbose: bool,

    #[arg(long, value_parser = humantime::parse_duration, default_value="10s")]
    pub timeout: std::time::Duration,

    #[command(subcommand)]
    pub command: Command,
}
impl Cli {
    pub fn parse_args() -> Self {
        <Self as Parser>::parse()
    }
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Choice {
    One,
    Two,
}
#[derive(Subcommand, Debug)]
pub enum Command {
    Command1 {
        #[command(subcommand)]
        command: Command1,
    },
    #[command(name = "command2_name")]
    Command2(Command2),
}

#[derive(Subcommand, Debug)]
pub enum Command1 {
    Command1A(Command1AParams),
    Command1B(Command1BParams),
}

#[derive(Args, Debug)]
pub struct Command1AParams {
    /// Param1
    #[arg(long, value_enum, default_value_t = Choice::Two)]
    pub param1: Choice,
}

#[derive(Args, Debug)]
pub struct Command1BParams;

#[derive(Args, Debug)]
pub struct Command2 {
    /// Param1
    #[arg(long)]
    pub param1: String,
}
