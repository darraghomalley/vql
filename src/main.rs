mod commands;
mod models;
mod utils;

use clap::{Parser, Subcommand};
use anyhow::Result;
use regex::Regex;
use colored::Colorize;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    /// Command to execute
    #[clap(subcommand)]
    command: Option<Commands>,
    
    /// Catch-all for VQL commands (using both formats)
    #[clap(allow_hyphen_values = true, trailing_var_arg = true)]
    args: Vec<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Process a VQL command string directly
    ProcessVQL {
        /// The VQL command to process
        command: String,
    },
    
    /// Setup VQL in the current directory
    Setup {
        /// Optional path to initialize VQL (defaults to current directory)
        #[clap(long)]
        path: Option<String>,
    },
    
    /// Load principles from a markdown file
    LoadPrinciples {
        /// Path to the principles markdown file
        path: String,
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Process formal subcommands first if provided
    if let Some(command) = &cli.command {
        match command {
            Commands::ProcessVQL { command } => {
                // Process direct VQL command
                commands::json_commands::process_command(command)
            },
            Commands::Setup { path } => {
                // Setup VQL directory
                let path_arg = if let Some(p) = path {
                    format!("setup {}", p)
                } else {
                    "setup".to_string()
                };
                commands::json_commands::process_command(&path_arg)
            },
            Commands::LoadPrinciples { path } => {
                // Load principles directly - bypass the argument parsing issues
                commands::json_commands::load_principles_from_markdown(path)
            }
        }
    } else if !cli.args.is_empty() {
        // Check for version flag
        if cli.args.len() == 1 && (cli.args[0] == "--version" || cli.args[0] == "-v") {
            println!("vql {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        } else {
            // Process args as a single command if they're provided
            commands::json_commands::process_command_with_args(&cli.args)
        }
    } else {
        // No arguments or subcommands, show help
        println!("VQL - Virtual Quality Language CLI\n");
        println!("Usage:");
        println!("  vql -<command> [args]                          # CLI commands with dash prefix");
        println!("  vql :<command> [args]                          # LLM commands (for AI use)");
        println!("  vql ':-pr.get(\"<path/to/principles.md>\")'       # Load principles (supports paths with spaces)\n");
        println!("CLI Command Examples:");
        println!("  vql -pr                                # List all perspectives");
        println!("  vql -er -add u User                    # Add entity");
        println!("  vql -at -add c Controller              # Add asset type");
        println!("  vql -ar -add uc \"C:/path/to/file.js\"   # Add asset reference");
        println!("  vql -st uc a \"Review content\"           # Store a review");
        println!("  vql -se uc t                           # Set exemplar status");
        println!("  vql -sc uc a H                         # Set compliance rating");
        println!("  vql uc ? | vql uc?                       # Retrieve all reviews for an asset");
        println!("  vql uc ? (a) | vql uc?(a)               # Retrieve specific review for an asset");
        println!("  vql uc ? (a,s) | vql uc?(a,s)           # Retrieve reviews for multiple perspectives\n");
        println!("Principles Loading (absolute system paths):");
        println!("  vql ':-pr.get(\"/home/user/my-principles.md\")'     # Load from absolute path");
        println!("  vql ':-pr.get(\"~/Documents/principles.md\")'       # Load with tilde expansion");
        println!("  vql -pr -get /home/user/principles.md             # Alternative (no spaces in path)\n");
        println!("For more information, try 'vql --help'");
        Ok(())
    }
}