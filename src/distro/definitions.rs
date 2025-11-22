use std::collections::HashMap;
use crate::utils::arch::Architecture;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DistroName {
    Adelie,
    Deepin,
    Debian,
    Chimera,
    Opensuse,
    Artix,
    Manjaro,
    Archlinux,
    Void,
    Fedora,
    Ubuntu,
    Rockylinux,
    Alpine,
    Pardus,
}

impl DistroName {
    pub fn as_str(&self) -> &'static str {
        match self {
            DistroName::Adelie => "adelie",
            DistroName::Deepin => "deepin",
            DistroName::Debian => "debian",
            DistroName::Chimera => "chimera",
            DistroName::Opensuse => "opensuse",
            DistroName::Artix => "artix",
            DistroName::Manjaro => "manjaro",
            DistroName::Archlinux => "archlinux",
            DistroName::Void => "void",
            DistroName::Fedora => "fedora",
            DistroName::Ubuntu => "ubuntu",
            DistroName::Rockylinux => "rockylinux",
            DistroName::Alpine => "alpine",
            DistroName::Pardus => "pardus",
        }
    }
    
    pub fn to_string(&self) -> String {
        match self {
            DistroName::Adelie => "Adelie".to_string(),
            DistroName::Deepin => "Deepin".to_string(),
            DistroName::Debian => "Debian".to_string(),
            DistroName::Chimera => "Chimera".to_string(),
            DistroName::Opensuse => "openSUSE".to_string(),
            DistroName::Artix => "Artix".to_string(),
            DistroName::Manjaro => "Manjaro".to_string(),
            DistroName::Archlinux => "Arch Linux".to_string(),
            DistroName::Void => "Void".to_string(),
            DistroName::Fedora => "Fedora".to_string(),
            DistroName::Ubuntu => "Ubuntu".to_string(),
            DistroName::Rockylinux => "Rocky Linux".to_string(),
            DistroName::Alpine => "Alpine".to_string(),
            DistroName::Pardus => "Pardus".to_string(),
        }
    }
}

use std::collections::HashMap;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct SystemMeta {
    pub name: String,
    pub os_type: String,
    pub created_at: String,
    pub user_group: String,
    pub permissions: String,
    pub mirror_url: Option<String>,
}

impl SystemMeta {
    pub fn new(name: String, os_type: String) -> Self {
        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| {
                let secs = d.as_secs();
                format!(
                    "{}",
                    chrono::DateTime::from_timestamp(secs as i64, 0)
                        .map(|dt| dt.format("%Y-%m-%dT%H:%M:%SZ").to_string())
                        .unwrap_or_else(|| format!("2025-01-01T00:00:00Z"))
                )
            })
            .unwrap_or_else(|_| "2025-01-01T00:00:00Z".to_string());
        
        let user_group = crate::utils::permissions::get_current_user()
            .unwrap_or_else(|_| "unknown".to_string());
        
        let permissions = if crate::utils::permissions::is_root_user() {
            "755".to_string()
        } else {
            "644".to_string()
        };
        
