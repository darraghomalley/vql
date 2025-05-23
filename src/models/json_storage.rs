use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Result, Context};
use serde::{Serialize, Deserialize};
use chrono::Utc;

/// Represents a command in the VQL system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandConfig {
    /// The name of the command (without the colon prefix)
    pub name: String,
    
    /// Description of what the command does
    pub description: String,
    
    /// When this command was created or last modified
    pub last_modified: String,
    
    /// Original command name if this was renamed (for built-in commands)
    pub original_name: Option<String>,
    
    /// Whether this is a built-in command that cannot be removed
    pub built_in: bool,
}

/// Represents an asset type in the VQL system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetType {
    /// Short name for the asset type (single character)
    pub short_name: String,
    
    /// Description of what this asset type represents
    pub description: String,
    
    /// When this asset type was created or last modified
    pub last_modified: String,
}

/// Represents an entity in the VQL system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    /// Short name for the entity
    pub short_name: String,
    
    /// Description of what this entity represents
    pub description: String,
    
    /// When this entity was created or last modified
    pub last_modified: String,
}

/// Represents a specific asset reference in the VQL system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetReference {
    /// Short name for the asset
    pub short_name: String,
    
    /// Entity this asset belongs to
    pub entity: String,
    
    /// Type of asset
    pub asset_type: String,
    
    /// File path to the asset
    pub path: String,
    
    /// When this asset was created or last modified
    pub last_modified: String,
    
    /// Is this an exemplar (best practice reference)?
    pub exemplar: bool,
    
    /// Map of principle short names to their reviews
    #[serde(default)]
    pub principle_reviews: HashMap<String, Review>,
    
    /// Architecture quality rating (H/M/L) - kept for backward compatibility
    pub arch_rating: Option<String>,
    
    /// Security quality rating (H/M/L) - kept for backward compatibility
    pub sec_rating: Option<String>,
    
    /// Performance quality rating (H/M/L) - kept for backward compatibility
    pub perf_rating: Option<String>,
    
    /// UI quality rating (H/M/L) - kept for backward compatibility
    pub ui_rating: Option<String>,
    
    /// Architecture analysis details - kept for backward compatibility
    pub arch_analysis: Option<String>,
    
    /// Security analysis details - kept for backward compatibility
    pub sec_analysis: Option<String>,
    
    /// Performance analysis details - kept for backward compatibility
    pub perf_analysis: Option<String>,
    
    /// UI analysis details - kept for backward compatibility
    pub ui_analysis: Option<String>,
}

/// Represents a principle in the VQL system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Principle {
    /// Short name for the principle (single character)
    pub short_name: String,
    
    /// Long name for the principle
    pub long_name: String,
    
    /// Guidance text for this principle
    pub guidance: Option<String>,
    
    /// When this principle was created or last modified
    pub last_modified: String,
}

/// Represents a review from a principle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    /// Rating for this review (H/M/L)
    pub rating: Option<String>,
    
    /// Analysis for this review
    pub analysis: Option<String>,
    
    /// When this review was last modified
    pub last_modified: String,
}

/// Main storage structure for VQL JSON data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonStorage {
    /// VQL version information
    pub version: String,
    
    /// When this storage was created
    pub created: String,
    
    /// When this storage was last modified
    pub last_modified: String,
    
    /// Map of command names (without colon) to their configurations
    pub commands: HashMap<String, CommandConfig>,
    
    /// Map of asset type short names to their configurations
    pub asset_types: HashMap<String, AssetType>,
    
    /// Map of entity short names to their configurations
    pub entities: HashMap<String, Entity>,
    
    /// Map of principle short names to their configurations
    #[serde(default)]
    pub principles: HashMap<String, Principle>,
    
    /// Map of asset reference short names to their configurations
    pub asset_references: HashMap<String, AssetReference>,
}

