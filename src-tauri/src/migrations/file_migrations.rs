use std::{collections::HashMap, fs::File};

use serde::{Deserialize, Serialize};
use tauri::AppHandle;

pub type Wikis = HashMap<String, Wiki>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Wiki {
    name: String,
    description: String,
    author: String,
    site_name: String,
    site_url: String,
    repo_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    deployment_url: String,
}

pub fn run_file_migrations(app_handle: &AppHandle) -> Result<(), String> {
    let base_path = app_handle.path_resolver().app_data_dir().unwrap();

    let wiki_json_file_path = base_path.join("wikis.json");
    let wikis_file = match File::open(&wiki_json_file_path) {
        Ok(file) => file,
        Err(err) => return Err(format!("Failed to read wikis file: {}", err)),
    };
    let wikis: Wikis = match serde_json::from_reader(wikis_file) {
        Ok(wikis) => wikis,
        Err(err) => return Err(format!("Failed to parse wikis file: {}", err)),
    };

    for (wiki_name, wiki) in wikis.iter() {
        println!("Migrating wiki: {}", wiki_name);
    }
    Ok(())
}
