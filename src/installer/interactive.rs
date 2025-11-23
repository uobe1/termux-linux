use std::io::{self, Write};
use crate::distro::{get_distros_for_arch, DistroDefinition, SystemMeta};
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
    
    install_distro(selected_distro, custom_name.clone(), mode, &arch, translator)?;
    
    print_success(&translator.t("install_complete_exclamation"));
    
    print!("\n{}", translator.t("prompt_start_system"));
    io::stdout().flush().unwrap();
    
    let mut start_input = String::new();
    io::stdin().read_line(&mut start_input).unwrap();
    
    if start_input.trim().eq_ignore_ascii_case("y") || start_input.trim().eq_ignore_ascii_case("yes") {
        crate::ui::print_info(&translator.t("starting_system"));
        
        let system_id = if let Some(name) = custom_name {
            name
        } else {
            format!("{}{}", selected_distro.name, 1)
        };
        
        let home_dir = crate::utils::fs::get_home_dir()?;
        let system_path = home_dir.join("termos").join(&system_id);
        
        if system_path.exists() {
            std::env::set_current_dir(&system_path)?;
            
            if std::path::Path::new("./start.sh").exists() {
                crate::utils::cmd::run_command("./start.sh")?;
            } else {
                crate::ui::print_error(&translator.t("start_script_not_found"));
            }
        } else {
            crate::ui::print_error(&translator.t_fmt("system_path_not_found", &[&system_id]));
        }
    }
    
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
    
    let home_dir = crate::utils::fs::get_home_dir()?;
    let termos_dir = home_dir.join("termos");
    std::fs::create_dir_all(&termos_dir)?;
    
    let system_id = if let Some(name) = custom_name {
        name
    } else {
        format!("{}{}", distro_def.name.as_str(), 1)
    };
    
    let system_dir = termos_dir.join(&system_id);
    if system_dir.exists() {
        crate::ui::print_error(&translator.t_fmt("system_already_exists", &[&system_id]));
        return Ok(());
    }
    
    std::fs::create_dir_all(&system_dir)?;
    
    let archive_path = system_dir.join("rootfs.tar.xz");
    
    crate::ui::print_info(&translator.t("downloading"));
    let mut download_progress = crate::ui::progress::DownloadProgressBar::new(
        translator.t("download_progress")
    );
    
    if let Ok(Some(content_length)) = crate::utils::net::get_content_length(url) {
        download_progress.set_total_size(content_length);
    }
    
    crate::utils::net::download_file_with_progress(
        url,
        archive_path.to_str().unwrap(),
        |downloaded, _| {
            download_progress.update(downloaded);
        },
    )?;
    
    download_progress.finish();
    crate::ui::print_success(&translator.t("download_complete"));
    
    crate::ui::print_info(&translator.t("extracting"));
    
    let total_files = crate::utils::fs::count_files_in_tar_xz(&archive_path)?;
    let mut extract_progress = crate::ui::progress::ExtractionProgressBar::new(
        total_files,
        translator.t("extract_progress")
    );
    
    crate::utils::fs::extract_tar_xz_with_progress(
        &archive_path,
        &system_dir,
        |extracted, file_name| {
            extract_progress.update(extracted, file_name);
        },
    )?;
    
    extract_progress.finish();
    crate::ui::print_success(&translator.t("extraction_complete"));
    
    let meta = SystemMeta::new(system_id.clone(), distro_def.name.to_string());
    let meta_path = system_dir.join("meta.txt");
    std::fs::write(&meta_path, meta.to_string())?;
    
    let config_manager = crate::config::ConfigManager::new()?;
    if let Some(init_commands) = config_manager.get_init_commands_for_distro(distro_def.name.as_str())? {
        crate::ui::print_info(&translator.t("executing_init_commands"));
        let commands: Vec<&str> = init_commands.lines().collect();
        for (i, cmd) in commands.iter().enumerate() {
            let cmd = cmd.trim();
            if !cmd.is_empty() {
                crate::ui::print_info(&format!("[{}] {}", i + 1, cmd));
            }
        }
    }
    
    let start_script_path = system_dir.join("start.sh");
    let shell_cmd = config_manager.get_shell_command()?.unwrap_or("/bin/bash --login".to_string());
    let start_script_content = format!(
        r#"#!/bin/bash
cd "$(dirname "$0")"
exec proot -r . -0 -w / -b /dev -b /proc -b /sys {}
"#,
        shell_cmd
    );
    std::fs::write(&start_script_path, start_script_content)?;
    
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(&start_script_path)?.permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(&start_script_path, perms)?;
    
    Ok(())
}