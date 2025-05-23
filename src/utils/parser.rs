use regex::Regex;
use std::collections::HashMap;
use crate::models::asset::AssetData;
use anyhow::{Result, Context};

/// Parse an asset from file content
pub fn parse_asset(content: &str, asset_ref: &str) -> Result<AssetData> {
    // Normalize asset reference to ensure it has + prefix
    let normalized_ref = if asset_ref.starts_with('+') {
        asset_ref.to_string()
    } else {
        format!("+{}", asset_ref)
    };
    
    // Regular expression to extract asset data with platform-aware line endings
    let pattern = format!(r"ASSET:{}\s*\|\s*(.*?)(?:\r\n|\n)((?:.|\n)*?)---", regex::escape(&normalized_ref));
    let re = Regex::new(&pattern)
        .context("Failed to compile regex pattern")?;
    
    let captures = re.captures(content)
        .context(format!("Asset {} not found in the content", normalized_ref))?;
    
    let mut result = AssetData {
        header: HashMap::new(),
        analysis: HashMap::new(),
    };
    
    // Parse header
    if let Some(header_text) = captures.get(1) {
        for part in header_text.as_str().split(" | ") {
            if let Some((key, value)) = part.split_once(':') {
                result.header.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
    }
    
    // Parse analysis sections with platform-aware line ending handling
    if let Some(analysis_text) = captures.get(2) {
        let text = analysis_text.as_str();
        
        // Architecture analysis
        if let Some(arch_match) = Regex::new(r"ARCH_ANALYSIS:(.*?)(?:SEC_ANALYSIS|PERF_ANALYSIS|$)")
            .ok()
            .and_then(|re| re.captures(text))
            .and_then(|cap| cap.get(1)) {
            result.analysis.insert("arch".to_string(), arch_match.as_str().trim().to_string());
        }
        
        // Security analysis
        if let Some(sec_match) = Regex::new(r"SEC_ANALYSIS:(.*?)(?:ARCH_ANALYSIS|PERF_ANALYSIS|$)")
            .ok()
            .and_then(|re| re.captures(text))
            .and_then(|cap| cap.get(1)) {
            result.analysis.insert("sec".to_string(), sec_match.as_str().trim().to_string());
        }
        
        // Performance analysis
        if let Some(perf_match) = Regex::new(r"PERF_ANALYSIS:(.*?)(?:ARCH_ANALYSIS|SEC_ANALYSIS|$)")
            .ok()
            .and_then(|re| re.captures(text))
            .and_then(|cap| cap.get(1)) {
            result.analysis.insert("perf".to_string(), perf_match.as_str().trim().to_string());
        }
    }
    
    Ok(result)
}

/// Create a new asset entry string
pub fn create_asset_entry(
    asset_ref: &str, 
    timestamp: &str, 
    exemplar: &str, 
    arch: &str, 
    sec: &str, 
    perf: &str
) -> String {
    format!(
        "ASSET:{} | LAST_UPDATE:{} | EXEMPLAR:{} | ARCH:{} | SEC:{} | PERF:{}\n\
        ARCH_ANALYSIS:Awaiting initial review.\n\
        SEC_ANALYSIS:Awaiting initial review.\n\
        PERF_ANALYSIS:Awaiting initial review.\n\
        ---",
        asset_ref, timestamp, exemplar, arch, sec, perf
    )
}

/// Update an asset entry in the file content
pub fn update_asset_entry(
    content: &str,
    asset_ref: &str,
    aspect: &str,
    rating: &str,
    analysis: &str
) -> Result<String> {
    // Normalize asset reference to ensure it has + prefix
    let normalized_ref = if asset_ref.starts_with('+') {
        asset_ref.to_string()
    } else {
        format!("+{}", asset_ref)
    };
    
    // Update header rating
    let header_pattern = format!(r"(ASSET:{}\s*\|.*?{}:)[^\s|]*", regex::escape(&normalized_ref), aspect.to_uppercase());
    let re = Regex::new(&header_pattern)
        .context("Failed to compile header regex pattern")?;
    
    let updated_content = re.replace(content, format!("$1{}", rating)).to_string();
    
    // Update analysis text - using a simpler regex approach without lookahead
    let analysis_key = format!("{}_ANALYSIS:", aspect.to_uppercase());
    let updated_content = match updated_content.find(&analysis_key) {
        Some(start_idx) => {
            // Find the end of this analysis section
            let section_start = start_idx + analysis_key.len();
            let section_end = find_analysis_section_end(&updated_content[section_start..]);
            
            // Replace just this section
            format!(
                "{}{}{}",
                &updated_content[..section_start],
                analysis,
                &updated_content[section_start + section_end..],
            )
        },
        None => updated_content // No change if section not found
    };
    
    Ok(updated_content)
}

/// Helper function to find the end of an analysis section
fn find_analysis_section_end(text: &str) -> usize {
    // Look for the next analysis section or end marker
    let next_arch = text.find("ARCH_ANALYSIS:");
    let next_sec = text.find("SEC_ANALYSIS:");
    let next_perf = text.find("PERF_ANALYSIS:");
    let end_marker = text.find("---");
    
    // Get the position of the nearest next section
    let positions = [next_arch, next_sec, next_perf, end_marker];
    let valid_positions: Vec<usize> = positions.iter()
        .filter_map(|&pos| pos)
        .collect();
    
    // If we found any section marker, return the position of the nearest one
    if !valid_positions.is_empty() {
        *valid_positions.iter().min().unwrap()
    } else {
        // If no markers found, return the end of the string
        text.len()
    }
}