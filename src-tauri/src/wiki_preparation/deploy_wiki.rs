use std::process::Command;

use tauri::AppHandle;

use crate::logger::{self, LogLevel};

#[tauri::command]
pub fn deploy_wiki(wiki_name: &str, app_handle: AppHandle) -> Result<String, String> {
    let data_dir = app_handle.path_resolver().app_data_dir().unwrap();
    let base_path = data_dir.join(wiki_name);
    let dist_directory = base_path.join("dist");

    // Check if git is initialized
    if !dist_directory.join(".git").try_exists().unwrap_or(false) {
        println!("{}", format!("{}", dist_directory.to_str().unwrap()));
        match Command::new("git")
            .arg("init")
            .arg(format!("{}", dist_directory.to_str().unwrap()))
            .output()
        {
            Ok(_) => {}
            Err(err) => {
                let error = format!("Error while initializing repo: {}", err);
                logger::write_log(&base_path, LogLevel::Error, &error);
                return Err(error);
            }
        };
    }

    Ok("Deploying Wiki".to_string())
}
