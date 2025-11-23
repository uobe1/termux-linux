use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use crate::utils::fs::get_home_dir;
use crate::config::defaults::get_default_mirror;

pub struct ConfigManager {
    config_dir: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let home = get_home_dir()?;
        let config_dir = home.join("termos");
        
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?;
        }
        
        let config_path = config_dir.join("config");
        if !config_path.exists() {
            let default_config = r#"# 镜像源配置
ubuntu-mirror = https://mirrors.ustc.edu.cn/ubuntu/
debian-mirror = https://mirrors.163.com/debian/
kali-mirror = http://http.kali.org/kali/
centos-mirror = https://mirrors.aliyun.com/centos/
fedora-mirror = https://mirrors.tuna.tsinghua.edu.cn/fedora/

# 自定义下载链接配置（可选）
# ubuntu-link = https://custom-mirror.com/ubuntu-rootfs-arm64.tar.xz
# debian-link = https://custom-mirror.com/debian-rootfs-arm64.tar.xz
# kali-link = https://custom-mirror.com/kali-rootfs-arm64.tar.xz
# centos-link = https://custom-mirror.com/centos-rootfs-arm64.tar.xz
# fedora-link = https://custom-mirror.com/fedora-rootfs-arm64.tar.xz

# Shell 配置（可选）
# 自定义登录 shell 命令，默认为 /bin/bash --login
# shell = /bin/zsh --login

# 自定义初始化命令（可选，支持多行格式）
# ubuntu-init = ---
# apt update
# apt install -y vim curl wget
# ---
# debian-init = ---
# apt update
# apt install -y build-essential git
# ---
"#;
            fs::write(&config_path, default_config)?;
            println!("已创建默认配置文件: {:?}", config_path);
        }
        
        Ok(Self { config_dir })
    }
    
    pub fn load_config(&self) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let mut config = HashMap::new();
        let config_path = self.config_dir.join("config");
        
        let config_path = if config_path.exists() {
            config_path
        } else {
            PathBuf::from("config")
        };
        
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let mut lines = content.lines().peekable();
            
            while let Some(line) = lines.next() {
                let trimmed = line.trim();
                
                if trimmed.is_empty() || trimmed.starts_with('#') {
                    continue;
                }
                
                if let Some((key, value)) = trimmed.split_once('=') {
                    let key = key.trim().to_string();
                    let value = value.trim();
                    
                    if value.starts_with("---") && !value.ends_with("---") {
                        let mut multi_line_value = String::new();
                        if value.len() > 3 {
                            multi_line_value.push_str(&value[3..]);
                            multi_line_value.push('\n');
                        }
                        
                        while let Some(next_line) = lines.next() {
                            if next_line.trim().ends_with("---") {
                                if next_line.len() > 3 {
                                    multi_line_value.push_str(&next_line[..next_line.len()-3]);
                                }
                                break;
                            } else {
                                multi_line_value.push_str(next_line);
                                multi_line_value.push('\n');
                            }
                        }
                        
                        config.insert(key, multi_line_value.trim().to_string());
                    } else if value.starts_with("---") && value.ends_with("---") && value.len() >= 6 {
                        let clean_value = &value[3..value.len()-3];
                        config.insert(key, clean_value.trim().to_string());
                    } else {
                        config.insert(key, value.to_string());
                    }
                }
            }
        }
        
        Ok(config)
    }
    
    pub fn get_mirror_for_distro(&self, distro_name: &str) -> Result<String, Box<dyn std::error::Error>> {
        let config = self.load_config()?;
        let mirror_key = format!("{}-mirror", distro_name.to_lowercase());
        
        if let Some(mirror) = config.get(&mirror_key) {
            Ok(mirror.clone())
        } else {
            get_default_mirror(distro_name)
        }
    }
    
    pub fn get_download_link_for_distro(&self, distro_name: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let config = self.load_config()?;
        let link_key = format!("{}-link", distro_name.to_lowercase());
        
        Ok(config.get(&link_key).cloned())
    }
    
    pub fn get_shell_command(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let config = self.load_config()?;
        Ok(config.get("shell").cloned())
    }
    
    pub fn get_init_commands_for_distro(&self, distro_name: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let config = self.load_config()?;
        let init_key = format!("{}-init", distro_name.to_lowercase());
        Ok(config.get(&init_key).cloned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

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
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config");
        
        let mut file = File::create(&config_path).unwrap();
        writeln!(file, "ubuntu-mirror = https://test.ubuntu.com/").unwrap();
        writeln!(file, "debian-mirror = https://test.debian.com/").unwrap();
        
        let config_manager = ConfigManager {
            config_dir: temp_dir.path().to_path_buf(),
        };
        
        let result = config_manager.load_config();
        assert!(result.is_ok());
        
        let config = result.unwrap();
        assert_eq!(config.get("ubuntu-mirror"), Some(&"https://test.ubuntu.com/".to_string()));
        assert_eq!(config.get("debian-mirror"), Some(&"https://test.debian.com/".to_string()));
    }

    #[test]
    fn test_get_mirror_for_distro_with_config() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config");
        
        let mut file = File::create(&config_path).unwrap();
        writeln!(file, "ubuntu-mirror = https://custom.ubuntu.com/").unwrap();
        
        let config_manager = ConfigManager {
            config_dir: temp_dir.path().to_path_buf(),
        };
        
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
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config");
        
        let mut file = File::create(&config_path).unwrap();
        writeln!(file, "ubuntu-link = https://custom.com/ubuntu.tar.xz").unwrap();
        
        let config_manager = ConfigManager {
            config_dir: temp_dir.path().to_path_buf(),
        };
        
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
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config");
        
        let mut file = File::create(&config_path).unwrap();
        writeln!(file, "shell = /bin/zsh --login").unwrap();
        
        let config_manager = ConfigManager {
            config_dir: temp_dir.path().to_path_buf(),
        };
        
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
