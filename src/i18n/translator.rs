use std::collections::HashMap;
use super::loader::Language;

pub struct Translator {
    lang: Language,
    strings: HashMap<String, String>,
}

impl Translator {
    pub fn new(lang: Language, strings: HashMap<String, String>) -> Self {
        Self { lang, strings }
    }
    
    pub fn t(&self, key: &str) -> String {
        self.strings.get(key).cloned().unwrap_or_else(|| {
            format!("[{}]", key)
        })
    }
    
    pub fn t_fmt(&self, key: &str, args: &[&str]) -> String {
        let template = self.t(key);
        let mut result = template;
        
        for (i, arg) in args.iter().enumerate() {
            let placeholder = format!("{{{}}}", i);
            result = result.replace(&placeholder, arg);
        }
        
        result
    }
    
    pub fn get_language(&self) -> Language {
        self.lang
    }
    
    pub fn get_language_name(&self) -> &'static str {
        self.lang.as_name()
    }
}

#[macro_export]
macro_rules! t {
    ($translator:expr, $key:expr) => {
        $translator.t($key)
    };
    ($translator:expr, $key:expr, $($arg:expr),+) => {
        $translator.t_fmt($key, &[$($arg),+])
    };
}
