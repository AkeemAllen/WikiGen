extern crate chrono;
use tauri::AppHandle;

use crate::helpers::copy_recursively;
use chrono::Local;

#[tauri::command]
pub async fn backup_wiki(wiki_name: &str, app_handle: AppHandle) -> Result<String, String> {
    let data_dir = app_handle.path_resolver().app_data_dir().unwrap();
    let base_path = data_dir.join(wiki_name);

    let date = Local::now();

    let backup_dir = data_dir.join("backups").join(format!(
        "{}_{}",
        wiki_name,
        date.format("%Y_%m_%d_%H_%M_%S").to_string()
    ));
    let _ = copy_recursively(base_path, backup_dir);
    Ok("Wiki Backed Up".to_string())
}
