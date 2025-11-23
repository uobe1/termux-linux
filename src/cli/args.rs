use std::env;
// use crate::distro::{LinuxDistro, DistroType};
use crate::system::uninstall_system_by_id;
use crate::ui::{print_info_theme, print_success_theme};
use crate::i18n::Translator;
use crate::ui::colors::Theme;

pub fn handle_command_line_args(args: &[String], translator: &Translator, theme: &Theme) -> Result<(), Box<dyn std::error::Error>> {
    // Skip --lang parameter if present (language is already handled in main)
    let mut start_idx = 1;
    if args.len() > 2 && args[1] == "--lang" {
        start_idx = 3;
    }
    
    if start_idx >= args.len() {
        return Ok(());
    }
    
    match args[start_idx].as_str() {
        "--list" => {
            let metas = crate::utils::get_system_metas()?;
            crate::ui::display_system_list(&metas, translator)?;
        }
        "--install" => {
            // TODO: Update to use new distro system
            println!("\n  {}\n", theme.error(&translator.t("error_specify_distro")));
            println!("  {}\n", theme.info(&translator.t_fmt("usage_install", &[&args[0]])));
        }
        "--uninstall" => {
            if args.len() < 3 {
                println!("\n  {}\n", theme.error(&translator.t("error_specify_system_id")));
                println!("  {}\n", theme.info(&translator.t_fmt("usage_uninstall", &[&args[0]])));
                return Ok(());
            }
            
            print_info_theme(&translator.t_fmt("uninstall_system", &[&args[2]]), theme);
            uninstall_system_by_id(&args[2], translator)?;
            print_success_theme(&translator.t("uninstall_complete"), theme);
        }
        "--help" => {
            display_help(translator, theme);
        }
        "--no-color" => {
            println!("\n  {}\n", theme.info(&translator.t("color_output_disabled")));
        }
        _ => {
            println!("\n  {}\n", theme.error(&translator.t_fmt("unknown_argument", &[&args[start_idx]])));
            display_help(translator, theme);
        }
    }
    
    Ok(())
}

fn display_help(translator: &Translator, theme: &Theme) {
    let program_name = env::args().next().unwrap_or_else(|| "insOs".to_string());
    
    println!("\n  {}\n    {}", theme.info(&translator.t("usage_header")), theme.info(&translator.t_fmt("usage_interactive", &[&program_name])));
    println!("    {}", theme.info(&translator.t_fmt("usage_list", &[&program_name])));
    println!("    {}", theme.info(&translator.t_fmt("usage_install_cmd", &[&program_name])));
    println!("    {}", theme.info(&translator.t_fmt("usage_uninstall_cmd", &[&program_name])));
    println!("    {}\n", theme.info(&translator.t_fmt("usage_help", &[&program_name])));
    println!("  {}\n", theme.info(&translator.t("supported_distros")));
    println!("  {}\n    {}", theme.info(&translator.t("install_options")), theme.info(&translator.t("option_name")));
    println!("    {}\n", theme.info(&translator.t("option_minimal")));
}