use crate::distro::SystemMeta;

pub fn get_terminal_width() -> usize {
    match std::env::var("COLUMNS") {
        Ok(width_str) => width_str.parse().unwrap_or(80),
        Err(_) => 80,
    }
}

pub fn display_system_list(metas: &[(String, SystemMeta)]) -> Result<(), Box<dyn std::error::Error>> {
    if metas.is_empty() {
        println!("这里没有任何东西...");
        return Ok(());
    }
    
    let width = get_terminal_width();
    let box_width = if width > 70 { width - 4 } else { 70 };
    
    println!("\n已安装系统:\n");
    
    for (system_id, meta) in metas {
        println!("┌{}┐", "─".repeat(box_width - 2));
        
        let name_line = format!("│ {} ", meta.name);
        println!("{}{}│", name_line, " ".repeat(box_width - name_line.len() - 1));
        
        let date = meta.created_at.split('T').next().unwrap_or("未知");
        let id_time_line = format!("│ {} {} ", system_id, date);
        println!("{}{}│", id_time_line, " ".repeat(box_width - id_time_line.len() - 1));
        
        let user_perm_line = format!("│ 用户组: {} 权限: {} ", meta.user_group, meta.permissions);
        println!("{}{}│", user_perm_line, " ".repeat(box_width - user_perm_line.len() - 1));
        
        println!("└{}┘", "─".repeat(box_width - 2));
        println!();
    }
    
    Ok(())
}