use std::io::{self, Write};
use crate::distro::{LinuxDistro, DistroType};

pub fn install_interactive() -> Result<(), Box<dyn std::error::Error>> {
    println!("请选择要安装的Linux发行版:");
    println!(" 1. Ubuntu");
    println!(" 2. Kali");
    println!(" 3. Debian");
    println!(" 4. CentOS");
    println!(" 5. Fedora");
    
    print!("请输入数字以选择: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let distro_type = match input.trim().parse::<i32>() {
        Ok(1) => DistroType::Ubuntu,
        Ok(2) => DistroType::Kali,
        Ok(3) => DistroType::Debian,
        Ok(4) => DistroType::CentOS,
        Ok(5) => DistroType::Fedora,
        _ => {
            println!("不合法的选择");
            return Ok(());
        }
    };
    
    print!("请输入系统名称 (留空使用默认): ");
    io::stdout().flush().unwrap();
    
    let mut name_input = String::new();
    io::stdin().read_line(&mut name_input).unwrap();
    let name = if name_input.trim().is_empty() { None } else { Some(name_input.trim().to_string()) };
    
    println!("请选择安装模式:");
    println!(" 1. 最小化安装");
    println!(" 2. 标准安装");
    println!(" 3. 自定义安装");
    
    print!("请选择: ");
    io::stdout().flush().unwrap();
    
    let mut mode_input = String::new();
    io::stdin().read_line(&mut mode_input).unwrap();
    
    match mode_input.trim().parse::<i32>() {
        Ok(1) => {
            let distro = if let Some(name) = name {
                LinuxDistro::with_name(distro_type, name)
            } else {
                LinuxDistro::new(distro_type)
            };
            distro.install()?;
        }
        Ok(2) | Ok(3) => {
            let distro = if let Some(name) = name {
                LinuxDistro::with_name(distro_type, name)
            } else {
                LinuxDistro::new(distro_type)
            };
            distro.install()?;
        }
        _ => {
            println!("不合法的选择！");
            return Ok(());
        }
    }
    
    Ok(())
}