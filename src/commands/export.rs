use anyhow::{Result, Context};
use colored::Colorize;
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use regex::Regex;

use crate::utils::filesystem;
use crate::utils::parser;

/// Export VQL data to a report
pub fn export_data(format: &str, include_details: bool) -> Result<()> {
    // Validate format
    if !["md", "html", "json"].contains(&format.to_lowercase().as_str()) {
        return Err(anyhow::anyhow!("Format must be one of: md, html, json"));
    }
    
    // Get VQL directory
    let vql_dir = filesystem::find_vql_root()
        .context("VQL directory not found. Run 'vql init' first.")?;
    
    // Output file path
    let output_file = match format.to_lowercase().as_str() {
        "md" => vql_dir.join("vql-report.md"),
        "html" => vql_dir.join("vql-report.html"),
        "json" => vql_dir.join("vql-report.json"),
        _ => vql_dir.join("vql-report.md"), // Default to markdown
    };
    
    // Collect data from all VQL files
    let mut assets = HashMap::new();
    
    // Process each asset type file
    for file_type in &["models", "controllers", "ui"] {
        let file_path = vql_dir.join(format!("{}.vql.ref", file_type));
        
        if !file_path.exists() {
            continue;
        }
        
        let content = fs::read_to_string(&file_path)
            .context(format!("Failed to read {}.vql.ref", file_type))?;
        
        // Extract all asset entries
        let asset_pattern = r"ASSET:(\+[^\s|]*)\s*\|(.*?)---";
        let re = Regex::new(asset_pattern).context("Failed to compile asset regex pattern")?;
        
        for captures in re.captures_iter(&content) {
            let asset_ref = captures.get(1).unwrap().as_str();
            let asset_data = parser::parse_asset(&content, asset_ref)?;
            assets.insert(asset_ref.to_string(), asset_data);
        }
    }
    
    // Generate the report based on the selected format
    match format.to_lowercase().as_str() {
        "md" => generate_markdown_report(&output_file, &assets, include_details)?,
        "html" => generate_html_report(&output_file, &assets, include_details)?,
        "json" => generate_json_report(&output_file, &assets, include_details)?,
        _ => generate_markdown_report(&output_file, &assets, include_details)?, // Default
    }
    
    println!("{} Generated {} report: {}", 
        "SUCCESS:".green().bold(), 
        format.to_uppercase().blue().bold(),
        output_file.display().to_string().blue());
    
    Ok(())
}

/// Generate markdown report
fn generate_markdown_report(
    output_file: &Path, 
    assets: &HashMap<String, crate::models::asset::AssetData>,
    include_details: bool
) -> Result<()> {
    let mut content = String::new();
    
    // Header
    content.push_str("# VQL Assessment Report\n\n");
    content.push_str(&format!("Generated: {}\n\n", chrono::Local::now().format("%Y-%m-%d %H:%M:%S")));
    
    // Summary table
    content.push_str("## Quality Assessment Summary\n\n");
    content.push_str("| Asset | Exemplar | Architecture | Security | Performance | Last Updated |\n");
    content.push_str("|-------|----------|--------------|----------|------------|--------------|\n");
    
    for (asset_ref, asset_data) in assets {
        let exemplar = asset_data.header.get("EXEMPLAR").cloned().unwrap_or_else(|| "?".to_string());
        let arch = asset_data.header.get("ARCH").cloned().unwrap_or_else(|| "?".to_string());
        let sec = asset_data.header.get("SEC").cloned().unwrap_or_else(|| "?".to_string());
        let perf = asset_data.header.get("PERF").cloned().unwrap_or_else(|| "?".to_string());
        let last_update = asset_data.header.get("LAST_UPDATE").cloned().unwrap_or_else(|| "?".to_string());
        
        content.push_str(&format!("| {} | {} | {} | {} | {} | {} |\n",
            asset_ref,
            exemplar,
            arch,
            sec,
            perf,
            last_update
        ));
    }
    
    // Detailed analysis (if requested)
    if include_details {
        content.push_str("\n## Detailed Analysis\n\n");
        
        for (asset_ref, asset_data) in assets {
            content.push_str(&format!("### {}\n\n", asset_ref));
            
            // Header information
            content.push_str("#### Asset Information\n\n");
            for (key, value) in &asset_data.header {
                content.push_str(&format!("- **{}**: {}\n", key, value));
            }
            
            // Analysis sections
            content.push_str("\n#### Architecture Analysis\n\n");
            content.push_str(asset_data.analysis.get("arch").unwrap_or(&"No analysis available.".to_string()));
            
            content.push_str("\n\n#### Security Analysis\n\n");
            content.push_str(asset_data.analysis.get("sec").unwrap_or(&"No analysis available.".to_string()));
            
            content.push_str("\n\n#### Performance Analysis\n\n");
            content.push_str(asset_data.analysis.get("perf").unwrap_or(&"No analysis available.".to_string()));
            
            content.push_str("\n\n---\n\n");
        }
    }
    
    // Write to file
    fs::write(output_file, content)
        .context(format!("Failed to write report to {}", output_file.display()))?;
    
    Ok(())
}

