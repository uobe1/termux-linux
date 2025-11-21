use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use crate::distro::SystemMeta;

pub fn get_home_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    match env::var("HOME") {
        Ok(home) => Ok(PathBuf::from(home)),
        Err(_) => Err("无法获取 HOME 目录".into()),
    }
}

pub fn get_installed_systems() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let home = get_home_dir()?;
    let ostermux_dir = home.join("termos");
    
    if !ostermux_dir.exists() {
        fs::create_dir_all(&ostermux_dir)?;
        return Ok(Vec::new());
    }
    
    let mut systems = Vec::new();
    
    for entry in fs::read_dir(ostermux_dir)? {
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

pub fn get_system_metas() -> Result<Vec<(String, SystemMeta)>, Box<dyn std::error::Error>> {
    let systems = get_installed_systems()?;
    let mut metas = Vec::new();
    
    for system_id in systems {
        let home = get_home_dir()?;
        let meta_path = home.join("termos").join(&system_id).join("meta.txt");
        
        if meta_path.exists() {
            let content = fs::read_to_string(&meta_path)?;
            if let Ok(meta) = SystemMeta::from_string(&content) {
                metas.push((system_id, meta));
            }
        }
    }
    
    Ok(metas)
}

pub fn run_command(command: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("bash")
        .arg("-c")
        .arg(command)
        .output()?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Err(format!("命令执行失败: {}\nstdout: {}\nstderr: {}", command, stdout, stderr).into());
    }
    
    Ok(())
}