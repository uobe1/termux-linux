use std::collections::HashMap;
use crate::utils::arch::Architecture;
use crate::distro::name::DistroName;
use crate::distro::definitions::distro_definition::DistroDefinition;

fn create_distro(
    name: DistroName,
    display_name: &str,
    urls: Vec<(Architecture, &str)>,
    description: &str,
) -> DistroDefinition {
    let mut url_map = HashMap::new();
    for (arch, url) in urls {
        url_map.insert(arch, url.to_string());
    }
    
    DistroDefinition {
        name,
        display_name: display_name.to_string(),
        urls: url_map,
        description: description.to_string(),
        default_packages: vec!["vim".to_string(), "curl".to_string(), "wget".to_string()],
    }
}

pub fn get_definitions() -> Vec<DistroDefinition> {
    vec![
        create_distro(
            DistroName::Archlinux,
            "Arch Linux",
            vec![
                (Architecture::Aarch64, "https://gh-proxy.org/https://github.com/termux/proot-distro/releases/download/v4.29.0/archlinux-aarch64-pd-v4.29.0.tar.xz"),
                (Architecture::Arm, "https://gh-proxy.org/https://github.com/termux/proot-distro/releases/download/v4.29.0/archlinux-arm-pd-v4.29.0.tar.xz"),
                (Architecture::X86_64, "https://gh-proxy.org/https://github.com/termux/proot-distro/releases/download/v4.29.0/archlinux-x86_64-pd-v4.29.0.tar.xz"),
                (Architecture::I686, "https://gh-proxy.org/https://github.com/termux/proot-distro/releases/download/v4.29.0/archlinux-i686-pd-v4.29.0.tar.xz"),
            ],
            "A simple, lightweight Linux distribution",
        ),
        create_distro(
            DistroName::Manjaro,
            "Manjaro",
            vec![
                (Architecture::Aarch64, "https://gh-proxy.org/https://github.com/termux/proot-distro/releases/download/v4.29.0/manjaro-aarch64-pd-v4.29.0.tar.xz"),
            ],
            "Manjaro is a user-friendly Linux distribution based on Arch Linux",
        ),
        create_distro(
            DistroName::Artix,
            "Artix",
            vec![
                (Architecture::Aarch64, "https://gh-proxy.org/https://github.com/termux/proot-distro/releases/download/v4.29.0/artix-aarch64-pd-v4.29.0.tar.xz"),
            ],
            "Artix Linux is a fork of Arch Linux with openrc",
        ),
        create_distro(
            DistroName::Void,
            "Void",
            vec![
                (Architecture::Aarch64, "https://gh-proxy.org/https://github.com/termux/proot-distro/releases/download/v4.29.0/void-aarch64-pd-v4.29.0.tar.xz"),
                (Architecture::Arm, "https://gh-proxy.org/https://github.com/termux/proot-distro/releases/download/v4.29.0/void-arm-pd-v4.29.0.tar.xz"),
                (Architecture::X86_64, "https://gh-proxy.org/https://github.com/termux/proot-distro/releases/download/v4.29.0/void-x86_64-pd-v4.29.0.tar.xz"),
                (Architecture::I686, "https://gh-proxy.org/https://github.com/termux/proot-distro/releases/download/v4.29.0/void-i686-pd-v4.29.0.tar.xz"),
            ],
            "Void is a general purpose operating system, based on the monolithic Linux kernel",
        ),
        create_distro(
            DistroName::Chimera,
            "Chimera",
            vec![
                (Architecture::Aarch64, "https://gh-proxy.org/https://github.com/termux/proot-distro/releases/download/v4.29.0/chimera-aarch64-pd-v4.29.0.tar.xz"),
                (Architecture::X86_64, "https://gh-proxy.org/https://github.com/termux/proot-distro/releases/download/v4.29.0/chimera-x86_64-pd-v4.29.0.tar.xz"),
                (Architecture::Riscv64, "https://gh-proxy.org/https://github.com/termux/proot-distro/releases/download/v4.29.0/chimera-riscv64-pd-v4.29.0.tar.xz"),
            ],
            "Chimera Linux is a modern, general-purpose non-GNU Linux distribution",
        ),
        create_distro(
            DistroName::Adelie,
            "Adelie",
            vec![
                (Architecture::Aarch64, "https://gh-proxy.org/https://github.com/termux/proot-distro/releases/download/v4.29.0/adelie-aarch64-pd-v4.29.0.tar.xz"),
                (Architecture::Arm, "https://gh-proxy.org/https://github.com/termux/proot-distro/releases/download/v4.29.0/adelie-armv7-pd-v4.29.0.tar.xz"),
                (Architecture::X86_64, "https://gh-proxy.org/https://github.com/termux/proot-distro/releases/download/v4.29.0/adelie-x86_64-pd-v4.29.0.tar.xz"),
            ],
            "A Linux distribution built on the shoulders of giants",
        ),
    ]
}