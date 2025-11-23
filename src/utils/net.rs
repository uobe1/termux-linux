pub use super::net_core::*;

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn test_check_network_connectivity() {
        let result = check_network_connectivity();
        assert!(result);
    }

    #[test]
    fn test_download_file() {
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("test_file.txt");
        
        let result = download_file("https://www.google.com/robots.txt", output_path.to_str().unwrap());
        assert!(result.is_ok());
        assert!(output_path.exists());
        
        let content = fs::read_to_string(&output_path).unwrap();
        assert!(!content.is_empty());
    }

    #[test]
    fn test_download_file_invalid_url() {
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("test_file.txt");
        
        let result = download_file("https://invalid-url-12345.com/file.txt", output_path.to_str().unwrap());
        assert!(result.is_err());
    }
}