        Self {
            name,
            os_type,
            created_at,
            user_group,
            permissions,
            mirror_url: None,
        }
    }
    
    pub fn to_string(&self) -> String {
        let mut result = format!("name = {}\n", self.name);
        result.push_str(&format!("os_type = {}\n", self.os_type));
        result.push_str(&format!("created_at = {}\n", self.created_at));
        result.push_str(&format!("user_group = {}\n", self.user_group));
        result.push_str(&format!("permissions = {}\n", self.permissions));
        if let Some(mirror) = &self.mirror_url {
            result.push_str(&format!("mirror_url = {}\n", mirror));
        }
        result
    }
    
    pub fn from_string(content: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut map = HashMap::new();
        
        for line in content.lines() {
            if let Some((key, value)) = line.split_once('=') {
                map.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
        
        let user_group = map.get("user_group")
            .map(|s| s.to_string())
            .or_else(|| crate::utils::permissions::get_current_user().ok())
            .unwrap_or_else(|| "unknown".to_string());
        
        let permissions = map.get("permissions")
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                if crate::utils::permissions::is_root_user() {
                    "755".to_string()
                } else {
                    "644".to_string()
                }
            });
        
        let created_at = map.get("created_at")
            .map(|s| s.to_string())
            .or_else(|| {
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .ok()
                    .map(|d| {
                        let secs = d.as_secs();
                        chrono::DateTime::from_timestamp(secs as i64, 0)
                            .map(|dt| dt.format("%Y-%m-%dT%H:%M:%SZ").to_string())
                            .unwrap_or_else(|| "2025-01-01T00:00:00Z".to_string())
                    })
            })
            .unwrap_or_else(|| "2025-01-01T00:00:00Z".to_string());
        
        Ok(SystemMeta {
            name: map.get("name").unwrap_or(&"".to_string()).clone(),
            os_type: map.get("os_type").unwrap_or(&"".to_string()).clone(),
            created_at,
            user_group,
            permissions,
            mirror_url: map.get("mirror_url").cloned(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct DistroDefinition {
    pub name: DistroName,
    pub display_name: String,
    pub urls: HashMap<Architecture, String>,
    pub description: String,
    pub default_packages: Vec<String>,
}

impl DistroDefinition {
    pub fn get_url(&self, arch: &Architecture) -> Option<&String> {
        self.urls.get(arch)
    }
    
    pub fn supports_arch(&self, arch: &Architecture) -> bool {
        self.urls.contains_key(arch)
    }
}

pub fn get_all_distros() -> Vec<DistroDefinition> {
    let mut distros = Vec::new();
    
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

pub fn get_distro_by_name(name: &str) -> Option<DistroDefinition> {
    let all_distros = get_all_distros();
    all_distros.into_iter().find(|d| d.name.as_str() == name.to_lowercase())
}

pub fn get_distros_for_arch(arch: &Architecture) -> Vec<DistroDefinition> {
    let all_distros = get_all_distros();
    all_distros.into_iter()
        .filter(|d| d.supports_arch(arch))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_distros_count() {
        let distros = get_all_distros();
        assert_eq!(distros.len(), 14);
    }

    #[test]
    fn test_distro_name_as_str() {
        assert_eq!(DistroName::Ubuntu.as_str(), "ubuntu");
        assert_eq!(DistroName::Debian.as_str(), "debian");
        assert_eq!(DistroName::Archlinux.as_str(), "archlinux");
    }

    #[test]
    fn test_get_distro_by_name() {
        let distro = get_distro_by_name("ubuntu");
        assert!(distro.is_some());
        assert_eq!(distro.unwrap().name.as_str(), "ubuntu");
        
        let distro = get_distro_by_name("nonexistent");
        assert!(distro.is_none());
    }

    #[test]
    fn test_get_distros_for_arch() {
        let aarch64_distros = get_distros_for_arch(&Architecture::Aarch64);
        assert!(aarch64_distros.len() > 0);
        
        for distro in &aarch64_distros {
            assert!(distro.supports_arch(&Architecture::Aarch64));
        }
    }

    #[test]
    fn test_ubuntu_urls() {
        let ubuntu = get_distro_by_name("ubuntu").unwrap();
        assert!(ubuntu.supports_arch(&Architecture::Aarch64));
        assert!(ubuntu.supports_arch(&Architecture::Arm));
        assert!(ubuntu.supports_arch(&Architecture::X86_64));
        assert!(!ubuntu.supports_arch(&Architecture::I686));
        assert!(!ubuntu.supports_arch(&Architecture::Riscv64));
    }

    #[test]
    fn test_alpine_all_archs() {
        let alpine = get_distro_by_name("alpine").unwrap();
        assert!(alpine.supports_arch(&Architecture::Aarch64));
        assert!(alpine.supports_arch(&Architecture::Arm));
        assert!(alpine.supports_arch(&Architecture::X86_64));
        assert!(alpine.supports_arch(&Architecture::I686));
        assert!(alpine.supports_arch(&Architecture::Riscv64));
    }
}
