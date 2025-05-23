use std::fs;
use std::io::Write;
use regex::Regex;
use anyhow::{Result, Context};
use chrono::Local;
use colored::Colorize;

use crate::utils::filesystem;
use crate::utils::parser;

/// Add a new asset reference to the system
/// 
/// This function adds a new asset reference to the VQL system. It validates input parameters,
/// creates the appropriate entry in the corresponding asset type file, and updates the reference document.
///
/// # Arguments
/// * `short_name` - Alphanumeric identifier for the asset
/// * `entity` - Entity reference starting with '.' (e.g., '.user')
/// * `asset_type` - Asset type reference starting with '^' followed by a single alphabetic character
/// * `path` - Filesystem path to the asset
///
/// # Returns
/// * `Result<()>` - Ok if successful, Err with detailed error message otherwise
pub fn add_asset_reference(short_name: &str, entity: &str, asset_type: &str, path: &str) -> Result<()> {
    // Validate inputs
    if !short_name.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(anyhow::anyhow!("Short name must contain only alphanumeric characters"));
    }
    
    if !entity.starts_with('.') {
        return Err(anyhow::anyhow!("Entity reference must start with a dot (.)"));
    }
    
    if !asset_type.starts_with('^') {
        return Err(anyhow::anyhow!("Asset type must start with a caret (^)"));
    }
    
    if asset_type.len() < 2 || !asset_type.chars().nth(1).unwrap().is_ascii_alphabetic() {
        return Err(anyhow::anyhow!("Asset type must have a single alphabetic character after the caret (^)"));
    }
    
    // Extract the type character from the asset type
    
    // Extract the single character after the ^ prefix
    let type_char = asset_type.chars().nth(1).unwrap();
    
    // Check if it's a valid asset type character (must be alphabetic)
    if !type_char.is_ascii_alphabetic() {
        return Err(anyhow::anyhow!("Asset type character must be alphabetic: {}", asset_type));
    }
    
    // Get the file type name from the short name
    let type_name = get_type_name_from_short_name(&type_char.to_string());
    let file_type = format!("{}s", type_name);
    
    // Get VQL directory
    let vql_dir = filesystem::find_vql_root()
        .context("VQL directory not found. Run 'vql init' first.")?;
    
    // Construct new asset reference entry
    let entity_name = entity.trim_start_matches('.');
    let asset_ref = format!("+{}{}", entity_name, type_char);
    let timestamp = Local::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    
    let new_entry = parser::create_asset_entry(
        &asset_ref, 
        &timestamp, 
        "F", // Exemplar status: False
        "?", // Architecture rating: Unknown
        "?", // Security rating: Unknown
        "?"  // Performance rating: Unknown
    );
    
    // Update the appropriate VQL reference file
    let file_path = vql_dir.join(format!("{}.vql.ref", file_type));
    let mut content = fs::read_to_string(&file_path)
        .context(format!("Failed to read {}.vql.ref", file_type))?;
    
    // Add a newline if the file doesn't end with one
    if !content.ends_with('\n') {
        content.push('\n');
    }
    
    content.push_str(&new_entry);
    
    // Always add a newline at the end
    if !content.ends_with('\n') {
        content.push('\n');
    }
    
    fs::write(&file_path, content)
        .context(format!("Failed to write to {}.vql.ref", file_type))?;
    
    // Update the reference document as well
    update_reference_document(&asset_ref, entity, asset_type, path)?;
    
    println!("{} Added new asset reference: {}", 
        "SUCCESS:".green().bold(), 
        asset_ref.blue().bold());
    
    Ok(())
}

