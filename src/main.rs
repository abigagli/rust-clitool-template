use anyhow::{Context as _, Result};
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;

mod args;

fn init_logging() -> Result<()> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_writer(std::io::stderr)
        .try_init()
        .map_err(anyhow::Error::from_boxed)?;
    Ok(())
}

// For now we rely on Rust's default process-exit behavior for `main() -> Result<()>` (via
// `Termination`) to report errors. If we later need more control over error formatting or exit
// codes, we can switch to the explicit `ExitCode`-based `main` below and handle `run()` errors
// directly.
// fn main() -> ExitCode {
//     match run() {
//         Ok(()) => std::process::ExitCode::SUCCESS,
//         Err(err) => {
//             eprintln!("Error: {err:?}");
//             std::process::ExitCode::FAILURE
//         }
//     }
// }
fn main() -> Result<()> {
    run()
}

fn run() -> Result<()> {
    init_logging().context("Failed to initialize logging")?;

    let cli = args::Cli::parse_args();
    debug!("cli: {cli:#?}");

    info!("Starting up");
    Ok(())
}

// Rust guideline compliant 2025-12-16
