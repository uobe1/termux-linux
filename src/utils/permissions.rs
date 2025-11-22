use std::process::Command;
use std::path::Path;
use std::fs;
use std::os::unix::fs::MetadataExt;

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
        .split_whitespace()
        .map(String::from)
        .collect();
    
    Ok(groups)
}

pub fn check_write_permission(path: &Path) -> bool {
    if !path.exists() {
        if let Some(parent) = path.parent() {
            return check_write_permission(parent);
        }
        return false;
    }
    
    match fs::metadata(path) {
        Ok(metadata) => {
            use std::os::unix::fs::PermissionsExt;
            let mode = metadata.permissions().mode();
            
            let is_owner = metadata.uid() == get_current_uid().unwrap_or(0);
            let is_group = get_user_groups().unwrap_or_default().contains(&get_group_name(metadata.gid()).unwrap_or_default());
            
            if is_owner {
                (mode & 0o200) != 0
            } else if is_group {
                (mode & 0o020) != 0
            } else {
                (mode & 0o002) != 0
            }
        }
        Err(_) => false,
    }
}

pub fn check_read_permission(path: &Path) -> bool {
    if !path.exists() {
        return false;
    }
    
    match fs::metadata(path) {
        Ok(metadata) => {
            use std::os::unix::fs::PermissionsExt;
            let mode = metadata.permissions().mode();
            
            let is_owner = metadata.uid() == get_current_uid().unwrap_or(0);
            let is_group = get_user_groups().unwrap_or_default().contains(&get_group_name(metadata.gid()).unwrap_or_default());
            
            if is_owner {
                (mode & 0o400) != 0
            } else if is_group {
                (mode & 0o040) != 0
            } else {
                (mode & 0o004) != 0
            }
        }
        Err(_) => false,
    }
}

pub fn ensure_dir_exists(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

pub fn get_current_uid() -> Result<u32, Box<dyn std::error::Error>> {
    let output = Command::new("id")
        .arg("-u")
        .output()
        .map_err(|e| format!("无法执行 id -u: {}", e))?;
    
    if !output.status.success() {
        return Err("id -u 命令执行失败".into());
    }
    
    let uid_str = String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string();
    
    let uid = uid_str.parse::<u32>()
        .map_err(|e| format!("无法解析 UID: {}", e))?;
    
    Ok(uid)
}

pub fn get_group_name(gid: u32) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("getent")
        .args(["group", &gid.to_string()])
        .output()
        .map_err(|e| format!("无法执行 getent group: {}", e))?;
    
    if !output.status.success() {
        return Err(format!("无法获取组 {} 的信息", gid).into());
    }
    
    let group_info = String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string();
    
    if let Some(group_name) = group_info.split(':').next() {
        Ok(group_name.to_string())
    } else {
        Err(format!("无法解析组信息: {}", group_info).into())
    }
}

pub fn check_execute_permission(path: &Path) -> bool {
    if !path.exists() {
        return false;
    }
    
    match fs::metadata(path) {
        Ok(metadata) => {
            use std::os::unix::fs::PermissionsExt;
            let mode = metadata.permissions().mode();
            
            let is_owner = metadata.uid() == get_current_uid().unwrap_or(0);
            let is_group = get_user_groups().unwrap_or_default().contains(&get_group_name(metadata.gid()).unwrap_or_default());
            
            if is_owner {
                (mode & 0o100) != 0
            } else if is_group {
                (mode & 0o010) != 0
            } else {
                (mode & 0o001) != 0
            }
        }
        Err(_) => false,
    }
}

pub fn is_root_user() -> bool {
    match get_current_uid() {
        Ok(uid) => uid == 0,
        Err(_) => false,
    }
}
