use crate::distro::{DistroDefinition, SystemMeta};
use crate::utils::arch::Architecture;
use crate::ui::{print_info, print_success, DownloadProgressBar, ExtractionProgressBar};
use crate::i18n::Translator;
use std::path::PathBuf;

pub fn install_distro(
    distro_def: &DistroDefinition,
    custom_name: Option<String>,
    mode: &str,
    arch: &Architecture,
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
    let mut download_progress = DownloadProgressBar::new(
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
    let mut extract_progress = ExtractionProgressBar::new(
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