use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetData {
    pub header: HashMap<String, String>,
    pub analysis: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssetType {
    Model,
    Controller,
    UI,
    Unknown,
}

impl AssetType {
    pub fn to_file_name(&self) -> &'static str {
        match self {
            AssetType::Model => "models",
            AssetType::Controller => "controllers",
            AssetType::UI => "ui",
            AssetType::Unknown => "unknown",
        }
    }
    
    pub fn from_short_name(short_name: &str) -> Self {
        let name = short_name.trim_start_matches('^');
        match name {
            "m" => AssetType::Model,
            "c" => AssetType::Controller,
            "u" => AssetType::UI,
            _ => AssetType::Unknown,
        }
    }
    
    pub fn to_short_name(&self) -> &'static str {
        match self {
            AssetType::Model => "^m",
            AssetType::Controller => "^c",
            AssetType::UI => "^u",
            AssetType::Unknown => "^?",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AspectType {
    Architecture,
    Security,
    Performance,
    All,
}

impl AspectType {
    pub fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "arch" => AspectType::Architecture,
            "sec" => AspectType::Security,
            "perf" => AspectType::Performance,
            "all" => AspectType::All,
            _ => AspectType::All,
        }
    }
    
    pub fn to_header_key(&self) -> &'static str {
        match self {
            AspectType::Architecture => "ARCH",
            AspectType::Security => "SEC",
            AspectType::Performance => "PERF",
            AspectType::All => "ALL",
        }
    }
    
    pub fn to_analysis_key(&self) -> &'static str {
        match self {
            AspectType::Architecture => "ARCH_ANALYSIS",
            AspectType::Security => "SEC_ANALYSIS",
            AspectType::Performance => "PERF_ANALYSIS",
            AspectType::All => "ALL_ANALYSIS",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Rating {
    High,
    Medium,
    Low,
    Unknown,
}

impl Rating {
    pub fn from_string(s: &str) -> Self {
        match s {
            "H" => Rating::High,
            "M" => Rating::Medium,
            "L" => Rating::Low,
            _ => Rating::Unknown,
        }
    }
    
    pub fn to_string(&self) -> &'static str {
        match self {
            Rating::High => "H",
            Rating::Medium => "M",
            Rating::Low => "L",
            Rating::Unknown => "?",
        }
    }
}