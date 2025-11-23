pub mod interactive;
pub mod core;

use crate::i18n::Translator;

pub fn install_interactive(translator: &Translator) -> Result<(), Box<dyn std::error::Error>> {
    interactive::install_interactive(translator)
}
