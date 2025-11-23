use std::process::Command;
use crate::i18n::Translator;

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

pub fn run_command_with_translator(command: &str, translator: &Translator) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("bash")
        .arg("-c")
        .arg(command)
        .output()?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let error_msg = translator.t_fmt("command_execution_failed_detailed", &[command, &stdout, &stderr]);
        return Err(error_msg.into());
    }
    
    Ok(())
}

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn check_command_exists(command: &str) -> bool {
    Command::new("which")
        .arg(command)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_command_success() {
        let result = run_command("echo 'test'");
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_command_failure() {
        let result = run_command("false");
        assert!(result.is_err());
    }

    #[test]
    fn test_run_command_with_output_success() {
        let result = run_command_with_output("echo 'hello world'");
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.trim(), "hello world");
    }

    #[test]
    fn test_run_command_with_output_multiline() {
        let result = run_command_with_output("echo 'line1'; echo 'line2'");
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("line1"));
        assert!(output.contains("line2"));
    }

    #[test]
    fn test_check_command_exists_true() {
        let result = check_command_exists("echo");
        assert!(result);
    }

    #[test]
    fn test_check_command_exists_false() {
        let result = check_command_exists("nonexistentcommand12345");
        assert!(!result);
    }
}