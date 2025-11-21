use std::io::{self, Write};
use crate::distro::{LinuxDistro, DistroType};
use crate::ui::{print_section, print_item, print_info, print_success};

pub fn install_interactive() -> Result<(), Box<dyn std::error::Error>> {
    print_section("选择 Linux 发行版");
    print_item("1.", "Ubuntu");
    print_item("2.", "Kali");
    print_item("3.", "Debian");
    print_item("4.", "CentOS");
    print_item("5.", "Fedora");
    
    print!("\n请输入数字以选择: ");
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
            println!("\n不合法的选择");
            return Ok(());
        }
    };
    
    print!("\n请输入系统名称 (留空使用默认): ");
    io::stdout().flush().unwrap();
    
    let mut name_input = String::new();
    io::stdin().read_line(&mut name_input).unwrap();
    let name = if name_input.trim().is_empty() { None } else { Some(name_input.trim().to_string()) };
    
    print_section("选择安装模式");
    print_item("1.", "最小化安装");
    print_item("2.", "标准安装");
    print_item("3.", "自定义安装");
    
    print!("\n请选择: ");
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
            print_info("开始最小化安装...");
            distro.install()?;
        }
        Ok(2) | Ok(3) => {
            let distro = if let Some(name) = name {
                LinuxDistro::with_name(distro_type, name)
            } else {
                LinuxDistro::new(distro_type)
            };
            print_info("开始标准安装...");
            distro.install()?;
        }
        _ => {
            println!("\n不合法的选择！");
            return Ok(());
        }
    }
    
    print_success("安装完成！");
    
    Ok(())
}