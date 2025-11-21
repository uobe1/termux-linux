use std::io;
use std::io::Write;
use crate::utils::get_installed_systems;
use crate::ui::{print_section, print_item, print_info, print_success};
use crate::i18n::Translator;

pub fn display_logo(translator: &Translator) {
    println!(r#"
  _____                              
 |_   _|__ _ __ _ __ ___  _   ___  __
   | |/ _ \ '__| '_ ` _ \| | | \/ /
   | |  __/ |  | | | | | | |_| |>  < 
   |_|\___|_|  |_| |_| |_|\__,_|_/\_\
  
  {}
  
  {}
  {}
  
"#, translator.t("termux_linux_installer"), translator.t("menu_option_1"), translator.t("menu_option_2"));
}

pub fn get_user_choice(translator: &Translator) -> Result<i32, std::num::ParseIntError> {
    print!("  {}", translator.t("select_operation"));
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    input.trim().parse()
}

pub fn uninstall_interactive(installed_systems: &[String], translator: &Translator) -> Result<(), Box<dyn std::error::Error>> {
    if installed_systems.is_empty() {
        println!("\n  {}\n", translator.t("no_systems_installed"));
        return Ok(());
    }
    
    print_section(&translator.t("select_system_to_uninstall"));
    
    for (i, system) in installed_systems.iter().enumerate() {
        print_item(&format!("{}.", i + 1), system);
    }
    
    print!("\n  {}", translator.t("select_prompt"));
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    if let Ok(choice) = input.trim().parse::<usize>() {
        if choice > 0 && choice <= installed_systems.len() {
            let system_id = &installed_systems[choice - 1];
            print_info(&translator.t_fmt("uninstalling_system", &[system_id]));
            crate::system::uninstall_system_by_id(system_id, translator)?;
            print_success(&translator.t("uninstall_complete"));
        } else {
            println!("\n  {}\n", translator.t("invalid_selection"));
        }
    } else {
        println!("\n  {}\n", translator.t("invalid_input"));
    }
    
    Ok(())
}