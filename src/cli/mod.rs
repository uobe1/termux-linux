pub mod args;
pub mod interactive;

pub use args::*;

use std::io;
use std::process::Command;
use crate::utils::get_system_metas;
use crate::installer::install_interactive;
use crate::ui::display_system_list;
use crate::i18n::Translator;

pub fn run_cli(translator: &Translator) -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 {
        return handle_command_line_args(&args, translator);
    }
    
    check_and_install_screenfetch(translator)?;
    
    loop {
        interactive::display_logo(translator);
        let installed_systems = crate::utils::get_installed_systems()?;
        let choice = interactive::get_user_choice(translator)?;
        
        match choice {
            1 => install_interactive(translator)?,
            2 => {
                let index = (choice - 1) as usize;
                crate::system::uninstall_system_by_id(&installed_systems[index], translator)?;
                crate::ui::print_success(&translator.t("uninstall_complete"));
            }
            3 => {
                let metas = get_system_metas()?;
                display_system_list(&metas, translator)?;
            }
            4 => {
                println!("\n  {}\n", translator.t("exiting_program"));
                std::process::exit(0);
            }
            _ => println!("\n  {}\n", translator.t("invalid_choice_menu")),
        }
        
        println!("\n  {}", translator.t("press_enter"));
        let _ = io::stdin().read_line(&mut String::new());
    }
}

fn check_and_install_screenfetch(translator: &Translator) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("pkg")
        .args(&["list-installed"])
        .output()?;
    
    if !String::from_utf8_lossy(&output.stdout).contains("screenfetch") {
        crate::ui::print_info(&translator.t_fmt("installing_package", &["screenfetch"]));
        Command::new("pkg")
            .args(&["install", "screenfetch", "-y"])
            .spawn()?
            .wait()?;
    }
    
    Ok(())
}