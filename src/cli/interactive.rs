use std::io;
use std::io::Write;
use crate::i18n::Translator;
use crate::ui::colors::Theme;

pub fn display_logo(translator: &Translator, theme: &Theme) {
    let logo = r#"
  _____                              
 |_   _|__ _ __ _ __ ___  _   ___  __
   | |/ _ \ '__| '_ ` _ \| | | \/ /
   | |  __/ |  | | | | | | |_| |>  < 
   |_|\___|_|  |_| |_| |_|\__,_|_/\_\
  
"#;
    println!("{}", theme.colorize(logo, "36")); // Cyan color for logo
    println!("  {}", theme.info(&translator.t("termux_linux_installer")));
    println!();
    println!("  {}", theme.progress(&translator.t("menu_option_1")));
    println!("  {}", theme.progress(&translator.t("menu_option_2")));
    println!();
}

pub fn get_user_choice(translator: &Translator, theme: &Theme) -> Result<i32, std::num::ParseIntError> {
    print!("  {}", theme.info(&translator.t("select_operation")));
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    input.trim().parse()
}