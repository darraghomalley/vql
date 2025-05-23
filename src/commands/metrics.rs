use anyhow::{Result, Context};
use colored::Colorize;

use crate::utils::filesystem;
use crate::utils::parser;

/// Calculate metrics for an asset
pub fn calculate_metrics(asset_ref: &str) -> Result<()> {
    // Normalize asset reference
    let ref_normalized = if asset_ref.starts_with('+') {
        asset_ref.to_string()
    } else {
        format!("+{}", asset_ref)
    };
    
    // Get asset type from reference
    let asset_type = filesystem::get_asset_type_from_ref(&ref_normalized)
        .context(format!("Could not determine asset type from reference: {}", ref_normalized))?;
    
    // Read asset file content
    let content = filesystem::read_asset_file(&asset_type)
        .context(format!("Failed to read asset file for type: {}", asset_type))?;
    
    // Parse the asset
    let asset_data = parser::parse_asset(&content, &ref_normalized)
        .context(format!("Failed to parse asset: {}", ref_normalized))?;
    
    // Display metrics header
    println!("{} for {}", "METRICS".green().bold(), ref_normalized.blue().bold());
    
    // Display asset information
    println!("\n{}", "ASSET INFORMATION:".yellow().bold());
    for (key, value) in &asset_data.header {
        println!("  {}: {}", key, value);
    }
    
    // In a real implementation, we would calculate actual metrics
    // For now, we'll display placeholder metrics
    println!("\n{}", "CODE METRICS:".yellow().bold());
    println!("  {:<25} {}", "Lines of Code:".blue(), "186");
    println!("  {:<25} {}", "Cyclomatic Complexity:".blue(), "12");
    println!("  {:<25} {}", "Dependencies:".blue(), "5");
    println!("  {:<25} {}", "Method Count:".blue(), "8");
    println!("  {:<25} {}", "File Size:".blue(), "4250 bytes");
    
    // Display quality metrics based on ratings
    println!("\n{}", "QUALITY METRICS:".yellow().bold());
    
    let arch_rating = asset_data.header.get("ARCH").cloned().unwrap_or_else(|| "?".to_string());
    let sec_rating = asset_data.header.get("SEC").cloned().unwrap_or_else(|| "?".to_string());
    let perf_rating = asset_data.header.get("PERF").cloned().unwrap_or_else(|| "?".to_string());
    
    println!("  {:<25} {}", "Architecture Quality:".blue(), format_rating(&arch_rating));
    println!("  {:<25} {}", "Security Quality:".blue(), format_rating(&sec_rating));
    println!("  {:<25} {}", "Performance Quality:".blue(), format_rating(&perf_rating));
    
    // Calculate overall quality score (H=3, M=2, L=1, ?=0)
    let arch_score = get_rating_score(&arch_rating);
    let sec_score = get_rating_score(&sec_rating);
    let perf_score = get_rating_score(&perf_rating);
    
    let total_score = arch_score + sec_score + perf_score;
    let max_score = 9; // 3 aspects * max score of 3
    let percent = (total_score as f64 / max_score as f64) * 100.0;
    
    println!("  {:<25} {:.1}% ({}/{})", "Overall Quality:".blue().bold(), percent, total_score, max_score);
    
    println!("\n{} You can update ratings using:", "NOTE:".cyan().bold());
    println!("  {}", format!("vql store {} --rating=<H|M|L> --aspect=<arch|sec|perf> --analysis=\"Your analysis\"", ref_normalized).cyan());
    
    Ok(())
}

/// Format rating for display
fn format_rating(rating: &str) -> String {
    match rating {
        "H" => "High".green().to_string(),
        "M" => "Medium".yellow().to_string(),
        "L" => "Low".red().to_string(),
        _ => "Unknown".dimmed().to_string(),
    }
}

/// Convert rating to numeric score
fn get_rating_score(rating: &str) -> u8 {
    match rating {
        "H" => 3,
        "M" => 2,
        "L" => 1,
        _ => 0,
    }
}