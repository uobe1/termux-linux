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
        
        // 确保配置目录存在
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?;
        }
        
        // 如果config文件不存在，创建默认配置
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
            for line in content.lines() {
                if let Some((key, value)) = line.split_once('=') {
                    let key = key.trim().to_string();
                    let value = value.trim().to_string();
                    config.insert(key, value);
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
    
    pub fn get_download_link_for_distro(&self, distro_name: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let config = self.load_config()?;
        let link_key = format!("{}-link", distro_name.to_lowercase());
        
        if let Some(link) = config.get(&link_key) {
            Ok(Some(link.clone()))
        } else {
            Ok(None)
        }
    }
}