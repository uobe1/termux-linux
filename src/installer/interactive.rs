use std::io::{self, Write};
use crate::distro::{LinuxDistro, DistroType};
use crate::ui::{print_section, print_item, print_info, print_success};
use crate::i18n::Translator;

pub fn install_interactive(translator: &Translator) -> Result<(), Box<dyn std::error::Error>> {
    print_section(&translator.t("distro_selection"));
    print_item("1.", "Ubuntu");
    print_item("2.", "Kali");
    print_item("3.", "Debian");
    print_item("4.", "CentOS");
    print_item("5.", "Fedora");
    
    print!("\n{}", translator.t("enter_number_select"));
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
            println!("\n{}", translator.t("invalid_choice"));
            return Ok(());
        }
    };
    
    print!("\n{}", translator.t("enter_system_name"));
    io::stdout().flush().unwrap();
    
    let mut name_input = String::new();
    io::stdin().read_line(&mut name_input).unwrap();
    let name = if name_input.trim().is_empty() { None } else { Some(name_input.trim().to_string()) };
    
    print_section(&translator.t("install_mode_selection"));
    print_item("1.", &translator.t("minimal_install"));
    print_item("2.", &translator.t("standard_install"));
    print_item("3.", &translator.t("custom_install"));
    
    print!("\n{}", translator.t("select_prompt"));
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
            print_info(&translator.t("starting_minimal"));
            distro.install(translator)?;
        }
        Ok(2) | Ok(3) => {
            let distro = if let Some(name) = name {
                LinuxDistro::with_name(distro_type, name)
            } else {
                LinuxDistro::new(distro_type)
            };
            print_info(&translator.t("starting_standard"));
            distro.install(translator)?;
        }
        _ => {
            println!("\n{}", translator.t("invalid_choice_exclamation"));
            return Ok(());
        }
    }
    
    print_success(&translator.t("install_complete_exclamation"));
    
    Ok(())
}