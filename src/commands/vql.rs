use anyhow::{Result, Context};
use colored::Colorize;
use std::env;

use crate::utils::filesystem;

/// Process VQL command interception
pub fn process_vql_command(command: &str) -> Result<()> {
    // Parse the VQL command
    // Only support the : prefix as standardized in the updated CLAUDE.md
    if command.starts_with(":vql") {
        return handle_vql_mode_command(command);
    } else if command.starts_with(":li") {
        return show_llm_indicators();
    } else if command.starts_with("+") && command.contains(".Review") {
        return process_review_command(command);
    } else if command.starts_with("+") && command.contains(".Why") {
        return process_why_command(command);
    } else if command.starts_with("+") && command.contains(".Exemplar") {
        return process_exemplar_command(command);
    } else if command.starts_with("+") && (command.contains(".ARCH") 
                                         || command.contains(".SEC") 
                                         || command.contains(".PERF")) {
        return process_rating_command(command);
    } else if command.starts_with(":Review") {
        return process_full_review_command();
    } else if command.starts_with(".er.Add") {
        return process_add_entity_command(command);
    } else if command.starts_with("^at.Add") {
        return process_add_asset_type_command(command);
    } else if command.starts_with("+ar.Add") {
        return process_add_asset_reference_command(command);
    } else if command == ".er" {
        return show_all_entities();
    } else if command == "^at" {
        return show_all_asset_types();
    } else if command == "+ar" {
        return show_all_asset_references();
    // We can remove this case since it's handled in handle_vql_mode_command
    // } else if command == ":vql" {
    //    return show_all_vql_commands();
    }
    
    println!("{} Unknown VQL command: {}", "ERROR:".red().bold(), command);
    Ok(())
}

/// Handle VQL mode commands (:vql on, :vql off)
fn handle_vql_mode_command(command: &str) -> Result<()> {
    // Check if the command is exactly ":vql" - if so, show all commands 
    if command == ":vql" {
        return show_all_vql_commands();
    } else if command.contains("on") {
        println!("{} VQL mode activated", "INFO:".blue().bold());
        // In a real implementation, we would store this state
    } else if command.contains("off") {
        println!("{} VQL mode deactivated", "INFO:".blue().bold());
        // In a real implementation, we would store this state
    } else {
        // Show complete VQL reference
        println!("{} Showing VQL reference", "INFO:".blue().bold());
        
        // Get VQL directory
        let vql_dir = filesystem::find_vql_root()
            .context("VQL directory not found. Run 'vql init' first.")?;
        
        let reference_path = vql_dir.join("vql-reference.md");
        if reference_path.exists() {
            let content = std::fs::read_to_string(&reference_path)
                .context("Failed to read vql-reference.md")?;
            
            // No need to replace | with : anymore since we only use : syntax now
            println!("\n{}", content);
        } else {
            println!("{} VQL reference file not found", "ERROR:".red().bold());
        }
    }
    
    Ok(())
}

/// Show LLM indicators (:li)
fn show_llm_indicators() -> Result<()> {
    println!("{} LLM Indicators:", "INFO:".blue().bold());
    
    // Get VQL directory
    let vql_dir = filesystem::find_vql_root()
        .context("VQL directory not found. Run 'vql init' first.")?;
    
    let reference_path = vql_dir.join("vql-reference.md");
    if reference_path.exists() {
        let content = std::fs::read_to_string(&reference_path)
            .context("Failed to read vql-reference.md")?;
        
        // Extract LLM Indicators section
        if let Some(start) = content.find("## LLM Indicators") {
            if let Some(end) = content[start..].find("##") {
                let section = &content[start..start+end];
                println!("\n{}", section);
            } else {
                let section = &content[start..];
                println!("\n{}", section);
            }
        } else {
            println!("{} LLM Indicators section not found in VQL reference", "ERROR:".red().bold());
        }
    } else {
        println!("{} VQL reference file not found", "ERROR:".red().bold());
    }
    
    Ok(())
}

