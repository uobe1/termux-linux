use crate::distro::SystemMeta;
use crate::i18n::Translator;
use crate::ui::colors::Theme;

#[allow(dead_code)]
pub fn get_terminal_width() -> usize {
    match std::env::var("COLUMNS") {
        Ok(width_str) => width_str.parse().unwrap_or(80),
        Err(_) => 80,
    }
}

pub fn display_system_list(metas: &[(String, SystemMeta)], translator: &Translator) -> Result<(), Box<dyn std::error::Error>> {
    if metas.is_empty() {
        println!("\n[ {} ]\n", translator.t("no_systems_here"));
        return Ok(());
    }
    
    println!("\n{}\n", translator.t("installed_systems_header"));
    
    for (system_id, meta) in metas {
        println!("  {}  ", meta.name);
        println!("  ──────────────────────────────────");
        
        let unknown_date = translator.t("unknown_date");
        let date = meta.created_at.split('T').next().unwrap_or(&unknown_date);
        let system_id_label = translator.t("system_id_label");
        let created_at_label = translator.t("created_at_label");
        let user_group_label = translator.t("user_group_label");
        let permissions_label = translator.t("permissions_label");
        println!("  {}: {}    {}: {}", system_id_label, system_id, created_at_label, date);
        println!("  {}: {}    {}: {}", user_group_label, meta.user_group, permissions_label, meta.permissions);
        println!("  {}: {}    {}: {}", translator.t("user_group_label"), meta.user_group, translator.t("permissions_label"), meta.permissions);
        
        println!();
    }
    
    Ok(())
}

pub fn print_section(title: &str) {
    println!("\n{}", title);
    println!("{}", "─".repeat(title.len()));
}

pub fn print_item(prefix: &str, content: &str) {
    println!("  {} {}", prefix, content);
}

pub fn print_success(message: &str) {
    println!("\n  ✓ {}\n", message);
}

#[allow(dead_code)]
pub fn print_error(message: &str) {
    println!("\n  ✗ {}\n", message);
}

pub fn print_info(message: &str) {
    println!("  ℹ {}", message);
}

pub fn print_success_theme(message: &str, theme: &Theme) {
    println!("\n  {}\n", theme.success(&format!("✓ {}", message)));
}

#[allow(dead_code)]
pub fn print_error_theme(message: &str, theme: &Theme) {
    println!("\n  {}\n", theme.error(&format!("✗ {}", message)));
}

pub fn print_info_theme(message: &str, theme: &Theme) {
    println!("  {}", theme.info(&format!("ℹ {}", message)));
}

