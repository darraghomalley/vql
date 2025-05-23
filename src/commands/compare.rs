use anyhow::{Result, Context};
use colored::Colorize;

use crate::utils::filesystem;
use crate::utils::parser;
use crate::models::asset::AspectType;

/// Compare two assets based on specified aspect
pub fn compare_assets(asset_ref1: &str, asset_ref2: &str, aspect: &str) -> Result<()> {
    // Normalize asset references
    let ref1 = if asset_ref1.starts_with('+') {
        asset_ref1.to_string()
    } else {
        format!("+{}", asset_ref1)
    };
    
    let ref2 = if asset_ref2.starts_with('+') {
        asset_ref2.to_string()
    } else {
        format!("+{}", asset_ref2)
    };
    
    // Get asset types from references
    let asset_type1 = filesystem::get_asset_type_from_ref(&ref1)
        .context(format!("Could not determine asset type from reference: {}", ref1))?;
    
    let asset_type2 = filesystem::get_asset_type_from_ref(&ref2)
        .context(format!("Could not determine asset type from reference: {}", ref2))?;
    
    // Read asset file content for both assets
    let content1 = filesystem::read_asset_file(&asset_type1)
        .context(format!("Failed to read asset file for type: {}", asset_type1))?;
    
    let content2 = filesystem::read_asset_file(&asset_type2)
        .context(format!("Failed to read asset file for type: {}", asset_type2))?;
    
    // Parse the assets
    let asset_data1 = parser::parse_asset(&content1, &ref1)
        .context(format!("Failed to parse asset: {}", ref1))?;
    
    let asset_data2 = parser::parse_asset(&content2, &ref2)
        .context(format!("Failed to parse asset: {}", ref2))?;
    
    // Convert aspect to proper type
    let aspect_type = AspectType::from_string(aspect);
    
    // Display comparison header
    println!("{} {} vs {}", 
        "COMPARISON:".green().bold(), 
        ref1.blue().bold(), 
        ref2.blue().bold());
    
    // Display asset header information
    println!("\n{}", "HEADER COMPARISON:".yellow().bold());
    
    println!("  {:<15} {:<15} {:<15}", "ATTRIBUTE", ref1, ref2);
    println!("  {}", "-".repeat(45));
    
    for key in ["EXEMPLAR", "ARCH", "SEC", "PERF", "LAST_UPDATE"] {
        let val1 = asset_data1.header.get(key).cloned().unwrap_or_else(|| "N/A".to_string());
        let val2 = asset_data2.header.get(key).cloned().unwrap_or_else(|| "N/A".to_string());
        println!("  {:<15} {:<15} {:<15}", key, val1, val2);
    }
    
    // Display analysis comparison based on the requested aspect
    println!("\n{}", "ANALYSIS COMPARISON:".yellow().bold());
    
    match aspect_type {
        AspectType::Architecture => {
            println!("\n{}", "ARCHITECTURE:".blue().bold());
            println!("\n{}: {}", ref1.blue(), asset_data1.analysis.get("arch").cloned().unwrap_or_else(|| "No analysis available.".to_string()));
            println!("\n{}: {}", ref2.blue(), asset_data2.analysis.get("arch").cloned().unwrap_or_else(|| "No analysis available.".to_string()));
        },
        AspectType::Security => {
            println!("\n{}", "SECURITY:".blue().bold());
            println!("\n{}: {}", ref1.blue(), asset_data1.analysis.get("sec").cloned().unwrap_or_else(|| "No analysis available.".to_string()));
            println!("\n{}: {}", ref2.blue(), asset_data2.analysis.get("sec").cloned().unwrap_or_else(|| "No analysis available.".to_string()));
        },
        AspectType::Performance => {
            println!("\n{}", "PERFORMANCE:".blue().bold());
            println!("\n{}: {}", ref1.blue(), asset_data1.analysis.get("perf").cloned().unwrap_or_else(|| "No analysis available.".to_string()));
            println!("\n{}: {}", ref2.blue(), asset_data2.analysis.get("perf").cloned().unwrap_or_else(|| "No analysis available.".to_string()));
        },
        AspectType::All => {
            // Compare all aspects
            println!("\n{}", "ARCHITECTURE:".blue().bold());
            println!("\n{}: {}", ref1.blue(), asset_data1.analysis.get("arch").cloned().unwrap_or_else(|| "No analysis available.".to_string()));
            println!("\n{}: {}", ref2.blue(), asset_data2.analysis.get("arch").cloned().unwrap_or_else(|| "No analysis available.".to_string()));
            
            println!("\n{}", "SECURITY:".blue().bold());
            println!("\n{}: {}", ref1.blue(), asset_data1.analysis.get("sec").cloned().unwrap_or_else(|| "No analysis available.".to_string()));
            println!("\n{}: {}", ref2.blue(), asset_data2.analysis.get("sec").cloned().unwrap_or_else(|| "No analysis available.".to_string()));
            
            println!("\n{}", "PERFORMANCE:".blue().bold());
            println!("\n{}: {}", ref1.blue(), asset_data1.analysis.get("perf").cloned().unwrap_or_else(|| "No analysis available.".to_string()));
            println!("\n{}: {}", ref2.blue(), asset_data2.analysis.get("perf").cloned().unwrap_or_else(|| "No analysis available.".to_string()));
        },
    }
    
    Ok(())
}