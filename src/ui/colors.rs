pub struct Theme {
    #[allow(dead_code)]
    pub no_color: bool,
}

impl Theme {
    pub fn new(no_color: bool) -> Self {
        Self { no_color }
    }
    
    #[allow(dead_code)]
    pub fn colorize(&self, text: &str, color_code: &str) -> String {
        if self.no_color {
            text.to_string()
        } else {
            format!("\x1b[{}m{}\x1b[0m", color_code, text)
        }
    }
    
    #[allow(dead_code)]
    pub fn success(&self, text: &str) -> String {
        self.colorize(text, "32")  // Green
    }
    
    #[allow(dead_code)]
    pub fn error(&self, text: &str) -> String {
        self.colorize(text, "31")  // Red
    }
    
    #[allow(dead_code)]
    pub fn warning(&self, text: &str) -> String {
        self.colorize(text, "33")  // Yellow
    }
    
    #[allow(dead_code)]
    pub fn info(&self, text: &str) -> String {
        self.colorize(text, "34")  // Blue
    }
    
    #[allow(dead_code)]
    pub fn progress(&self, text: &str) -> String {
        self.colorize(text, "36")  // Cyan
    }
}

#[allow(dead_code)]
pub fn detect_color_support() -> bool {
    std::env::var("NO_COLOR").is_err()
}