impl JsonStorage {
    /// Creates a new JsonStorage with default built-in commands
    pub fn new() -> Self {
        let now = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        
        let mut commands = HashMap::new();
        let mut principles = HashMap::new();
        
        // Add built-in commands
        commands.insert("ar".to_string(), CommandConfig {
            name: "ar".to_string(),
            description: "Asset Register - Manages asset references".to_string(),
            last_modified: now.clone(),
            original_name: None,
            built_in: true,
        });
        
        commands.insert("at".to_string(), CommandConfig {
            name: "at".to_string(),
            description: "Asset Type - Manages asset types".to_string(),
            last_modified: now.clone(),
            original_name: None,
            built_in: true,
        });
        
        commands.insert("er".to_string(), CommandConfig {
            name: "er".to_string(),
            description: "Entity Register - Manages entity definitions".to_string(),
            last_modified: now.clone(),
            original_name: None,
            built_in: true,
        });
        
        commands.insert("pr".to_string(), CommandConfig {
            name: "pr".to_string(),
            description: "Principle - Manages principles for reviewing assets".to_string(),
            last_modified: now.clone(),
            original_name: None,
            built_in: true,
        });
        
        commands.insert("setup".to_string(), CommandConfig {
            name: "setup".to_string(),
            description: "Creates VQL directory in current location".to_string(),
            last_modified: now.clone(),
            original_name: None,
            built_in: true,
        });
        
        commands.insert("st".to_string(), CommandConfig {
            name: "st".to_string(),
            description: "Store - Stores a review for an asset".to_string(),
            last_modified: now.clone(),
            original_name: None,
            built_in: true,
        });
        
        commands.insert("se".to_string(), CommandConfig {
            name: "se".to_string(),
            description: "Set Exemplar - Sets exemplar status for an asset".to_string(),
            last_modified: now.clone(),
            original_name: None,
            built_in: true,
        });
        
        commands.insert("sc".to_string(), CommandConfig {
            name: "sc".to_string(),
            description: "Set Compliance - Sets compliance rating for an asset".to_string(),
            last_modified: now.clone(),
            original_name: None,
            built_in: true,
        });
        
        // Special commands for review and refactor (LLM only)
        commands.insert("rv".to_string(), CommandConfig {
            name: "rv".to_string(),
            description: "Review - AI-assisted review of assets (LLM only)".to_string(),
            last_modified: now.clone(),
            original_name: None,
            built_in: true,
        });
        
        commands.insert("rf".to_string(), CommandConfig {
            name: "rf".to_string(),
            description: "Refactor - AI-assisted refactoring of assets (LLM only)".to_string(),
            last_modified: now.clone(),
            original_name: None,
            built_in: true,
        });
        
        // Add default principles
        principles.insert("a".to_string(), Principle {
            short_name: "a".to_string(),
            long_name: "Architecture".to_string(),
            guidance: Some("Architecture evaluation guidelines".to_string()),
            last_modified: now.clone(),
        });
        
        principles.insert("s".to_string(), Principle {
            short_name: "s".to_string(),
            long_name: "Security".to_string(),
            guidance: Some("Security evaluation guidelines".to_string()),
            last_modified: now.clone(),
        });
        
        principles.insert("p".to_string(), Principle {
            short_name: "p".to_string(),
            long_name: "Performance".to_string(),
            guidance: Some("Performance evaluation guidelines".to_string()),
            last_modified: now.clone(),
        });
        
        principles.insert("u".to_string(), Principle {
            short_name: "u".to_string(),
            long_name: "UI/UX".to_string(),
            guidance: Some("UI/UX evaluation guidelines".to_string()),
            last_modified: now.clone(),
        });
        
        JsonStorage {
            version: "1.0.0".to_string(),
            created: now.clone(),
            last_modified: now,
            commands,
            asset_types: HashMap::new(),
            entities: HashMap::new(),
            principles,
            asset_references: HashMap::new(),
        }
    }
    
    /// Find and load JSON storage from the specified path or create a new one
    pub fn load_or_create(vql_path: &Path) -> Result<Self> {
        let json_file_path = vql_path.join("vql_storage.json");
        
        if json_file_path.exists() {
            // Load existing storage
            let content = fs::read_to_string(&json_file_path)
                .context(format!("Failed to read VQL storage at {}", json_file_path.display()))?;
                
            let storage: JsonStorage = serde_json::from_str(&content)
                .context("Failed to parse VQL JSON storage")?;
                
            Ok(storage)
        } else {
            // Create new storage
            let storage = JsonStorage::new();
            storage.save(vql_path)?;
            Ok(storage)
        }
    }
    
