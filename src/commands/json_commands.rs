use anyhow::{Result, Context, anyhow};
use colored::Colorize;
use std::env;
use std::path::{Path, PathBuf};
use std::fs;
use regex::Regex;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

use crate::models::json_storage::{JsonStorage, find_vql_storage};

/// Process a command (with or without colon prefix) or asset.method format
/// Process a command in either LLM format or CLI format
pub fn process_command(command: &str) -> Result<()> {
    // Check if this is an LLM command format (starts with colon)
    if command.starts_with(':') {
        // This is an LLM command format
        return process_llm_command(command);
    } else {
        // This is a CLI command format
        return process_cli_command(command);
    }
}

/// Process a command in LLM format (colon-prefixed)
fn process_llm_command(command: &str) -> Result<()> {
    // Remove colon prefix
    let command = command.trim_start_matches(':');
    
    // Check if this is a special human command
    match command {
        "-vql on" => return handle_vql_mode(true),
        "-vql off" => return handle_vql_mode(false),
        "uc ?" => return show_asset_why("uc", None),
        "mcp" => return handle_interface_mode("mcp"),
        "cli" => return handle_interface_mode("cli"),
        _ => {}
    }
    
    // Check for generic LLM operations first
    // Format: :rn(old, new) - generic rename
    let rn_re = Regex::new(r"^rn\(([^,]+),\s*([^)]+)\)$").unwrap();
    if let Some(captures) = rn_re.captures(command) {
        let old_name = captures.get(1).unwrap().as_str().trim();
        let new_name = captures.get(2).unwrap().as_str().trim();
        return rename_item(old_name, new_name);
    }
    
    // Format: :dl(name) - generic delete
    let dl_re = Regex::new(r"^dl\(([^)]+)\)$").unwrap();
    if let Some(captures) = dl_re.captures(command) {
        let name = captures.get(1).unwrap().as_str().trim();
        return delete_item(name);
    }
    
    // Format: :ls() or :ls - list all types
    if command == "ls()" || command == "ls" {
        println!("\nVQL Summary:");
        show_principles()?;
        println!();
        list_entities()?;
        println!();
        list_asset_types()?;
        println!();
        list_asset_references()?;
        return Ok(());
    }
    
    // Check for type-specific list commands
    match command {
        "pr()" | "pr" => return show_principles(),
        "er()" | "er" => return list_entities(),
        "at()" | "at" => return list_asset_types(),
        "ar()" | "ar" => return list_asset_references(),
        _ => {}
    }
    
    // Format: :pr.add(short, long, "guidance")
    let pr_add_re = Regex::new(r#"^pr\.add\(([^,]+),\s*([^,]+)(?:,\s*"([^"]*)")?\)$"#).unwrap();
    if let Some(captures) = pr_add_re.captures(command) {
        let short_name = captures.get(1).unwrap().as_str().trim();
        let long_name = captures.get(2).unwrap().as_str().trim();
        let guidance = captures.get(3).map(|m| m.as_str());
        return add_principle(short_name, long_name, guidance);
    }
    
    // Format: :er.add(short, long)
    let er_add_re = Regex::new(r"^er\.add\(([^,]+),\s*([^)]+)\)$").unwrap();
    if let Some(captures) = er_add_re.captures(command) {
        let short_name = captures.get(1).unwrap().as_str().trim();
        let long_name = captures.get(2).unwrap().as_str().trim();
        return add_entity(&[short_name, long_name]);
    }
    
    // Format: :at.add(short, description)
    let at_add_re = Regex::new(r"^at\.add\(([^,]+),\s*([^)]+)\)$").unwrap();
    if let Some(captures) = at_add_re.captures(command) {
        let short_name = captures.get(1).unwrap().as_str().trim();
        let description = captures.get(2).unwrap().as_str().trim();
        return add_asset_type(&[short_name, description]);
    }
    
    // Format: :ar.add(short, entity, type, "path")
    let ar_add_re = Regex::new(r#"^ar\.add\(([^,]+),\s*([^,]+),\s*([^,]+),\s*"([^"]*)"\)$"#).unwrap();
    if let Some(captures) = ar_add_re.captures(command) {
        let short_name = captures.get(1).unwrap().as_str().trim();
        let entity = captures.get(2).unwrap().as_str().trim();
        let asset_type = captures.get(3).unwrap().as_str().trim();
        let path = captures.get(4).unwrap().as_str();
        return add_asset_reference(&[short_name, entity, asset_type, path]);
    }
    
    // Check for asset question format like :uc ? (a) or :uc?(a) - with or without space before/after question mark
    let asset_question_re = Regex::new(r"^([a-zA-Z0-9_]+)(\s+)?\?(\s*)?\(([^)]*)\)$").unwrap();
    if let Some(captures) = asset_question_re.captures(command) {
        let asset_name = captures.get(1).unwrap().as_str();
        let principle = captures.get(4).unwrap().as_str();
        
        // If the principle contains commas, use show_asset_why with the principle
        if principle.contains(',') {
            return show_asset_why(asset_name, Some(principle));
        } else {
            return show_asset_principle_review(asset_name, principle);
        }
    }
    
    // Check for simple asset question format like :uc ? or :uc? - with or without space
    let simple_question_re = Regex::new(r"^([a-zA-Z0-9_]+)(\s+)?\?$").unwrap();
    if let Some(captures) = simple_question_re.captures(command) {
        let asset_name = captures.get(1).unwrap().as_str();
        return show_asset_why(asset_name, None);
    }
    
    // Check for principle command with get method
    let principle_get_re = Regex::new(r"^-pr\.get\(([^)]*)\)$").unwrap();
    if let Some(captures) = principle_get_re.captures(command) {
        let path_with_quotes = captures.get(1).unwrap().as_str().trim();
        // Remove surrounding quotes if present
        let path = path_with_quotes.trim_matches('"');
        
        return load_principles_from_md(path);
    }
    
    // Check for asset reference add command (LLM format with commas)
    let ar_add_re = Regex::new(r"^-ar\.add\(([^,]+),\s*([^,]+),\s*([^,]+),\s*([^)]*)\)$").unwrap();
    if let Some(captures) = ar_add_re.captures(command) {
        let asset_ref = captures.get(1).unwrap().as_str().trim();
        let entity = captures.get(2).unwrap().as_str().trim();
        let asset_type = captures.get(3).unwrap().as_str().trim();
        let path_with_quotes = captures.get(4).unwrap().as_str().trim();
        // Remove surrounding quotes if present
        let path = path_with_quotes.trim_matches('"');
        
        return add_asset_reference(&[asset_ref, entity, asset_type, path]);
    }
    
    // Check if this is an asset method with specialized LLM syntax
    let asset_llm_method_re = Regex::new(r"^([a-zA-Z0-9_]+)\.([a-z]{2})\(([^)]*)\)$").unwrap();
    if let Some(captures) = asset_llm_method_re.captures(command) {
        let asset_name = captures.get(1).unwrap().as_str();
        let method = captures.get(2).unwrap().as_str();
        let args = captures.get(3).unwrap().as_str();
        
        // Handle the specialized LLM methods
        match method {
            "st" => return handle_asset_store(asset_name, args),
            "rv" => return handle_asset_review(asset_name, args),
            "rf" => return handle_asset_refactor(asset_name, args),
            "se" => return handle_asset_set_exemplar(asset_name, args),
            "sc" => return handle_asset_set_compliance(asset_name, args),
            _ => {} // Continue with regular processing
        }
    }
    
    // Check for global commands with special format
    let global_llm_command_re = Regex::new(r"^-([a-z]{2})\(([^)]*)\)$").unwrap();
    if let Some(captures) = global_llm_command_re.captures(command) {
        let method = captures.get(1).unwrap().as_str();
        let args = captures.get(2).unwrap().as_str();
        
        // Handle global LLM commands
        match method {
            "rv" => return handle_global_review(args),
            "rf" => return handle_global_refactor(args),
            "su" => {
                // Handle VQL setup with project folder path
                let project_path = args.trim().trim_matches('"'); // Remove quotes if present
                
                // Handle tilde expansion for home directory
                let expanded_path = if project_path.starts_with("~") {
                    // Get the home directory
                    if let Ok(home) = std::env::var("HOME") {
                        // Replace ~ with the actual home directory
                        let rel_path = &project_path[1..];
                        format!("{}{}", home, rel_path)
                    } else {
                        // If HOME isn't available, keep the original
                        project_path.to_string()
                    }
                } else {
                    project_path.to_string()
                };
                
                return setup_vql_directory_with_args(&[&expanded_path]);
            },
            _ => {} // Continue with regular processing
        }
    }
    
    // Check for simple principle commands
    match command {
        "-pr" => return show_principles(),
        "-er" => return list_entities(),
        "-at" => return list_asset_types(),
        "-ar" => return list_asset_references(),
        _ => {}
    }
    
    // If it's not one of the specialized LLM formats, 
    // fall back to the regular command processing
    process_cli_command(command)
}

/// Process a command in CLI format
fn process_cli_command(command: &str) -> Result<()> {
    // Check if this is a dash-prefixed command format (CLI style)
    if command.starts_with('-') {
        // This is a CLI-style flag command
        return process_cli_flag_command(command);
    }
    
    // Note: Setup command has been moved to use -su flag format
    // The old setup command format is no longer supported
    
    // Check if this is an asset question format (for retrieving reviews)
    // Support "asset ? (principle)" or "asset?(principle)" format with or without spaces
    let question_re = Regex::new(r"^([a-zA-Z0-9_]+)(\s+)?\?(\s*)?(\(([^)]*)\))?$").unwrap();
    if let Some(captures) = question_re.captures(command) {
        let asset_name = captures.get(1).unwrap().as_str();
        let principle = captures.get(5).map(|m| m.as_str());
        
        return if let Some(p) = principle {
            // If the principle contains commas, use show_asset_why with the principle
            if p.contains(',') {
                show_asset_why(asset_name, Some(p))
            } else {
                // Otherwise use the specific principle review
                show_asset_principle_review(asset_name, p)
            }
        } else {
            show_asset_why(asset_name, None)
        };
    }
    
    // If we get here, we don't recognize the command format
    Err(anyhow!("Unknown command format: {}. Commands must start with - (CLI) or : (LLM).", command))
}

/// Process a CLI flag-based command (like -pr -add)
fn process_cli_flag_command(command: &str) -> Result<()> {
    // Split by whitespace to separate flags
    let parts: Vec<&str> = command.split_whitespace().collect();
    
    if parts.is_empty() {
        return Err(anyhow!("Invalid command format"));
    }
    
    // Normalize main command (remove leading dash if present)
    let main_cmd = parts[0].trim_start_matches('-');
    
    // Handle first part as main command
    match main_cmd {
        "rn" => {
            // Generic rename: -rn old_name new_name
            if parts.len() < 3 {
                return Err(anyhow!("Not enough arguments for rename. Usage: -rn old_name new_name"));
            }
            
            let old_name = parts[1];
            let new_name = parts[2];
            
            return rename_item(old_name, new_name);
        },
        "dl" => {
            // Generic delete: -dl name
            if parts.len() < 2 {
                return Err(anyhow!("Not enough arguments for delete. Usage: -dl name"));
            }
            
            let name = parts[1];
            
            return delete_item(name);
        },
        "pr" => {
            // Principle commands
            if parts.len() > 1 {
                // Check if subcommand starts with dash
                if !parts[1].starts_with('-') {
                    return Err(anyhow!("Invalid subcommand format. Subcommands must start with - (e.g., -add)"));
                }
                
                // Remove leading dash
                let subcmd = parts[1].trim_start_matches('-');
                
                match subcmd {
                    "add" => {
                        // -pr -add a Architecture "Architecture Guidelines"
                        if parts.len() < 4 {
                            return Err(anyhow!("Not enough arguments for principle add"));
                        }
                        
                        let short_name = parts[2];
                        let long_name = parts[3];
                        
                        // Check if there's a guidance string
                        let guidance = if parts.len() > 4 {
                            Some(parts[4])
                        } else {
                            None
                        };
                        
                        return add_principle(short_name, long_name, guidance.as_deref());
                    },
                    "get" => {
                        // -pr -get "path/to/principles.md"
                        if parts.len() < 3 {
                            return Err(anyhow!("Not enough arguments for principle get. Usage: -pr -get \"path/to/principles.md\""));
                        }
                        
                        let file_path = parts[2];
                        return load_principles_from_md(file_path);
                    },
                    "rn" => {
                        // -pr -rn a arch
                        if parts.len() < 4 {
                            return Err(anyhow!("Not enough arguments for principle rename. Usage: -pr -rn old_name new_name"));
                        }
                        
                        let old_name = parts[2];
                        let new_name = parts[3];
                        
                        return rename_principle(old_name, new_name);
                    },
                    "dl" => {
                        // -pr -dl arch
                        if parts.len() < 3 {
                            return Err(anyhow!("Not enough arguments for principle delete. Usage: -pr -dl name"));
                        }
                        
                        let name = parts[2];
                        
                        return delete_principle(name);
                    },
                    _ => return Err(anyhow!("Unknown principle subcommand: {}", parts[1])),
                }
            } else {
                // Just pr by itself
                return show_principles();
            }
        },
        "er" => {
            // Entity commands
            if parts.len() > 1 {
                // Check if subcommand starts with dash
                if !parts[1].starts_with('-') {
                    return Err(anyhow!("Invalid subcommand format. Subcommands must start with - (e.g., -add)"));
                }
                
                // Remove leading dash
                let subcmd = parts[1].trim_start_matches('-');
                
                match subcmd {
                    "add" => {
                        // -er -add u User
                        if parts.len() < 3 {
                            return Err(anyhow!("Not enough arguments for entity add"));
                        }
                        
                        let short_name = parts[2];
                        let long_name = if parts.len() > 3 { parts[3] } else { short_name };
                        
                        return add_entity(&[short_name, long_name]);
                    },
                    "rn" => {
                        // -er -rn u usr
                        if parts.len() < 4 {
                            return Err(anyhow!("Not enough arguments for entity rename. Usage: -er -rn old_name new_name"));
                        }
                        
                        let old_name = parts[2];
                        let new_name = parts[3];
                        
                        return rename_entity(old_name, new_name);
                    },
                    "dl" => {
                        // -er -dl ex
                        if parts.len() < 3 {
                            return Err(anyhow!("Not enough arguments for entity delete. Usage: -er -dl name"));
                        }
                        
                        let name = parts[2];
                        
                        return delete_entity(name);
                    },
                    _ => return Err(anyhow!("Unknown entity subcommand: {}", parts[1])),
                }
            } else {
                // Just er by itself
                return list_entities();
            }
        },
        "at" => {
            // Asset type commands
            if parts.len() > 1 {
                // Check if subcommand starts with dash
                if !parts[1].starts_with('-') {
                    return Err(anyhow!("Invalid subcommand format. Subcommands must start with - (e.g., -add)"));
                }
                
                // Remove leading dash
                let subcmd = parts[1].trim_start_matches('-');
                
                match subcmd {
                    "add" => {
                        // -at -add c Controller
                        if parts.len() < 4 {
                            return Err(anyhow!("Not enough arguments for asset type add"));
                        }
                        
                        let short_name = parts[2];
                        let description = parts[3];
                        
                        return add_asset_type(&[short_name, description]);
                    },
                    "rn" => {
                        // -at -rn c ctrl
                        if parts.len() < 4 {
                            return Err(anyhow!("Not enough arguments for asset type rename. Usage: -at -rn old_name new_name"));
                        }
                        
                        let old_name = parts[2];
                        let new_name = parts[3];
                        
                        return rename_asset_type(old_name, new_name);
                    },
                    "dl" => {
                        // -at -dl model
                        if parts.len() < 3 {
                            return Err(anyhow!("Not enough arguments for asset type delete. Usage: -at -dl name"));
                        }
                        
                        let name = parts[2];
                        
                        return delete_asset_type(name);
                    },
                    _ => return Err(anyhow!("Unknown asset type subcommand: {}", parts[1])),
                }
            } else {
                // Just at by itself
                return list_asset_types();
            }
        },
        "ar" => {
            // Asset reference commands
            if parts.len() > 1 {
                // Check if subcommand starts with dash
                if !parts[1].starts_with('-') {
                    return Err(anyhow!("Invalid subcommand format. Subcommands must start with - (e.g., -add)"));
                }
                
                // Remove leading dash
                let subcmd = parts[1].trim_start_matches('-');
                
                match subcmd {
                    "add" => {
                        // Format: -ar -add short_name entity asset_type path (space-separated only)
                        if parts.len() >= 6 {
                            // All parameters specified
                            let short_name = parts[2];
                            let entity = parts[3];
                            let asset_type = parts[4];
                            let path = parts[5];
                            
                            return add_asset_reference(&[short_name, entity, asset_type, path]);
                        } else {
                            return Err(anyhow!("Not enough arguments for asset reference add. Usage: -ar -add shortName entityType assetType path"));
                        }
                    },
                    "rn" => {
                        // -ar -rn uc userctrl
                        if parts.len() < 4 {
                            return Err(anyhow!("Not enough arguments for asset reference rename. Usage: -ar -rn old_name new_name"));
                        }
                        
                        let old_name = parts[2];
                        let new_name = parts[3];
                        
                        return rename_asset_reference(old_name, new_name);
                    },
                    "dl" => {
                        // -ar -dl example
                        if parts.len() < 3 {
                            return Err(anyhow!("Not enough arguments for asset delete. Usage: -ar -dl name"));
                        }
                        
                        let name = parts[2];
                        
                        return delete_asset_reference(name);
                    },
                    _ => return Err(anyhow!("Unknown asset reference subcommand: {}", parts[1])),
                }
            } else {
                // Just ar by itself
                return list_asset_references();
            }
        },
        "st" => {
            // Store command: -st asset_name principle "Review Content"
            
            if parts.len() < 4 {
                return Err(anyhow!("Not enough arguments for store command. Usage: -st asset_name principle \"Review Content\""));
            }
            
            let asset_name = parts[1];
            let principle = parts[2];
            
            // Handle quoted content properly - join remaining parts and strip quotes if present
            let content_raw = parts[3..].join(" ");
            let content = content_raw.trim_matches('"');
            
            return store_asset_review(asset_name, principle, content);
        },
        "se" => {
            // Set exemplar: -se asset t|f
            if parts.len() < 3 {
                return Err(anyhow!("Not enough arguments for set exemplar command. Usage: -se asset t|f"));
            }
            
            let asset_name = parts[1];
            let status = parts[2];
            
            return set_asset_exemplar(&[asset_name, status]);
        },
        "sc" => {
            // Set compliance: -sc asset principle H|M|L
            if parts.len() < 4 {
                return Err(anyhow!("Not enough arguments for set compliance command. Usage: -sc asset principle H|M|L"));
            }
            
            let asset_name = parts[1];
            let principle = parts[2];
            let rating = parts[3];
            
            return set_asset_compliance(&[asset_name, principle, rating]);
        },
        "su" => {
            // Setup VQL: -su "path/to/directory"
            if parts.len() > 1 {
                // Path is provided - handle quoted paths
                let path_raw = parts[1..].join(" ");
                let path = path_raw.trim_matches('"');
                return setup_vql_directory_with_args(&[path]);
            } else {
                // No path provided, use current directory
                return setup_vql_directory();
            }
        },
        _ => return Err(anyhow!("Unknown command: {}", command)),
    }
}

/// Setup VQL directory with optional path from command string
fn setup_vql_directory_with_path(command: &str) -> Result<()> {
    // Parse the command to extract path if provided
    let parts: Vec<&str> = command.split_whitespace().collect();
    
    if parts.len() > 1 {
        // Path is provided
        let path = parts[1];
        setup_vql_directory_with_args(&[path])
    } else {
        // No path provided, use current directory
        setup_vql_directory()
    }
}

/// Setup VQL directory in the current location
fn setup_vql_directory() -> Result<()> {
    // This is the command without colon: "setup"
    
    // Get the current directory
    let current_dir = env::current_dir()
        .context("Failed to get current directory")?;
        
    setup_vql_directory_in_path(&current_dir)
}

/// Setup VQL directory with optional path argument
fn setup_vql_directory_with_args(args: &[&str]) -> Result<()> {
    if args.is_empty() {
        // No path provided, use current directory
        setup_vql_directory()
    } else {
        // Use provided path
        let path_str = args[0];
        
        // Handle tilde expansion for home directory
        let expanded_path = if path_str.starts_with("~") {
            // Get the home directory
            if let Ok(home) = std::env::var("HOME") {
                // Replace ~ with the actual home directory
                let rel_path = &path_str[1..];
                let new_path = format!("{}{}", home, rel_path);
                new_path
            } else {
                // If HOME isn't available, keep the original
                path_str.to_string()
            }
        } else {
            path_str.to_string()
        };
        
        let path = Path::new(&expanded_path);
        
        if path.exists() && path.is_dir() {
            setup_vql_directory_in_path(path)
        } else {
            // Try to create the directory if it doesn't exist
            if !path.exists() {
                if let Err(e) = std::fs::create_dir_all(path) {
                    return Err(anyhow!("Failed to create directory '{}': {}", expanded_path, e));
                }
                // Now directory should exist, proceed with setup
                setup_vql_directory_in_path(path)
            } else {
                Err(anyhow!("Invalid directory path: {}", expanded_path))
            }
        }
    }
}

/// Setup VQL directory in the specified path
fn setup_vql_directory_in_path(path: &Path) -> Result<()> {
    // Create VQL directory
    let vql_dir = path.join("VQL");
    
    if vql_dir.exists() {
        println!("{} VQL directory already exists at {}", 
            "INFO:".blue().bold(), 
            vql_dir.display().to_string().blue());
            
        // Check if json storage exists, if not, create it
        let json_file_path = vql_dir.join("vql_storage.json");
        if !json_file_path.exists() {
            let storage = JsonStorage::new();
            storage.save(&vql_dir)?;
            
            println!("{} Created new VQL storage file", 
                "SUCCESS:".green().bold());
        }
    } else {
        // Create directory and initial storage
        fs::create_dir_all(&vql_dir)
            .context(format!("Failed to create VQL directory at {}", vql_dir.display()))?;
            
        let storage = JsonStorage::new();
        storage.save(&vql_dir)?;
        
        println!("{} VQL initialized successfully in: {}", 
            "SUCCESS:".green().bold(), 
            vql_dir.display().to_string().blue());
    }
    
    // Display available commands
    println!("\nAvailable commands:");
    println!("  {} or {} - Asset Register commands", ":ar".blue(), "ar".blue());
    println!("  {} or {} - Asset Type commands", ":at".blue(), "at".blue());
    println!("  {} or {} - Entity Register commands", ":er".blue(), "er".blue());
    println!("  {} or {} - Command management", ":cmd".blue(), "cmd".blue());
    println!("  {} or {} - Show this help", ":help".blue(), "help".blue());
    
    Ok(())
}

/// Display help information
fn show_help() -> Result<()> {
    println!("{}", "VQL CLI Help".bold());
    
    println!("\n{}", "CLI Commands:".bold());
    println!("  {} - Setup VQL in the current directory", "setup".blue());
    println!("  {} - Asset Register commands", "-ar".blue());
    println!("  {} - Asset Type commands", "-at".blue());
    println!("  {} - Entity Register commands", "-er".blue());
    println!("  {} - Principle commands", "-pr".blue());
    println!("  {} - Store a review", "-str".blue());
    println!("  {} - Set exemplar status", "-se".blue());
    println!("  {} - Set compliance rating", "-sc".blue());
    
    println!("\n{}", "CLI Command Examples:".bold());
    println!("  {} - List all principles", "vql -pr".blue());
    println!("  {} - Add a new principle", "vql -pr -add a Architecture \"Architecture Principles\"".blue());
    println!("  {} - Add a new entity", "vql -er -add u User".blue());
    println!("  {} - Add a new asset type", "vql -at -add c Controller".blue());
    println!("  {} - Add a new asset reference", "vql -ar -add uc u c \"C:/Project/UserController.js\"".blue());
    println!("  {} - Store a review for an asset", "vql -st uc a \"Review Content\"".blue());
    println!("  {} - Set exemplar status", "vql -se uc t".blue());
    println!("  {} - Set compliance rating", "vql -sc uc a H".blue());
    
    println!("\n{}", "LLM Commands (colon-prefixed):".bold());
    println!("  {} - Turn VQL mode on/off", ":-vql on|off".blue());
    println!("  {} - Show all principles", ":-pr".blue());
    println!("  {} - Show all entities", ":-er".blue());
    println!("  {} - Show all asset types", ":-at".blue());
    println!("  {} - Show all asset references", ":-ar".blue());
    
    println!("\n{}", "LLM Asset Command Examples:".bold());
    println!("  {} or {} - Get all reviews for an asset", ":uc ?".blue(), ":uc?".blue());
    println!("  {} or {} - Get specific principle review", ":uc ? (a)".blue(), ":uc?(a)".blue());
    println!("  {} or {} - Get multiple principle reviews", ":uc ? (a,s)".blue(), ":uc?(a,s)".blue());
    println!("  {} - Store a review", ":uc.st(a, \"Review content\")".blue());
    println!("  {} - Set exemplar status", ":uc.se(t)".blue());
    println!("  {} - Set compliance rating", ":uc.sc(a, H)".blue());
    
    println!("\n{}", "LLM-Only Commands (AI-assisted):".bold());
    println!("  {} - Review specific asset", ":uc.rv(*)".blue());
    println!("  {} - Review specific asset with principles", ":uc.rv(a s)".blue());
    println!("  {} - Review all assets", ":-rv(*)".blue());
    println!("  {} - Refactor specific asset", ":uc.rf(*)".blue());
    println!("  {} - Refactor specific asset with principles", ":uc.rf(a s)".blue());
    println!("  {} - Refactor all assets", ":-rf(*)".blue());
    
    Ok(())
}

/// Show asset register command usage
fn show_ar_usage() -> Result<()> {
    println!("{}", "Asset Register Commands".bold());
    println!("\nAvailable commands: (colon prefix is optional)");
    println!("  {} or {} - List all asset references", ":ar.list".blue(), "ar.list".blue());
    println!("  {} or {} - Add a new asset reference", ":ar.add(shortName, entity, assetType, path)".blue(), "ar.add(shortName, entity, assetType, path)".blue());
    println!("  {} or {} - Review an asset", ":ar.review(shortName, aspect, analysis)".blue(), "ar.review(shortName, aspect, analysis)".blue());
    println!("  {} or {} - Set asset compliance", ":ar.setCompliance(shortName, aspect, rating)".blue(), "ar.setCompliance(shortName, aspect, rating)".blue());
    println!("  {} or {} - Set asset exemplar status", ":ar.setExemplar(shortName, status)".blue(), "ar.setExemplar(shortName, status)".blue());
    
    println!("\nYou can also use direct asset methods:");
    println!("  {} - Review an asset", "assetName.review(aspect, analysis)".blue());
    println!("  {} - Set asset compliance", "assetName.setCompliance(aspect, rating)".blue());
    
    Ok(())
}

/// Show asset type command usage
fn show_at_usage() -> Result<()> {
    println!("{}", "Asset Type Commands".bold());
    println!("\nAvailable commands: (colon prefix is optional)");
    println!("  {} or {} - List all asset types", ":at.list".blue(), "at.list".blue());
    println!("  {} or {} - Add a new asset type", ":at.add(shortName, description)".blue(), "at.add(shortName, description)".blue());
    
    Ok(())
}

/// Show entity register command usage
fn show_er_usage() -> Result<()> {
    println!("{}", "Entity Register Commands".bold());
    println!("\nAvailable commands: (colon prefix is optional)");
    println!("  {} or {} - List all entities", ":er.list".blue(), "er.list".blue());
    println!("  {} or {} - Add a new entity", ":er.add(shortName, description)".blue(), "er.add(shortName, description)".blue());
    
    Ok(())
}

/// Show command management usage
fn show_cmd_usage() -> Result<()> {
    println!("{}", "Command Management".bold());
    println!("\nAvailable commands: (colon prefix is optional)");
    println!("  {} or {} - List all commands", ":cmd.list".blue(), "cmd.list".blue());
    println!("  {} or {} - Add a new command", ":cmd.add(name, description)".blue(), "cmd.add(name, description)".blue());
    println!("  {} or {} - Rename a command", ":cmd.rename(oldName, newName)".blue(), "cmd.rename(oldName, newName)".blue());
    
    Ok(())
}

/// Show principle command usage
fn show_pr_usage() -> Result<()> {
    println!("{}", "Principle Commands".bold());
    println!("\nAvailable commands: (colon prefix is optional)");
    println!("  {} or {} - List all principles", ":pr.list".blue(), "pr.list".blue());
    println!("  {} or {} - Add a new principle", ":pr.add(shortName, longName, guidance)".blue(), "pr.add(shortName, longName, guidance)".blue());
    println!("\nCLI format:");
    println!("  {} - List all principles", "-pr".blue());
    println!("  {} - Add a new principle", "-pr -add a Architecture \"Architecture Guidelines\"".blue());
    
    Ok(())
}

// Define the remaining functions...

/// Add a new principle to the storage
fn add_principle(short_name: &str, long_name: &str, guidance: Option<&str>) -> Result<()> {
    if short_name.chars().count() != 1 {
        return Err(anyhow!("Principle short name must be a single character"));
    }
    
    // Find VQL storage
    let (vql_dir, mut storage) = find_vql_storage()
        .context("Failed to find or load VQL storage")?;
    
    // Add or update principle
    storage.add_principle(short_name, long_name, guidance)?;
    
    // Save changes
    storage.save(&vql_dir)?;
    
    println!("{} Added principle: {} ({})", 
        "SUCCESS:".green().bold(), 
        short_name.blue(),
        long_name);
    
    Ok(())
}

/// Show all principles
fn show_principles() -> Result<()> {
    // Find VQL storage
    let (_, storage) = find_vql_storage()
        .context("Failed to find or load VQL storage")?;
    
    // Get all principles
    let principles = &storage.principles;
    
    if principles.is_empty() {
        println!("{} No principles defined", "INFO:".blue().bold());
        return Ok(());
    }
    
    println!("{}", "Principles:".bold());
    
    // Sort principles by short name
    let mut sorted_principles: Vec<_> = principles.values().collect();
    sorted_principles.sort_by(|a, b| a.short_name.cmp(&b.short_name));
    
    for princ in sorted_principles {
        println!("  {} ({}): {}", 
            princ.short_name.blue().bold(),
            princ.long_name,
            princ.guidance.as_deref().unwrap_or("No guidance provided"));
    }
    
    Ok(())
}

/// Add a new entity
fn add_entity(args: &[&str]) -> Result<()> {
    if args.len() < 2 {
        return Err(anyhow!("Not enough arguments. Usage: add_entity short_name description"));
    }
    
    let short_name = args[0];
    let description = args[1];
    
    // Find VQL storage
    let (vql_dir, mut storage) = find_vql_storage()
        .context("Failed to find or load VQL storage")?;
    
    // Add or update entity
    storage.add_entity(short_name, description)?;
    
    // Save changes
    storage.save(&vql_dir)?;
    
    println!("{} Added entity: {} ({})", 
        "SUCCESS:".green().bold(), 
        short_name.blue(),
        description);
    
    Ok(())
}

/// List all entities
fn list_entities() -> Result<()> {
    // Find VQL storage
    let (_, storage) = find_vql_storage()
        .context("Failed to find or load VQL storage")?;
    
    // Get all entities
    let entities = &storage.entities;
    
    if entities.is_empty() {
        println!("{} No entities defined", "INFO:".blue().bold());
        return Ok(());
    }
    
    println!("{}", "Entities:".bold());
    
    // Sort entities by short name
    let mut sorted_entities: Vec<_> = entities.values().collect();
    sorted_entities.sort_by(|a, b| a.short_name.cmp(&b.short_name));
    
    for entity in sorted_entities {
        println!("  {} ({})", 
            entity.short_name.blue().bold(),
            entity.description);
    }
    
    Ok(())
}

/// Add a new asset type
fn add_asset_type(args: &[&str]) -> Result<()> {
    if args.len() < 2 {
        return Err(anyhow!("Not enough arguments. Usage: add_asset_type short_name description"));
    }
    
    let short_name = args[0];
    let description = args[1];
    
    if short_name.chars().count() != 1 {
        return Err(anyhow!("Asset type short name must be a single character"));
    }
    
    // Find VQL storage
    let (vql_dir, mut storage) = find_vql_storage()
        .context("Failed to find or load VQL storage")?;
    
    // Add or update asset type
    storage.add_asset_type(short_name, description)?;
    
    // Save changes
    storage.save(&vql_dir)?;
    
    println!("{} Added asset type: {} ({})", 
        "SUCCESS:".green().bold(), 
        short_name.blue(),
        description);
    
    Ok(())
}

/// List all asset types
fn list_asset_types() -> Result<()> {
    // Find VQL storage
    let (_, storage) = find_vql_storage()
        .context("Failed to find or load VQL storage")?;
    
    // Get all asset types
    let asset_types = &storage.asset_types;
    
    if asset_types.is_empty() {
        println!("{} No asset types defined", "INFO:".blue().bold());
        return Ok(());
    }
    
    println!("{}", "Asset Types:".bold());
    
    // Sort asset types by short name
    let mut sorted_asset_types: Vec<_> = asset_types.values().collect();
    sorted_asset_types.sort_by(|a, b| a.short_name.cmp(&b.short_name));
    
    for asset_type in sorted_asset_types {
        println!("  {} ({})", 
            asset_type.short_name.blue().bold(),
            asset_type.description);
    }
    
    Ok(())
}

/// Add a new asset reference
fn add_asset_reference(args: &[&str]) -> Result<()> {
    if args.len() < 4 {
        return Err(anyhow!("Not enough arguments. Usage: add_asset_reference short_name entity asset_type path"));
    }
    
    let short_name = args[0];
    let entity = args[1];
    let asset_type = args[2];
    let path = args[3];
    
    // Find VQL storage first to get its directory
    let (vql_dir, mut storage) = find_vql_storage()
        .context("Failed to find or load VQL storage")?;
        
    // Resolve the path relative to the VQL directory
    // First get the absolute path of the VQL directory
    let vql_dir_abs = fs::canonicalize(&vql_dir)
        .context("Failed to canonicalize VQL directory path")?;
        
    // Get the parent of the VQL directory
    let vql_parent = vql_dir_abs.parent().unwrap_or(&vql_dir_abs);
    
    // Resolve the path based on whether it's absolute or relative
    let resolved_path = if path.starts_with("/") {
        // Absolute path
        PathBuf::from(path)
    } else {
        // Relative path (relative to VQL directory's parent)
        vql_parent.join(path)
    };
    
    // Check if the file exists
    if !resolved_path.exists() {
        return Err(anyhow!("File not found: {}. The file must exist to be added as an asset reference. Path is resolved relative to the VQL directory's parent.", path));
    }
    if !resolved_path.is_file() {
        return Err(anyhow!("Path is not a file: {}. Only files can be added as asset references.", path));
    };
    
    // Add or update asset reference
    storage.add_asset_reference(short_name, entity, asset_type, path)?;
    
    // Save changes
    storage.save(&vql_dir)?;
    
    println!("{} Added asset reference: {} (Entity: {}, Type: {}, Path: {})", 
        "SUCCESS:".green().bold(), 
        short_name.blue().bold(),
        entity,
        asset_type,
        path);
    
    Ok(())
}

/// Rename a principle
fn rename_principle(old_name: &str, new_name: &str) -> Result<()> {
    // Find VQL storage
    let (vql_dir, mut storage) = find_vql_storage()
        .context("Failed to find or load VQL storage")?;
    
    // Count affected assets before rename
    let affected_assets: Vec<String> = storage.asset_references
        .iter()
        .filter_map(|(asset_name, asset)| {
            if asset.principle_reviews.contains_key(old_name) {
                Some(asset_name.clone())
            } else {
                None
            }
        })
        .collect();
    
    // Rename the principle
    storage.rename_principle(old_name, new_name)?;
    
    // Save storage
    storage.save(&vql_dir)?;
    
    println!("{} Renamed principle '{}' to '{}'", 
        "SUCCESS:".green().bold(),
        old_name.blue().bold(),
        new_name.blue().bold());
        
    if !affected_assets.is_empty() {
        println!("{} Updated principle key in {} asset review(s):", 
            "CASCADE:".yellow().bold(),
            affected_assets.len());
        for asset_name in affected_assets {
            println!("  - {}", asset_name);
        }
    }
    
    Ok(())
}

/// Rename an entity
fn rename_entity(old_name: &str, new_name: &str) -> Result<()> {
    // Find VQL storage
    let (vql_dir, mut storage) = find_vql_storage()
        .context("Failed to find or load VQL storage")?;
    
    // Count affected assets before rename
    let affected_assets: Vec<String> = storage.asset_references
        .iter()
        .filter_map(|(asset_name, asset)| {
            if asset.entity == old_name {
                Some(asset_name.clone())
            } else {
                None
            }
        })
        .collect();
    
    // Rename the entity
    storage.rename_entity(old_name, new_name)?;
    
    // Save storage
    storage.save(&vql_dir)?;
    
    println!("{} Renamed entity '{}' to '{}'", 
        "SUCCESS:".green().bold(),
        old_name.blue().bold(),
        new_name.blue().bold());
        
    if !affected_assets.is_empty() {
        println!("{} Updated entity reference in {} asset(s):", 
            "CASCADE:".yellow().bold(),
            affected_assets.len());
        for asset_name in affected_assets {
            println!("  - {}", asset_name);
        }
    }
    
    Ok(())
}

/// Rename an asset type
fn rename_asset_type(old_name: &str, new_name: &str) -> Result<()> {
    // Find VQL storage
    let (vql_dir, mut storage) = find_vql_storage()
        .context("Failed to find or load VQL storage")?;
    
    // Count affected assets before rename
    let affected_assets: Vec<String> = storage.asset_references
        .iter()
        .filter_map(|(asset_name, asset)| {
            if asset.asset_type == old_name {
                Some(asset_name.clone())
            } else {
                None
            }
        })
        .collect();
    
    // Rename the asset type
    storage.rename_asset_type(old_name, new_name)?;
    
    // Save storage
    storage.save(&vql_dir)?;
    
    println!("{} Renamed asset type '{}' to '{}'", 
        "SUCCESS:".green().bold(),
        old_name.blue().bold(),
        new_name.blue().bold());
        
    if !affected_assets.is_empty() {
        println!("{} Updated asset type reference in {} asset(s):", 
            "CASCADE:".yellow().bold(),
            affected_assets.len());
        for asset_name in affected_assets {
            println!("  - {}", asset_name);
        }
    }
    
    Ok(())
}

/// Rename an asset reference
fn rename_asset_reference(old_name: &str, new_name: &str) -> Result<()> {
    // Find VQL storage
    let (vql_dir, mut storage) = find_vql_storage()
        .context("Failed to find or load VQL storage")?;
    
    // Rename the asset reference
    storage.rename_asset_reference(old_name, new_name)?;
    
    // Save storage
    storage.save(&vql_dir)?;
    
    println!("{} Renamed asset '{}' to '{}'", 
        "SUCCESS:".green().bold(),
        old_name.blue().bold(),
        new_name.blue().bold());
    
    Ok(())
}

/// Delete a principle
fn delete_principle(name: &str) -> Result<()> {
    // Find VQL storage
    let (vql_dir, mut storage) = find_vql_storage()
        .context("Failed to find or load VQL storage")?;
    
    // Count affected assets before deletion
    let affected_assets: Vec<(String, usize)> = storage.asset_references
        .iter()
        .filter_map(|(asset_name, asset)| {
            if asset.principle_reviews.contains_key(name) {
                Some((asset_name.clone(), asset.principle_reviews.len()))
            } else {
                None
            }
        })
        .collect();
    
    // Delete the principle
    storage.delete_principle(name)?;
    
    // Save storage
    storage.save(&vql_dir)?;
    
    println!("{} Deleted principle '{}'", 
        "SUCCESS:".green().bold(),
        name.blue().bold());
    
    if !affected_assets.is_empty() {
        println!("{} Removed principle '{}' from {} asset(s):", 
            "CASCADE:".yellow().bold(),
            name,
            affected_assets.len());
        for (asset_name, _) in affected_assets {
            println!("  - {}", asset_name);
        }
    }
    
    Ok(())
}

/// Delete an entity
fn delete_entity(name: &str) -> Result<()> {
    // Find VQL storage
    let (vql_dir, mut storage) = find_vql_storage()
        .context("Failed to find or load VQL storage")?;
    
    // Delete the entity (will fail if assets exist)
    storage.delete_entity(name)?;
    
    // Save storage
    storage.save(&vql_dir)?;
    
    println!("{} Deleted entity '{}'", 
        "SUCCESS:".green().bold(),
        name.blue().bold());
    
    Ok(())
}

/// Delete an asset type
fn delete_asset_type(name: &str) -> Result<()> {
    // Find VQL storage
    let (vql_dir, mut storage) = find_vql_storage()
        .context("Failed to find or load VQL storage")?;
    
    // Delete the asset type (will fail if assets exist)
    storage.delete_asset_type(name)?;
    
    // Save storage
    storage.save(&vql_dir)?;
    
    println!("{} Deleted asset type '{}'", 
        "SUCCESS:".green().bold(),
        name.blue().bold());
    
    Ok(())
}

/// Delete an asset reference
fn delete_asset_reference(name: &str) -> Result<()> {
    // Find VQL storage
    let (vql_dir, mut storage) = find_vql_storage()
        .context("Failed to find or load VQL storage")?;
    
    // Get review count before deletion for feedback
    let review_count = storage.asset_references
        .get(name)
        .map(|asset| asset.principle_reviews.len())
        .unwrap_or(0);
    
    // Delete the asset
    storage.delete_asset_reference(name)?;
    
    // Save storage
    storage.save(&vql_dir)?;
    
    println!("{} Deleted asset '{}'", 
        "SUCCESS:".green().bold(),
        name.blue().bold());
        
    if review_count > 0 {
        println!("{} Removed {} review(s) with the asset", 
            "CASCADE:".yellow().bold(),
            review_count);
    }
    
    Ok(())
}

/// Generic rename handler that determines item type automatically
fn rename_item(old_name: &str, new_name: &str) -> Result<()> {
    // Find VQL storage
    let (vql_dir, mut storage) = find_vql_storage()
        .context("Failed to find or load VQL storage")?;
    
    // Determine item type
    let item_type = storage.find_item_type(old_name)
        .ok_or_else(|| anyhow!("Item '{}' not found in any category", old_name))?;
    
    // Delegate to specific rename function based on type
    match item_type {
        "principle" => {
            drop(storage);
            rename_principle(old_name, new_name)
        },
        "entity" => {
            drop(storage);
            rename_entity(old_name, new_name)
        },
        "asset_type" => {
            drop(storage);
            rename_asset_type(old_name, new_name)
        },
        "asset" => {
            drop(storage);
            rename_asset_reference(old_name, new_name)
        },
        _ => Err(anyhow!("Unknown item type"))
    }
}

/// Generic delete handler that determines item type automatically
fn delete_item(name: &str) -> Result<()> {
    // Find VQL storage
    let (vql_dir, mut storage) = find_vql_storage()
        .context("Failed to find or load VQL storage")?;
    
    // Determine item type
    let item_type = storage.find_item_type(name)
        .ok_or_else(|| anyhow!("Item '{}' not found in any category", name))?;
    
    // Delegate to specific delete function based on type
    match item_type {
        "principle" => {
            drop(storage);
            delete_principle(name)
        },
        "entity" => {
            drop(storage);
            delete_entity(name)
        },
        "asset_type" => {
            drop(storage);
            delete_asset_type(name)
        },
        "asset" => {
            drop(storage);
            delete_asset_reference(name)
        },
        _ => Err(anyhow!("Unknown item type"))
    }
}

/// List all asset references
fn list_asset_references() -> Result<()> {
    // Find VQL storage
    let (_, storage) = find_vql_storage()
        .context("Failed to find or load VQL storage")?;
    
    // Get all asset references
    let asset_references = &storage.asset_references;
    
    if asset_references.is_empty() {
        println!("{} No asset references defined", "INFO:".blue().bold());
        return Ok(());
    }
    
    println!("{}", "Asset References:".bold());
    
    // Sort asset references by short name
    let mut sorted_asset_refs: Vec<_> = asset_references.values().collect();
    sorted_asset_refs.sort_by(|a, b| a.short_name.cmp(&b.short_name));
    
    // Find the maximum length of each column for alignment
    let max_name_len = sorted_asset_refs.iter()
        .map(|a| a.short_name.len() + (if a.exemplar { 10 } else { 0 }))
        .max()
        .unwrap_or(10);
    let max_entity_len = sorted_asset_refs.iter()
        .map(|a| a.entity.len())
        .max()
        .unwrap_or(6);
    let max_type_len = sorted_asset_refs.iter()
        .map(|a| a.asset_type.len())
        .max()
        .unwrap_or(4);
    
    // Print header row
    println!("  {:<width_name$}  {:<width_entity$}  {:<width_type$}  {}", 
        "Asset".bold(),
        "Entity".bold(),
        "Type".bold(),
        "Path".bold(),
        width_name = max_name_len,
        width_entity = max_entity_len,
        width_type = max_type_len);
    
    // Print a separator line
    println!("  {}", "-".repeat(max_name_len + max_entity_len + max_type_len + 20));
    
    for asset_ref in sorted_asset_refs {
        let exemplar_str = if asset_ref.exemplar { " (Exemplar)".green() } else { "".normal() };
        let asset_name = format!("{}{}", asset_ref.short_name.blue().bold(), exemplar_str);
        
        // Extract just the filename from the full path
        let filename = std::path::Path::new(&asset_ref.path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(&asset_ref.path);
        
        println!("  {:<width_name$}  {:<width_entity$}  {:<width_type$}  {}", 
            asset_name,
            asset_ref.entity,
            asset_ref.asset_type,
            filename,
            width_name = max_name_len,
            width_entity = max_entity_len,
            width_type = max_type_len);
    }
    
    Ok(())
}

/// Set asset exemplar status
fn set_asset_exemplar(args: &[&str]) -> Result<()> {
    if args.len() < 2 {
        return Err(anyhow!("Not enough arguments. Usage: set_asset_exemplar asset_name true|false"));
    }
    
    let asset_name = args[0];
    let status_str = args[1].to_lowercase();
    
    let status = match status_str.as_str() {
        "true" | "t" | "yes" | "y" => true,
        "false" | "f" | "no" | "n" => false,
        _ => return Err(anyhow!("Invalid exemplar status: {}. Use true/t/yes/y or false/f/no/n", status_str)),
    };
    
    // Find VQL storage
    let (vql_dir, mut storage) = find_vql_storage()
        .context("Failed to find or load VQL storage")?;
    
    // Set exemplar status
    storage.set_asset_exemplar(asset_name, status)?;
    
    // Save changes
    storage.save(&vql_dir)?;
    
    println!("{} Set asset {} exemplar status to {}", 
        "SUCCESS:".green().bold(), 
        asset_name.blue().bold(),
        if status { "true".green() } else { "false".red() });
    
    Ok(())
}

/// Store asset review and try to extract rating from the analysis text
fn store_asset_review(asset_name: &str, principle: &str, analysis: &str) -> Result<()> {
    // Find VQL storage
    let (vql_dir, mut storage) = find_vql_storage()
        .context("Failed to find or load VQL storage")?;
    
    // Try to extract rating from analysis
    let rating = extract_rating_from_text(analysis);
    
    // Store review with auto-extracted rating if available
    storage.store_asset_review(asset_name, principle, rating.as_deref(), analysis)?;
    
    // Save changes
    storage.save(&vql_dir)?;
    
    println!("{} Stored review for asset {} from {} principle{}", 
        "SUCCESS:".green().bold(), 
        asset_name.blue().bold(),
        principle,
        if let Some(r) = rating {
            format!(" with {} compliance rating", r)
        } else {
            "".to_string()
        });
    
    Ok(())
}

/// Extract rating (H/M/L) from text
fn extract_rating_from_text(text: &str) -> Option<String> {
    let text_lower = text.to_lowercase();
    
    // Check for explicit ratings
    if text_lower.contains("high compliance") || text_lower.contains("compliance: high") {
        return Some("H".to_string());
    } else if text_lower.contains("medium compliance") || text_lower.contains("compliance: medium") {
        return Some("M".to_string());
    } else if text_lower.contains("low compliance") || text_lower.contains("compliance: low") {
        return Some("L".to_string());
    }
    
    // Fallback for simple mentions like "rated as HIGH"
    if text_lower.contains(" high ") || text_lower.contains(" high.") || text_lower.contains(" high,") {
        return Some("H".to_string());
    } else if text_lower.contains(" medium ") || text_lower.contains(" medium.") || text_lower.contains(" medium,") {
        return Some("M".to_string());
    } else if text_lower.contains(" low ") || text_lower.contains(" low.") || text_lower.contains(" low,") {
        return Some("L".to_string());
    }
    
    None
}

/// Set asset compliance rating
fn set_asset_compliance(args: &[&str]) -> Result<()> {
    if args.len() < 3 {
        return Err(anyhow!("Not enough arguments. Usage: set_asset_compliance asset_name principle rating"));
    }
    
    let asset_name = args[0];
    let principle = args[1];
    let rating = args[2];
    
    // Validate rating
    if !["H", "M", "L", "h", "m", "l"].contains(&rating) {
        return Err(anyhow!("Invalid rating: {}. Must be H, M, or L", rating));
    }
    
    // Find VQL storage
    let (vql_dir, mut storage) = find_vql_storage()
        .context("Failed to find or load VQL storage")?;
    
    // Store review with rating (empty analysis)
    storage.store_asset_review(asset_name, principle, Some(rating.to_uppercase().as_str()), "")?;
    
    // Save changes
    storage.save(&vql_dir)?;
    
    println!("{} Set {} principle compliance rating for asset {} to {}", 
        "SUCCESS:".green().bold(), 
        principle,
        asset_name.blue().bold(),
        rating.to_uppercase());
    
    Ok(())
}

/// Show asset reviews from a specific principle
fn show_asset_principle_review(asset_name: &str, principle: &str) -> Result<()> {
    // Find VQL storage
    let (_, storage) = find_vql_storage()
        .context("Failed to find or load VQL storage")?;
    
    // Get review for the asset and principle
    let review = storage.get_asset_review(asset_name, principle)?;
    
    if let Some(review) = review {
        println!("{} Review for asset {} from {} principle:", 
            "INFO:".blue().bold(), 
            asset_name.blue().bold(),
            principle);
            
        if let Some(rating) = &review.rating {
            println!("  Rating: {}", get_rating_display(rating));
        } else {
            println!("  Rating: Not rated");
        }
        
        if let Some(analysis) = &review.analysis {
            println!("  Analysis: {}", analysis);
        } else {
            println!("  Analysis: No analysis provided");
        }
        
        println!("  Last modified: {}", review.last_modified);
    } else {
        println!("{} No review found for asset {} from {} principle", 
            "INFO:".blue().bold(), 
            asset_name.blue().bold(),
            principle);
    }
    
    Ok(())
}

/// Show all asset reviews (why this exists)
fn show_asset_why(asset_name: &str, principle: Option<&str>) -> Result<()> {
    // Find VQL storage
    let (_, storage) = find_vql_storage()
        .context("Failed to find or load VQL storage")?;
    
    // First, make sure the asset exists
    let asset = match storage.asset_references.get(asset_name) {
        Some(asset) => asset,
        None => return Err(anyhow!("Asset {} not found", asset_name)),
    };
    
    println!("{} Asset Information: {}", 
        "INFO:".blue().bold(), 
        asset_name.blue().bold());
        
    println!("  Entity: {}", asset.entity);
    println!("  Type: {}", asset.asset_type);
    println!("  Path: {}", asset.path);
    println!("  Exemplar: {}", if asset.exemplar { "Yes".green() } else { "No".red() });
    println!("  Last modified: {}", asset.last_modified);
    
    // If specific principle requested, check if it contains commas
    if let Some(p) = principle {
        if p.contains(',') {
            // Multiple principles requested
            let principles: Vec<&str> = p.split(',').map(|s| s.trim()).collect();
            println!("\n  Reviews for selected principles:");
            
            for &princ in &principles {
                if let Some(review) = asset.principle_reviews.get(princ) {
                    println!("    {} Principle:", princ);
                    
                    if let Some(rating) = &review.rating {
                        println!("      Rating: {}", get_rating_display(rating));
                    } else {
                        println!("      Rating: Not rated");
                    }
                    
                    if let Some(analysis) = &review.analysis {
                        println!("      Analysis: {}", analysis);
                    } else {
                        println!("      Analysis: No analysis provided");
                    }
                    
                    println!("");
                } else {
                    println!("    {} Principle: No review", princ);
                }
            }
        } else {
            // Single principle
            if let Some(review) = asset.principle_reviews.get(p) {
                println!("\n  {} Principle:", p);
                
                if let Some(rating) = &review.rating {
                    println!("    Rating: {}", get_rating_display(rating));
                } else {
                    println!("    Rating: Not rated");
                }
                
                if let Some(analysis) = &review.analysis {
                    println!("    Analysis: {}", analysis);
                } else {
                    println!("    Analysis: No analysis provided");
                }
            } else {
                println!("\n  {} Principle: No review", p);
            }
        }
    } else {
        // Show all principles
        println!("\n  Reviews:");
        
        if asset.principle_reviews.is_empty() {
            println!("    No reviews available");
        } else {
            for (princ, review) in &asset.principle_reviews {
                println!("    {} Principle:", princ);
                
                if let Some(rating) = &review.rating {
                    println!("      Rating: {}", get_rating_display(rating));
                } else {
                    println!("      Rating: Not rated");
                }
                
                if let Some(analysis) = &review.analysis {
                    println!("      Analysis: {}", analysis);
                } else {
                    println!("      Analysis: No analysis provided");
                }
                
                println!("");
            }
        }
    }
    
    Ok(())
}

/// Display a rating with color
fn get_rating_display(rating: &str) -> colored::ColoredString {
    match rating {
        "H" => "High".green().bold(),
        "M" => "Medium".yellow().bold(),
        "L" => "Low".red().bold(),
        _ => rating.normal(),
    }
}

/// Load principles from markdown file
fn load_principles_from_md(file_path: &str) -> Result<()> {
    // Find VQL storage 
    let (vql_dir, mut storage) = find_vql_storage()
        .context("Failed to find or load VQL storage")?;
    
    // Handle tilde expansion for home directory
    let expanded_path = if file_path.starts_with("~") {
        // Get the home directory
        if let Ok(home) = std::env::var("HOME") {
            // Replace ~ with the actual home directory
            let rel_path = &file_path[1..];
            format!("{}{}", home, rel_path)
        } else {
            // If HOME isn't available, keep the original
            file_path.to_string()
        }
    } else {
        file_path.to_string()
    };
    
    // Open and read the file
    let file = fs::File::open(&expanded_path)
        .context(format!("Failed to open principles file: {}", expanded_path))?;
    
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    
    // Pattern to match principle headers - e.g. "# Architecture Principles (a)"
    let header_regex = Regex::new(r"^# (.*) \(([a-z])\)$").unwrap();
    
    // Keep track of principles we've found
    let mut principles_added = 0;
    let mut current_principle: Option<(String, String, String)> = None;
    let mut current_content = String::new();
    
    // Process file line by line
    while let Some(Ok(line)) = lines.next() {
        // Check if this is a header line
        if let Some(captures) = header_regex.captures(&line) {
            // If we were processing a principle, save it
            if let Some((short_name, long_name, content)) = current_principle.take() {
                storage.add_principle(&short_name, &long_name, Some(&content))?;
                principles_added += 1;
            }
            
            // Start tracking a new principle
            let long_name = captures.get(1).unwrap().as_str().to_string();
            let short_name = captures.get(2).unwrap().as_str().to_string();
            current_principle = Some((short_name, long_name, String::new()));
            current_content.clear();
        } else if let Some((_, _, ref mut content)) = current_principle {
            // Add this line to the current principle's content
            if !current_content.is_empty() {
                current_content.push('\n');
            }
            current_content.push_str(&line);
            *content = current_content.clone();
        }
    }
    
    // Save the last principle if there is one
    if let Some((short_name, long_name, content)) = current_principle {
        storage.add_principle(&short_name, &long_name, Some(&content))?;
        principles_added += 1;
    }
    
    // Save changes to storage
    storage.save(&vql_dir)?;
    
    println!("{} Loaded {} principles from {}", 
        "SUCCESS:".green().bold(), 
        principles_added,
        expanded_path.blue());
    
    Ok(())
}

// Special LLM command handlers

/// Handle VQL mode (enable/disable)
fn handle_vql_mode(_enabled: bool) -> Result<()> {
    // This is just a placeholder for now
    println!("VQL mode command received");
    Ok(())
}

/// Handle interface mode switching
fn handle_interface_mode(mode: &str) -> Result<()> {
    match mode {
        "mcp" => println!("Switched to MCP interface mode"),
        "cli" => println!("Switched to CLI interface mode"),
        _ => return Err(anyhow!("Unknown interface mode: {}", mode)),
    }
    Ok(())
}

/// Handle asset store command (LLM format with commas)
fn handle_asset_store(asset_name: &str, args: &str) -> Result<()> {
    // Parse args: principle, content (with commas, as per VQL Prompt file)
    let parts: Vec<&str> = args.split(',').map(|s| s.trim()).collect();
    
    if parts.is_empty() {
        return Err(anyhow!("Not enough arguments for asset store command"));
    }
    
    let principle = parts[0];
    
    // Join all remaining parts together as the content (in case there are multiple commas)
    let content = if parts.len() > 1 { 
        parts[1..].join(", ") 
    } else { 
        String::new() 
    };
    
    // Store the review
    store_asset_review(asset_name, principle, &content)
}

/// Parse and validate a comma-separated list of principles
fn parse_principle_list(args: &str, storage: &JsonStorage) -> Result<Vec<String>> {
    // Handle wildcard for all principles
    if args.trim() == "*" || args.trim() == "-pr" {
        return Ok(storage.principles.keys().cloned().collect());
    }
    
    // Parse comma-separated list
    let requested: Vec<&str> = args.split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    
    // Remove duplicates while preserving order
    let mut seen = std::collections::HashSet::new();
    let mut unique_principles = Vec::new();
    
    for principle in requested {
        // Validate that the principle exists
        if !storage.principles.contains_key(principle) {
            return Err(anyhow!("Unknown principle: '{}'. Available principles: {}", 
                principle,
                storage.principles.keys().cloned().collect::<Vec<_>>().join(", ")
            ));
        }
        
        // Add only if not seen before
        if seen.insert(principle.to_string()) {
            unique_principles.push(principle.to_string());
        }
    }
    
    if unique_principles.is_empty() {
        return Err(anyhow!("No valid principles specified"));
    }
    
    Ok(unique_principles)
}

/// Handle asset review command (LLM-only)
fn handle_asset_review(asset_name: &str, args: &str) -> Result<()> {
    // Load storage to validate asset and principles
    let (_vql_dir, storage) = find_vql_storage()
        .context("Failed to find or load VQL storage")?;
    
    // Validate asset exists
    if !storage.asset_references.contains_key(asset_name) {
        return Err(anyhow!("Unknown asset: '{}'. Available assets: {}", 
            asset_name,
            storage.asset_references.keys().cloned().collect::<Vec<_>>().join(", ")
        ));
    }
    
    // Parse and validate principles
    let principles = parse_principle_list(args, &storage)?;
    
    // Get asset details
    let asset = &storage.asset_references[asset_name];
    
    println!("LLM Review Request:");
    println!("Asset: {} ({})", asset_name, asset.path);
    println!("Principles to review: {}", principles.join(", "));
    
    // Return review instructions
    println!("\nReview Instructions:");
    println!("1. Read asset from: {}", asset.path);
    println!("2. Review for principles: {}", principles.join(", "));
    println!("3. For each principle:");
    for principle in &principles {
        if let Some(p) = storage.principles.get(principle) {
            println!("   - {} ({}): {}", 
                principle, 
                p.long_name,
                p.guidance.as_ref().unwrap_or(&"No guidance".to_string())
            );
        }
    }
    println!("4. Rate each principle (H/M/L)");
    println!("5. Provide detailed analysis");
    println!("6. Store results using :{}.st({}, \"Review with rating...\")", asset_name, principles[0]);
    
    Ok(())
}

/// Handle asset refactor command (LLM-only)
fn handle_asset_refactor(asset_name: &str, args: &str) -> Result<()> {
    // Load storage to validate asset and principles
    let (_vql_dir, storage) = find_vql_storage()
        .context("Failed to find or load VQL storage")?;
    
    // Parse the refactor arguments - could be principles, or principles + reference assets
    let (principles, reference_assets) = if args.contains(',') {
        // Split by comma to analyze the parts
        let parts: Vec<&str> = args.split(',').map(|s| s.trim()).collect();
        
        // Find where principles end and reference assets begin
        let mut principle_parts = Vec::new();
        let mut reference_parts = Vec::new();
        let mut found_reference = false;
        
        for part in parts {
            // Check if this is an asset reference (exists in storage)
            if !found_reference && (part == "-pr" || storage.principles.contains_key(part)) {
                principle_parts.push(part);
            } else {
                found_reference = true;
                reference_parts.push(part);
            }
        }
        
        // Parse principles
        let principles = if principle_parts.is_empty() {
            vec![]
        } else {
            parse_principle_list(&principle_parts.join(","), &storage)?
        };
        
        // Collect reference assets
        let references = if reference_parts.is_empty() {
            None
        } else {
            Some(reference_parts.into_iter().map(|s| s.to_string()).collect::<Vec<_>>())
        };
        
        (principles, references)
    } else if !args.contains('*') && !args.contains("-pr") && storage.asset_references.contains_key(args.trim()) {
        // Single asset reference - refactor using another asset as reference
        let ref_asset = args.trim();
        // Get all principles from the reference asset's reviews
        let principles = if let Some(ref_asset_data) = storage.asset_references.get(ref_asset) {
            ref_asset_data.principle_reviews.keys().cloned().collect()
        } else {
            vec![]
        };
        (principles, Some(vec![ref_asset.to_string()]))
    } else {
        // Standard principle list (including -pr)
        (parse_principle_list(args, &storage)?, None)
    };
    
    // Validate asset exists
    if !storage.asset_references.contains_key(asset_name) {
        return Err(anyhow!("Unknown asset: '{}'. Available assets: {}", 
            asset_name,
            storage.asset_references.keys().cloned().collect::<Vec<_>>().join(", ")
        ));
    }
    
    // Get asset details
    let asset = &storage.asset_references[asset_name];
    
    println!("LLM Refactor Request:");
    println!("Asset: {} ({})", asset_name, asset.path);
    
    if let Some(ref ref_assets) = reference_assets {
        println!("Using reference assets: {}", ref_assets.join(", "));
        for ref_asset in ref_assets {
            if let Some(ref_data) = storage.asset_references.get(ref_asset) {
                println!("  - {} ({})", ref_asset, ref_data.path);
            }
        }
        if !principles.is_empty() {
            println!("Principles to refactor for: {}", principles.join(", "));
        }
    } else {
        println!("Principles to refactor for: {}", principles.join(", "));
    }
    
    // Return refactor instructions
    println!("\nRefactor Instructions:");
    println!("1. Read asset from: {}", asset.path);
    
    if reference_assets.is_some() {
        println!("2. Read reference assets and analyze their patterns");
        println!("3. Apply similar patterns to improve the target asset");
    } else {
        println!("2. Consider principles: {}", principles.join(", "));
        for principle in &principles {
            if let Some(p) = storage.principles.get(principle) {
                println!("   - {} ({}): {}", 
                    principle, 
                    p.long_name,
                    p.guidance.as_ref().unwrap_or(&"No guidance".to_string())
                );
            }
        }
        println!("3. Identify improvements for each principle");
    }
    
    println!("4. Apply refactoring changes");
    println!("5. MANDATORY: Review refactored code and update all reviews");
    println!("6. Store updated reviews with 'After refactoring:' prefix");
    
    Ok(())
}

/// Handle asset set exemplar command
fn handle_asset_set_exemplar(asset_name: &str, args: &str) -> Result<()> {
    // Parse args: status
    let status_str = args.trim().to_lowercase();
    
    let status = match status_str.as_str() {
        "true" | "t" | "yes" | "y" => true,
        "false" | "f" | "no" | "n" => false,
        _ => return Err(anyhow!("Invalid exemplar status: {}. Use true/t/yes/y or false/f/no/n", status_str)),
    };
    
    // Set exemplar status
    set_asset_exemplar(&[asset_name, &status_str])
}

/// Handle asset set compliance command
fn handle_asset_set_compliance(asset_name: &str, args: &str) -> Result<()> {
    // Parse args: principle, rating
    let parts: Vec<&str> = args.split(',').map(|s| s.trim()).collect();
    
    if parts.len() < 2 {
        return Err(anyhow!("Not enough arguments for asset set compliance command"));
    }
    
    let principle = parts[0];
    let rating = parts[1];
    
    // Set compliance rating
    set_asset_compliance(&[asset_name, principle, rating])
}

/// Handle global review command (LLM-only)
fn handle_global_review(args: &str) -> Result<()> {
    // Load storage to validate principles
    let (_vql_dir, storage) = find_vql_storage()
        .context("Failed to find or load VQL storage")?;
    
    // Parse and validate principles
    let principles = parse_principle_list(args, &storage)?;
    
    // Get all assets
    let assets: Vec<String> = storage.asset_references.keys().cloned().collect();
    
    if assets.is_empty() {
        return Err(anyhow!("No assets found in the project"));
    }
    
    println!("LLM Global Review Request:");
    println!("Total assets: {}", assets.len());
    println!("Principles to review: {}", principles.join(", "));
    
    // Return review instructions
    println!("\nGlobal Review Instructions:");
    println!("1. Review all {} assets:", assets.len());
    for asset_name in &assets {
        if let Some(asset) = storage.asset_references.get(asset_name) {
            println!("   - {} ({})", asset_name, asset.path);
        }
    }
    println!("2. For each asset, review principles: {}", principles.join(", "));
    println!("3. Rate each principle (H/M/L)");
    println!("4. Provide detailed analysis");
    println!("5. Store results using :[asset].st([principle], \"Review with rating...\")");
    println!("\nTotal reviews to perform: {} assets × {} principles = {} reviews", 
        assets.len(), 
        principles.len(), 
        assets.len() * principles.len()
    );
    
    Ok(())
}

/// Handle global refactor command (LLM-only)
fn handle_global_refactor(args: &str) -> Result<()> {
    // Load storage to validate principles
    let (_vql_dir, storage) = find_vql_storage()
        .context("Failed to find or load VQL storage")?;
    
    // Parse and validate principles
    let principles = parse_principle_list(args, &storage)?;
    
    // Get all assets
    let assets: Vec<String> = storage.asset_references.keys().cloned().collect();
    
    if assets.is_empty() {
        return Err(anyhow!("No assets found in the project"));
    }
    
    println!("LLM Global Refactor Request:");
    println!("Total assets: {}", assets.len());
    println!("Principles to refactor for: {}", principles.join(", "));
    
    // Return refactor instructions
    println!("\nGlobal Refactor Instructions:");
    println!("1. Process all {} assets:", assets.len());
    for asset_name in &assets {
        if let Some(asset) = storage.asset_references.get(asset_name) {
            println!("   - {} ({})", asset_name, asset.path);
        }
    }
    println!("2. For each asset:");
    println!("   a. Read the current implementation");
    println!("   b. Consider principles: {}", principles.join(", "));
    for principle in &principles {
        if let Some(p) = storage.principles.get(principle) {
            println!("      - {} ({}): {}", 
                principle, 
                p.long_name,
                p.guidance.as_ref().unwrap_or(&"No guidance".to_string())
            );
        }
    }
    println!("   c. Identify improvements for each principle");
    println!("   d. Apply refactoring changes");
    println!("   e. MANDATORY: Review refactored code and update all reviews");
    println!("3. Store updated reviews with 'After refactoring:' prefix");
    println!("\nTotal refactorings: {} assets × {} principles = {} potential improvements", 
        assets.len(), 
        principles.len(), 
        assets.len() * principles.len()
    );
    
    Ok(())
}