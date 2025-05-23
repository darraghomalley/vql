use std::path::Path;

/// Check if a path is the filesystem root (platform aware)
pub fn is_filesystem_root(path: &Path) -> bool {
    if cfg!(windows) {
        // Windows: Check for paths like "C:\" or "D:\"
        path.to_string_lossy().ends_with(":\\") || path.to_string_lossy() == "\\"
    } else {
        // Unix: Root is "/"
        path.to_string_lossy() == "/"
    }
}

/// Check if the current platform is Windows
pub fn is_windows() -> bool {
    cfg!(target_os = "windows")
}

/// Normalize a path for the current platform
pub fn normalize_path(path: &str) -> String {
    if is_windows() {
        // Handle Windows paths
        path.replace("/", "\\")
    } else {
        // Unix paths
        path.to_string()
    }
}

/// Check if a file is hidden (platform aware)
pub fn is_hidden(path: &Path) -> bool {
    #[cfg(target_os = "windows")]
    {
        // On Windows, use attributes to check if hidden
        // This is a simplification; real implementation would use Windows API
        path.file_name()
            .and_then(|s| s.to_str())
            .map(|s| s.starts_with("."))
            .unwrap_or(false)
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        // On Unix, files starting with . are hidden
        path.file_name()
            .and_then(|s| s.to_str())
            .map(|s| s.starts_with("."))
            .unwrap_or(false)
    }
}

/// Check if color output is supported in the current terminal
pub fn supports_color() -> bool {
    // A basic check for color support
    // More sophisticated implementations would check environment variables
    // and terminal capabilities
    std::env::var("NO_COLOR").is_err() && atty::is(atty::Stream::Stdout)
}

/// Colorize output text based on the current platform's capabilities
pub fn colorize_output(text: &str, color_code: &str) -> String {
    if !supports_color() {
        return text.to_string();
    }
    
    format!("\x1b[{}m{}\x1b[0m", color_code, text)
}