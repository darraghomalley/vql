use std::path::{Path, PathBuf};
use std::fs;
use std::io::{self, Write};
use std::env;
use anyhow::{Result, Context};
use crate::utils::platform;
use colored::Colorize;

/// Find the VQL root directory by traversing upward from the current directory
pub fn find_vql_root() -> Option<PathBuf> {
    let mut current_dir = env::current_dir().ok()?;
    
    loop {
        let vql_dir = current_dir.join("vql");
        if vql_dir.is_dir() {
            return Some(vql_dir);
        }
        
        // Handle platform-specific root directories
        if platform::is_filesystem_root(&current_dir) {
            break;
        }
        
        // Move up to parent directory
        if !current_dir.pop() {
            break;
        }
    }
    
    None
}

/// Create a new VQL directory with initial structure
pub fn create_vql_directory(path: &str) -> Result<PathBuf> {
    let target_path = Path::new(path).canonicalize()
        .context(format!("Failed to resolve path: {}", path))?;
    
    let vql_dir = target_path.join("vql");
    
    // Create vql directory
    fs::create_dir_all(&vql_dir)
        .context("Failed to create vql directory")?;
    
    // Create initial VQL reference files
    for short_name in &["m", "c", "u"] {
        // Get the file type name from the short name
        let type_name = get_type_name_from_short_name(short_name);
        let file_type = format!("{}s", type_name);
        
        let file_path = vql_dir.join(format!("{}.vql.ref", file_type));
        if !file_path.exists() {
            let mut file = fs::File::create(&file_path)
                .context(format!("Failed to create {}.vql.ref", file_type))?;
            writeln!(file, "# VQL {} CACHE v1.0", file_type.to_uppercase())
                .context(format!("Failed to write to {}.vql.ref", file_type))?;
            writeln!(file, "# This file stores architectural assessments for {} components", type_name)
                .context(format!("Failed to write to {}.vql.ref", file_type))?;
            writeln!(file, "# Format: ASSET:[+ref] | LAST_UPDATE:[timestamp] | EXEMPLAR:[T/F] | ARCH:[H/M/L] | SEC:[H/M/L] | PERF:[H/M/L]")
                .context(format!("Failed to write to {}.vql.ref", file_type))?;
            writeln!(file, "# Followed by section-specific analysis\n")
                .context(format!("Failed to write to {}.vql.ref", file_type))?;
        }
    }
    
    // Create reference document
    let reference_path = vql_dir.join("vql-reference.md");
    if !reference_path.exists() {
        let mut file = fs::File::create(&reference_path)
            .context("Failed to create vql-reference.md")?;
        writeln!(file, "# Vibe Query Language (VQL) - Complete Reference v1.1\n")
            .context("Failed to write to vql-reference.md")?;
        writeln!(file, "VQL is a structured communication protocol for efficient software architecture discussions and assessments. It provides standardized commands, indicators, and references to guide the development of high-quality, consistent software architecture.\n")
            .context("Failed to write to vql-reference.md")?;
        
        // Add standard sections
        writeln!(file, "## Human Commands (:hc)\n")
            .context("Failed to write to vql-reference.md")?;
        writeln!(file, "## LLM Indicators (:li)\n")
            .context("Failed to write to vql-reference.md")?;
        writeln!(file, "## Entity References (.er)\nVQL      Entity Name                     Description\n")
            .context("Failed to write to vql-reference.md")?;
        writeln!(file, "## Asset Types (^at)\nVQL       Asset Type                     Description\n- ^m:     Model                          Database model / entity definition\n- ^c:     Controller                     API controller handling business logic\n- ^u:     UI                             User interface component\n\n### Additional asset types can be added with 'vql add-asset-type'\n")
            .context("Failed to write to vql-reference.md")?;
        writeln!(file, "## Asset References (+ar)\nIMPORTANT: Asset References identify specific code assets in the codebase. They use the + prefix for clarity.\nAsset Reference Principal Entity                        Type             AssetPath                                                      PathStatus   Exemplar ARCH SEC PERF LastReview\n")
            .context("Failed to write to vql-reference.md")?;
    }
    
    // Create workflows document
    let workflows_path = vql_dir.join("vql-workflows.md");
    if !workflows_path.exists() {
        let mut file = fs::File::create(&workflows_path)
            .context("Failed to create vql-workflows.md")?;
        writeln!(file, "# VQL Workflow Guide\n")
            .context("Failed to write to vql-workflows.md")?;
        writeln!(file, "This document contains detailed workflow instructions for using VQL commands. It is loaded into context only when VQL mode is active.\n")
            .context("Failed to write to vql-workflows.md")?;
        
        // Add workflow sections
        writeln!(file, "## Command Workflows\n")
            .context("Failed to write to vql-workflows.md")?;
            
        // Add asset type information
        writeln!(file, "### Asset Types\n")
            .context("Failed to write to vql-workflows.md")?;
        writeln!(file, "VQL supports dynamic asset types. Each asset type:")
            .context("Failed to write to vql-workflows.md")?;
        writeln!(file, "1. Is identified by a single alphabetic character (e.g., 'm' for model, 'c' for controller)")
            .context("Failed to write to vql-workflows.md")?;
        writeln!(file, "2. Has its own .vql file (e.g., models.vql, controllers.vql, services.vql)")
            .context("Failed to write to vql-workflows.md")?;
        writeln!(file, "3. Can be created with the 'vql add-asset-type' command\n")
            .context("Failed to write to vql-workflows.md")?;
        writeln!(file, "When processing commands for a specific asset type, use the appropriate file based on the asset's type character.\n")
            .context("Failed to write to vql-workflows.md")?;
        
        // Review workflow
        writeln!(file, "### Review Workflow")
            .context("Failed to write to vql-workflows.md")?;
        writeln!(file, "When a user requests a review using the `+AssetRef.Review(X)` command:\n")
            .context("Failed to write to vql-workflows.md")?;
        writeln!(file, "1. You must analyze the specified asset based on the requested aspects (A=Architecture, S=Security, P=Performance)")
            .context("Failed to write to vql-workflows.md")?;
        writeln!(file, "2. For each aspect analyzed, you MUST store the results using the CLI command:")
            .context("Failed to write to vql-workflows.md")?;
        writeln!(file, "   ```")
            .context("Failed to write to vql-workflows.md")?;
        writeln!(file, "   vql store +AssetRef --rating=X --aspect=Y --analysis=\"Your detailed analysis...\"")
            .context("Failed to write to vql-workflows.md")?;
        writeln!(file, "   ```")
            .context("Failed to write to vql-workflows.md")?;
        writeln!(file, "   Where:")
            .context("Failed to write to vql-workflows.md")?;
        writeln!(file, "   - X is your rating (H, M, or L)")
            .context("Failed to write to vql-workflows.md")?;
        writeln!(file, "   - Y is the aspect (arch, sec, or perf)")
            .context("Failed to write to vql-workflows.md")?;
        writeln!(file, "   - \"Your detailed analysis...\" is your comprehensive assessment\n")
            .context("Failed to write to vql-workflows.md")?;
        writeln!(file, "3. After storing all aspects, confirm to the user that the review is complete and all results have been stored\n")
            .context("Failed to write to vql-workflows.md")?;
        
        // Exemplar workflow
        writeln!(file, "### Exemplar Setting Workflow")
            .context("Failed to write to vql-workflows.md")?;
        writeln!(file, "When a user requests to set an exemplar status using `+AssetRef.Exemplar=T/F`:\n")
            .context("Failed to write to vql-workflows.md")?;
        writeln!(file, "1. You must evaluate if the asset meets exemplar criteria (if setting to T)")
            .context("Failed to write to vql-workflows.md")?;
        writeln!(file, "2. Store the exemplar status using the CLI command:")
            .context("Failed to write to vql-workflows.md")?;
        writeln!(file, "   ```")
            .context("Failed to write to vql-workflows.md")?;
        writeln!(file, "   vql set-exemplar +AssetRef --status=X")
            .context("Failed to write to vql-workflows.md")?;
        writeln!(file, "   ```")
            .context("Failed to write to vql-workflows.md")?;
        writeln!(file, "   Where X is true or false\n")
            .context("Failed to write to vql-workflows.md")?;
        
        // Asset rating workflow
        writeln!(file, "### Asset Rating Workflow")
            .context("Failed to write to vql-workflows.md")?;
        writeln!(file, "When a user sets a rating using `+AssetRef.ASPECT=X` (where ASPECT is ARCH, SEC, or PERF):\n")
            .context("Failed to write to vql-workflows.md")?;
        writeln!(file, "1. You must use the appropriate CLI command to store the rating:")
            .context("Failed to write to vql-workflows.md")?;
        writeln!(file, "   ```")
            .context("Failed to write to vql-workflows.md")?;
        writeln!(file, "   vql store +AssetRef --rating=X --aspect=Y --analysis=\"Rating updated by user directive\"")
            .context("Failed to write to vql-workflows.md")?;
        writeln!(file, "   ```\n")
            .context("Failed to write to vql-workflows.md")?;
        
        // General behavior
        writeln!(file, "## General VQL Mode Behavior\n")
            .context("Failed to write to vql-workflows.md")?;
        writeln!(file, "When VQL mode is active:")
            .context("Failed to write to vql-workflows.md")?;
        writeln!(file, "1. You MUST include LLM indicators at the end of EVERY response")
            .context("Failed to write to vql-workflows.md")?;
        writeln!(file, "2. You MUST automatically process any VQL commands in user messages")
            .context("Failed to write to vql-workflows.md")?;
        writeln!(file, "3. You MUST maintain awareness of the asset references and their current state")
            .context("Failed to write to vql-workflows.md")?;
        writeln!(file, "4. You MUST use the CLI to store any changes or assessments you make\n")
            .context("Failed to write to vql-workflows.md")?;
        writeln!(file, "IMPORTANT: Never claim to perform a VQL operation without actually storing the results using the appropriate CLI command.")
            .context("Failed to write to vql-workflows.md")?;
    }
    
    Ok(vql_dir)
}

