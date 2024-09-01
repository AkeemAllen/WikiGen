use std::fs::File;

use tauri::AppHandle;

use super::file_migrations::Wikis;

pub async fn run_db_migrations(app_handle: &AppHandle) -> Result<(), String> {
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