    /// Save storage to the specified path
    pub fn save(&self, vql_path: &Path) -> Result<()> {
        // Make sure the directory exists
        if !vql_path.exists() {
            fs::create_dir_all(vql_path)
                .context(format!("Failed to create VQL directory at {}", vql_path.display()))?;
        }
        
        let json_file_path = vql_path.join("vql_storage.json");
        
        // Serialize with pretty printing
        let json_content = serde_json::to_string_pretty(self)
            .context("Failed to serialize VQL storage to JSON")?;
            
        fs::write(&json_file_path, json_content)
            .context(format!("Failed to write VQL storage to {}", json_file_path.display()))?;
            
        Ok(())
    }
    
    /// Add or update a command
    pub fn add_command(&mut self, name: &str, description: &str) -> Result<()> {
        // Command names shouldn't have the colon prefix when stored
        let name = name.trim_start_matches(':');
        
        // Check if command already exists
        if self.commands.contains_key(name) {
            return Err(anyhow::anyhow!("Command :{} already exists", name));
        }
        
        // Create new command config
        let command = CommandConfig {
            name: name.to_string(),
            description: description.to_string(),
            last_modified: Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
            original_name: None,
            built_in: false,
        };
        
        // Add to commands map
        self.commands.insert(name.to_string(), command);
        
        // Update last modified timestamp
        self.last_modified = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        
        Ok(())
    }
    
    /// Rename a command
    pub fn rename_command(&mut self, old_name: &str, new_name: &str) -> Result<()> {
        // Remove colon prefixes if present
        let old_name = old_name.trim_start_matches(':');
        let new_name = new_name.trim_start_matches(':');
        
        // Check if new name already exists
        if self.commands.contains_key(new_name) {
            return Err(anyhow::anyhow!("Command :{} already exists", new_name));
        }
        
        // Get the existing command
        let command = match self.commands.get(old_name) {
            Some(cmd) => cmd.clone(),
            None => return Err(anyhow::anyhow!("Command :{} not found", old_name)),
        };
        
        // Ensure the command is allowed to be renamed
        if command.built_in && command.original_name.is_none() {
            // This is the first time renaming a built-in command, we'll record the original name
            let mut updated_command = command.clone();
            updated_command.name = new_name.to_string();
            updated_command.original_name = Some(old_name.to_string());
            updated_command.last_modified = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
            
            // Remove old command and add new one
            self.commands.remove(old_name);
            self.commands.insert(new_name.to_string(), updated_command);
        } else {
            // For non-built-in commands or already renamed built-in commands
            let mut updated_command = command.clone();
            updated_command.name = new_name.to_string();
            updated_command.last_modified = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
            
            // Remove old command and add new one
            self.commands.remove(old_name);
            self.commands.insert(new_name.to_string(), updated_command);
        }
        
        // Update last modified timestamp
        self.last_modified = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        
        Ok(())
    }
    
    /// Get a command by name
    pub fn get_command(&self, name: &str) -> Option<&CommandConfig> {
        let name = name.trim_start_matches(':');
        self.commands.get(name)
    }
    
    /// Check if a command exists
    pub fn command_exists(&self, name: &str) -> bool {
        let name = name.trim_start_matches(':');
        self.commands.contains_key(name)
    }
    
    /// Add or update an asset type
    pub fn add_asset_type(&mut self, short_name: &str, description: &str) -> Result<()> {
        // Validate short name is a single character
        if short_name.chars().count() != 1 {
            return Err(anyhow::anyhow!("Asset type short name must be a single character"));
        }
        
        // Create new asset type
        let asset_type = AssetType {
            short_name: short_name.to_string(),
            description: description.to_string(),
            last_modified: Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        };
        
        // Add to asset types map
        self.asset_types.insert(short_name.to_string(), asset_type);
        
        // Update last modified timestamp
        self.last_modified = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        
        Ok(())
    }
    