/// Calculate MD5 hash of file contents
pub fn get_file_hash(file_path: &Path) -> Result<String> {
    let content = fs::read(file_path)
        .context(format!("Failed to read file for hashing: {}", file_path.display()))?;
    
    let digest = md5::compute(&content);
    Ok(format!("{:x}", digest))
}

/// Get the asset file path based on asset type
pub fn get_asset_file_path(asset_type: &str) -> Result<PathBuf> {
    let vql_dir = find_vql_root()
        .context("VQL directory not found. Run 'vql init' first")?;
    
    if !asset_type.starts_with('^') || asset_type.len() < 2 {
        return Err(anyhow::anyhow!("Invalid asset type format: {}", asset_type));
    }
    
    // Extract the single character after the ^ prefix
    let type_char = &asset_type[1..2];
    
    // Verify it's an alphabetic character
    if !type_char.chars().next().unwrap().is_ascii_alphabetic() {
        return Err(anyhow::anyhow!("Asset type character must be alphabetic: {}", asset_type));
    }
    
    // Get the file type name from the short name
    let type_name = get_type_name_from_short_name(type_char);
    let file_type = format!("{}s", type_name);
    
    let file_path = vql_dir.join(format!("{}.vql.ref", file_type));
    
    // Check if the file exists, if not, create it (in case it was manually deleted)
    if !file_path.exists() {
        let mut file = fs::File::create(&file_path)
            .context(format!("Failed to create {}.vql.ref", file_type))?;
        
        writeln!(file, "# VQL {} CACHE v1.0", file_type.to_uppercase())
            .context(format!("Failed to write to {}.vql.ref", file_type))?;
        writeln!(file, "# This file stores architectural assessments for {} components", file_type)
            .context(format!("Failed to write to {}.vql.ref", file_type))?;
        writeln!(file, "# Format: ASSET:[+ref] | LAST_UPDATE:[timestamp] | EXEMPLAR:[T/F] | ARCH:[H/M/L] | SEC:[H/M/L] | PERF:[H/M/L]")
            .context(format!("Failed to write to {}.vql.ref", file_type))?;
        writeln!(file, "# Followed by section-specific analysis\n")
            .context(format!("Failed to write to {}.vql.ref", file_type))?;
        
        println!("Created missing VQL reference file: {}.vql.ref", file_type.blue());
    }
    
    Ok(file_path)
}

