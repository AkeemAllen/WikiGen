// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_wiki])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use std::{fs, io, path::Path};

#[tauri::command]
fn create_wiki(
    wiki_name: &str,
    wiki_description: &str,
    wiki_author: &str,
    site_name: &str,
    dir: &str,
    resource_dir: &str,
) -> String {
    let base_path: String = format!("{}{}/dist", dir, wiki_name);
    let path_exists: bool = Path::new(&base_path).exists();

    if path_exists {
        return format!("{} already exists", wiki_name);
    }
    fs::create_dir_all(&base_path).unwrap();

    // Copy Starting Data to new wiki
    let target = format!("{}{}/data", dir, wiki_name);
    let source = format!("{}generator_assets/starting_data", resource_dir);
    let _ = copy_recursively(source, target);

    return format!("{} Wiki created and initialized", wiki_name);
}

/// Copy files from source to destination recursively.
pub fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
