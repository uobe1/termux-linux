use std::io::{self, Write};

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