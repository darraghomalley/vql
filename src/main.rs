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
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Process args as a single command if they're provided
    if !cli.args.is_empty() {
        // Join args with spaces to form a single command string
        let command = cli.args.join(" ");
        
        // Process the command with our json_commands handler
        return commands::json_commands::process_command(&command);
    }
    
    // Process formal subcommands if provided
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
            }
        }
    } else {
        // No arguments or subcommands, show help
        println!("VQL - Virtual Quality Language CLI\n");
        println!("Usage:");
        println!("  vql -<command> [args]          # CLI commands with dash prefix");
        println!("  vql :<command> [args]          # LLM commands (for AI use)\n");
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
        println!("For more information, try 'vql --help'");
        Ok(())
    }
}