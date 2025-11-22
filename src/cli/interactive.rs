use std::io;
use std::io::Write;
use crate::i18n::Translator;

pub fn display_logo(translator: &Translator) {
    println!(r#"
  _____                              
 |_   _|__ _ __ _ __ ___  _   ___  __
   | |/ _ \ '__| '_ ` _ \| | | \/ /
   | |  __/ |  | | | | | | |_| |>  < 
   |_|\___|_|  |_| |_| |_|\__,_|_/\_\
  
  {}
  
  {}
  {}
  
"#, translator.t("termux_linux_installer"), translator.t("menu_option_1"), translator.t("menu_option_2"));
}

pub fn get_user_choice(translator: &Translator) -> Result<i32, std::num::ParseIntError> {
    print!("  {}", translator.t("select_operation"));
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    input.trim().parse()
}