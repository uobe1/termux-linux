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
