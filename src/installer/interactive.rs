use std::io::{self, Write};
use crate::distro::{get_distros_for_arch, DistroDefinition};
use crate::utils::arch::get_architecture;
use crate::ui::{print_section, print_item, print_info, print_success};
use crate::i18n::Translator;

pub fn install_interactive(translator: &Translator) -> Result<(), Box<dyn std::error::Error>> {
    let arch = get_architecture(translator)?;
    let available_distros = get_distros_for_arch(&arch);
    
    if available_distros.is_empty() {
        print_info(&translator.t("no_distros_for_arch"));
        return Ok(());
    }
    
    print_section(&translator.t("distro_selection"));
    for (i, distro) in available_distros.iter().enumerate() {
        print_item(&format!("{}.", i + 1), &distro.display_name);
    }
    
    print!("\n{}", translator.t("enter_number_select"));
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let selected_distro = match input.trim().parse::<usize>() {
        Ok(num) if num > 0 && num <= available_distros.len() => &available_distros[num - 1],
        _ => {
            println!("\n{}", translator.t("invalid_choice"));
            return Ok(());
        }
    };
    
    print!("\n{}", translator.t("enter_system_name"));
    io::stdout().flush().unwrap();
    
    let mut name_input = String::new();
    io::stdin().read_line(&mut name_input).unwrap();
    let custom_name = if name_input.trim().is_empty() { 
        None 
    } else { 
        Some(name_input.trim().to_string()) 
    };
    
    print_section(&translator.t("install_mode_selection"));
    print_item("1.", &translator.t("minimal_install"));
    print_item("2.", &translator.t("standard_install"));
    print_item("3.", &translator.t("custom_install"));
    
    print!("\n{}", translator.t("select_prompt"));
    io::stdout().flush().unwrap();
    
    let mut mode_input = String::new();
    io::stdin().read_line(&mut mode_input).unwrap();
    
    let mode = match mode_input.trim().parse::<i32>() {
        Ok(1) => "minimal",
        Ok(2) | Ok(3) => "standard",
        _ => {
            println!("\n{}", translator.t("invalid_choice_exclamation"));
            return Ok(());
        }
    };
    
    if mode == "minimal" {
        print_info(&translator.t("starting_minimal"));
    } else {
        print_info(&translator.t("starting_standard"));
    }
    
    install_distro(selected_distro, custom_name, mode, &arch, translator)?;
    
    print_success(&translator.t("install_complete_exclamation"));
    
    Ok(())
}

fn install_distro(
    distro_def: &DistroDefinition,
    custom_name: Option<String>,
    mode: &str,
    arch: &crate::utils::arch::Architecture,
    translator: &Translator,
) -> Result<(), Box<dyn std::error::Error>> {
    print_info(&format!("Installing {}...", distro_def.display_name));
    
    let url = match distro_def.get_url(&arch) {
        Some(url) => url,
        None => {
            crate::ui::print_error(&format!("No URL found for architecture {}", arch.to_str()));
            return Ok(());
        }
    };
    
    crate::ui::print_info(&format!("Download URL: {}", url));
    crate::ui::print_info(&format!("Default packages: {:?}", distro_def.default_packages));
    
    Ok(())
}