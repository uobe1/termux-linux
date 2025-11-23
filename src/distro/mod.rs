pub mod name;
pub mod meta;
pub mod definitions;

pub use name::DistroName;
pub use meta::SystemMeta;
pub use definitions::distro_definition::DistroDefinition;
pub use definitions::base::{get_all_distros, get_distro_by_name, get_distros_for_arch};
