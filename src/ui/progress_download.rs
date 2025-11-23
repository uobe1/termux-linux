use std::io::{self, Write};

pub struct DownloadProgressBar {
    total_size: u64,
    downloaded: u64,
    width: usize,
    message: String,
}

impl DownloadProgressBar {
    pub fn new(message: String) -> Self {
        Self {
            total_size: 0,
            downloaded: 0,
            width: 20,
            message,
        }
    }
    
    pub fn set_total_size(&mut self, total: u64) {
        self.total_size = total;
    }
    
    pub fn update(&mut self, downloaded: u64) {
        self.downloaded = downloaded;
        self.draw();
    }
    
    fn draw(&self) {
        let percentage = if self.total_size > 0 {
            (self.downloaded as f64 / self.total_size as f64 * 100.0) as u64
        } else {
            0
        };
        
        let filled_width = if self.total_size > 0 {
            ((self.downloaded as f64 / self.total_size as f64) * self.width as f64) as usize
        } else {
            0
        };
        
        let bar = format!(
            "[{}{}]",
            "=".repeat(filled_width),
            " ".repeat(self.width.saturating_sub(filled_width))
        );
        
        let downloaded_mb = self.downloaded as f64 / 1024.0 / 1024.0;
        let total_mb = if self.total_size > 0 {
            format!("{:.1} MB", self.total_size as f64 / 1024.0 / 1024.0)
        } else {
            "未知".to_string()
        };
        
        print!("\r  {} {:.1}/{} MB {:>3}% {}", 
            self.message, downloaded_mb, total_mb, percentage, bar);
        io::stdout().flush().unwrap();
        
        if self.total_size > 0 && self.downloaded >= self.total_size {
            println!();
        }
    }
    
    pub fn finish(&self) {
        println!();
    }
}