/// Add a new entity reference to the system
pub fn add_entity_reference(short_name: &str, description: &str) -> Result<()> {
    // Validate inputs
    if !short_name.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(anyhow::anyhow!("Short name must contain only alphanumeric characters"));
    }
    
    // Get VQL directory
    let vql_dir = filesystem::find_vql_root()
        .context("VQL directory not found. Run 'vql init' first.")?;
    
    // Update the reference document
    let reference_path = vql_dir.join("vql-reference.md");
    let content = fs::read_to_string(&reference_path)
        .context("Failed to read vql-reference.md")?;
    
    // Find the Entity References section by simple string search
    if let Some(start) = content.find("## Entity References (.er)") {
        // Find the end of the section (next section heading or EOF)
        let section_end = content[start..].find("\n## ")
            .map(|pos| start + pos)
            .unwrap_or(content.len());
        
        let entity_section = &content[start..section_end];
        
        // Format the new entity reference with proper alignment
        let entity_ref = format!("- .{:<6} {:<35} {}", 
            short_name, 
            short_name.to_string().to_uppercase(), // Entity name is capitalized version of short name
            description
        );
        
        // Insert the new entity before the end of the section or before any commands subsection
        let (insertion_point, suffix) = if let Some(commands_pos) = entity_section.find("### Entity Reference Commands") {
            (start + commands_pos, &entity_section[commands_pos..])
        } else {
            (section_end, "")
        };
        
        // Build the updated content
        let mut updated_content = content[..insertion_point].to_string();
        if !updated_content.ends_with('\n') {
            updated_content.push('\n');
        }
        updated_content.push_str(&entity_ref);
        updated_content.push('\n');
        if !suffix.is_empty() {
            updated_content.push_str(suffix);
        }
        if section_end < content.len() {
            updated_content.push_str(&content[section_end..]);
        }
        
        // Write back the updated content
        fs::write(&reference_path, updated_content)
            .context("Failed to write to vql-reference.md")?;
        
        println!("{} Added new entity reference: {}", 
            "SUCCESS:".green().bold(), 
            format!(".{}", short_name).blue().bold());
        
        Ok(())
    } else {
        Err(anyhow::anyhow!("Entity References section not found in VQL reference document"))
    }
}

/// Add a new asset type to the system
///
/// This function adds a new asset type to the VQL system. It validates input parameters,
/// creates a new VQL reference file (.vql.ref) for this asset type (if it doesn't exist already), and updates 
/// the reference document with the new type information.
///
/// # Arguments
/// * `short_name` - Single alphabetic character identifier for the asset type
/// * `description` - Human-readable description of what this asset type represents
///
/// # Returns
/// * `Result<()>` - Ok if successful, Err with detailed error message otherwise
pub fn add_asset_type(short_name: &str, description: &str) -> Result<()> {
    // Validate inputs
    if !short_name.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(anyhow::anyhow!("Short name must contain only alphanumeric characters"));
    }
    
    if short_name.len() != 1 {
        return Err(anyhow::anyhow!("Asset type short name must be a single character"));
    }
    
    // Get VQL directory
    let vql_dir = filesystem::find_vql_root()
        .context("VQL directory not found. Run 'vql init' first.")?;
    
    // Create the corresponding VQL reference file for this asset type
    let file_name = format!("{}s.vql.ref", get_type_name_from_short_name(short_name));
    let vql_file_path = vql_dir.join(&file_name);
    
    // Only create the file if it doesn't already exist
    if !vql_file_path.exists() {
        let mut file = fs::File::create(&vql_file_path)
            .context(format!("Failed to create {}", file_name))?;
        
        // Create the file with appropriate headers
        writeln!(file, "# VQL {} CACHE v1.0", get_type_name_from_short_name(short_name).to_uppercase())
            .context(format!("Failed to write to {}", file_name))?;
        writeln!(file, "# This file stores architectural assessments for {} components", get_type_name_from_short_name(short_name))
            .context(format!("Failed to write to {}", file_name))?;
        writeln!(file, "# Format: ASSET:[+ref] | LAST_UPDATE:[timestamp] | EXEMPLAR:[T/F] | ARCH:[H/M/L] | SEC:[H/M/L] | PERF:[H/M/L]")
            .context(format!("Failed to write to {}", file_name))?;
        writeln!(file, "# Followed by section-specific analysis\n")
            .context(format!("Failed to write to {}", file_name))?;
        
        println!("Created new VQL reference file: {}", file_name.blue());
    }
    
    // Update the reference document
    let reference_path = vql_dir.join("vql-reference.md");
    let content = fs::read_to_string(&reference_path)
        .context("Failed to read vql-reference.md")?;
    
    // Find the Asset Types section by simple string search
    if let Some(start) = content.find("## Asset Types (^at)") {
        // Find the end of the section (next section heading or EOF)
        let section_end = content[start..].find("\n## ")
            .map(|pos| start + pos)
            .unwrap_or(content.len());
        
        let types_section = &content[start..section_end];
        
        // Format the new asset type with proper alignment
        let type_ref = format!("- ^{:<6} {:<35} {}", 
            short_name, 
            short_name.to_string().to_uppercase(), // Type name is capitalized version of short name
            description
        );
        
        // Insert the new type before the end of the section or before any commands subsection
        let (insertion_point, suffix) = if let Some(commands_pos) = types_section.find("### Asset Type Commands") {
            (start + commands_pos, &types_section[commands_pos..])
        } else {
            (section_end, "")
        };
        
        // Build the updated content
        let mut updated_content = content[..insertion_point].to_string();
        if !updated_content.ends_with('\n') {
            updated_content.push('\n');
        }
        updated_content.push_str(&type_ref);
        updated_content.push('\n');
        if !suffix.is_empty() {
            updated_content.push_str(suffix);
        }
        if section_end < content.len() {
            updated_content.push_str(&content[section_end..]);
        }
        
        // Write back the updated content
        fs::write(&reference_path, updated_content)
            .context("Failed to write to vql-reference.md")?;
        
        println!("{} Added new asset type: {}", 
            "SUCCESS:".green().bold(), 
            format!("^{}", short_name).blue().bold());
        
        Ok(())
    } else {
        Err(anyhow::anyhow!("Asset Types section not found in VQL reference document"))
    }
}

