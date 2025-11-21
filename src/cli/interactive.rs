use std::io;
use std::io::Write;
use crate::utils::get_installed_systems;
use crate::ui::{print_section, print_item, print_info, print_success};

pub fn display_logo() {
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

pub fn get_user_choice() -> Result<i32, std::num::ParseIntError> {
    print!("  请选择要执行的操作: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    input.trim().parse()
}

pub fn uninstall_interactive(installed_systems: &[String]) -> Result<(), Box<dyn std::error::Error>> {
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
            crate::system::uninstall_system_by_id(system_id)?;
            print_success("卸载完成");
        } else {
            println!("\n  不合法的选择\n");
        }
    } else {
        println!("\n  不合法的输入\n");
    }
    
    Ok(())
}