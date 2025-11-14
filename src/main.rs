use std::io::{self, Write};
use std::process::Command;
use std::env;

mod distro;
mod utils;

use distro::{LinuxDistro, DistroType};
use utils::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 检查命令行参数
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        return handle_command_line_args(&args);
    }
    
    // 检查并安装 screenfetch
    check_and_install_screenfetch()?;
    
    // 显示主菜单
    loop {
        display_logo();
        
        // 获取已安装的系统
        let installed_systems = get_installed_systems()?;
        
        let choice = get_user_choice()?;
        
        match choice {
            1 => interactive_install()?,
            2 => interactive_uninstall(&installed_systems)?,
            3 => {
                let metas = get_system_metas()?;
                display_system_list(&metas)?;
            }
            4 => {
                println!("退出脚本");
                std::process::exit(0);
            }
            _ => println!("不合法的输入选项"),
        }
        
        println!("\n按 Enter 键继续...");
        let _ = io::stdin().read_line(&mut String::new());
    }
}

fn display_logo() {
    println!(r#"  _____                              
 |_   _|__ _ __ _ __ ___  _   ___  __
   | |/ _ \ '__| '_ ` _ \| | | \/ /
   | |  __/ |  | | | | | | |_| |>  < 
   |_|\___|_|  |_| |_| |_|\__,_|_/\_\"#);
    println!();
    println!();
    println!(" 1. 安装系统         2. 卸载系统");
    println!(" 3. 查询已安装系统   4. 退出脚本");
    println!();
}

fn get_user_choice() -> Result<i32, std::num::ParseIntError> {
    print!("\n请选择要执行的操作: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    input.trim().parse()
}



fn handle_command_line_args(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    match args[1].as_str() {
        "--list" => {
            let metas = get_system_metas()?;
            display_system_list(&metas)?;
        }
        "--install" => {
            if args.len() < 3 {
                println!("错误: 请指定要安装的发行版");
                println!("用法: {} --install <发行版> [选项]", args[0]);
                return Ok(());
            }
            
            let distro_type = match args[2].to_lowercase().as_str() {
                "ubuntu" => DistroType::Ubuntu,
                "kali" => DistroType::Kali,
                "debian" => DistroType::Debian,
                "centos" => DistroType::CentOS,
                "fedora" => DistroType::Fedora,
                _ => {
                    println!("错误: 不支持的发行版");
                    return Ok(());
                }
            };
            
            let mut name = None;
            let mut minimal = false;
            
            // 简单的参数解析
            for i in 3..args.len() {
                match args[i].as_str() {
                    "--name" => {
                        if i + 1 < args.len() {
                            name = Some(args[i + 1].clone());
                        }
                    }
                    "--minimal" => {
                        minimal = true;
                    }
                    _ => {}
                }
            }
            
            let distro = if let Some(name) = name {
                LinuxDistro::with_name(distro_type, name)
            } else {
                LinuxDistro::new(distro_type)
            };
            distro.install()?;
        }
        "--uninstall" => {
            if args.len() < 3 {
                println!("错误: 请指定要卸载的系统ID");
                println!("用法: {} --uninstall <系统ID>", args[0]);
                return Ok(());
            }
            
            uninstall_system_by_id(&args[2])?;
        }
        "--help" => {
            display_help();
        }
        _ => {
            println!("未知参数: {}", args[1]);
            display_help();
        }
    }
    
    Ok(())
}

fn display_help() {
    println!("用法:");
    println!("  {}                    # 交互式界面", env::args().next().unwrap_or_else(|| "termux-linux-install".to_string()));
    println!("  {} --list             # 列出已安装系统", env::args().next().unwrap_or_else(|| "termux-linux-install".to_string()));
    println!("  {} --install <distro> # 安装指定发行版", env::args().next().unwrap_or_else(|| "termux-linux-install".to_string()));
    println!("  {} --uninstall <id>   # 卸载指定系统", env::args().next().unwrap_or_else(|| "termux-linux-install".to_string()));
    println!("  {} --help             # 显示帮助", env::args().next().unwrap_or_else(|| "termux-linux-install".to_string()));
    println!();
    println!("支持的发行版: ubuntu, kali, debian, centos, fedora");
    println!();
    println!("安装选项:");
    println!("  --name <名称>        # 自定义系统名称");
    println!("  --minimal           # 最小化安装");
}

fn interactive_install() -> Result<(), Box<dyn std::error::Error>> {
    println!("请选择要安装的Linux发行版:");
    println!(" 1. Ubuntu");
    println!(" 2. Kali");
    println!(" 3. Debian");
    println!(" 4. CentOS");
    println!(" 5. Fedora");
    
    print!("请选择: ");
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
            // 最小化安装
            let distro = if let Some(name) = name {
                LinuxDistro::with_name(distro_type, name)
            } else {
                LinuxDistro::new(distro_type)
            };
            distro.install()?;
        }
        Ok(2) | Ok(3) => {
            // 标准安装或自定义安装
            let distro = if let Some(name) = name {
                LinuxDistro::with_name(distro_type, name)
            } else {
                LinuxDistro::new(distro_type)
            };
            distro.install()?;
        }
        _ => {
            println!("不合法的选择");
            return Ok(());
        }
    }
    
    Ok(())
}

fn interactive_uninstall(installed_systems: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if installed_systems.is_empty() {
        println!("暂没有安装系统哦");
        return Ok(());
    }
    
    println!("请选择要卸载的系统:");
    for (i, system) in installed_systems.iter().enumerate() {
        println!(" {}. {}", i + 1, system);
    }
    
    print!("请选择: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    if let Ok(choice) = input.trim().parse::<usize>() {
        if choice > 0 && choice <= installed_systems.len() {
            let system_id = &installed_systems[choice - 1];
            uninstall_system_by_id(system_id)?;
        } else {
            println!("不合法的选择");
        }
    } else {
        println!("不合法的输入");
    }
    
    Ok(())
}

fn uninstall_system_by_id(system_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n正在卸载 {} 请耐心等待", system_id);
    
    let home = get_home_dir()?;
    let system_dir = home.join("Termux-Linux").join(system_id);
    
    if system_dir.exists() {
        run_command(&format!("chmod 777 -R {}", system_dir.display()))?;
        run_command(&format!("rm -rf {}", system_dir.display()))?;
        println!("\n卸载完成!");
    } else {
        println!("系统 {} 不存在", system_id);
    }
    
    Ok(())
}

fn check_and_install_screenfetch() -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("pkg")
        .args(&["list-installed"])
        .output()?;
    
    if !String::from_utf8_lossy(&output.stdout).contains("screenfetch") {
        println!("正在安装相关依赖包: screenfetch");
        Command::new("pkg")
            .args(&["install", "screenfetch", "-y"])
            .spawn()?
            .wait()?;
    }
    
    Ok(())
}