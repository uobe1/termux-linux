use std::collections::HashMap;
use crate::utils::arch::Architecture;
use crate::distro::name::DistroName;
use super::base::DistroDefinition;

pub fn get_definitions() -> Vec<DistroDefinition> {
    let mut distros = Vec::new();
    
    distros.push(DistroDefinition {
        name: DistroName::Archlinux,
        display_name: "Arch Linux".to_string(),
        urls: {
            let mut urls = HashMap::new();
            urls.insert(Architecture::Aarch64, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.29.0/archlinux-aarch64-pd-v4.29.0.tar.xz".to_string());
            urls.insert(Architecture::Arm, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.29.0/archlinux-arm-pd-v4.29.0.tar.xz".to_string());
            urls.insert(Architecture::X86_64, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.29.0/archlinux-x86_64-pd-v4.29.0.tar.xz".to_string());
            urls.insert(Architecture::I686, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.29.0/archlinux-i686-pd-v4.29.0.tar.xz".to_string());
            urls
        },
        description: "A simple, lightweight Linux distribution".to_string(),
        default_packages: vec!["vim".to_string(), "curl".to_string(), "wget".to_string()],
    });
    
    distros.push(DistroDefinition {
        name: DistroName::Manjaro,
        display_name: "Manjaro".to_string(),
        urls: {
            let mut urls = HashMap::new();
            urls.insert(Architecture::Aarch64, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.29.0/manjaro-aarch64-pd-v4.29.0.tar.xz".to_string());
            urls
        },
        description: "Manjaro is a user-friendly Linux distribution based on Arch Linux".to_string(),
        default_packages: vec!["vim".to_string(), "curl".to_string(), "wget".to_string()],
    });
    
    distros.push(DistroDefinition {
        name: DistroName::Artix,
        display_name: "Artix".to_string(),
        urls: {
            let mut urls = HashMap::new();
            urls.insert(Architecture::Aarch64, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.29.0/artix-aarch64-pd-v4.29.0.tar.xz".to_string());
            urls
        },
        description: "Artix Linux is a fork of Arch Linux with openrc".to_string(),
        default_packages: vec!["vim".to_string(), "curl".to_string(), "wget".to_string()],
    });
    
    distros.push(DistroDefinition {
        name: DistroName::Void,
        display_name: "Void".to_string(),
        urls: {
            let mut urls = HashMap::new();
            urls.insert(Architecture::Aarch64, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.29.0/void-aarch64-pd-v4.29.0.tar.xz".to_string());
            urls.insert(Architecture::Arm, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.29.0/void-arm-pd-v4.29.0.tar.xz".to_string());
            urls.insert(Architecture::X86_64, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.29.0/void-x86_64-pd-v4.29.0.tar.xz".to_string());
            urls.insert(Architecture::I686, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.29.0/void-i686-pd-v4.29.0.tar.xz".to_string());
            urls
        },
        description: "Void is a general purpose operating system, based on the monolithic Linux kernel".to_string(),
        default_packages: vec!["vim".to_string(), "curl".to_string(), "wget".to_string()],
    });
    
    distros.push(DistroDefinition {
        name: DistroName::Chimera,
        display_name: "Chimera".to_string(),
        urls: {
            let mut urls = HashMap::new();
            urls.insert(Architecture::Aarch64, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.29.0/chimera-aarch64-pd-v4.29.0.tar.xz".to_string());
            urls.insert(Architecture::X86_64, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.29.0/chimera-x86_64-pd-v4.29.0.tar.xz".to_string());
            urls.insert(Architecture::Riscv64, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.29.0/chimera-riscv64-pd-v4.29.0.tar.xz".to_string());
            urls
        },
        description: "Chimera Linux is a modern, general-purpose non-GNU Linux distribution".to_string(),
        default_packages: vec!["vim".to_string(), "curl".to_string(), "wget".to_string()],
    });
    
    distros.push(DistroDefinition {
        name: DistroName::Adelie,
        display_name: "Adelie".to_string(),
        urls: {
            let mut urls = HashMap::new();
            urls.insert(Architecture::Aarch64, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.29.0/adelie-aarch64-pd-v4.29.0.tar.xz".to_string());
            urls.insert(Architecture::Arm, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.29.0/adelie-armv7-pd-v4.29.0.tar.xz".to_string());
            urls.insert(Architecture::X86_64, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.29.0/adelie-x86_64-pd-v4.29.0.tar.xz".to_string());
            urls
        },
        description: "A Linux distribution built on the shoulders of giants".to_string(),
        default_packages: vec!["vim".to_string(), "curl".to_string(), "wget".to_string()],
    });
    
    distros
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::arch::Architecture;

    #[test]
    fn test_arch_definitions() {
        let definitions = get_definitions();
        assert!(definitions.len() >= 5);
        
        let archlinux = definitions.iter().find(|d| matches!(d.name, DistroName::Archlinux));
        assert!(archlinux.is_some());
        
        let arch_def = archlinux.unwrap();
        assert!(arch_def.supports_arch(&Architecture::Aarch64));
        assert!(arch_def.supports_arch(&Architecture::Arm));
        assert!(arch_def.supports_arch(&Architecture::X86_64));
        assert!(arch_def.supports_arch(&Architecture::I686));
        assert!(!arch_def.supports_arch(&Architecture::Riscv64));
    }

    }
