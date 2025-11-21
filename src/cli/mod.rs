pub mod args;
pub mod interactive;

pub use args::*;
pub use interactive::*;

use std::io;
use std::process::Command;
use crate::utils::get_system_metas;
use crate::installer::install_interactive;
use crate::system::uninstall_system_by_id;
use crate::ui::display_system_list;

pub fn run_cli() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 {
        return handle_command_line_args(&args);
    }
    
    check_and_install_screenfetch()?;
    
    loop {
        interactive::display_logo();
        let installed_systems = crate::utils::get_installed_systems()?;
        let choice = interactive::get_user_choice()?;
        
        match choice {
            1 => install_interactive()?,
            2 => interactive::uninstall_interactive(&installed_systems)?,
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

fn check_and_install_screenfetch() -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("pkg")
        .args(&["list-installed"])
        .output()?;
    
    if !String::from_utf8_lossy(&output.stdout).contains("screenfetch") {
        crate::ui::print_info("正在安装相关依赖包: screenfetch");
        Command::new("pkg")
            .args(&["install", "screenfetch", "-y"])
            .spawn()?
            .wait()?;
    }
    
    Ok(())
}