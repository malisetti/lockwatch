use std::path::Path;
use std::process::ExitCode;
use std::time::Duration;

use clap::Parser;
use lockwatch::{wait_for_unlock, WaitOutcome};

#[derive(Parser)]
#[command(
    name = "lockwatch",
    about = "Wait for an exclusive flock(2) on PATH to be released"
)]
struct Args {
    /// Lock file path
    path: std::path::PathBuf,

    /// Maximum seconds to wait
    #[arg(long, default_value_t = 120)]
    timeout: u64,

    /// Poll interval in milliseconds
    #[arg(long = "poll-ms", default_value_t = 200)]
    poll_ms: u64,
}

fn main() -> ExitCode {
    let args = Args::parse();
    if !args.path.exists() {
        return ExitCode::from(2);
    }

    match run(&args.path, args.timeout, args.poll_ms) {
        Ok(code) => ExitCode::from(code),
        Err(_) => ExitCode::from(2),
    }
}

fn run(path: &Path, timeout_secs: u64, poll_ms: u64) -> std::io::Result<u8> {
    let timeout = Duration::from_secs(timeout_secs);
    let poll = Duration::from_millis(poll_ms);
    match wait_for_unlock(path, timeout, poll)? {
        WaitOutcome::Unlocked => Ok(0),
        WaitOutcome::Timeout => Ok(1),
    }
}
