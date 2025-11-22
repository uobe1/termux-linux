use std::env;
use crate::distro::{LinuxDistro, DistroType};
use crate::system::uninstall_system_by_id;
use crate::ui::{print_info, print_success};
use crate::i18n::Translator;
use crate::ui::colors::Theme;

pub fn handle_command_line_args(args: &[String], translator: &Translator, theme: &Theme) -> Result<(), Box<dyn std::error::Error>> {
    match args[1].as_str() {
        "--list" => {
            let metas = crate::utils::get_system_metas()?;
            crate::ui::display_system_list(&metas, translator)?;
        }
        "--install" => {
            if args.len() < 3 {
                println!("\n  {}\n", translator.t("error_specify_distro"));
                println!("  {}\n", translator.t_fmt("usage_install", &[&args[0]]));
                return Ok(());
            }
            
            let distro_type = match args[2].to_lowercase().as_str() {
                "ubuntu" => DistroType::Ubuntu,
                "kali" => DistroType::Kali,
                "debian" => DistroType::Debian,
                "centos" => DistroType::CentOS,
                "fedora" => DistroType::Fedora,
                _ => {
                    println!("\n  {}\n", translator.t("error_unsupported_distro"));
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
            
            print_info(&translator.t_fmt("installing_package", &[&distro_type.to_string()]));
            distro.install()?;
            print_success(&translator.t("installation_complete"));
        }
        "--uninstall" => {
            if args.len() < 3 {
                println!("\n  {}\n", translator.t("error_specify_system_id"));
                println!("  {}\n", translator.t_fmt("usage_uninstall", &[&args[0]]));
                return Ok(());
            }
            
            print_info(&translator.t_fmt("uninstall_system", &[&args[2]]));
            uninstall_system_by_id(&args[2], translator)?;
            print_success(&translator.t("uninstall_complete"));
        }
        "--help" => {
            display_help(translator);
        }
        "--no-color" => {
            println!("\n  {}\n", translator.t("color_output_disabled"));
        }
        _ => {
            println!("\n  {}\n", translator.t_fmt("unknown_argument", &[&args[1]]));
            display_help(translator);
        }
    }
    
    Ok(())
}

fn display_help(translator: &Translator) {
    let program_name = env::args().next().unwrap_or_else(|| "insOs".to_string());
    
    println!("\n  {}\n    {}", translator.t("usage_header"), translator.t_fmt("usage_interactive", &[&program_name]));
    println!("    {}", translator.t_fmt("usage_list", &[&program_name]));
    println!("    {}", translator.t_fmt("usage_install_cmd", &[&program_name]));
    println!("    {}", translator.t_fmt("usage_uninstall_cmd", &[&program_name]));
    println!("    {}\n", translator.t_fmt("usage_help", &[&program_name]));
    println!("  {}\n", translator.t("supported_distros"));
    println!("  {}\n    {}", translator.t("install_options"), translator.t("option_name"));
    println!("    {}\n", translator.t("option_minimal"));
}