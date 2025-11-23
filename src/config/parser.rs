use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use crate::utils::fs::get_home_dir;
use crate::config::defaults::get_default_mirror;
use crate::config::default_config::get_default_config_content;

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
            let default_config = get_default_config_content();
            fs::write(&config_path, default_config)?;
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
mod parser_tests_core;
#[cfg(test)]
mod parser_tests_init;
