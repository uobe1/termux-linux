pub use super::fs_core::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_get_home_dir() {
        let result = get_home_dir();
        assert!(result.is_ok());
        let home = result.unwrap();
        assert!(home.is_absolute());
    }

    #[test]
    fn test_ensure_dir() {
        let temp_dir = TempDir::new().unwrap();
        let test_path = temp_dir.path().join("test_subdir");
        
        assert!(!test_path.exists());
        
        let result = ensure_dir(&test_path.to_path_buf());
        assert!(result.is_ok());
        assert!(test_path.exists());
        assert!(test_path.is_dir());
    }

    #[test]
    fn test_ensure_dir_already_exists() {
        let temp_dir = TempDir::new().unwrap();
        let test_path = temp_dir.path().to_path_buf();
        
        assert!(test_path.exists());
        
        let result = ensure_dir(&test_path);
        assert!(result.is_ok());
        assert!(test_path.exists());
    }

    #[test]
    fn test_read_config_file() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_config");
        
        let mut file = File::create(&config_path).unwrap();
        writeln!(file, "key1 = value1").unwrap();
        writeln!(file, "key2 = value2").unwrap();
        writeln!(file, "# comment").unwrap();
        writeln!(file, "key3 = value3 with spaces").unwrap();
        
        let result = read_config_file(&config_path.to_path_buf());
        assert!(result.is_ok());
        
        let config = result.unwrap();
        assert_eq!(config.get("key1"), Some(&"value1".to_string()));
        assert_eq!(config.get("key2"), Some(&"value2".to_string()));
        assert_eq!(config.get("key3"), Some(&"value3 with spaces".to_string()));
        assert_eq!(config.len(), 3);
    }

    #[test]
    fn test_read_config_file_not_exists() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("nonexistent_config");
        
        let result = read_config_file(&config_path.to_path_buf());
        assert!(result.is_ok());
        
        let config = result.unwrap();
        assert!(config.is_empty());
    }

    #[test]
    fn test_read_config_file_empty() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("empty_config");
        
        File::create(&config_path).unwrap();
        
        let result = read_config_file(&config_path.to_path_buf());
        assert!(result.is_ok());
        
        let config = result.unwrap();
        assert!(config.is_empty());
    }
}