/// Process asset review command (+asset.Review(A|S|P))
fn process_review_command(command: &str) -> Result<()> {
    // Parse the asset reference and aspect
    let parts: Vec<&str> = command.split('.').collect();
    if parts.len() < 2 {
        return Err(anyhow::anyhow!("Invalid review command format"));
    }
    
    let asset_ref = parts[0].trim();
    
    // Extract aspect from Review(A|S|P)
    if let Some(aspect_char) = command.chars().find(|&c| c == 'A' || c == 'S' || c == 'P') {
        let aspect = match aspect_char {
            'A' => "arch",
            'S' => "sec",
            'P' => "perf",
            _ => "all",
        };
        
        println!("{} Preparing {} for {} review", 
            "INFO:".blue().bold(), 
            asset_ref.blue().bold(),
            aspect.yellow());
        
        // In a real implementation, we would call the prepare command
        // For now, just print this message
        println!("This would call: vql prepare {} --aspect={}", asset_ref, aspect);
    } else {
        return Err(anyhow::anyhow!("Invalid aspect in review command"));
    }
    
    Ok(())
}

/// Process asset why command (+asset.Why(A|S|P))
fn process_why_command(command: &str) -> Result<()> {
    // Parse the asset reference and aspect
    let parts: Vec<&str> = command.split('.').collect();
    if parts.len() < 2 {
        return Err(anyhow::anyhow!("Invalid why command format"));
    }
    
    let asset_ref = parts[0].trim();
    
    // Extract aspect from Why(A|S|P)
    if let Some(aspect_char) = command.chars().find(|&c| c == 'A' || c == 'S' || c == 'P') {
        let aspect = match aspect_char {
            'A' => "arch",
            'S' => "sec",
            'P' => "perf",
            _ => "all",
        };
        
        println!("{} Explaining {} rating justification for {}", 
            "INFO:".blue().bold(), 
            aspect.yellow(),
            asset_ref.blue().bold());
        
        // In a real implementation, we would fetch and display the analysis section
        // For now, just print this message
        println!("This would display the analysis for {} from the {} aspect", asset_ref, aspect);
    } else {
        // If no aspect is specified, provide justification for all aspects
        println!("{} Explaining all quality ratings for {}", 
            "INFO:".blue().bold(), 
            asset_ref.blue().bold());
        
        println!("This would display the complete analysis for {}", asset_ref);
    }
    
    Ok(())
}

/// Process exemplar command (+asset.Exemplar=T|F)
fn process_exemplar_command(command: &str) -> Result<()> {
    // Parse the asset reference and exemplar status
    if let Some((asset_part, status_part)) = command.split_once('.') {
        let asset_ref = asset_part.trim();
        
        if let Some(status_char) = status_part.chars().find(|&c| c == 'T' || c == 'F') {
            let status = status_char == 'T';
            
            println!("{} Setting exemplar status of {} to {}", 
                "INFO:".blue().bold(), 
                asset_ref.blue().bold(),
                if status { "TRUE".green().bold() } else { "FALSE".red().bold() });
            
            // In a real implementation, we would call the set-exemplar command
            // For now, just print this message
            println!("This would call: vql set-exemplar {} --status={}", asset_ref, status);
        } else {
            return Err(anyhow::anyhow!("Invalid status in exemplar command"));
        }
    } else {
        return Err(anyhow::anyhow!("Invalid exemplar command format"));
    }
    
    Ok(())
}

/// Process rating command (+asset.ASPECT=H|M|L)
fn process_rating_command(command: &str) -> Result<()> {
    // Parse the asset reference, aspect, and rating
    if let Some((asset_part, rest)) = command.split_once('.') {
        let asset_ref = asset_part.trim();
        
        if let Some((aspect_part, rating_part)) = rest.split_once('=') {
            let aspect = match aspect_part.trim() {
                "ARCH" => "arch",
                "SEC" => "sec",
                "PERF" => "perf",
                _ => return Err(anyhow::anyhow!("Invalid aspect in rating command")),
            };
            
            let rating = rating_part.trim();
            if !["H", "M", "L", "h", "m", "l"].contains(&rating) {
                return Err(anyhow::anyhow!("Invalid rating (must be H, M, or L)"));
            }
            
            println!("{} Setting {} rating of {} to {}", 
                "INFO:".blue().bold(), 
                aspect.yellow(),
                asset_ref.blue().bold(),
                rating.green().bold());
            
            // In a real implementation, we would call the store command
            // For now, just print this message
            println!("This would call: vql store {} --rating={} --aspect={} --analysis=\"\"", 
                asset_ref, rating, aspect);
        } else {
            return Err(anyhow::anyhow!("Invalid rating command format"));
        }
    } else {
        return Err(anyhow::anyhow!("Invalid rating command format"));
    }
    
    Ok(())
}

