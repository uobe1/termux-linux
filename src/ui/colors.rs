pub struct Theme {
    pub no_color: bool,
}

impl Theme {
    pub fn new(no_color: bool) -> Self {
        Self { no_color }
    }
    
    pub fn colorize(&self, text: &str, color_code: &str) -> String {
        if self.no_color {
            text.to_string()
        } else {
            format!("\x1b[{}m{}\x1b[0m", color_code, text)
        }
    }
    
    pub fn success(&self, text: &str) -> String {
        self.colorize(text, "32")  // Green
    }
    
    pub fn error(&self, text: &str) -> String {
        self.colorize(text, "31")  // Red
    }
    
    pub fn warning(&self, text: &str) -> String {
        self.colorize(text, "33")  // Yellow
    }
    
    pub fn info(&self, text: &str) -> String {
        self.colorize(text, "34")  // Blue
    }
    
    pub fn progress(&self, text: &str) -> String {
        self.colorize(text, "36")  // Cyan
    }
}

pub fn detect_color_support() -> bool {
    std::env::var("NO_COLOR").is_err()
}