    /// Add or update an entity
    pub fn add_entity(&mut self, short_name: &str, description: &str) -> Result<()> {
        // Create new entity
        let entity = Entity {
            short_name: short_name.to_string(),
            description: description.to_string(),
            last_modified: Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        };
        
        // Add to entities map
        self.entities.insert(short_name.to_string(), entity);
        
        // Update last modified timestamp
        self.last_modified = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        
        Ok(())
    }
    
    /// Add or update an asset reference
    pub fn add_asset_reference(
        &mut self, 
        short_name: &str, 
        entity: &str, 
        asset_type: &str, 
        path: &str
    ) -> Result<()> {
        // Validate entity exists
        if !self.entities.contains_key(entity) {
            return Err(anyhow::anyhow!("Entity {} does not exist", entity));
        }
        
        // Validate asset type exists
        if !self.asset_types.contains_key(asset_type) {
            return Err(anyhow::anyhow!("Asset type {} does not exist", asset_type));
        }
        
        // Create new asset reference
        let asset_reference = AssetReference {
            short_name: short_name.to_string(),
            entity: entity.to_string(),
            asset_type: asset_type.to_string(),
            path: path.to_string(),
            last_modified: Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
            exemplar: false,
            principle_reviews: HashMap::new(),
            arch_rating: None,
            sec_rating: None,
            perf_rating: None,
            ui_rating: None,
            arch_analysis: None,
            sec_analysis: None,
            perf_analysis: None,
            ui_analysis: None,
        };
        
        // Add to asset references map
        self.asset_references.insert(short_name.to_string(), asset_reference);
        
        // Update last modified timestamp
        self.last_modified = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        
        Ok(())
    }
    
    /// Set review data for an asset
    pub fn set_asset_review(
        &mut self,
        asset_name: &str,
        aspect: &str,
        rating: &str,
        analysis: &str
    ) -> Result<()> {
        // Find the asset
        let asset = match self.asset_references.get_mut(asset_name) {
            Some(asset) => asset,
            None => return Err(anyhow::anyhow!("Asset {} not found", asset_name)),
        };
        
        // Validate rating
        if !["H", "M", "L"].contains(&rating) {
            return Err(anyhow::anyhow!("Invalid rating: {}. Must be H, M, or L", rating));
        }
        
        // Update the appropriate fields based on aspect
        match aspect.to_lowercase().as_str() {
            "arch" => {
                asset.arch_rating = Some(rating.to_string());
                asset.arch_analysis = Some(analysis.to_string());
            },
            "sec" => {
                asset.sec_rating = Some(rating.to_string());
                asset.sec_analysis = Some(analysis.to_string());
            },
            "perf" => {
                asset.perf_rating = Some(rating.to_string());
                asset.perf_analysis = Some(analysis.to_string());
            },
            "ui" => {
                asset.ui_rating = Some(rating.to_string());
                asset.ui_analysis = Some(analysis.to_string());
            },
            _ => return Err(anyhow::anyhow!("Invalid aspect: {}. Must be arch, sec, perf, or ui", aspect)),
        }
        
        // Update asset last modified
        asset.last_modified = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        
        // Update storage last modified
        self.last_modified = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        
        Ok(())
    }
    
    /// Set exemplar status for an asset
    pub fn set_asset_exemplar(&mut self, asset_name: &str, status: bool) -> Result<()> {
        // Find the asset
        let asset = match self.asset_references.get_mut(asset_name) {
            Some(asset) => asset,
            None => return Err(anyhow::anyhow!("Asset {} not found", asset_name)),
        };
        
        // Update exemplar status
        asset.exemplar = status;
        
        // Update asset last modified
        asset.last_modified = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        
        // Update storage last modified
        self.last_modified = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        
        Ok(())
    }
    
    /// Add or update a principle
    pub fn add_principle(&mut self, short_name: &str, long_name: &str, guidance: Option<&str>) -> Result<()> {
        // Validate short name (single character)
        if short_name.chars().count() != 1 {
            return Err(anyhow::anyhow!("Principle short name must be a single character"));
        }
        
        // Create new principle
        let principle = Principle {
            short_name: short_name.to_string(),
            long_name: long_name.to_string(),
            guidance: guidance.map(|g| g.to_string()),
            last_modified: Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        };
        
        // Add to principles map
        self.principles.insert(short_name.to_string(), principle);
        
        // Update last modified timestamp
        self.last_modified = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        
        Ok(())
    }
    
