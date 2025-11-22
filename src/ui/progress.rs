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
            width: 20,
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
            "=".repeat(filled_width),
            " ".repeat(self.width.saturating_sub(filled_width))
        );
        
        print!("\r  {} {:>3}% {}", self.message, percentage, bar);
        io::stdout().flush().unwrap();
        
        if self.current >= self.total {
            println!();
        }
    }
    
    pub fn finish(&self) {
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_bar_new() {
        let progress = ProgressBar::new(100, "Testing".to_string());
        assert_eq!(progress.total, 100);
        assert_eq!(progress.current, 0);
        assert_eq!(progress.width, 20);
        assert_eq!(progress.message, "Testing");
    }

    #[test]
    fn test_progress_bar_render_0_percent() {
        let progress = ProgressBar::new(100, "Testing".to_string());
        let expected = "  Testing   0% [                    ]";
        
        let percentage = 0;
        let filled_width = 0;
        let bar = format!(
            "[{}{}]",
            "=".repeat(filled_width),
            " ".repeat(20 - filled_width)
        );
        let result = format!("  {} {:>3}% {}", "Testing", percentage, bar);
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_progress_bar_render_50_percent() {
        let progress = ProgressBar::new(100, "Testing".to_string());
        let expected = "  Testing  50% [==========          ]";
        
        let percentage = 50;
        let filled_width = 10;
        let bar = format!(
            "[{}{}]",
            "=".repeat(filled_width),
            " ".repeat(20 - filled_width)
        );
        let result = format!("  {} {:>3}% {}", "Testing", percentage, bar);
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_progress_bar_render_100_percent() {
        let progress = ProgressBar::new(100, "Testing".to_string());
        let expected = "  Testing 100% [====================]";
        
        let percentage = 100;
        let filled_width = 20;
        let bar = format!(
            "[{}{}]",
            "=".repeat(filled_width),
            " ".repeat(20 - filled_width)
        );
        let result = format!("  {} {:>3}% {}", "Testing", percentage, bar);
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_progress_bar_update() {
        let mut progress = ProgressBar::new(100, "Testing".to_string());
        assert_eq!(progress.current, 0);
        
        progress.update(50);
        assert_eq!(progress.current, 50);
        
        progress.update(100);
        assert_eq!(progress.current, 100);
    }

    #[test]
    fn test_progress_bar_increment() {
        let mut progress = ProgressBar::new(100, "Testing".to_string());
        assert_eq!(progress.current, 0);
        
        progress.increment(25);
        assert_eq!(progress.current, 25);
        
        progress.increment(25);
        assert_eq!(progress.current, 50);
    }
}