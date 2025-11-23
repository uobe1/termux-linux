use std::collections::HashMap;
use crate::utils::arch::Architecture;
use crate::distro::name::DistroName;
use crate::distro::definitions::distro_definition::DistroDefinition;

pub fn get_definitions() -> Vec<DistroDefinition> {
    let mut distros = Vec::new();
    
    distros.push(DistroDefinition {
        name: DistroName::Fedora,
        display_name: "Fedora".to_string(),
        urls: {
            let mut urls = HashMap::new();
            urls.insert(Architecture::Aarch64, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.29.0/fedora-aarch64-pd-v4.29.0.tar.xz".to_string());
            urls.insert(Architecture::X86_64, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.29.0/fedora-x86_64-pd-v4.29.0.tar.xz".to_string());
            urls
        },
        description: "Fedora creates an innovative, free, and open source platform for hardware, clouds, and containers".to_string(),
        default_packages: vec![
            "@development-tools".to_string(),
            "curl".to_string(),
            "wget".to_string(),
            "git".to_string(),
            "vim".to_string(),
            "htop".to_string(),
            "tmux".to_string(),
            "dnf-plugins-core".to_string(),
        ],
    });
    
    distros.push(DistroDefinition {
        name: DistroName::Rockylinux,
        display_name: "Rocky Linux".to_string(),
        urls: {
            let mut urls = HashMap::new();
            urls.insert(Architecture::Aarch64, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.29.0/rocky-aarch64-pd-v4.29.0.tar.xz".to_string());
            urls.insert(Architecture::X86_64, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.29.0/rocky-x86_64-pd-v4.29.0.tar.xz".to_string());
            urls
        },
        description: "Rocky Linux is a community enterprise operating system designed to be 100% bug-for-bug compatible with Enterprise Linux".to_string(),
        default_packages: vec!["vim".to_string(), "curl".to_string(), "wget".to_string()],
    });
    
    distros
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::arch::Architecture;

    #[test]
    fn test_fedora_definitions() {
        let definitions = get_definitions();
        assert_eq!(definitions.len(), 2);
        
        let fedora = definitions.iter().find(|d| matches!(d.name, DistroName::Fedora));
        assert!(fedora.is_some());
        
        let fedora_def = fedora.unwrap();
        assert!(fedora_def.supports_arch(&Architecture::Aarch64));
        assert!(fedora_def.supports_arch(&Architecture::X86_64));
        assert!(!fedora_def.supports_arch(&Architecture::Arm));
        assert!(!fedora_def.supports_arch(&Architecture::I686));
        assert!(!fedora_def.supports_arch(&Architecture::Riscv64));
    }

    #[test]
    fn test_rocky_definitions() {
        let definitions = get_definitions();
        let rocky = definitions.iter().find(|d| matches!(d.name, DistroName::Rockylinux));
        assert!(rocky.is_some());
        
        let rocky_def = rocky.unwrap();
        assert!(rocky_def.supports_arch(&Architecture::Aarch64));
        assert!(rocky_def.supports_arch(&Architecture::X86_64));
        assert!(!rocky_def.supports_arch(&Architecture::Arm));
        assert!(!rocky_def.supports_arch(&Architecture::I686));
        assert!(!rocky_def.supports_arch(&Architecture::Riscv64));
    }
}
