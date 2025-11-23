use std::collections::HashMap;
use crate::utils::arch::Architecture;
use crate::distro::name::DistroName;
use crate::distro::definitions::distro_definition::DistroDefinition;

pub fn get_definitions() -> Vec<DistroDefinition> {
    let mut distros = Vec::new();
    
    distros.push(DistroDefinition {
        name: DistroName::Alpine,
        display_name: "Alpine".to_string(),
        urls: {
            let mut urls = HashMap::new();
            urls.insert(Architecture::Aarch64, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.30.1/alpine-aarch64-pd-v4.30.1.tar.xz".to_string());
            urls.insert(Architecture::Arm, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.30.1/alpine-arm-pd-v4.30.1.tar.xz".to_string());
            urls.insert(Architecture::X86_64, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.30.1/alpine-x86_64-pd-v4.30.1.tar.xz".to_string());
            urls.insert(Architecture::I686, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.30.1/alpine-i686-pd-v4.30.1.tar.xz".to_string());
            urls.insert(Architecture::Riscv64, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.30.1/alpine-riscv64-pd-v4.30.1.tar.xz".to_string());
            urls
        },
        description: "Alpine Linux is a security-oriented, lightweight Linux distribution based on musl libc and busybox".to_string(),
        default_packages: vec!["vim".to_string(), "curl".to_string(), "wget".to_string()],
    });
    
    distros.push(DistroDefinition {
        name: DistroName::Opensuse,
        display_name: "openSUSE".to_string(),
        urls: {
            let mut urls = HashMap::new();
            urls.insert(Architecture::Aarch64, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.29.0/opensuse-aarch64-pd-v4.29.0.tar.xz".to_string());
            urls.insert(Architecture::Arm, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.29.0/opensuse-arm-pd-v4.29.0.tar.xz".to_string());
            urls.insert(Architecture::X86_64, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.29.0/opensuse-x86_64-pd-v4.29.0.tar.xz".to_string());
            urls.insert(Architecture::I686, "https://ghproxy.com/https://github.com/termux/proot-distro/releases/download/v4.29.0/opensuse-i686-pd-v4.29.0.tar.xz".to_string());
            urls
        },
        description: "The makers' choice for sysadmins, developers and desktop users".to_string(),
        default_packages: vec!["vim".to_string(), "curl".to_string(), "wget".to_string()],
    });
    
    distros
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::arch::Architecture;

    #[test]
    fn test_alpine_all_archs() {
        let definitions = get_definitions();
        let alpine = definitions.iter().find(|d| matches!(d.name, DistroName::Alpine));
        assert!(alpine.is_some());
        
        let alpine_def = alpine.unwrap();
        assert!(alpine_def.supports_arch(&Architecture::Aarch64));
        assert!(alpine_def.supports_arch(&Architecture::Arm));
        assert!(alpine_def.supports_arch(&Architecture::X86_64));
        assert!(alpine_def.supports_arch(&Architecture::I686));
        assert!(alpine_def.supports_arch(&Architecture::Riscv64));
    }

    #[test]
    fn test_opensuse_definitions() {
        let definitions = get_definitions();
        let opensuse = definitions.iter().find(|d| matches!(d.name, DistroName::Opensuse));
        assert!(opensuse.is_some());
        
        let opensuse_def = opensuse.unwrap();
        assert!(opensuse_def.supports_arch(&Architecture::Aarch64));
        assert!(opensuse_def.supports_arch(&Architecture::Arm));
        assert!(opensuse_def.supports_arch(&Architecture::X86_64));
        assert!(opensuse_def.supports_arch(&Architecture::I686));
        assert!(!opensuse_def.supports_arch(&Architecture::Riscv64));
    }
}
