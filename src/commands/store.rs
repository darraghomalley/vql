use anyhow::{Result, Context};
use colored::Colorize;

use crate::models::json_storage::find_vql_storage;

/// Store review results for an asset
pub fn store_review(asset_ref: &str, rating: &str, aspect: &str, analysis: &str) -> Result<()> {
    // Validate rating
    if !["H", "M", "L", "h", "m", "l"].contains(&rating) {
        return Err(anyhow::anyhow!("Rating must be one of: H, M, L"));
    }
    
    // Validate aspect
    if !["arch", "sec", "perf", "ui", "ARCH", "SEC", "PERF", "UI"].contains(&aspect) {
        return Err(anyhow::anyhow!("Aspect must be one of: arch, sec, perf, ui"));
    }
    
    // Normalize rating (uppercase)
    let normalized_rating = rating.to_uppercase();
    
    // Normalize aspect (lowercase for analysis keys)
    let normalized_aspect = aspect.to_lowercase();
    
    // Get the asset name (remove + prefix if present)
    let asset_name = asset_ref.trim_start_matches('+');
    
    // Get the VQL directory and JSON storage
    let (vql_dir, mut json_storage) = find_vql_storage()
        .context("VQL directory not found. Run 'vql init' first")?;
    
    // Update the asset review in JSON storage
    json_storage.set_asset_review(
        asset_name,
        &normalized_aspect,
        &normalized_rating,
        analysis
    )?;
    
    // Save the updated JSON storage
    json_storage.save(&vql_dir)?;
    
    println!("{} Updated {} rating for {}: {}", 
        "SUCCESS:".green().bold(), 
        normalized_aspect.blue().bold(), 
        asset_name.blue().bold(),
        normalized_rating.green());
    
    println!("\n{}", "ANALYSIS:".yellow().bold());
    println!("{}", analysis);
    
    println!("\n{} The asset has been updated with the new rating and analysis.", 
        "NOTE:".cyan().bold());
    
    Ok(())
}