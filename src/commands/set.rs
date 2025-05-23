use anyhow::{Result, Context};
use colored::Colorize;

use crate::models::json_storage::find_vql_storage;

/// Set exemplar status for an asset
pub fn set_exemplar(asset_ref: &str, status: bool) -> Result<()> {
    // Remove + prefix if present
    let asset_name = asset_ref.trim_start_matches('+');
    
    // Get VQL directory and JSON storage
    let (vql_dir, mut storage) = find_vql_storage()
        .context("VQL directory not found. Run 'vql init' first")?;
    
    // Update status in JSON storage
    storage.set_asset_exemplar(asset_name, status)?;
    
    // Save changes to JSON storage
    storage.save(&vql_dir)?;
    
    println!("{} Updated exemplar status for {} to {}", 
        "SUCCESS:".green().bold(), 
        asset_name.blue().bold(),
        if status { "TRUE".green().bold() } else { "FALSE".red().bold() });
    
    if status {
        println!("\n{} This asset is now marked as an exemplar and will be used as a reference for quality standards.", 
            "NOTE:".cyan().bold());
    } else {
        println!("\n{} This asset is no longer marked as an exemplar.", 
            "NOTE:".cyan().bold());
    }
    
    Ok(())
}