use std::collections::HashMap;
use crate::utils::arch::Architecture;
use crate::distro::name::DistroName;

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