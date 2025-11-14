use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;

use crate::utils::get_home_dir;

pub struct ConfigManager {
    config_dir: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let home = get_home_dir()?;
        let config_dir = home.join("Ostermux");
        Ok(Self { config_dir })
    }
    
    pub fn load_mirror_config(&self) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let mut mirrors = HashMap::new();
        let config_path = self.config_dir.join("mirrors.conf");
        
        let config_path = if config_path.exists() {
            config_path
        } else {
            PathBuf::from("mirrors.conf")
        };
        
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            for line in content.lines() {
                if let Some((key, value)) = line.split_once('=') {
                    let key = key.trim().to_string();
                    let value = value.trim().to_string();
                    mirrors.insert(key, value);
                }
            }
        }
        
        Ok(mirrors)
    }
    
    pub fn get_mirror_for_distro(&self, distro_name: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mirrors = self.load_mirror_config()?;
        let mirror_key = format!("{}-mirror", distro_name.to_lowercase());
        
        if let Some(mirror) = mirrors.get(&mirror_key) {
            Ok(mirror.clone())
        } else {
            match distro_name.to_lowercase().as_str() {
                "ubuntu" => Ok("https://mirrors.ustc.edu.cn/ubuntu/".to_string()),
                "debian" => Ok("https://mirrors.163.com/debian/".to_string()),
                "kali" => Ok("http://http.kali.org/kali/".to_string()),
                "centos" => Ok("https://mirrors.aliyun.com/centos/".to_string()),
                "fedora" => Ok("https://mirrors.tuna.tsinghua.edu.cn/fedora/".to_string()),
                _ => Err(format!("未找到 {} 的镜像源配置", distro_name).into()),
            }
        }
    }
}