use std::process::Command;

pub fn check_network_connectivity() -> bool {
    Command::new("ping")
        .args(&["-c", "1", "-W", "5", "8.8.8.8"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub fn download_file(url: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("wget")
        .args(&["-O", output_path, url])
        .status()?;
    
    if !status.success() {
        return Err(format!("下载失败: {}", url).into());
    }
    
    Ok(())
}

pub fn check_url_reachable(url: &str) -> bool {
    Command::new("curl")
        .args(&["-I", "--connect-timeout", "5", url])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
