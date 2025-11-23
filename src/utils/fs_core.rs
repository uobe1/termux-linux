use std::env;
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use crate::distro::SystemMeta;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

pub fn get_home_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    match env::var("HOME") {
        Ok(home) => Ok(PathBuf::from(home)),
        Err(_) => Err("无法获取 HOME 目录".into()),
    }
}

pub fn get_installed_systems() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let home = get_home_dir()?;
    let termos_dir = home.join("termos");
    
    if !termos_dir.exists() {
        fs::create_dir_all(&termos_dir)?;
        return Ok(Vec::new());
    }
    
    let mut systems = Vec::new();
    
    for entry in fs::read_dir(&termos_dir)? {
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

#[allow(dead_code)]
pub fn ensure_dir(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

#[allow(dead_code)]
pub fn read_config_file(path: &PathBuf) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut config = HashMap::new();
    
    if path.exists() {
        let content = fs::read_to_string(path)?;
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

pub fn extract_tar_xz_with_progress<F>(
    archive_path: &PathBuf,
    extract_dir: &PathBuf,
    mut progress_callback: F,
) -> Result<u64, Box<dyn std::error::Error>>
where
    F: FnMut(u64, &str),
{
    let _total_files = count_files_in_tar_xz(archive_path)?;
    
    let mut child = Command::new("tar")
        .args(&["-xJf", archive_path.to_str().unwrap(), "-C", extract_dir.to_str().unwrap(), "--verbose"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    
    let stdout = child.stdout.take().ok_or("Failed to capture stdout")?;
    let reader = BufReader::new(stdout);
    let mut extracted_files = 0u64;
    
    for line_result in reader.lines() {
        let line = line_result?;
        let file_name = line.trim().to_string();
        
        extracted_files += 1;
        progress_callback(extracted_files, &file_name);
    }
    
    let status = child.wait()?;
    if !status.success() {
        return Err("解压失败".into());
    }
    
    Ok(extracted_files)
}

pub fn count_files_in_tar_xz(archive_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let output = Command::new("tar")
        .args(&["-tf", archive_path.to_str().unwrap()])
        .output()?;
    
    if !output.status.success() {
        return Ok(100);
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let file_count = stdout.lines().count() as u64;
    
    Ok(if file_count > 0 { file_count } else { 100 })
}

pub fn extract_tar_xz(archive_path: &PathBuf, extract_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("tar")
        .args(&["-xJf", archive_path.to_str().unwrap(), "-C", extract_dir.to_str().unwrap()])
        .status()?;
    
    if !status.success() {
        return Err("解压失败".into());
    }
    
    Ok(())
}