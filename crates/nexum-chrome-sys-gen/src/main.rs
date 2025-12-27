//! CLI tool for generating Chrome Extension API Rust bindings.

use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;

use nexum_chrome_sys_gen::{generate, load_api_data};

#[derive(Parser, Debug)]
#[command(name = "nexum-chrome-sys-gen")]
#[command(about = "Generate Rust bindings for Chrome Extension APIs")]
struct Args {
    /// Path to chrome-api.json
    #[arg(short, long)]
    input: PathBuf,

    /// Output directory for generated files
    #[arg(short, long)]
    output: PathBuf,

    /// Only generate specific namespaces (comma-separated)
    #[arg(long, value_delimiter = ',')]
    only: Option<Vec<String>>,

    /// Skip specific namespaces (comma-separated)
    #[arg(long, value_delimiter = ',')]
    skip: Option<Vec<String>>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Load the API data
    let api_data = load_api_data(&args.input)
        .with_context(|| format!("Failed to load {}", args.input.display()))?;

    println!(
        "Loaded {} namespaces from {}",
        api_data.api.len(),
        args.input.display()
    );

    // Generate the bindings
    generate::Config {
        output_dir: args.output,
        only_namespaces: args.only,
        skip_namespaces: args.skip,
    }
    .run(&api_data)?;

    Ok(())
}
