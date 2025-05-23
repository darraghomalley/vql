use anyhow::{Result, Context};
use colored::Colorize;
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use regex::Regex;

use crate::utils::filesystem;
use crate::utils::parser;

/// Check for changes since last review
pub fn check_changes() -> Result<()> {
    // Get VQL directory
    let vql_dir = filesystem::find_vql_root()
        .context("VQL directory not found. Run 'vql init' first.")?;
    
    // Track changes for all assets
    let mut changed_assets = HashMap::new();
    let mut unchanged_assets = HashMap::new();
    
    // Process each asset type file
    for file_type in &["models", "controllers", "ui"] {
        let vql_file_path = vql_dir.join(format!("{}.vql.ref", file_type));
        
        if !vql_file_path.exists() {
            continue;
        }
        
        let content = fs::read_to_string(&vql_file_path)
            .context(format!("Failed to read {}.vql.ref", file_type))?;
        
        // Extract all asset entries
        let asset_pattern = r"ASSET:(\+[^\s|]*)\s*\|(.*?)---";
        let re = Regex::new(asset_pattern).context("Failed to compile asset regex pattern")?;
        
        for captures in re.captures_iter(&content) {
            let asset_ref = captures.get(1).unwrap().as_str();
            let asset_data = parser::parse_asset(&content, asset_ref)?;
            
            // Check if the asset has a file hash and path
            if let (Some(file_hash), Some(file_path)) = (
                asset_data.header.get("FILE_HASH"),
                asset_data.header.get("FILE_PATH")
            ) {
                let path = Path::new(file_path);
                
                // Calculate current hash if file exists
                if path.exists() {
                    let current_hash = filesystem::get_file_hash(path)?;
                    
                    if current_hash != *file_hash {
                        changed_assets.insert(asset_ref.to_string(), (asset_data, current_hash));
                    } else {
                        unchanged_assets.insert(asset_ref.to_string(), asset_data);
                    }
                } else {
                    // File doesn't exist anymore
                    changed_assets.insert(asset_ref.to_string(), (asset_data, "FILE_MISSING".to_string()));
                }
            } else {
                // Missing hash or path - cannot determine changes
                unchanged_assets.insert(asset_ref.to_string(), asset_data);
            }
        }
    }
    
    // Display results
    println!("{}", "CHANGE DETECTION RESULTS".green().bold());
    
    if changed_assets.is_empty() {
        println!("\n{} All assets are up to date with their last review.", "✓".green());
    } else {
        println!("\n{} The following assets have changed since their last review:", "!".yellow().bold());
        
        for (asset_ref, (asset_data, current_hash)) in &changed_assets {
            let last_update = asset_data.header.get("LAST_UPDATE")
                .cloned()
                .unwrap_or_else(|| "unknown".to_string());
            
            let status = if current_hash == "FILE_MISSING" {
                "MISSING".red().bold()
            } else {
                "CHANGED".yellow().bold()
            };
            
            println!("  {} {} (Last review: {}, Status: {})", 
                "●".yellow(),
                asset_ref.blue().bold(),
                last_update,
                status);
        }
        
        println!("\n{} To review a changed asset:", "ACTION:".cyan().bold());
        println!("  {}", "vql prepare <asset-ref> --aspect=<arch|sec|perf|all>".cyan());
    }
    
    println!("\n{} Total assets: {}", "SUMMARY:".yellow().bold(), changed_assets.len() + unchanged_assets.len());
    println!("  Changed: {}", changed_assets.len());
    println!("  Unchanged: {}", unchanged_assets.len());
    
    Ok(())
}