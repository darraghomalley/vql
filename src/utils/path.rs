use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use std::env;
use anyhow::{Result, Context, anyhow};

/// VQL Path Resolver for handling project-relative paths
pub struct PathResolver {
    /// The root directory of the VQL workspace
    workspace_root: PathBuf,
}

impl PathResolver {
    /// Create a new PathResolver with the current working directory as workspace root
    pub fn new() -> Result<Self> {
        let workspace_root = env::current_dir()
            .context("Failed to get current directory")?;
        Ok(Self { workspace_root })
    }
    
    /// Create a new PathResolver with a specific workspace root
    pub fn with_root(workspace_root: PathBuf) -> Self {
        Self { workspace_root }
    }
    
    /// Convert an absolute path to a project-relative path
    pub fn to_project_relative(&self, absolute_path: &str) -> Result<String> {
        let abs_path = Path::new(absolute_path);
        
        // If already project-relative, just normalize and return
        if abs_path.is_relative() {
            return Ok(self.normalize(absolute_path));
        }
        
        // Convert to project-relative
        let relative = abs_path.strip_prefix(&self.workspace_root)
            .context(format!(
                "Path {} is not within workspace root {}", 
                absolute_path, 
                self.workspace_root.display()
            ))?;
        
        // Convert to string with forward slashes
        Ok(self.normalize(&relative.to_string_lossy()))
    }
    
    /// Convert a project-relative path to an absolute path
    pub fn to_absolute(&self, project_relative_path: &str) -> Result<PathBuf> {
        let rel_path = Path::new(project_relative_path);
        
        // If already absolute, just return it
        if rel_path.is_absolute() {
            return Ok(rel_path.to_path_buf());
        }
        
        // Join with workspace root
        Ok(self.workspace_root.join(rel_path))
    }
    
    /// Normalize a path to always use forward slashes
    pub fn normalize(&self, path: &str) -> String {
        // Replace platform-specific separators with forward slashes
        if MAIN_SEPARATOR == '\\' {
            path.replace('\\', "/")
        } else {
            path.to_string()
        }
    }
    
    /// Validate that a path exists within the workspace boundary
    pub fn validate_workspace_boundary(&self, path: &str) -> Result<bool> {
        let abs_path = if Path::new(path).is_absolute() {
            PathBuf::from(path)
        } else {
            self.to_absolute(path)?
        };
        
        // Canonicalize the path to resolve .. and . components
        let canonical_path = abs_path.canonicalize()
            .context("Failed to canonicalize path for validation")?;
        
        // Canonicalize workspace root too for consistent comparison
        let canonical_workspace = self.workspace_root.canonicalize()
            .context("Failed to canonicalize workspace root")?;
        
        // Check if the canonical path starts with canonical workspace root
        Ok(canonical_path.starts_with(&canonical_workspace))
    }
    
    /// Get the workspace root
    pub fn workspace_root(&self) -> &Path {
        &self.workspace_root
    }
}

/// Find the VQL workspace root by looking for VQL directory
/// The workspace root is always the parent of the VQL directory
pub fn find_workspace_root() -> Result<PathBuf> {
    let mut current_dir = env::current_dir()
        .context("Failed to get current directory")?;
    
    loop {
        // Check for VQL directory (case insensitive)
        let vql_lower = current_dir.join("vql");
        let vql_upper = current_dir.join("VQL");
        
        if vql_lower.is_dir() || vql_upper.is_dir() {
            // Return the parent of VQL directory as workspace root
            return Ok(current_dir);
        }
        
        // Move up to parent directory
        if !current_dir.pop() {
            // No VQL directory found, return error
            return Err(anyhow::anyhow!(
                "VQL directory not found. Please run 'vql -su' to initialize VQL first."
            ));
        }
    }
}

/// Find the VQL directory from current location
pub fn find_vql_directory() -> Result<PathBuf> {
    let workspace_root = find_workspace_root()?;
    
    // Check for both cases
    let vql_lower = workspace_root.join("vql");
    let vql_upper = workspace_root.join("VQL");
    
    if vql_upper.is_dir() {
        Ok(vql_upper)
    } else if vql_lower.is_dir() {
        Ok(vql_lower)
    } else {
        Err(anyhow::anyhow!("VQL directory not found"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    
    #[test]
    fn test_normalize_paths() {
        let resolver = PathResolver::with_root(PathBuf::from("/home/user/project"));
        
        // Test forward slashes remain unchanged
        assert_eq!(resolver.normalize("src/lib.rs"), "src/lib.rs");
        
        // Test backslashes are converted
        assert_eq!(resolver.normalize("src\\lib.rs"), "src/lib.rs");
    }
    
    #[test]
    fn test_project_relative_conversion() {
        let resolver = PathResolver::with_root(PathBuf::from("/home/user/project"));
        
        // Test absolute to project-relative
        let result = resolver.to_project_relative("/home/user/project/src/lib.rs").unwrap();
        assert_eq!(result, "src/lib.rs");
        
        // Test already project-relative
        let result = resolver.to_project_relative("src/lib.rs").unwrap();
        assert_eq!(result, "src/lib.rs");
    }
    
    #[test]
    fn test_absolute_conversion() {
        let resolver = PathResolver::with_root(PathBuf::from("/home/user/project"));
        
        // Test project-relative to absolute
        let result = resolver.to_absolute("src/lib.rs").unwrap();
        assert_eq!(result, PathBuf::from("/home/user/project/src/lib.rs"));
        
        // Test already absolute
        let result = resolver.to_absolute("/home/user/project/src/lib.rs").unwrap();
        assert_eq!(result, PathBuf::from("/home/user/project/src/lib.rs"));
    }
}