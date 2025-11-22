use std::process::Command;

pub fn check_network_connectivity() -> bool {
    Command::new("ping")
        .args(["-c", "1", "-W", "5", "8.8.8.8"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub fn download_file(url: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("wget")
        .args(["-O", output_path, url])
        .status()?;
    
    if !status.success() {
        return Err(format!("下载失败: {}", url).into());
    }
    
    Ok(())
}

#[allow(dead_code)]
pub fn check_url_reachable(url: &str) -> bool {
    Command::new("curl")
        .args(["-I", "--connect-timeout", "5", url])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

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
    fn test_check_url_reachable() {
        let result = check_url_reachable("https://www.google.com");
        assert!(result);
    }

    #[test]
    fn test_check_url_reachable_false() {
        let result = check_url_reachable("https://nonexistent-domain-12345.com");
        assert!(!result);
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
