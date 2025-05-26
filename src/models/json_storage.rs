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
    /// Creates a new empty JsonStorage structure
    pub fn new() -> Self {
        let now = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        
        JsonStorage {
            version: "1.0.0".to_string(),
            created: now.clone(),
            last_modified: now,
            commands: HashMap::new(),
            asset_types: HashMap::new(),
            entities: HashMap::new(),
            principles: HashMap::new(),
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
    
    /// Find what type an item is by its name
    pub fn find_item_type(&self, name: &str) -> Option<&'static str> {
        if self.principles.contains_key(name) {
            Some("principle")
        } else if self.entities.contains_key(name) {
            Some("entity")
        } else if self.asset_types.contains_key(name) {
            Some("asset_type")
        } else if self.asset_references.contains_key(name) {
            Some("asset")
        } else {
            None
        }
    }
    
    /// Check if a name is available across all user-defined types
    pub fn check_name_availability(&self, name: &str) -> Result<()> {
        // Check principles
        if let Some(principle) = self.principles.get(name) {
            return Err(anyhow::anyhow!(
                "Short name '{}' already in use by principle '{}'", 
                name, 
                principle.long_name
            ));
        }
        
        // Check entities
        if let Some(entity) = self.entities.get(name) {
            return Err(anyhow::anyhow!(
                "Short name '{}' already in use by entity '{}'", 
                name, 
                entity.description
            ));
        }
        
        // Check asset types
        if let Some(asset_type) = self.asset_types.get(name) {
            return Err(anyhow::anyhow!(
                "Short name '{}' already in use by asset type '{}'", 
                name, 
                asset_type.description
            ));
        }
        
        // Check asset references
        if let Some(asset_ref) = self.asset_references.get(name) {
            return Err(anyhow::anyhow!(
                "Short name '{}' already in use by asset '{}'", 
                name, 
                asset_ref.path
            ));
        }
        
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
        
        // Check name availability across all types
        self.check_name_availability(short_name)?;
        
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
        // Check name availability across all types
        self.check_name_availability(short_name)?;
        
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
        // Check name availability across all types
        self.check_name_availability(short_name)?;
        
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
        
        // Check name availability across all types
        self.check_name_availability(short_name)?;
        
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
    
    /// Rename a principle
    pub fn rename_principle(&mut self, old_name: &str, new_name: &str) -> Result<()> {
        // Check if old principle exists
        if !self.principles.contains_key(old_name) {
            return Err(anyhow::anyhow!("Principle '{}' not found", old_name));
        }
        
        // Check if new name is available
        self.check_name_availability(new_name)?;
        
        // Get the principle data
        let principle = self.principles.remove(old_name)
            .ok_or_else(|| anyhow::anyhow!("Failed to remove principle"))?;
        
        // Update principle with new name
        let mut updated_principle = principle;
        updated_principle.short_name = new_name.to_string();
        updated_principle.last_modified = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        
        // Insert with new name
        self.principles.insert(new_name.to_string(), updated_principle);
        
        // Cascade: Update principle keys in all asset reviews
        for asset in self.asset_references.values_mut() {
            if let Some(review) = asset.principle_reviews.remove(old_name) {
                asset.principle_reviews.insert(new_name.to_string(), review);
            }
        }
        
        // Update storage last modified
        self.last_modified = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        
        Ok(())
    }
    
    /// Rename an entity
    pub fn rename_entity(&mut self, old_name: &str, new_name: &str) -> Result<()> {
        // Check if old entity exists
        if !self.entities.contains_key(old_name) {
            return Err(anyhow::anyhow!("Entity '{}' not found", old_name));
        }
        
        // Check if new name is available
        self.check_name_availability(new_name)?;
        
        // Get the entity data
        let entity = self.entities.remove(old_name)
            .ok_or_else(|| anyhow::anyhow!("Failed to remove entity"))?;
        
        // Update entity with new name
        let mut updated_entity = entity;
        updated_entity.short_name = new_name.to_string();
        updated_entity.last_modified = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        
        // Insert with new name
        self.entities.insert(new_name.to_string(), updated_entity);
        
        // Cascade: Update entity references in all assets
        for asset in self.asset_references.values_mut() {
            if asset.entity == old_name {
                asset.entity = new_name.to_string();
                asset.last_modified = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
            }
        }
        
        // Update storage last modified
        self.last_modified = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        
        Ok(())
    }
    
    /// Rename an asset type
    pub fn rename_asset_type(&mut self, old_name: &str, new_name: &str) -> Result<()> {
        // Check if old asset type exists
        if !self.asset_types.contains_key(old_name) {
            return Err(anyhow::anyhow!("Asset type '{}' not found", old_name));
        }
        
        // Check if new name is available
        self.check_name_availability(new_name)?;
        
        // Get the asset type data
        let asset_type = self.asset_types.remove(old_name)
            .ok_or_else(|| anyhow::anyhow!("Failed to remove asset type"))?;
        
        // Update asset type with new name
        let mut updated_asset_type = asset_type;
        updated_asset_type.short_name = new_name.to_string();
        updated_asset_type.last_modified = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        
        // Insert with new name
        self.asset_types.insert(new_name.to_string(), updated_asset_type);
        
        // Cascade: Update asset type references in all assets
        for asset in self.asset_references.values_mut() {
            if asset.asset_type == old_name {
                asset.asset_type = new_name.to_string();
                asset.last_modified = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
            }
        }
        
        // Update storage last modified
        self.last_modified = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        
        Ok(())
    }
    
    /// Rename an asset reference
    pub fn rename_asset_reference(&mut self, old_name: &str, new_name: &str) -> Result<()> {
        // Check if old asset exists
        if !self.asset_references.contains_key(old_name) {
            return Err(anyhow::anyhow!("Asset '{}' not found", old_name));
        }
        
        // Check if new name is available
        self.check_name_availability(new_name)?;
        
        // Get the asset data
        let asset = self.asset_references.remove(old_name)
            .ok_or_else(|| anyhow::anyhow!("Failed to remove asset"))?;
        
        // Update asset with new name
        let mut updated_asset = asset;
        updated_asset.short_name = new_name.to_string();
        updated_asset.last_modified = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        
        // Insert with new name
        self.asset_references.insert(new_name.to_string(), updated_asset);
        
        // Update storage last modified
        self.last_modified = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        
        Ok(())
    }
    
    /// Delete a principle (cascades to remove from all asset reviews)
    pub fn delete_principle(&mut self, name: &str) -> Result<()> {
        // Check if principle exists
        if !self.principles.contains_key(name) {
            return Err(anyhow::anyhow!("Principle '{}' not found", name));
        }
        
        // Remove principle
        self.principles.remove(name);
        
        // Cascade: Remove this principle from all asset reviews
        for asset in self.asset_references.values_mut() {
            asset.principle_reviews.remove(name);
        }
        
        // Update storage last modified
        self.last_modified = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        
        Ok(())
    }
    
    /// Delete an entity (blocks if assets exist)
    pub fn delete_entity(&mut self, name: &str) -> Result<()> {
        // Check if entity exists
        if !self.entities.contains_key(name) {
            return Err(anyhow::anyhow!("Entity '{}' not found", name));
        }
        
        // Check if any assets use this entity
        let assets_using_entity: Vec<String> = self.asset_references
            .iter()
            .filter(|(_, asset)| asset.entity == name)
            .map(|(name, _)| name.clone())
            .collect();
            
        if !assets_using_entity.is_empty() {
            return Err(anyhow::anyhow!(
                "Cannot delete entity '{}' - it is used by {} asset(s): {}", 
                name,
                assets_using_entity.len(),
                assets_using_entity.join(", ")
            ));
        }
        
        // Safe to remove
        self.entities.remove(name);
        
        // Update storage last modified
        self.last_modified = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        
        Ok(())
    }
    
    /// Delete an asset type (blocks if assets exist)
    pub fn delete_asset_type(&mut self, name: &str) -> Result<()> {
        // Check if asset type exists
        if !self.asset_types.contains_key(name) {
            return Err(anyhow::anyhow!("Asset type '{}' not found", name));
        }
        
        // Check if any assets use this type
        let assets_using_type: Vec<String> = self.asset_references
            .iter()
            .filter(|(_, asset)| asset.asset_type == name)
            .map(|(name, _)| name.clone())
            .collect();
            
        if !assets_using_type.is_empty() {
            return Err(anyhow::anyhow!(
                "Cannot delete asset type '{}' - it is used by {} asset(s): {}", 
                name,
                assets_using_type.len(),
                assets_using_type.join(", ")
            ));
        }
        
        // Safe to remove
        self.asset_types.remove(name);
        
        // Update storage last modified
        self.last_modified = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        
        Ok(())
    }
    
    /// Delete an asset reference (removes all its reviews)
    pub fn delete_asset_reference(&mut self, name: &str) -> Result<()> {
        // Check if asset exists
        if !self.asset_references.contains_key(name) {
            return Err(anyhow::anyhow!("Asset '{}' not found", name));
        }
        
        // Remove the asset (reviews are removed with it)
        self.asset_references.remove(name);
        
        // Update storage last modified
        self.last_modified = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        
        Ok(())
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