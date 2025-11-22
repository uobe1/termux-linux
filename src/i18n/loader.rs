use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    Chinese,
    English,
}

impl Language {
    pub fn from_code(code: &str) -> Option<Self> {
        match code.to_lowercase().as_str() {
            "zh" | "cn" | "chinese" => Some(Language::Chinese),
            "en" | "english" => Some(Language::English),
            _ => None,
        }
    }
    
    pub fn as_code(&self) -> &'static str {
        match self {
            Language::Chinese => "zh",
            Language::English => "en",
        }
    }
    
    #[allow(dead_code)]
    pub fn as_name(&self) -> &'static str {
        match self {
            Language::Chinese => "中文",
            Language::English => "English",
        }
    }
}

pub struct I18nLoader {
    locale_dir: PathBuf,
}

impl I18nLoader {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let locale_dir = PathBuf::from("/data/data/com.termux/files/home/projects/tl/src/i18n/locales");
        Ok(Self { locale_dir })
    }
    
    pub fn load_language(&self, lang: Language) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let file_name = format!("{}.toml", lang.as_code());
        let file_path = self.locale_dir.join(&file_name);
        
        if !file_path.exists() {
            return Ok(self.get_default_strings(lang));
        }
        
        let content = fs::read_to_string(&file_path)?;
        let strings: HashMap<String, String> = toml::from_str(&content)?;
        
        Ok(strings)
    }
    
    fn get_default_strings(&self, lang: Language) -> HashMap<String, String> {
        let mut strings = HashMap::new();
        
        match lang {
            Language::Chinese => {
                strings.insert("welcome".to_string(), "欢迎使用 TermuxForLinux".to_string());
                strings.insert("install_success".to_string(), "安装成功".to_string());
                strings.insert("uninstall_success".to_string(), "卸载完成".to_string());
                strings.insert("system_not_found".to_string(), "系统不存在".to_string());
                strings.insert("press_enter".to_string(), "按回车键继续...".to_string());
                strings.insert("invalid_choice".to_string(), "无效的选择".to_string());
                strings.insert("installation_complete".to_string(), "安装完成".to_string());
                strings.insert("downloading".to_string(), "正在下载".to_string());
                strings.insert("extracting".to_string(), "正在解压".to_string());
                strings.insert("configuring".to_string(), "正在配置".to_string());
            }
            Language::English => {
                strings.insert("welcome".to_string(), "Welcome to TermuxForLinux".to_string());
                strings.insert("install_success".to_string(), "Installation successful".to_string());
                strings.insert("uninstall_success".to_string(), "Uninstall complete".to_string());
                strings.insert("system_not_found".to_string(), "System not found".to_string());
                strings.insert("press_enter".to_string(), "Press Enter to continue...".to_string());
                strings.insert("invalid_choice".to_string(), "Invalid choice".to_string());
                strings.insert("installation_complete".to_string(), "Installation complete".to_string());
                strings.insert("downloading".to_string(), "Downloading".to_string());
                strings.insert("extracting".to_string(), "Extracting".to_string());
                strings.insert("configuring".to_string(), "Configuring".to_string());
            }
        }
        
        strings
    }
}
