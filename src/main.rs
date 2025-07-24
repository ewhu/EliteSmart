// src/main.rs
/*
 * Main executable for EliteSmart
 */

use clap::Parser;
use elitesmart::{Result, run};

#[derive(Parser)]
#[command(version, about = "EliteSmart - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
