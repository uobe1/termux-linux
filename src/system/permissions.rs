use std::process::Command;
use std::path::Path;

pub fn get_current_user() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("whoami")
        .output()
        .map_err(|e| format!("无法执行 whoami: {}", e))?;
    
    if !output.status.success() {
        return Err("whoami 命令执行失败".into());
    }
    
    let user = String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string();
    
    Ok(user)
}

pub fn get_user_groups() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let output = Command::new("groups")
        .output()
        .map_err(|e| format!("无法执行 groups: {}", e))?;
    
    if !output.status.success() {
        return Err("groups 命令执行失败".into());
    }
    
    let groups_str = String::from_utf8_lossy(&output.stdout);
    let groups: Vec<String> = groups_str
        .trim()
        .split_whitespace()
        .map(String::from)
        .collect();
    
    Ok(groups)
}

pub fn check_write_permission(path: &Path) -> bool {
    match path.metadata() {
        Ok(metadata) => {
            use std::os::unix::fs::PermissionsExt;
            let mode = metadata.permissions().mode();
            (mode & 0o222) != 0
        }
        Err(_) => false,
    }
}

pub fn check_read_permission(path: &Path) -> bool {
    path.exists() && path.is_dir()
}

pub fn ensure_dir_exists(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}
