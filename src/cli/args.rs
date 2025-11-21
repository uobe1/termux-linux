use std::env;
use crate::distro::{LinuxDistro, DistroType};
use crate::system::uninstall_system_by_id;
use crate::ui::{print_info, print_success};

pub fn handle_command_line_args(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    match args[1].as_str() {
        "--list" => {
            let metas = crate::utils::get_system_metas()?;
            crate::ui::display_system_list(&metas)?;
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
    let program_name = env::args().next().unwrap_or_else(|| "insOs".to_string());
    
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