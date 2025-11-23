#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    fn create_temp_config(content: &[&str]) -> (TempDir, ConfigManager) {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config");
        let mut file = File::create(&config_path).unwrap();
        for line in content {
            writeln!(file, "{}", line).unwrap();
        }
        let config_manager = ConfigManager {
            config_dir: temp_dir.path().to_path_buf(),
        };
        (temp_dir, config_manager)
    }

    #[test]
    fn test_config_manager_new() {
        let result = ConfigManager::new();
        assert!(result.is_ok());
        
        let config_manager = result.unwrap();
        assert!(config_manager.config_dir.exists());
        assert!(config_manager.config_dir.is_dir());
        
        let config_file = config_manager.config_dir.join("config");
        assert!(config_file.exists());
    }

    #[test]
    fn test_load_config() {
        let (_temp_dir, config_manager) = create_temp_config(&[
            "ubuntu-mirror = https://test.ubuntu.com/",
            "debian-mirror = https://test.debian.com/",
        ]);
        
        let result = config_manager.load_config();
        assert!(result.is_ok());
        
        let config = result.unwrap();
        assert_eq!(config.get("ubuntu-mirror"), Some(&"https://test.ubuntu.com/".to_string()));
        assert_eq!(config.get("debian-mirror"), Some(&"https://test.debian.com/".to_string()));
    }

    #[test]
    fn test_get_mirror_for_distro_with_config() {
        let (_temp_dir, config_manager) = create_temp_config(&[
            "ubuntu-mirror = https://custom.ubuntu.com/",
        ]);
        
        let result = config_manager.get_mirror_for_distro("ubuntu");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://custom.ubuntu.com/");
    }

    #[test]
    fn test_get_mirror_for_distro_default() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config");
        File::create(&config_path).unwrap();
        
        let config_manager = ConfigManager {
            config_dir: temp_dir.path().to_path_buf(),
        };
        
        let result = config_manager.get_mirror_for_distro("ubuntu");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://mirrors.ustc.edu.cn/ubuntu/");
    }

    #[test]
    fn test_get_mirror_for_distro_invalid() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config");
        File::create(&config_path).unwrap();
        
        let config_manager = ConfigManager {
            config_dir: temp_dir.path().to_path_buf(),
        };
        
        let result = config_manager.get_mirror_for_distro("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_download_link_for_distro_with_link() {
        let (_temp_dir, config_manager) = create_temp_config(&[
            "ubuntu-link = https://custom.com/ubuntu.tar.xz",
        ]);
        
        let result = config_manager.get_download_link_for_distro("ubuntu");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some("https://custom.com/ubuntu.tar.xz".to_string()));
    }

    #[test]
    fn test_get_download_link_for_distro_without_link() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config");
        File::create(&config_path).unwrap();
        
        let config_manager = ConfigManager {
            config_dir: temp_dir.path().to_path_buf(),
        };
        
        let result = config_manager.get_download_link_for_distro("ubuntu");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_get_shell_command_with_config() {
        let (_temp_dir, config_manager) = create_temp_config(&[
            "shell = /bin/zsh --login",
        ]);
        
        let result = config_manager.get_shell_command();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some("/bin/zsh --login".to_string()));
    }

    #[test]
    fn test_get_shell_command_without_config() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config");
        File::create(&config_path).unwrap();
        
        let config_manager = ConfigManager {
            config_dir: temp_dir.path().to_path_buf(),
        };
        
        let result = config_manager.get_shell_command();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }
}