/// Process full review command (:Review)
fn process_full_review_command() -> Result<()> {
    println!("{} Processing full review of all assets", "INFO:".blue().bold());
    
    // In a real implementation, we would iterate through all assets
    // For now, just print this message
    println!("This would review all assets in the system");
    
    Ok(())
}

/// Process add entity command (.er.Add(...))
fn process_add_entity_command(command: &str) -> Result<()> {
    // Parse command parameters
    if let Some(params) = extract_command_params(command) {
        if let (Some(short_name), Some(description)) = (params.get("shortName"), params.get("description")) {
            println!("{} Adding new entity reference: .{}", 
                "INFO:".blue().bold(), 
                short_name.blue().bold());
            
            // In a real implementation, we would call the add-entity-ref command
            // For now, just print this message
            println!("This would call: vql add-entity-ref --short-name={} --description=\"{}\"", 
                short_name, description);
        } else {
            return Err(anyhow::anyhow!("Missing required parameters for add entity command"));
        }
    } else {
        return Err(anyhow::anyhow!("Invalid add entity command format"));
    }
    
    Ok(())
}

/// Process add asset type command (^at.Add(...))
fn process_add_asset_type_command(command: &str) -> Result<()> {
    // Parse command parameters
    if let Some(params) = extract_command_params(command) {
        if let (Some(short_name), Some(description)) = (params.get("shortName"), params.get("description")) {
            println!("{} Adding new asset type: ^{}", 
                "INFO:".blue().bold(), 
                short_name.blue().bold());
            
            // In a real implementation, we would call the add-asset-type command
            // For now, just print this message
            println!("This would call: vql add-asset-type --short-name={} --description=\"{}\"", 
                short_name, description);
        } else {
            return Err(anyhow::anyhow!("Missing required parameters for add asset type command"));
        }
    } else {
        return Err(anyhow::anyhow!("Invalid add asset type command format"));
    }
    
    Ok(())
}

/// Process add asset reference command (+ar.Add(...))
fn process_add_asset_reference_command(command: &str) -> Result<()> {
    // Parse command parameters
    if let Some(params) = extract_command_params(command) {
        if let (Some(short_name), Some(entity), Some(type_ref), Some(path)) = 
            (params.get("shortName"), params.get("entity"), params.get("type"), params.get("path")) {
            println!("{} Adding new asset reference with short name: {}", 
                "INFO:".blue().bold(), 
                short_name.blue().bold());
            
            // In a real implementation, we would call the add-asset-ref command
            // For now, just print this message
            println!("This would call: vql add-asset-ref --short-name={} --entity={} --asset-type={} --path=\"{}\"", 
                short_name, entity, type_ref, path);
        } else {
            return Err(anyhow::anyhow!("Missing required parameters for add asset reference command"));
        }
    } else {
        return Err(anyhow::anyhow!("Invalid add asset reference command format"));
    }
    
    Ok(())
}

/// Show all entity references
fn show_all_entities() -> Result<()> {
    println!("{} Showing all entity references", "INFO:".blue().bold());
    
    // Get VQL directory
    let vql_dir = filesystem::find_vql_root()
        .context("VQL directory not found. Run 'vql init' first.")?;
    
    let reference_path = vql_dir.join("vql-reference.md");
    if reference_path.exists() {
        let content = std::fs::read_to_string(&reference_path)
            .context("Failed to read vql-reference.md")?;
        
        // Extract Entity References section
        if let Some(start) = content.find("## Entity References (.er)") {
            let section_end = if let Some(end_pos) = content[start..].find("\n## ") {
                start + end_pos
            } else {
                content.len()
            };
            
            let entity_section = &content[start..section_end];
            println!("\n{}", entity_section);
        } else {
            println!("{} Entity References section not found in VQL reference", "ERROR:".red().bold());
        }
    } else {
        println!("{} VQL reference file not found", "ERROR:".red().bold());
    }
    
    Ok(())
}

