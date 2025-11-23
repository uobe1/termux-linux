use std::collections::HashMap;
use crate::utils::arch::Architecture;
use crate::distro::name::DistroName;
use crate::distro::definitions::distro_definition::DistroDefinition;

pub fn get_definition() -> Vec<DistroDefinition> {
    let mut distros = Vec::new();
    
    let mut urls = HashMap::new();
    urls.insert(Architecture::Aarch64, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.29.0/adelie-aarch64-pd-v4.29.0.tar.xz".to_string());
    urls.insert(Architecture::Arm, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.29.0/adelie-armv7-pd-v4.29.0.tar.xz".to_string());
    urls.insert(Architecture::X86_64, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.29.0/adelie-x86_64-pd-v4.29.0.tar.xz".to_string());
    
    distros.push(DistroDefinition {
        name: DistroName::Adelie,
        display_name: "Adelie Linux".to_string(),
        urls,
        description: "Adelie Linux is a free, independent Linux distribution focused on simplicity, compatibility, and security.".to_string(),
        default_packages: vec!["base-devel".to_string()],
    });
    
    distros
}
