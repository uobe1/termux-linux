use std::collections::HashMap;
use crate::utils::arch::Architecture;
use super::name::DistroName;
use super::definitions::distro_definition::DistroDefinition;

pub fn get_all_distros() -> Vec<DistroDefinition> {
    let mut distros = Vec::new();
    
    distros.extend(super::adelie::get_definition());
    distros.extend(super::debian::get_definitions());
    distros.extend(super::arch::get_definitions());
    distros.extend(super::redhat::get_definitions());
    distros.extend(super::others::get_definitions());
    
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
    use crate::utils::arch::Architecture;

    #[test]
    fn test_all_distros_count() {
        let distros = get_all_distros();
        assert_eq!(distros.len(), 14);
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
}
