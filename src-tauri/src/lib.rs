use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Serialize, Deserialize)]
pub struct Site {
    name: String,
    status: String,
    url: String,
}

// Function to strip ANSI color codes from a string
fn strip_ansi_codes(input: &str) -> String {
    // Regular expression to match ANSI escape sequences
    let re = regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    re.replace_all(input, "").to_string()
}

#[tauri::command]
fn get_sites() -> Result<Vec<Site>, String> {
    // Find the sail directory - we assume it's in the home directory
    let home_dir =
        std::env::var("HOME").map_err(|_| "Could not find HOME directory".to_string())?;
    let sail_path = format!("{}/sail", home_dir);
    let sail_binary = format!("{}/bin/sail", sail_path);

    // Run the sail site list command
    let output = Command::new(&sail_binary)
        .args(["site", "list"])
        .current_dir(&sail_path)
        .output()
        .map_err(|e| format!("Failed to execute sail command: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "Sail command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let cleaned_stdout = strip_ansi_codes(&stdout);
    let mut sites = Vec::new();

    // Parse the output - skip the header lines and parse each site
    for line in cleaned_stdout.lines() {
        let line = line.trim();
        if line.starts_with("Sites:") || line.starts_with("------") || line.is_empty() {
            continue;
        }

        // Parse lines like: "  qaat-dev             DOWN   https://qaat-dev.sail"
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            let site = Site {
                name: parts[0].to_string(),
                status: parts[1].to_string(),
                url: parts[2].to_string(),
            };
            sites.push(site);
        }
    }

    Ok(sites)
}

#[tauri::command]
fn open_in_vscode(site_name: &str) -> Result<(), String> {
    // Find the sail directory - we assume it's in the home directory
    let home_dir = std::env::var("HOME").map_err(|_| "Could not find HOME directory".to_string())?;
    let sail_path = format!("{}/sail", home_dir);
    let site_path = format!("{}/sites/{}", sail_path, site_name);
    
    // Check if the site directory exists
    if !std::path::Path::new(&site_path).exists() {
        return Err(format!("Site directory not found: {}", site_path));
    }
    
    // Run the code command
    let output = Command::new("code")
        .arg(&site_path)
        .output()
        .map_err(|e| format!("Failed to execute code command: {}", e))?;
    
    if !output.status.success() {
        return Err(format!("Code command failed: {}", String::from_utf8_lossy(&output.stderr)));
    }
    
    Ok(())
}

#[tauri::command]
fn toggle_site(site_name: &str, current_status: &str) -> Result<(), String> {
    // Find the sail directory - we assume it's in the home directory
    let home_dir = std::env::var("HOME").map_err(|_| "Could not find HOME directory".to_string())?;
    let sail_path = format!("{}/sail", home_dir);
    let site_path = format!("{}/sites/{}", sail_path, site_name);
    let sail_binary = format!("{}/bin/sail", sail_path);
    
    // Check if the site directory exists
    if !std::path::Path::new(&site_path).exists() {
        return Err(format!("Site directory not found: {}", site_path));
    }
    
    // Determine which command to run based on current status
    let command = if current_status == "UP" { "down" } else { "up" };
    
    // Run the sail command from the site directory
    let output = Command::new(&sail_binary)
        .arg(command)
        .current_dir(&site_path)
        .output()
        .map_err(|e| format!("Failed to execute sail {} command: {}", command, e))?;
    
    if !output.status.success() {
        return Err(format!(
            "Sail {} command failed: {}",
            command,
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_sites, open_in_vscode, toggle_site])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