/// Helper function to get a type name from a short name
fn get_type_name_from_short_name(short_name: &str) -> String {
    match short_name {
        "m" => "model".to_string(),
        "c" => "controller".to_string(),
        "u" => "ui".to_string(),
        "s" => "service".to_string(),
        "r" => "repository".to_string(),
        "h" => "helper".to_string(),
        "t" => "test".to_string(),
        "d" => "middleware".to_string(), // d for middleware
        "a" => "api".to_string(),
        "v" => "view".to_string(),
        "g" => "config".to_string(), // g for config
        _ => format!("{}type", short_name) // Default to shortname + "type" if not recognized
    }
}

/// Helper function to update the reference document with a new asset reference
fn update_reference_document(asset_ref: &str, entity: &str, asset_type: &str, path: &str) -> Result<()> {
    let vql_dir = filesystem::find_vql_root()
        .context("VQL directory not found.")?;
    
    let reference_path = vql_dir.join("vql-reference.md");
    let content = fs::read_to_string(&reference_path)
        .context("Failed to read vql-reference.md")?;
    
    // Find the Asset References section by simple string search
    if let Some(start) = content.find("## Asset References (+ar)") {
        // Find the end of the section (next section heading or EOF)
        let section_end = content[start..].find("\n## ")
            .map(|pos| start + pos)
            .unwrap_or(content.len());
        
        // Parse the entity name from its reference
        let entity_name = entity.trim_start_matches('.');
        
        // Format the new asset reference with proper alignment
        let type_name = if asset_type.starts_with('^') && asset_type.len() >= 2 {
            let type_char = asset_type.chars().nth(1).unwrap();
            let full_name = get_type_name_from_short_name(&type_char.to_string());
            format!("{} ({})", full_name, asset_type)
        } else {
            asset_type.to_string()
        };
        
        let new_ref = format!("- {:<12} {:<35} {:<18} {:<60} {:<12} {:<8} ? ? ? ?",
            asset_ref,
            format!("{} ({})", entity_name.to_uppercase(), entity),
            type_name,
            path,
            "CONFIRMED",
            "F" // Not an exemplar by default
        );
        
        // Insert the new reference before the end of the section or before any commands subsection
        let (insertion_point, suffix) = if let Some(commands_pos) = content[start..section_end].find("## Asset Reference Commands") {
            (start + commands_pos, &content[start + commands_pos..section_end])
        } else {
            (section_end, "")
        };
        
        // Build the updated content
        let mut updated_content = content[..insertion_point].to_string();
        if !updated_content.ends_with('\n') {
            updated_content.push('\n');
        }
        updated_content.push_str(&new_ref);
        updated_content.push('\n');
        if !suffix.is_empty() {
            updated_content.push_str(suffix);
        }
        if section_end < content.len() {
            updated_content.push_str(&content[section_end..]);
        }
        
        // Write back the updated content
        fs::write(&reference_path, updated_content)
            .context("Failed to write to vql-reference.md")?;
        
        Ok(())
    } else {
        Err(anyhow::anyhow!("Asset References section not found in VQL reference document"))
    }
}