/// Generate HTML report
fn generate_html_report(
    output_file: &Path, 
    assets: &HashMap<String, crate::models::asset::AssetData>,
    include_details: bool
) -> Result<()> {
    let mut content = String::new();
    
    // HTML header
    content.push_str("<!DOCTYPE html>\n");
    content.push_str("<html lang=\"en\">\n");
    content.push_str("<head>\n");
    content.push_str("  <meta charset=\"UTF-8\">\n");
    content.push_str("  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
    content.push_str("  <title>VQL Assessment Report</title>\n");
    content.push_str("  <style>\n");
    content.push_str("    body { font-family: Arial, sans-serif; margin: 20px; }\n");
    content.push_str("    table { border-collapse: collapse; width: 100%; }\n");
    content.push_str("    th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }\n");
    content.push_str("    th { background-color: #f2f2f2; }\n");
    content.push_str("    tr:nth-child(even) { background-color: #f9f9f9; }\n");
    content.push_str("    .H { color: green; font-weight: bold; }\n");
    content.push_str("    .M { color: orange; }\n");
    content.push_str("    .L { color: red; }\n");
    content.push_str("    .T { color: green; }\n");
    content.push_str("    .F { color: gray; }\n");
    content.push_str("    .asset-details { margin-top: 30px; border-top: 1px solid #ddd; padding-top: 20px; }\n");
    content.push_str("  </style>\n");
    content.push_str("</head>\n");
    content.push_str("<body>\n");
    
    // Header
    content.push_str("  <h1>VQL Assessment Report</h1>\n");
    content.push_str(&format!("  <p>Generated: {}</p>\n\n", chrono::Local::now().format("%Y-%m-%d %H:%M:%S")));
    
    // Summary table
    content.push_str("  <h2>Quality Assessment Summary</h2>\n");
    content.push_str("  <table>\n");
    content.push_str("    <tr>\n");
    content.push_str("      <th>Asset</th>\n");
    content.push_str("      <th>Exemplar</th>\n");
    content.push_str("      <th>Architecture</th>\n");
    content.push_str("      <th>Security</th>\n");
    content.push_str("      <th>Performance</th>\n");
    content.push_str("      <th>Last Updated</th>\n");
    content.push_str("    </tr>\n");
    
    for (asset_ref, asset_data) in assets {
        let exemplar = asset_data.header.get("EXEMPLAR").cloned().unwrap_or_else(|| "?".to_string());
        let arch = asset_data.header.get("ARCH").cloned().unwrap_or_else(|| "?".to_string());
        let sec = asset_data.header.get("SEC").cloned().unwrap_or_else(|| "?".to_string());
        let perf = asset_data.header.get("PERF").cloned().unwrap_or_else(|| "?".to_string());
        let last_update = asset_data.header.get("LAST_UPDATE").cloned().unwrap_or_else(|| "?".to_string());
        
        content.push_str("    <tr>\n");
        content.push_str(&format!("      <td>{}</td>\n", asset_ref));
        content.push_str(&format!("      <td class=\"{}\">{}</td>\n", exemplar, exemplar));
        content.push_str(&format!("      <td class=\"{}\">{}</td>\n", arch, arch));
        content.push_str(&format!("      <td class=\"{}\">{}</td>\n", sec, sec));
        content.push_str(&format!("      <td class=\"{}\">{}</td>\n", perf, perf));
        content.push_str(&format!("      <td>{}</td>\n", last_update));
        content.push_str("    </tr>\n");
    }
    
    content.push_str("  </table>\n");
    
    // Detailed analysis (if requested)
    if include_details {
        content.push_str("\n  <h2>Detailed Analysis</h2>\n");
        
        for (asset_ref, asset_data) in assets {
            content.push_str(&format!("  <div class=\"asset-details\" id=\"{}\">\n", asset_ref.replace("+", "")));
            content.push_str(&format!("    <h3>{}</h3>\n", asset_ref));
            
            // Header information
            content.push_str("    <h4>Asset Information</h4>\n");
            content.push_str("    <ul>\n");
            for (key, value) in &asset_data.header {
                content.push_str(&format!("      <li><strong>{}:</strong> {}</li>\n", key, value));
            }
            content.push_str("    </ul>\n");
            
            // Analysis sections
            content.push_str("    <h4>Architecture Analysis</h4>\n");
            content.push_str(&format!("    <p>{}</p>\n", 
                asset_data.analysis.get("arch").unwrap_or(&"No analysis available.".to_string())));
            
            content.push_str("    <h4>Security Analysis</h4>\n");
            content.push_str(&format!("    <p>{}</p>\n", 
                asset_data.analysis.get("sec").unwrap_or(&"No analysis available.".to_string())));
            
            content.push_str("    <h4>Performance Analysis</h4>\n");
            content.push_str(&format!("    <p>{}</p>\n", 
                asset_data.analysis.get("perf").unwrap_or(&"No analysis available.".to_string())));
            
            content.push_str("  </div>\n");
        }
    }
    
    // HTML footer
    content.push_str("</body>\n");
    content.push_str("</html>\n");
    
    // Write to file
    fs::write(output_file, content)
        .context(format!("Failed to write report to {}", output_file.display()))?;
    
    Ok(())
}

/// Generate JSON report
fn generate_json_report(
    output_file: &Path, 
    assets: &HashMap<String, crate::models::asset::AssetData>,
    include_details: bool
) -> Result<()> {
    let mut report = serde_json::Map::new();
    
    // Add metadata
    let mut metadata = serde_json::Map::new();
    metadata.insert("generated".to_string(), serde_json::Value::String(
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    ));
    metadata.insert("include_details".to_string(), serde_json::Value::Bool(include_details));
    report.insert("metadata".to_string(), serde_json::Value::Object(metadata));
    
    // Add assets
    let mut assets_json = serde_json::Map::new();
    
    for (asset_ref, asset_data) in assets {
        let mut asset_json = serde_json::Map::new();
        
        // Add header data
        for (key, value) in &asset_data.header {
            asset_json.insert(key.clone(), serde_json::Value::String(value.clone()));
        }
        
        // Add analysis if details requested
        if include_details {
            let mut analysis = serde_json::Map::new();
            
            for (key, value) in &asset_data.analysis {
                analysis.insert(key.clone(), serde_json::Value::String(value.clone()));
            }
            
            asset_json.insert("analysis".to_string(), serde_json::Value::Object(analysis));
        }
        
        assets_json.insert(asset_ref.clone(), serde_json::Value::Object(asset_json));
    }
    
    report.insert("assets".to_string(), serde_json::Value::Object(assets_json));
    
    // Write to file
    let json_content = serde_json::to_string_pretty(&report)
        .context("Failed to serialize to JSON")?;
    
    fs::write(output_file, json_content)
        .context(format!("Failed to write report to {}", output_file.display()))?;
    
    Ok(())
}