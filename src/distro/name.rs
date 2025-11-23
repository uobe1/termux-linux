use std::fmt;

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

impl fmt::Display for DistroName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distro_name_as_str() {
        assert_eq!(DistroName::Ubuntu.as_str(), "ubuntu");
        assert_eq!(DistroName::Debian.as_str(), "debian");
        assert_eq!(DistroName::Archlinux.as_str(), "archlinux");
    }

    #[test]
    fn test_distro_name_to_string() {
        assert_eq!(DistroName::Ubuntu.to_string(), "Ubuntu");
        assert_eq!(DistroName::Opensuse.to_string(), "openSUSE");
        assert_eq!(DistroName::Rockylinux.to_string(), "Rocky Linux");
    }

    #[test]
    fn test_distro_name_display() {
        assert_eq!(format!("{}", DistroName::Alpine), "alpine");
        assert_eq!(format!("{}", DistroName::Fedora), "fedora");
    }
}
