use std::collections::HashMap;
use super::SystemMeta;
use crate::utils::permissions;
use std::time::{SystemTime, UNIX_EPOCH};

impl SystemMeta {
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
            .or_else(|| permissions::get_current_user().ok())
            .unwrap_or_else(|| "unknown".to_string());
        
        let permissions = map.get("permissions")
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                if permissions::is_root_user() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_meta_to_string() {
        let meta = SystemMeta {
            name: "test".to_string(),
            os_type: "debian".to_string(),
            created_at: "2025-01-01T00:00:00Z".to_string(),
            user_group: "user:group".to_string(),
            permissions: "755".to_string(),
            mirror_url: Some("https://mirror.example.com".to_string()),
        };
        
        let content = meta.to_string();
        assert!(content.contains("name = test"));
        assert!(content.contains("os_type = debian"));
        assert!(content.contains("created_at = 2025-01-01T00:00:00Z"));
        assert!(content.contains("user_group = user:group"));
        assert!(content.contains("permissions = 755"));
        assert!(content.contains("mirror_url = https://mirror.example.com"));
    }

    #[test]
    fn test_system_meta_from_string() {
        let content = r#"name = test-system
os_type = fedora
created_at = 2025-01-01T12:00:00Z
user_group = testuser:testgroup
permissions = 644
mirror_url = https://mirror.example.com
"#;
        
        let meta = SystemMeta::from_string(content).unwrap();
        assert_eq!(meta.name, "test-system");
        assert_eq!(meta.os_type, "fedora");
        assert_eq!(meta.created_at, "2025-01-01T12:00:00Z");
        assert_eq!(meta.user_group, "testuser:testgroup");
        assert_eq!(meta.permissions, "644");
        assert_eq!(meta.mirror_url, Some("https://mirror.example.com".to_string()));
    }

    #[test]
    fn test_system_meta_from_string_partial() {
        let content = r#"name = partial-system
os_type = alpine
"#;
        
        let meta = SystemMeta::from_string(content).unwrap();
        assert_eq!(meta.name, "partial-system");
        assert_eq!(meta.os_type, "alpine");
        assert!(!meta.created_at.is_empty());
        assert!(!meta.user_group.is_empty());
        assert!(!meta.permissions.is_empty());
        assert!(meta.mirror_url.is_none());
    }
}