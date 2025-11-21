use std::fs;
use std::path::PathBuf;

use crate::utils::*;

pub fn uninstall_system_by_id(system_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n正在卸载 {}", system_id);
    
    let home = get_home_dir()?;
    let system_dir = home.join("termos").join(system_id);
    
    if system_dir.exists() {
        run_command(&format!("chmod 777 -R {}", system_dir.display()))?;
        run_command(&format!("rm -rf {}", system_dir.display()))?;
        println!("\n卸载完成！");
    } else {
        println!("系统 {} 不存在！", system_id);
    }
    
    Ok(())
}

#[allow(dead_code)]
pub struct SystemManager {
    base_dir: PathBuf,
}

impl SystemManager {
    #[allow(dead_code)]
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let home = get_home_dir()?;
        let base_dir = home.join("termos");
        Ok(Self { base_dir })
    }
    
    #[allow(dead_code)]
    pub fn get_installed_systems(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        if !self.base_dir.exists() {
            fs::create_dir_all(&self.base_dir)?;
            return Ok(Vec::new());
        }
        
        let mut systems = Vec::new();
        
        for entry in fs::read_dir(&self.base_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                if let Some(name) = path.file_name() {
                    if let Some(name_str) = name.to_str() {
                        systems.push(name_str.to_string());
                    }
                }
            }
        }
        
        Ok(systems)
    }
    
    #[allow(dead_code)]
    pub fn get_system_dir(&self, system_id: &str) -> PathBuf {
        self.base_dir.join(system_id)
    }
    
    #[allow(dead_code)]
    pub fn system_exists(&self, system_id: &str) -> bool {
        self.get_system_dir(system_id).exists()
    }
}