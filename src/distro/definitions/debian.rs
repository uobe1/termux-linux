pub use crate::distro::definitions::debian_core::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::arch::Architecture;

    #[test]
    fn test_debian_definitions() {
        let definitions = get_definitions();
        assert!(definitions.len() >= 4);
        
        let debian = definitions.iter().find(|d| matches!(d.name, crate::distro::name::DistroName::Debian));
        assert!(debian.is_some());
        
        let debian_def = debian.unwrap();
        assert!(debian_def.supports_arch(&Architecture::Aarch64));
        assert!(debian_def.supports_arch(&Architecture::Arm));
        assert!(debian_def.supports_arch(&Architecture::X86_64));
        assert!(debian_def.supports_arch(&Architecture::I686));
        assert!(!debian_def.supports_arch(&Architecture::Riscv64));
    }

    #[test]
    fn test_ubuntu_definitions() {
        let definitions = get_definitions();
        let ubuntu = definitions.iter().find(|d| matches!(d.name, crate::distro::name::DistroName::Ubuntu));
        assert!(ubuntu.is_some());
        
        let ubuntu_def = ubuntu.unwrap();
        assert!(ubuntu_def.supports_arch(&Architecture::Aarch64));
        assert!(ubuntu_def.supports_arch(&Architecture::Arm));
        assert!(ubuntu_def.supports_arch(&Architecture::X86_64));
        assert!(!ubuntu_def.supports_arch(&Architecture::I686));
        assert!(!ubuntu_def.supports_arch(&Architecture::Riscv64));
    }
}