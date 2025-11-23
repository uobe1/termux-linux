pub mod args;
pub mod interactive;

pub use args::*;

use std::io;
use std::io::Write;
use std::process::Command;
use crate::utils::get_system_metas;
use crate::installer::install_interactive;
use crate::ui::display_system_list;
use crate::i18n::Translator;
use crate::ui::colors::Theme;

pub fn run_cli(translator: &Translator, theme: &Theme) -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 {
        handle_command_line_args(&args, translator, theme)?;
        // If handle_command_line_args returns Ok(()) without doing anything,
        // it means only --lang was provided, so continue to interactive menu
        if args.len() == 3 && args[1] == "--lang" {
            // Continue to interactive menu
        } else {
            return Ok(());
        }
    }
    
    check_and_install_screenfetch(translator)?;
    
    loop {
        interactive::display_logo(translator, theme);
        let installed_systems = crate::utils::get_installed_systems()?;
        let choice = interactive::get_user_choice(translator, theme)?;
        
        match choice {
            1 => install_interactive(translator)?,
            2 => {
                if installed_systems.is_empty() {
                    crate::ui::print_info(&translator.t("no_installed_systems"));
                } else {
                    println!("\n  {}", translator.t("select_system_to_uninstall"));
                    for (i, system) in installed_systems.iter().enumerate() {
                        println!("  {}. {}", i + 1, system);
                    }
                    
                    print!("\n  {}", translator.t("enter_system_number"));
                    std::io::stdout().flush()?;
                    
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input)?;
                    
                    match input.trim().parse::<usize>() {
                        Ok(num) if num > 0 && num <= installed_systems.len() => {
                            let system_id = &installed_systems[num - 1];
                            crate::system::uninstall_system_by_id(system_id, translator)?;
                            crate::ui::print_success(&translator.t("uninstall_complete"));
                        }
                        _ => crate::ui::print_error(&translator.t("invalid_selection")),
                    }
                }
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
        .args(["list-installed"])
        .output()?;
    
    if !String::from_utf8_lossy(&output.stdout).contains("screenfetch") {
        crate::ui::print_info(&translator.t_fmt("installing_package", &["screenfetch"]));
        Command::new("pkg")
            .args(["install", "screenfetch", "-y"])
            .spawn()?
            .wait()?;
    }
    
    Ok(())
}