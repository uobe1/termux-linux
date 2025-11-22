pub fn get_default_mirror(distro_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    match distro_name.to_lowercase().as_str() {
        "ubuntu" => Ok("https://mirrors.ustc.edu.cn/ubuntu/".to_string()),
        "debian" => Ok("https://mirrors.163.com/debian/".to_string()),
        "kali" => Ok("http://http.kali.org/kali/".to_string()),
        "centos" => Ok("https://mirrors.aliyun.com/centos/".to_string()),
        "fedora" => Ok("https://mirrors.tuna.tsinghua.edu.cn/fedora/".to_string()),
        _ => Err(format!("未找到 {} 的镜像源配置", distro_name).into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_default_mirror_ubuntu() {
        let result = get_default_mirror("ubuntu");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://mirrors.ustc.edu.cn/ubuntu/");
    }

    #[test]
    fn test_get_default_mirror_debian() {
        let result = get_default_mirror("debian");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://mirrors.163.com/debian/");
    }

    #[test]
    fn test_get_default_mirror_kali() {
        let result = get_default_mirror("kali");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "http://http.kali.org/kali/");
    }

    #[test]
    fn test_get_default_mirror_centos() {
        let result = get_default_mirror("centos");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://mirrors.aliyun.com/centos/");
    }

    #[test]
    fn test_get_default_mirror_fedora() {
        let result = get_default_mirror("fedora");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://mirrors.tuna.tsinghua.edu.cn/fedora/");
    }

    #[test]
    fn test_get_default_mirror_case_insensitive() {
        let result = get_default_mirror("Ubuntu");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://mirrors.ustc.edu.cn/ubuntu/");
    }

    #[test]
    fn test_get_default_mirror_invalid() {
        let result = get_default_mirror("nonexistent");
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("未找到"));
        assert!(error_msg.contains("nonexistent"));
    }
}
