use crate::utils::permissions;
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
        
        let user_group = permissions::get_current_user()
            .unwrap_or_else(|_| "unknown".to_string());
        
        let permissions = if permissions::is_root_user() {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_meta_new() {
        let meta = SystemMeta::new("test-system".to_string(), "ubuntu".to_string());
        assert_eq!(meta.name, "test-system");
        assert_eq!(meta.os_type, "ubuntu");
        assert!(!meta.created_at.is_empty());
        assert!(!meta.user_group.is_empty());
        assert!(!meta.permissions.is_empty());
        assert!(meta.mirror_url.is_none());
    }
}