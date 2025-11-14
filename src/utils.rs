use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::collections::HashMap;
use crate::distro::SystemMeta;

pub fn get_home_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    match env::var("HOME") {
        Ok(home) => Ok(PathBuf::from(home)),
        Err(_) => Err("无法获取 HOME 目录".into()),
    }
}

pub fn get_installed_systems() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let home = get_home_dir()?;
    let ostermux_dir = home.join("Ostermux");
    
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
        let meta_path = home.join("Ostermux").join(&system_id).join("meta.txt");
        
        if meta_path.exists() {
            let content = fs::read_to_string(&meta_path)?;
            if let Ok(meta) = SystemMeta::from_string(&content) {
                metas.push((system_id, meta));
            }
        }
    }
    
    Ok(metas)
}

pub fn get_terminal_width() -> usize {
    // 简单实现：使用环境变量或默认值
    match std::env::var("COLUMNS") {
        Ok(width_str) => width_str.parse().unwrap_or(80),
        Err(_) => 80, // 默认宽度
    }
}

pub fn display_system_list(metas: &[(String, SystemMeta)]) -> Result<(), Box<dyn std::error::Error>> {
    if metas.is_empty() {
        println!("暂没有安装系统哦 赶紧来体验一下吧");
        return Ok(());
    }
    
    let width = get_terminal_width();
    let box_width = if width > 70 { width - 4 } else { 70 };
    
    println!("\n已安装系统:\n");
    
    for (system_id, meta) in metas {
        // 顶部边框
        println!("┌{}┐", "─".repeat(box_width - 2));
        
        // 系统名称 (亮字)
        let name_line = format!("│ {} ", meta.name);
        println!("{}{}│", name_line, " ".repeat(box_width - name_line.len() - 1));
        
        // 系统ID和创建时间
        let date = meta.created_at.split('T').next().unwrap_or("未知");
        let id_time_line = format!("│ {} {} ", system_id, date);
        println!("{}{}│", id_time_line, " ".repeat(box_width - id_time_line.len() - 1));
        
        // 用户组和权限
        let user_perm_line = format!("│ 用户组: {} 权限: {} ", meta.user_group, meta.permissions);
        println!("{}{}│", user_perm_line, " ".repeat(box_width - user_perm_line.len() - 1));
        
        // 底部边框
        println!("└{}┘", "─".repeat(box_width - 2));
        println!();
    }
    
    Ok(())
}

pub fn load_mirror_config() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut mirrors = HashMap::new();
    let config_path = get_home_dir()?.join("Ostermux").join("mirrors.conf");
    
    // 如果配置文件不存在，尝试从当前目录读取
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

pub fn get_mirror_for_distro(distro_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mirrors = load_mirror_config()?;
    let mirror_key = format!("{}-mirror", distro_name.to_lowercase());
    
    if let Some(mirror) = mirrors.get(&mirror_key) {
        Ok(mirror.clone())
    } else {
        // 返回默认镜像源
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