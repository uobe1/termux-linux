use std::process::Command;
use crate::i18n::Translator;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Architecture {
    Aarch64,
    Arm,
    X86_64,
    I686,
    Riscv64,
}

impl Architecture {
    pub fn to_str(&self) -> &'static str {
        match self {
            Architecture::Aarch64 => "aarch64",
            Architecture::Arm => "arm",
            Architecture::X86_64 => "x86_64",
            Architecture::I686 => "i686",
            Architecture::Riscv64 => "riscv64",
        }
    }
    
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "aarch64" | "arm64" => Some(Architecture::Aarch64),
            "arm" | "armv7" | "armhf" => Some(Architecture::Arm),
            "x86_64" | "amd64" => Some(Architecture::X86_64),
            "i686" | "x86" => Some(Architecture::I686),
            "riscv64" => Some(Architecture::Riscv64),
            _ => None,
        }
    }
}

pub fn detect_architecture() -> Result<Architecture, Box<dyn std::error::Error>> {
    let output = Command::new("uname")
        .arg("-m")
        .output()?;
    
    if !output.status.success() {
        return Err("Failed to detect architecture".into());
    }
    
    let arch_str = String::from_utf8(output.stdout)?
        .trim()
        .to_string();
    
    Architecture::from_str(&arch_str)
        .ok_or_else(|| format!("Unsupported architecture: {}", arch_str).into())
}

pub fn prompt_manual_selection(translator: &Translator) -> Result<Architecture, Box<dyn std::error::Error>> {
    println!("{}", translator.t("arch_detection_prompt"));
    println!("{}", translator.t("arch_option_1"));
    println!("{}", translator.t("arch_option_2"));
    println!("{}", translator.t("arch_option_3"));
    println!("{}", translator.t("arch_option_4"));
    println!("{}", translator.t("arch_option_5"));
    
    loop {
        print!("{}", translator.t("arch_select_prompt"));
        std::io::Write::flush(&mut std::io::stdout())?;
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        
        match input.trim() {
            "1" => return Ok(Architecture::Aarch64),
            "2" => return Ok(Architecture::Arm),
            "3" => return Ok(Architecture::X86_64),
            "4" => return Ok(Architecture::I686),
            "5" => return Ok(Architecture::Riscv64),
            _ => println!("{}", translator.t("arch_invalid_choice")),
        }
    }
}

pub fn get_architecture(translator: &Translator) -> Result<Architecture, Box<dyn std::error::Error>> {
    match detect_architecture() {
        Ok(arch) => {
            println!("{}: {}", translator.t("arch_detected"), arch.to_str());
            Ok(arch)
        }
        Err(e) => {
            println!("{}: {}", translator.t("arch_detection_failed"), e);
            prompt_manual_selection(translator)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_architecture_to_str() {
        assert_eq!(Architecture::Aarch64.to_str(), "aarch64");
        assert_eq!(Architecture::Arm.to_str(), "arm");
        assert_eq!(Architecture::X86_64.to_str(), "x86_64");
        assert_eq!(Architecture::I686.to_str(), "i686");
        assert_eq!(Architecture::Riscv64.to_str(), "riscv64");
    }

    #[test]
    fn test_architecture_from_str() {
        assert_eq!(Architecture::from_str("aarch64"), Some(Architecture::Aarch64));
        assert_eq!(Architecture::from_str("arm64"), Some(Architecture::Aarch64));
        assert_eq!(Architecture::from_str("arm"), Some(Architecture::Arm));
        assert_eq!(Architecture::from_str("x86_64"), Some(Architecture::X86_64));
        assert_eq!(Architecture::from_str("amd64"), Some(Architecture::X86_64));
        assert_eq!(Architecture::from_str("i686"), Some(Architecture::I686));
        assert_eq!(Architecture::from_str("riscv64"), Some(Architecture::Riscv64));
        assert_eq!(Architecture::from_str("unknown"), None);
    }
}
