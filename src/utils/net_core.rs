use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn check_network_connectivity() -> bool {
    Command::new("ping")
        .args(["-c", "1", "-W", "5", "8.8.8.8"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[allow(dead_code)]
pub fn download_file(url: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("wget")
        .args(["-O", output_path, url])
        .status()?;
    
    if !status.success() {
        return Err(format!("下载失败: {}", url).into());
    }
    
    Ok(())
}

pub fn download_file_with_progress<F>(
    url: &str,
    output_path: &str,
    mut progress_callback: F,
) -> Result<u64, Box<dyn std::error::Error>>
where
    F: FnMut(u64, u64),
{
    let mut child = Command::new("wget")
        .args(["-O", output_path, "--progress=bar:force", url])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    
    let stderr = child.stderr.take().ok_or("Failed to capture stderr")?;
    let reader = BufReader::new(stderr);
    let mut total_size: u64 = 0;
    
    for line_result in reader.lines() {
        let line = line_result?;
        
        if let Some(size) = parse_total_size(&line) {
            total_size = size;
        }
        
        if let Some(current) = parse_current_progress(&line) {
            progress_callback(current, total_size);
        }
        
        if line.contains("100%") && line.contains("=") {
            progress_callback(total_size, total_size);
        }
    }
    
    let status = child.wait()?;
    if !status.success() {
        return Err(format!("下载失败: {}", url).into());
    }
    
    Ok(total_size)
}

fn parse_total_size(line: &str) -> Option<u64> {
    if line.contains("Length:") {
        let parts: Vec<&str> = line.split_whitespace().collect();
        for (i, part) in parts.iter().enumerate() {
            if *part == "Length:" && i + 2 < parts.len() {
                if let Ok(size) = parts[i + 1].parse::<u64>() {
                    return Some(size);
                }
            }
        }
    }
    None
}

fn parse_current_progress(line: &str) -> Option<u64> {
    if line.contains('%') && line.contains('[') {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let mut percent_val = 0.0f64;
        let mut has_percent = false;
        
        for part in &parts {
            if part.ends_with('%') {
                if let Ok(percent) = part.trim_end_matches('%').parse::<f64>() {
                    percent_val = percent;
                    has_percent = true;
                    break;
                }
            }
        }
        
        if has_percent {
            for part in &parts {
                if part.contains('K') || part.contains('M') || part.contains('G') {
                    return parse_size_with_unit(part).map(|total| {
                        (total as f64 * percent_val / 100.0) as u64
                    });
                }
            }
        }
    }
    None
}

fn parse_size_with_unit(s: &str) -> Option<u64> {
    let s = s.trim();
    if s.ends_with('K') {
        s.trim_end_matches('K').parse::<f64>().ok().map(|n| (n * 1024.0) as u64)
    } else if s.ends_with('M') {
        s.trim_end_matches('M').parse::<f64>().ok().map(|n| (n * 1024.0 * 1024.0) as u64)
    } else if s.ends_with('G') {
        s.trim_end_matches('G').parse::<f64>().ok().map(|n| (n * 1024.0 * 1024.0 * 1024.0) as u64)
    } else {
        s.parse::<u64>().ok()
    }
}

pub fn get_content_length(url: &str) -> Result<Option<u64>, Box<dyn std::error::Error>> {
    let output = Command::new("curl")
        .args(["-sI", url])
        .output()?;
    
    if !output.status.success() {
        return Ok(None);
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.starts_with("Content-Length:") {
            if let Ok(length) = line[16..].trim().parse::<u64>() {
                return Ok(Some(length));
            }
        }
    }
    
    Ok(None)
}