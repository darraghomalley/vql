use anyhow::{Result, Context};
use colored::Colorize;

use crate::utils::filesystem;
use crate::utils::parser;
use crate::models::asset::AspectType;

/// Prepare an asset for review
pub fn prepare_asset(asset_ref: &str, aspect: &str) -> Result<()> {
    // Normalize asset reference to ensure it has + prefix
    let normalized_ref = if asset_ref.starts_with('+') {
        asset_ref.to_string()
    } else {
        format!("+{}", asset_ref)
    };
    
    // Get asset type from reference
    let asset_type = filesystem::get_asset_type_from_ref(&normalized_ref)
        .context(format!("Could not determine asset type from reference: {}", normalized_ref))?;
    
    // Read the asset file content
    let content = filesystem::read_asset_file(&asset_type)
        .context(format!("Failed to read asset file for type: {}", asset_type))?;
    
    // Parse the asset
    let asset_data = parser::parse_asset(&content, &normalized_ref)
        .context(format!("Failed to parse asset: {}", normalized_ref))?;
    
    // Convert aspect to proper type
    let aspect_type = AspectType::from_string(aspect);
    
    // Display asset information for review
    println!("{} prepared for review: {}", "ASSET".green().bold(), normalized_ref.blue().bold());
    println!("{} {}", "ASPECT:".yellow(), aspect.to_uppercase());
    
    // Display asset header information
    println!("\n{}", "ASSET INFO:".yellow().bold());
    for (key, value) in &asset_data.header {
        println!("  {}: {}", key, value);
    }
    
    // Display relevant analysis based on the requested aspect
    println!("\n{}", "CURRENT ANALYSIS:".yellow().bold());
    
    match aspect_type {
        AspectType::Architecture => {
            if let Some(analysis) = asset_data.analysis.get("arch") {
                println!("\n{}\n{}", "ARCHITECTURE:".blue().bold(), analysis);
            } else {
                println!("\n{}\n{}", "ARCHITECTURE:".blue().bold(), "No analysis available.");
            }
        },
        AspectType::Security => {
            if let Some(analysis) = asset_data.analysis.get("sec") {
                println!("\n{}\n{}", "SECURITY:".blue().bold(), analysis);
            } else {
                println!("\n{}\n{}", "SECURITY:".blue().bold(), "No analysis available.");
            }
        },
        AspectType::Performance => {
            if let Some(analysis) = asset_data.analysis.get("perf") {
                println!("\n{}\n{}", "PERFORMANCE:".blue().bold(), analysis);
            } else {
                println!("\n{}\n{}", "PERFORMANCE:".blue().bold(), "No analysis available.");
            }
        },
        AspectType::All => {
            // Display all analyses
            if let Some(analysis) = asset_data.analysis.get("arch") {
                println!("\n{}\n{}", "ARCHITECTURE:".blue().bold(), analysis);
            } else {
                println!("\n{}\n{}", "ARCHITECTURE:".blue().bold(), "No analysis available.");
            }
            
            if let Some(analysis) = asset_data.analysis.get("sec") {
                println!("\n{}\n{}", "SECURITY:".blue().bold(), analysis);
            } else {
                println!("\n{}\n{}", "SECURITY:".blue().bold(), "No analysis available.");
            }
            
            if let Some(analysis) = asset_data.analysis.get("perf") {
                println!("\n{}\n{}", "PERFORMANCE:".blue().bold(), analysis);
            } else {
                println!("\n{}\n{}", "PERFORMANCE:".blue().bold(), "No analysis available.");
            }
        },
    }
    
    // Display next steps
    println!("\n{}", "NEXT STEPS:".green().bold());
    println!("  Review the asset and provide a rating (H/M/L) along with detailed analysis:");
    println!("  {}", format!("vql store {} --rating=<H|M|L> --aspect={} --analysis=\"Your detailed analysis\"", 
        normalized_ref, aspect.to_lowercase()).cyan());
    
    Ok(())
}