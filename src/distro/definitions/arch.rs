pub use crate::distro::definitions::arch_core::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::arch::Architecture;

    #[test]
    fn test_arch_definitions() {
        let definitions = get_definitions();
        assert!(definitions.len() >= 5);
        
        let archlinux = definitions.iter().find(|d| matches!(d.name, crate::distro::name::DistroName::Archlinux));
        assert!(archlinux.is_some());
        
        let arch_def = archlinux.unwrap();
        assert!(arch_def.supports_arch(&Architecture::Aarch64));
        assert!(arch_def.supports_arch(&Architecture::Arm));
        assert!(arch_def.supports_arch(&Architecture::X86_64));
        assert!(arch_def.supports_arch(&Architecture::I686));
        assert!(!arch_def.supports_arch(&Architecture::Riscv64));
    }
}