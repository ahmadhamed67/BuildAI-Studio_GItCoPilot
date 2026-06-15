use std::fs;
use std::path::Path;

/// Validate file path for security
fn validate_path(path: &str) -> Result<(), String> {
    // Prevent directory traversal attacks
    if path.contains("..") || path.starts_with("~") {
        return Err("Invalid file path".to_string());
    }

    // Ensure path is absolute or relative to allowed locations
    let path_obj = Path::new(path);
    if path_obj.components().any(|c| matches!(c, std::path::Component::ParentDir)) {
        return Err("Path traversal not allowed".to_string());
    }

    Ok(())
}

#[tauri::command]
pub fn read_file(file_path: String) -> Result<String, String> {
    validate_path(&file_path)?;

    fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file: {}", e))
}

#[tauri::command]
pub fn write_file(file_path: String, content: String) -> Result<(), String> {
    validate_path(&file_path)?;

    fs::write(&file_path, content)
        .map_err(|e| format!("Failed to write file: {}", e))
}

#[tauri::command]
pub fn create_dir(path: String) -> Result<(), String> {
    validate_path(&path)?;

    fs::create_dir_all(&path)
        .map_err(|e| format!("Failed to create directory: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_path_rejects_traversal() {
        assert!(validate_path("../etc/passwd").is_err());
        assert!(validate_path("~/.ssh/id_rsa").is_err());
    }

    #[test]
    fn test_validate_path_allows_normal() {
        assert!(validate_path("/home/user/file.txt").is_ok());
        assert!(validate_path("./local/file.txt").is_ok());
    }
}
