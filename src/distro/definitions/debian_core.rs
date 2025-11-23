use std::collections::HashMap;
use crate::utils::arch::Architecture;
use crate::distro::name::DistroName;
use crate::distro::definitions::distro_definition::DistroDefinition;

pub fn get_definitions() -> Vec<DistroDefinition> {
    let mut distros = Vec::new();
    
    distros.push(DistroDefinition {
        name: DistroName::Debian,
        display_name: "Debian".to_string(),
        urls: {
            let mut urls = HashMap::new();
            urls.insert(Architecture::Aarch64, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.29.0/debian-trixie-aarch64-pd-v4.29.0.tar.xz".to_string());
            urls.insert(Architecture::Arm, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.29.0/debian-trixie-arm-pd-v4.29.0.tar.xz".to_string());
            urls.insert(Architecture::X86_64, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.29.0/debian-trixie-x86_64-pd-v4.29.0.tar.xz".to_string());
            urls.insert(Architecture::I686, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.29.0/debian-trixie-i686-pd-v4.29.0.tar.xz".to_string());
            urls
        },
        description: "The universal operating system".to_string(),
        default_packages: vec![
            "build-essential".to_string(),
            "devscripts".to_string(),
            "curl".to_string(),
            "wget".to_string(),
            "git".to_string(),
            "vim".to_string(),
            "htop".to_string(),
            "tmux".to_string(),
        ],
    });
    
    distros.push(DistroDefinition {
        name: DistroName::Ubuntu,
        display_name: "Ubuntu".to_string(),
        urls: {
            let mut urls = HashMap::new();
            urls.insert(Architecture::Aarch64, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.30.1/ubuntu-questing-aarch64-pd-v4.30.1.tar.xz".to_string());
            urls.insert(Architecture::Arm, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.30.1/ubuntu-questing-arm-pd-v4.30.1.tar.xz".to_string());
            urls.insert(Architecture::X86_64, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.30.1/ubuntu-questing-x86_64-pd-v4.30.1.tar.xz".to_string());
            urls
        },
        description: "Ubuntu is a Debian-based Linux operating system".to_string(),
        default_packages: vec![
            "build-essential".to_string(),
            "curl".to_string(),
            "wget".to_string(),
            "git".to_string(),
            "vim".to_string(),
            "htop".to_string(),
        ],
    });
    
    distros.push(DistroDefinition {
        name: DistroName::Deepin,
        display_name: "Deepin".to_string(),
        urls: {
            let mut urls = HashMap::new();
            urls.insert(Architecture::Aarch64, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.29.0/deepin-aarch64-pd-v4.29.0.tar.xz".to_string());
            urls.insert(Architecture::X86_64, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.29.0/deepin-x86_64-pd-v4.29.0.tar.xz".to_string());
            urls
        },
        description: "Deepin is a Linux distribution based on Debian".to_string(),
        default_packages: vec!["vim".to_string(), "curl".to_string(), "wget".to_string()],
    });
    
    distros.push(DistroDefinition {
        name: DistroName::Pardus,
        display_name: "Pardus".to_string(),
        urls: {
            let mut urls = HashMap::new();
            urls.insert(Architecture::Aarch64, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.29.0/pardus-aarch64-pd-v4.29.0.tar.xz".to_string());
            urls.insert(Architecture::X86_64, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.29.0/pardus-x86_64-pd-v4.29.0.tar.xz".to_string());
            urls.insert(Architecture::I686, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.29.0/pardus-i686-pd-v4.29.0.tar.xz".to_string());
            urls
        },
        description: "Pardus is a Debian-based Linux distribution".to_string(),
        default_packages: vec!["vim".to_string(), "curl".to_string(), "wget".to_string()],
    });
    
    distros
}