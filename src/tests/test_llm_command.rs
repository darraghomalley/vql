use anyhow::Result;
use std::path::Path;
use crate::commands::json_commands::process_command;

pub fn test_llm_ar_add() -> Result<()> {
    // First add the entity using CLI format (not LLM format)
    println!("Adding entity 'l' (Lib)...");
    let add_entity_command = "-er -add l Lib";
    match process_command(add_entity_command) {
        Ok(_) => println!("Entity added successfully"),
        Err(e) => println!("Error adding entity: {}", e),
    }
    
    // Add asset type using CLI format
    println!("Adding asset type 'r' (Repository)...");
    let add_asset_type_command = "-at -add r Repository";
    match process_command(add_asset_type_command) {
        Ok(_) => println!("Asset type added successfully"),
        Err(e) => println!("Error adding asset type: {}", e),
    }
    
    // Get the project path for the test
    let lib_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/lib.rs");
    let lib_path_str = lib_path.to_str().unwrap();
    
    // Test the process_command function with a formatted :-ar.add command
    let command = format!(":-ar.add(lr, l, r, \"{}\")", lib_path_str);
    println!("Testing command: {}", command);
    
    // Process the command
    match process_command(&command) {
        Ok(_) => println!("Command processed successfully"),
        Err(e) => println!("Error processing command: {}", e),
    }
    
    Ok(())
}