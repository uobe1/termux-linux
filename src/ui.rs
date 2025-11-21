use crate::distro::SystemMeta;
use std::io::{self, Write};

pub fn get_terminal_width() -> usize {
    match std::env::var("COLUMNS") {
        Ok(width_str) => width_str.parse().unwrap_or(80),
        Err(_) => 80,
    }
}

pub fn display_system_list(metas: &[(String, SystemMeta)]) -> Result<(), Box<dyn std::error::Error>> {
    if metas.is_empty() {
        println!("\n[ 这里没有任何系统... ]\n");
        return Ok(());
    }
    
    println!("\n已安装系统:\n");
    
    for (system_id, meta) in metas {
        println!("  {}  ", meta.name);
        println!("  ──────────────────────────────────");
        
        let date = meta.created_at.split('T').next().unwrap_or("未知");
        println!("  系统ID: {}    创建时间: {}", system_id, date);
        println!("  用户组: {}    权限: {}", meta.user_group, meta.permissions);
        
        println!();
    }
    
    Ok(())
}

pub struct ProgressBar {
    total: u64,
    current: u64,
    width: usize,
    message: String,
}

impl ProgressBar {
    pub fn new(total: u64, message: String) -> Self {
        Self {
            total,
            current: 0,
            width: 40,
            message,
        }
    }
    
    pub fn update(&mut self, current: u64) {
        self.current = current;
        self.draw();
    }
    
    pub fn increment(&mut self, amount: u64) {
        self.current += amount;
        self.draw();
    }
    
    fn draw(&self) {
        let percentage = if self.total > 0 {
            (self.current as f64 / self.total as f64 * 100.0) as u64
        } else {
            0
        };
        
        let filled_width = if self.total > 0 {
            ((self.current as f64 / self.total as f64) * self.width as f64) as usize
        } else {
            0
        };
        
        let bar = format!(
            "[{}{}]",
            "█".repeat(filled_width),
            "░".repeat(self.width.saturating_sub(filled_width))
        );
        
        print!("\r  {} {} {}%", self.message, bar, percentage);
        io::stdout().flush().unwrap();
        
        if self.current >= self.total {
            println!();
        }
    }
    
    pub fn finish(&self) {
        println!();
    }
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

pub fn print_error(message: &str) {
    println!("\n  ✗ {}\n", message);
}

pub fn print_info(message: &str) {
    println!("  ℹ {}", message);
}