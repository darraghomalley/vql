use anyhow::Result;
use crate::utils::filesystem;
use colored::Colorize;

/// Initialize VQL in the specified directory
pub fn initialize_vql(path: &str) -> Result<()> {
    let vql_dir = filesystem::create_vql_directory(path)?;
    
    println!("{} VQL initialized successfully in: {}", 
        "SUCCESS:".green().bold(), 
        vql_dir.display().to_string().blue());
    
    println!("\nCreated the following structure:");
    println!("  {} - for model assets", "models.vql.ref".blue());
    println!("  {} - for controller assets", "controllers.vql.ref".blue());
    println!("  {} - for UI assets", "ui.vql.ref".blue());
    println!("  {} - for reference documentation", "vql-reference.md".blue());
    
    println!("\n{} You can now add asset, entity, and type references to your project:", 
        "NEXT STEPS:".yellow().bold());
    println!("  {}", "vql add-asset-ref --short-name=<name> --entity=<entity> --asset-type=<type> --path=<path>".cyan());
    println!("  {}", "vql add-entity-ref --short-name=<name> --description=<description>".cyan());
    println!("  {}", "vql add-asset-type --short-name=<name> --description=<description>".cyan());
    
    Ok(())
}