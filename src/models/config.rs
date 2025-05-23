use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VqlConfig {
    #[serde(default)]
    pub asset_paths: AssetPaths,
    #[serde(default = "default_ignore_patterns")]
    pub ignore_patterns: Vec<String>,
    #[serde(default)]
    pub llm_integration: LlmIntegration,
}

impl Default for VqlConfig {
    fn default() -> Self {
        Self {
            asset_paths: AssetPaths::default(),
            ignore_patterns: default_ignore_patterns(),
            llm_integration: LlmIntegration::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetPaths {
    #[serde(default = "default_models_path")]
    pub models: String,
    #[serde(default = "default_controllers_path")]
    pub controllers: String,
    #[serde(default = "default_ui_path")]
    pub ui: String,
}

impl Default for AssetPaths {
    fn default() -> Self {
        Self {
            models: default_models_path(),
            controllers: default_controllers_path(),
            ui: default_ui_path(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmIntegration {
    #[serde(default = "default_true")]
    pub enable_command_interception: bool,
    #[serde(default = "default_true")]
    pub optimize_for_tokens: bool,
    #[serde(default = "default_true")]
    pub context_preservation: bool,
    #[serde(default = "default_true")]
    pub indicators_auto_include: bool,
    #[serde(default = "default_detail_level")]
    pub default_detail_level: String,
    #[serde(default = "default_cache_strategy")]
    pub cache_strategy: String,
    #[serde(default = "default_token_budget")]
    pub token_budget: usize,
    #[serde(default = "default_session_timeout")]
    pub session_timeout: u64,
    #[serde(default = "default_true")]
    pub command_batching_enabled: bool,
}

impl Default for LlmIntegration {
    fn default() -> Self {
        Self {
            enable_command_interception: default_true(),
            optimize_for_tokens: default_true(),
            context_preservation: default_true(),
            indicators_auto_include: default_true(),
            default_detail_level: default_detail_level(),
            cache_strategy: default_cache_strategy(),
            token_budget: default_token_budget(),
            session_timeout: default_session_timeout(),
            command_batching_enabled: default_true(),
        }
    }
}

fn default_true() -> bool {
    true
}

fn default_models_path() -> String {
    "./server/models".to_string()
}

fn default_controllers_path() -> String {
    "./server/controllers".to_string()
}

fn default_ui_path() -> String {
    "./client/src/components".to_string()
}

fn default_ignore_patterns() -> Vec<String> {
    vec!["**/node_modules/**".to_string(), "**/dist/**".to_string()]
}

fn default_detail_level() -> String {
    "standard".to_string()
}

fn default_cache_strategy() -> String {
    "aggressive".to_string()
}

fn default_token_budget() -> usize {
    2000
}

fn default_session_timeout() -> u64 {
    3600
}