mod cli;
mod distro;
mod utils;
mod installer;
mod system;
mod config;
mod ui;
mod i18n;

use cli::run_cli;
use i18n::{I18nLoader, Translator, Language};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    let lang = detect_language(&args);
    let loader = I18nLoader::new()?;
    let strings = loader.load_language(lang)?;
    let translator = Translator::new(lang, strings);
    
    run_cli(&translator)
}

fn detect_language(args: &[String]) -> Language {
    for i in 0..args.len() {
        if args[i] == "--lang" && i + 1 < args.len() {
            if let Some(lang) = Language::from_code(&args[i + 1]) {
                return lang;
            }
        }
    }
    
    if let Ok(locale) = env::var("LANG") {
        if locale.to_lowercase().starts_with("zh") {
            return Language::Chinese;
        }
    }
    
    Language::English
}