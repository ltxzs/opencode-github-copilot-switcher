use crate::error::AppError;
use dirs_next::{data_local_dir, home_dir};
use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn update_json_file(
    file_path: &PathBuf,
    transform: impl FnOnce(&mut Value),
) -> Result<(), AppError> {
    if let Some(parent) = file_path.parent() {
        if !parent.exists() {
            let _ = fs::create_dir_all(parent);
        }
    }

    let mut data: Value = if file_path.exists() {
        if let Ok(content) = fs::read_to_string(file_path) {
            serde_json::from_str(&content).unwrap_or_else(|_| json!({}))
        } else {
            json!({})
        }
    } else {
        json!({})
    };

    transform(&mut data);

    let temp_path = file_path.with_extension("json.tmp");
    let content = serde_json::to_string_pretty(&data)?;
    fs::write(&temp_path, content)?;
    fs::rename(temp_path, file_path)?;

    Ok(())
}

fn get_local_dir(app_name: &str) -> Option<PathBuf> {
    data_local_dir().map(|mut p| {
        p.push(app_name);
        p
    })
}

fn get_opencode_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Some(d) = get_local_dir("opencode") {
        dirs.push(d); // %LOCALAPPDATA%\opencode on Windows
    }
    if let Some(mut d) = home_dir() {
        d.push(".local");
        d.push("share");
        d.push("opencode");
        dirs.push(d); // ~/.local/share/opencode
    }
    if let Some(mut d) = home_dir() {
        d.push(".config");
        d.push("opencode");
        dirs.push(d); // ~/.config/opencode
    }
    dirs
}

pub fn read_current_token() -> Option<String> {
    let mut latest_token = None;
    let mut latest_time = std::time::SystemTime::UNIX_EPOCH;

    for mut dir in get_opencode_dirs() {
        dir.push("auth.json");
        if let Ok(metadata) = fs::metadata(&dir) {
            if let Ok(modified) = metadata.modified() {
                if modified >= latest_time {
                    if let Ok(content) = fs::read_to_string(&dir) {
                        if let Ok(data) = serde_json::from_str::<Value>(&content) {
                            let mut found_token = None;
                            // Try direct format
                            if let Some(token) = data.get("github-copilot").and_then(|v| v.get("access")).and_then(|v| v.as_str()) {
                                found_token = Some(token.to_string());
                            } 
                            // Try nested format
                            else if let Some(token) = data.get("auth").and_then(|v| v.get("github-copilot")).and_then(|v| v.get("access")).and_then(|v| v.as_str()) {
                                found_token = Some(token.to_string());
                            }
                            
                            if found_token.is_some() {
                                latest_time = modified;
                                latest_token = found_token;
                            }
                        }
                    }
                }
            }
        }
    }
    latest_token
}

pub fn update_auth_json(access_token: &str, _username: &str) -> Result<(), AppError> {
    let mut wrote_any = false;
    let dirs = get_opencode_dirs();

    let update_fn = |data: &mut Value| {
        if !data.is_object() {
            *data = json!({});
        }
        // Direct injection for newer opencode
        data["github-copilot"] = json!({
            "type": "oauth",
            "refresh": access_token,
            "access": access_token,
            "expires": 0
        });
        // Nested inside auth object if it exists or create it
        if let Some(auth_obj) = data.get_mut("auth") {
            if auth_obj.is_object() {
                auth_obj["github-copilot"] = json!({
                    "type": "oauth",
                    "refresh": access_token,
                    "access": access_token,
                    "expires": 0
                });
            }
        } else {
            data["auth"] = json!({
                "github-copilot": {
                    "type": "oauth",
                    "refresh": access_token,
                    "access": access_token,
                    "expires": 0
                }
            });
        }
    };

    // Only update existing auth.json files to avoid polluting unrelated directories
    for mut dir in dirs.clone() {
        dir.push("auth.json");
        if dir.exists() {
            let _ = update_json_file(&dir, update_fn);
            wrote_any = true;
        }
    }

    // If no auth.json exists anywhere, create one in the most likely default location (first in the list)
    if !wrote_any {
        if let Some(mut fallback_dir) = dirs.first().cloned() {
            fallback_dir.push("auth.json");
            let _ = update_json_file(&fallback_dir, update_fn);
        }
    }

    // Force kill OpenCode node processes to force reload
    let _ = Command::new("powershell")
        .args(&[
            "-WindowStyle", "Hidden",
            "-Command",
            "Get-CimInstance Win32_Process | Where-Object { $_.Name -eq 'node.exe' -and $_.CommandLine -match 'opencode' } | Invoke-CimMethod -MethodName Terminate"
        ])
        .spawn();

    Ok(())
}
