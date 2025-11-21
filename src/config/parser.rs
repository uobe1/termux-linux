use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use crate::utils::fs::get_home_dir;

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
            get_default_mirror(distro_name)
        }
    }
    
    pub fn get_download_link_for_distro(&self, distro_name: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let config = self.load_config()?;
        let link_key = format!("{}-link", distro_name.to_lowercase());
        
        Ok(config.get(&link_key).cloned())
    }
}