/// Read the content of an asset file
pub fn read_asset_file(asset_type: &str) -> Result<String> {
    let file_path = get_asset_file_path(asset_type)?;
    let content = fs::read_to_string(&file_path)
        .context(format!("Failed to read asset file: {}", file_path.display()))?;
    
    Ok(content)
}

/// Write content to an asset file
pub fn write_asset_file(asset_type: &str, content: &str) -> Result<()> {
    let file_path = get_asset_file_path(asset_type)?;
    fs::write(&file_path, content)
        .context(format!("Failed to write to asset file: {}", file_path.display()))?;
    
    Ok(())
}

/// Get the type name (e.g., "model", "controller") from a short name (e.g., "m", "c")
/// This function maps single-character type codes to their full type names
pub fn get_type_name_from_short_name(short_name: &str) -> String {
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

/// Determine asset type from a reference (e.g., "userm" -> "^m")
/// This extracts the type character from the end of an asset reference
pub fn get_asset_type_from_ref(asset_ref: &str) -> Result<String> {
    // We no longer require a + prefix, but remove it if it exists
    let ref_without_prefix = asset_ref.trim_start_matches('+');
    
    // Get the last character which indicates the asset type
    if let Some(last_char) = ref_without_prefix.chars().last() {
        // Allow any single alphabetic character as a type
        if last_char.is_ascii_alphabetic() {
            Ok(format!("^{}", last_char))
        } else {
            Err(anyhow::anyhow!("Unknown asset type for reference: {}", asset_ref))
        }
    } else {
        Err(anyhow::anyhow!("Invalid asset reference format: {}", asset_ref))
    }
}