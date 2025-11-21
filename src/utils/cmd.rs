use std::process::Command;

pub fn run_command(command: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("bash")
        .arg("-c")
        .arg(command)
        .output()?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Err(format!("命令执行失败: {}\nstdout: {}\nstderr: {}", command, stdout, stderr).into());
    }
    
    Ok(())
}

pub fn run_command_with_output(command: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("bash")
        .arg("-c")
        .arg(command)
        .output()?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("命令执行失败: {}\nstderr: {}", command, stderr).into());
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(stdout)
}

pub fn check_command_exists(command: &str) -> bool {
    Command::new("which")
        .arg(command)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}