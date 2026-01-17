use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vault {
    pub id: String,
    pub path: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
struct ObsidianJson {
    vaults: std::collections::HashMap<String, VaultEntry>,
}

#[derive(Debug, Deserialize)]
struct VaultEntry {
    path: String,
}

#[derive(Debug, Deserialize, Default)]
struct DailyNotesConfig {
    folder: Option<String>,
    format: Option<String>,
    #[allow(dead_code)]
    template: Option<String>,
}

#[tauri::command]
pub fn get_vaults() -> Result<Vec<Vault>, String> {
    let home = dirs::home_dir().ok_or("Could not find home directory")?;
    let obsidian_json_path = home
        .join("Library")
        .join("Application Support")
        .join("obsidian")
        .join("obsidian.json");

    if !obsidian_json_path.exists() {
        return Err("Obsidian config not found. Is Obsidian installed?".to_string());
    }

    let content = fs::read_to_string(&obsidian_json_path)
        .map_err(|e| format!("Failed to read obsidian.json: {}", e))?;

    let obsidian_config: ObsidianJson = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse obsidian.json: {}", e))?;

    let vaults: Vec<Vault> = obsidian_config
        .vaults
        .into_iter()
        .filter_map(|(id, entry)| {
            let path = PathBuf::from(&entry.path);
            if path.exists() {
                let name = path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| entry.path.clone());
                Some(Vault {
                    id,
                    path: entry.path,
                    name,
                })
            } else {
                None
            }
        })
        .collect();

    Ok(vaults)
}

#[tauri::command]
pub fn add_task_to_daily_note(
    vault_path: String,
    task_content: String,
    due_date: Option<String>,
) -> Result<String, String> {
    let vault_path = PathBuf::from(&vault_path);

    // Read daily notes config
    let daily_notes_config = read_daily_notes_config(&vault_path);

    // Determine the daily note path
    let today = Local::now().date_naive();
    let daily_note_path = get_daily_note_path(&vault_path, &daily_notes_config, today)?;

    // Ensure the directory exists
    if let Some(parent) = daily_note_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    // Format the task
    let formatted_task = format_task(&task_content, due_date.as_deref());

    // Read existing content or create new
    let existing_content = fs::read_to_string(&daily_note_path).unwrap_or_default();

    // Append task to the file
    let new_content = if existing_content.is_empty() {
        // Create with header if new file
        let date_str = today.format("%Y-%m-%d").to_string();
        format!("# {}\n\n## Tasks\n\n{}\n", date_str, formatted_task)
    } else if existing_content.contains("## Tasks") {
        // Find the Tasks section and append there
        let mut lines: Vec<&str> = existing_content.lines().collect();
        if let Some(pos) = lines.iter().position(|l| l.starts_with("## Tasks")) {
            // Find the next section or end of file
            let insert_pos = lines
                .iter()
                .skip(pos + 1)
                .position(|l| l.starts_with("## "))
                .map(|p| p + pos + 1)
                .unwrap_or(lines.len());

            // Find the last non-empty line before next section
            let mut actual_insert = insert_pos;
            while actual_insert > pos + 1 && lines.get(actual_insert - 1).map(|l| l.is_empty()).unwrap_or(false) {
                actual_insert -= 1;
            }

            lines.insert(actual_insert, &formatted_task);
            lines.join("\n") + "\n"
        } else {
            format!("{}\n{}\n", existing_content.trim_end(), formatted_task)
        }
    } else {
        // Append Tasks section
        format!(
            "{}\n\n## Tasks\n\n{}\n",
            existing_content.trim_end(),
            formatted_task
        )
    };

    fs::write(&daily_note_path, new_content)
        .map_err(|e| format!("Failed to write to daily note: {}", e))?;

    Ok(daily_note_path.to_string_lossy().to_string())
}

fn read_daily_notes_config(vault_path: &PathBuf) -> DailyNotesConfig {
    let config_path = vault_path
        .join(".obsidian")
        .join("daily-notes.json");

    if let Ok(content) = fs::read_to_string(&config_path) {
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        DailyNotesConfig::default()
    }
}

fn get_daily_note_path(
    vault_path: &PathBuf,
    config: &DailyNotesConfig,
    date: NaiveDate,
) -> Result<PathBuf, String> {
    let format = config.format.as_deref().unwrap_or("YYYY-MM-DD");
    let folder = config.folder.as_deref().unwrap_or("");

    // Convert Obsidian date format to chrono format
    let chrono_format = format
        .replace("YYYY", "%Y")
        .replace("YY", "%y")
        .replace("MM", "%m")
        .replace("DD", "%d")
        .replace("ddd", "%a")
        .replace("dddd", "%A")
        .replace("MMM", "%b")
        .replace("MMMM", "%B");

    let filename = date.format(&chrono_format).to_string();

    let mut path = vault_path.clone();
    if !folder.is_empty() {
        // Handle date patterns in folder names
        let folder_formatted = if folder.contains("YYYY") || folder.contains("MM") || folder.contains("DD") {
            let folder_chrono = folder
                .replace("YYYY", "%Y")
                .replace("YY", "%y")
                .replace("MM", "%m")
                .replace("DD", "%d");
            date.format(&folder_chrono).to_string()
        } else {
            folder.to_string()
        };
        path = path.join(folder_formatted);
    }
    path = path.join(format!("{}.md", filename));

    Ok(path)
}

fn format_task(content: &str, due_date: Option<&str>) -> String {
    let task = content.trim();
    match due_date {
        Some(date) => format!("- [ ] {} \u{23F3} {}", task, date),
        None => format!("- [ ] {}", task),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_task_with_date() {
        let result = format_task("Buy groceries", Some("2024-01-17"));
        assert_eq!(result, "- [ ] Buy groceries \u{23F3} 2024-01-17");
    }

    #[test]
    fn test_format_task_without_date() {
        let result = format_task("Buy groceries", None);
        assert_eq!(result, "- [ ] Buy groceries");
    }
}
