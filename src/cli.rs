use std::io::{self, Write};
use std::env;
use std::process::Command;

use crate::distro::{LinuxDistro, DistroType};
use crate::utils::{get_system_metas, get_installed_systems};
use crate::ui::{display_system_list, print_section, print_item, print_info, print_success, print_error};
use crate::installer::install_interactive;
use crate::system::uninstall_system_by_id;

pub fn run_cli() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        return handle_command_line_args(&args);
    }
    
    check_and_install_screenfetch()?;
    
    loop {
        display_logo();
        let installed_systems = get_installed_systems()?;
        let choice = get_user_choice()?;
        
        match choice {
            1 => install_interactive()?,
            2 => uninstall_interactive(&installed_systems)?,
            3 => {
                let metas = get_system_metas()?;
                display_system_list(&metas)?;
            }
            4 => {
                println!("\n  退出程序\n");
                std::process::exit(0);
            }
            _ => println!("\n  不合法的输入选项\n"),
        }
        
        println!("\n  按 Enter 键继续...");
        let _ = io::stdin().read_line(&mut String::new());
    }
}

fn display_logo() {
    println!(r#"
  _____                              
 |_   _|__ _ __ _ __ ___  _   ___  __
   | |/ _ \ '__| '_ ` _ \| | | \/ /
   | |  __/ |  | | | | | | |_| |>  < 
   |_|\___|_|  |_| |_| |_|\__,_|_/\_\
  
  Termux Linux 安装器
  
  1. 安装系统         2. 卸载系统
  3. 查询已安装系统   4. 退出程序
  
"#);
}

fn get_user_choice() -> Result<i32, std::num::ParseIntError> {
    print!("  请选择要执行的操作: ");
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
                println!("\n  错误: 请指定要安装的发行版\n");
                println!("  用法: {} --install <发行版> [选项]\n", args[0]);
                return Ok(());
            }
            
            let distro_type = match args[2].to_lowercase().as_str() {
                "ubuntu" => DistroType::Ubuntu,
                "kali" => DistroType::Kali,
                "debian" => DistroType::Debian,
                "centos" => DistroType::CentOS,
                "fedora" => DistroType::Fedora,
                _ => {
                    println!("\n  错误: 不支持的发行版\n");
                    return Ok(());
                }
            };
            
            let mut name = None;
            let mut _minimal = false;
            
            for i in 3..args.len() {
                match args[i].as_str() {
                    "--name" => {
                        if i + 1 < args.len() {
                            name = Some(args[i + 1].clone());
                        }
                    }
                    "--minimal" => {
                        _minimal = true;
                    }
                    _ => {}
                }
            }
            
            let distro = if let Some(name) = name {
                LinuxDistro::with_name(distro_type, name)
            } else {
                LinuxDistro::new(distro_type)
            };
            
            print_info(&format!("正在安装 {}", distro_type));
            distro.install()?;
            print_success("安装完成");
        }
        "--uninstall" => {
            if args.len() < 3 {
                println!("\n  错误: 请指定要卸载的系统ID\n");
                println!("  用法: {} --uninstall <系统ID>\n", args[0]);
                return Ok(());
            }
            
            print_info(&format!("正在卸载 {}", args[2]));
            uninstall_system_by_id(&args[2])?;
            print_success("卸载完成");
        }
        "--help" => {
            display_help();
        }
        _ => {
            println!("\n  未知参数: {}\n", args[1]);
            display_help();
        }
    }
    
    Ok(())
}

fn display_help() {
    let program_name = env::args().next().unwrap_or_else(|| "termux-linux-install".to_string());
    
    println!(r#"
  用法:
    {}                    # 交互式界面
    {} --list             # 列出已安装系统
    {} --install <distro> # 安装指定发行版
    {} --uninstall <id>   # 卸载指定系统
    {} --help             # 显示帮助
  
  支持的发行版: ubuntu, kali, debian, centos, fedora
  
  安装选项:
    --name <名称>        # 自定义系统名称
    --minimal           # 最小化安装
  
"#, program_name, program_name, program_name, program_name, program_name);
}

fn uninstall_interactive(installed_systems: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if installed_systems.is_empty() {
        println!("\n  暂没有安装系统哦\n");
        return Ok(());
    }
    
    print_section("选择要卸载的系统");
    
    for (i, system) in installed_systems.iter().enumerate() {
        print_item(&format!("{}.", i + 1), system);
    }
    
    print!("\n  请选择: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    if let Ok(choice) = input.trim().parse::<usize>() {
        if choice > 0 && choice <= installed_systems.len() {
            let system_id = &installed_systems[choice - 1];
            print_info(&format!("正在卸载 {}", system_id));
            uninstall_system_by_id(system_id)?;
            print_success("卸载完成");
        } else {
            println!("\n  不合法的选择\n");
        }
    } else {
        println!("\n  不合法的输入\n");
    }
    
    Ok(())
}

fn check_and_install_screenfetch() -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("pkg")
        .args(&["list-installed"])
        .output()?;
    
    if !String::from_utf8_lossy(&output.stdout).contains("screenfetch") {
        print_info("正在安装相关依赖包: screenfetch");
        Command::new("pkg")
            .args(&["install", "screenfetch", "-y"])
            .spawn()?
            .wait()?;
    }
    
    Ok(())
}