/// Show all asset types
fn show_all_asset_types() -> Result<()> {
    println!("{} Showing all asset types", "INFO:".blue().bold());
    
    // Get VQL directory
    let vql_dir = filesystem::find_vql_root()
        .context("VQL directory not found. Run 'vql init' first.")?;
    
    let reference_path = vql_dir.join("vql-reference.md");
    if reference_path.exists() {
        let content = std::fs::read_to_string(&reference_path)
            .context("Failed to read vql-reference.md")?;
        
        // Extract Asset Types section
        if let Some(start) = content.find("## Asset Types (^at)") {
            let section_end = if let Some(end_pos) = content[start..].find("\n## ") {
                start + end_pos
            } else {
                content.len()
            };
            
            let types_section = &content[start..section_end];
            println!("\n{}", types_section);
        } else {
            println!("{} Asset Types section not found in VQL reference", "ERROR:".red().bold());
        }
    } else {
        println!("{} VQL reference file not found", "ERROR:".red().bold());
    }
    
    Ok(())
}

/// Show all asset references
fn show_all_asset_references() -> Result<()> {
    println!("{} Showing all asset references", "INFO:".blue().bold());
    
    // Get VQL directory
    let vql_dir = filesystem::find_vql_root()
        .context("VQL directory not found. Run 'vql init' first.")?;
    
    let reference_path = vql_dir.join("vql-reference.md");
    if reference_path.exists() {
        let content = std::fs::read_to_string(&reference_path)
            .context("Failed to read vql-reference.md")?;
        
        // Extract Asset References section
        if let Some(start) = content.find("## Asset References (+ar)") {
            let section_end = if let Some(end_pos) = content[start..].find("\n## ") {
                start + end_pos
            } else {
                content.len()
            };
            
            let references_section = &content[start..section_end];
            println!("\n{}", references_section);
        } else {
            println!("{} Asset References section not found in VQL reference", "ERROR:".red().bold());
        }
    } else {
        println!("{} VQL reference file not found", "ERROR:".red().bold());
    }
    
    Ok(())
}

/// Show all VQL commands
fn show_all_vql_commands() -> Result<()> {
    println!("{} Showing all VQL commands", "INFO:".blue().bold());
    
    println!("\n{}",
        "## VQL Commands Reference\n\n\
        ### Entity Commands (.er)\n\
        - .er - Show all entity references\n\
        - .er.Add(shortName=X,description=Y) - Add a new entity reference\n\n\
        ### Asset Type Commands (^at)\n\
        - ^at - Show all asset types\n\
        - ^at.Add(shortName=X,description=Y) - Add a new asset type\n\n\
        ### Asset Reference Commands (+ar)\n\
        - +ar - Show all asset references\n\
        - +ar.Add(shortName=X,entity=Y,type=Z,path=P) - Add a new asset reference\n\n\
        ### Asset Review Commands\n\
        - +AssetRef.Review(A|S|P) - Review architecture (A), security (S), or performance (P)\n\
        - +AssetRef.Why(A|S|P) - Explain justification for quality ratings\n\
        - +AssetRef.Exemplar=T|F - Set exemplar status to true (T) or false (F)\n\
        - +AssetRef.ARCH=H|M|L - Set architecture rating to high (H), medium (M), or low (L)\n\
        - +AssetRef.SEC=H|M|L - Set security rating to high (H), medium (M), or low (L)\n\
        - +AssetRef.PERF=H|M|L - Set performance rating to high (H), medium (M), or low (L)\n\n\
        ### System Commands\n\
        - :vql on|off - Turn VQL mode on or off\n\
        - :vql - Show all VQL commands\n\
        - :li - Show LLM indicators reference\n\
        - :Review - Process comprehensive review of all assets"
    );
    
    Ok(())
}

/// Helper function to extract parameters from a command string
fn extract_command_params(command: &str) -> Option<std::collections::HashMap<String, String>> {
    let start = command.find('(')?;
    let end = command.rfind(')')?;
    
    let params_str = &command[start+1..end];
    let mut params = std::collections::HashMap::new();
    
    for param in params_str.split(',') {
        if let Some((key, value)) = param.split_once('=') {
            params.insert(key.trim().to_string(), value.trim().to_string());
        }
    }
    
    Some(params)
}