    /// Store review data for an asset with a specific principle
    pub fn store_asset_review(
        &mut self,
        asset_name: &str,
        principle: &str,
        rating: Option<&str>,
        analysis: &str
    ) -> Result<()> {
        // Find the asset
        let asset = match self.asset_references.get_mut(asset_name) {
            Some(asset) => asset,
            None => return Err(anyhow::anyhow!("Asset {} not found", asset_name)),
        };
        
        // Validate principle exists
        if !self.principles.contains_key(principle) {
            return Err(anyhow::anyhow!("Principle {} does not exist", principle));
        }
        
        // Validate rating if provided
        if let Some(r) = rating {
            if !["H", "M", "L"].contains(&r) {
                return Err(anyhow::anyhow!("Invalid rating: {}. Must be H, M, or L", r));
            }
        }
        
        // Create review
        let review = Review {
            rating: rating.map(|r| r.to_string()),
            analysis: Some(analysis.to_string()),
            last_modified: Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        };
        
        // Add review to asset
        asset.principle_reviews.insert(principle.to_string(), review);
        
        // Update asset last modified
        asset.last_modified = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        
        // Update storage last modified
        self.last_modified = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        
        // For backward compatibility, also update the legacy fields
        match principle {
            "a" => {
                asset.arch_rating = rating.map(|r| r.to_string());
                asset.arch_analysis = Some(analysis.to_string());
            },
            "s" => {
                asset.sec_rating = rating.map(|r| r.to_string());
                asset.sec_analysis = Some(analysis.to_string());
            },
            "p" => {
                asset.perf_rating = rating.map(|r| r.to_string());
                asset.perf_analysis = Some(analysis.to_string());
            },
            "u" => {
                asset.ui_rating = rating.map(|r| r.to_string());
                asset.ui_analysis = Some(analysis.to_string());
            },
            _ => {
                // For other principles, don't update legacy fields
            }
        }
        
        Ok(())
    }
    
    /// Get review for an asset from a specific principle
    pub fn get_asset_review(&self, asset_name: &str, principle: &str) -> Result<Option<&Review>> {
        // Find the asset
        let asset = match self.asset_references.get(asset_name) {
            Some(asset) => asset,
            None => return Err(anyhow::anyhow!("Asset {} not found", asset_name)),
        };
        
        // Return the review if it exists
        Ok(asset.principle_reviews.get(principle))
    }
    
    /// Get all reviews for an asset
    pub fn get_asset_reviews(&self, asset_name: &str) -> Result<&HashMap<String, Review>> {
        // Find the asset
        let asset = match self.asset_references.get(asset_name) {
            Some(asset) => asset,
            None => return Err(anyhow::anyhow!("Asset {} not found", asset_name)),
        };
        
        // Return all reviews
        Ok(&asset.principle_reviews)
    }
}

/// Helper function to find the VQL JSON storage file in the current directory or ancestors
pub fn find_vql_storage() -> Result<(PathBuf, JsonStorage)> {
    let current_dir = std::env::current_dir()
        .context("Failed to get current directory")?;
    
    let mut search_dir = current_dir.clone();
    
    loop {
        // Check if VQL directory exists in this directory
        let vql_dir = search_dir.join("VQL");
        
        if vql_dir.exists() && vql_dir.is_dir() {
            // Check if storage file exists
            let storage_path = vql_dir.join("vql_storage.json");
            
            if storage_path.exists() {
                // Load the storage
                let content = fs::read_to_string(&storage_path)
                    .context(format!("Failed to read VQL storage at {}", storage_path.display()))?;
                    
                let storage: JsonStorage = serde_json::from_str(&content)
                    .context("Failed to parse VQL JSON storage")?;
                    
                return Ok((vql_dir, storage));
            }
        }
        
        // Move up to parent directory
        if !search_dir.pop() {
            // No more parent directories
            break;
        }
    }
    
    // If we get here, no VQL directory was found
    Err(anyhow::anyhow!("VQL directory not found in current directory or ancestors"))
}