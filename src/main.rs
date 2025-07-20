use clap::{Parser, Subcommand};
use std::process;

mod commands;
mod config;
mod drift;
mod error;
mod parser;

use crate::commands::{
    diff::DiffCommand,
    index::IndexCommand,
    init::InitCommand,
    inventory::InventoryCommand,
    propose::ProposeCommand,
};
use crate::config::Config;

#[derive(Parser)]
#[command(name = "adrscan")]
#[command(about = "Architecture Decision Record (ADR) management and drift detection")]
#[command(version, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Configuration file path
    #[arg(short, long, global = true)]
    config: Option<std::path::PathBuf>,

    /// Verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize ADR directory and configuration
    Init(InitCommand),
    /// Scan and inventory all existing ADRs and project state
    Inventory(InventoryCommand),
    /// Perform drift detection by diffing current state against baseline
    Diff(DiffCommand),
    /// Auto-generate draft ADRs for detected drift
    Propose(ProposeCommand),
    /// Generate or update an index of ADRs
    Index(IndexCommand),
}

fn main() {
    let cli = Cli::parse();

    // Initialize logging
    let log_level = if cli.verbose {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };
    
    env_logger::Builder::from_default_env()
        .filter_level(log_level)
        .init();

    // Load configuration
    let config = match Config::load(cli.config.as_deref()) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading configuration: {}", e);
            process::exit(1);
        }
    };

    // Execute command
    let result = match cli.command {
        Commands::Init(cmd) => cmd.execute(&config),
        Commands::Inventory(cmd) => cmd.execute(&config),
        Commands::Diff(cmd) => cmd.execute(&config),
        Commands::Propose(cmd) => cmd.execute(&config),
        Commands::Index(cmd) => cmd.execute(&config),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}