#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_get_init_commands_single_line() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config");
        
        let mut file = File::create(&config_path).unwrap();
        writeln!(file, "ubuntu-init = apt update && apt install -y vim").unwrap();
        
        let config_manager = ConfigManager {
            config_dir: temp_dir.path().to_path_buf(),
        };
        
        let result = config_manager.get_init_commands_for_distro("ubuntu");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some("apt update && apt install -y vim".to_string()));
    }

    #[test]
    fn test_get_init_commands_multi_line() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config");
        
        let mut file = File::create(&config_path).unwrap();
        writeln!(file, "ubuntu-init = ---").unwrap();
        writeln!(file, "apt update").unwrap();
        writeln!(file, "apt install -y vim curl wget").unwrap();
        writeln!(file, "apt install -y build-essential git").unwrap();
        writeln!(file, "---").unwrap();
        
        let config_manager = ConfigManager {
            config_dir: temp_dir.path().to_path_buf(),
        };
        
        let result = config_manager.get_init_commands_for_distro("ubuntu");
        assert!(result.is_ok());
        let commands = result.unwrap();
        assert!(commands.is_some());
        
        let command_str = commands.unwrap();
        assert!(command_str.contains("apt update"));
        assert!(command_str.contains("apt install -y vim curl wget"));
        assert!(command_str.contains("apt install -y build-essential git"));
    }

    #[test]
    fn test_get_init_commands_multi_line_with_empty_lines() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config");
        
        let mut file = File::create(&config_path).unwrap();
        writeln!(file, "debian-init = ---").unwrap();
        writeln!(file, "apt update").unwrap();
        writeln!(file, "").unwrap();
        writeln!(file, "apt install -y python3 python3-pip").unwrap();
        writeln!(file, "").unwrap();
        writeln!(file, "pip3 install --upgrade pip").unwrap();
        writeln!(file, "---").unwrap();
        
        let config_manager = ConfigManager {
            config_dir: temp_dir.path().to_path_buf(),
        };
        
        let result = config_manager.get_init_commands_for_distro("debian");
        assert!(result.is_ok());
        let commands = result.unwrap();
        assert!(commands.is_some());
        
        let command_str = commands.unwrap();
        assert!(command_str.contains("apt update"));
        assert!(command_str.contains("apt install -y python3 python3-pip"));
        assert!(command_str.contains("pip3 install --upgrade pip"));
    }

    #[test]
    fn test_get_init_commands_without_config() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config");
        File::create(&config_path).unwrap();
        
        let config_manager = ConfigManager {
            config_dir: temp_dir.path().to_path_buf(),
        };
        
        let result = config_manager.get_init_commands_for_distro("ubuntu");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_load_config_with_mixed_settings() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config");
        
        let mut file = File::create(&config_path).unwrap();
        writeln!(file, "ubuntu-mirror = https://custom.ubuntu.com/").unwrap();
        writeln!(file, "ubuntu-init = ---").unwrap();
        writeln!(file, "apt update").unwrap();
        writeln!(file, "apt install -y vim").unwrap();
        writeln!(file, "---").unwrap();
        writeln!(file, "shell = /bin/bash --login").unwrap();
        
        let config_manager = ConfigManager {
            config_dir: temp_dir.path().to_path_buf(),
        };
        
        let result = config_manager.load_config();
        assert!(result.is_ok());
        
        let config = result.unwrap();
        assert_eq!(config.get("ubuntu-mirror"), Some(&"https://custom.ubuntu.com/".to_string()));
        assert_eq!(config.get("shell"), Some(&"/bin/bash --login".to_string()));
        
        let init_commands = config.get("ubuntu-init");
        assert!(init_commands.is_some());
        let init_str = init_commands.unwrap();
        assert!(init_str.contains("apt update"));
        assert!(init_str.contains("apt install -y vim"));
    }
}
