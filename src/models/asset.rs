use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetData {
    pub header: HashMap<String, String>,
    pub analysis: HashMap<String, String>,
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