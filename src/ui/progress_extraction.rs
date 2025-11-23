use std::io::{self, Write};

pub struct ExtractionProgressBar {
    total_files: u64,
    extracted_files: u64,
    width: usize,
    message: String,
    current_file: String,
}

impl ExtractionProgressBar {
    pub fn new(total_files: u64, message: String) -> Self {
        Self {
            total_files,
            extracted_files: 0,
            width: 20,
            message,
            current_file: String::new(),
        }
    }
    
    pub fn update(&mut self, extracted: u64, current_file: &str) {
        self.extracted_files = extracted;
        self.current_file = current_file.to_string();
        self.draw();
    }
    
    pub fn increment(&mut self, current_file: &str) {
        self.extracted_files += 1;
        self.current_file = current_file.to_string();
        self.draw();
    }
    
    fn draw(&self) {
        let percentage = if self.total_files > 0 {
            (self.extracted_files as f64 / self.total_files as f64 * 100.0) as u64
        } else {
            0
        };
        
        let filled_width = if self.total_files > 0 {
            ((self.extracted_files as f64 / self.total_files as f64) * self.width as f64) as usize
        } else {
            0
        };
        
        let bar = format!(
            "[{}{}]",
            "=".repeat(filled_width),
            " ".repeat(self.width.saturating_sub(filled_width))
        );
        
        let file_display = if self.current_file.len() > 30 {
            format!("...{}", &self.current_file[self.current_file.len()-27..])
        } else {
            self.current_file.clone()
        };
        
        print!("\r  {} {:>3}% {} [{}]", 
            self.message, percentage, bar, file_display);
        io::stdout().flush().unwrap();
        
        if self.total_files > 0 && self.extracted_files >= self.total_files {
            println!();
        }
    }
    
    pub fn finish(&self) {
        println!();
    }
}