use std::{
    fs::{self, File},
    path::PathBuf,
};

use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};
use tauri::{Emitter, Manager};
use tauri_plugin_store::StoreExt;

use crate::{
    logger::{self, write_log, LogLevel},
    page_generators::game_routes::Routes,
    structs::mkdocs_structs::MKDocsConfig,
};

pub async fn get_sqlite_connection(
    sqlite_file_path: std::path::PathBuf,
) -> Result<Pool<Sqlite>, String> {
    let sqlite_connection_string = format!("sqlite:{}", sqlite_file_path.to_str().unwrap());
    if !Sqlite::database_exists(&sqlite_connection_string)
        .await
        .unwrap_or(false)
    {
        return Err("Database does not exist".to_string());
    }
    match SqlitePool::connect(&sqlite_connection_string).await {
        Ok(conn) => Ok(conn),
        Err(err) => Err(format!("Failed to connect to database: {}", err)),
    }
}

pub fn get_mkdocs_config(file_path: &PathBuf) -> Result<MKDocsConfig, String> {
    let mkdocs_yaml_file = match std::fs::File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            return Err(format!("Failed to open mkdocs yaml file: {}", err));
        }
    };
    let config: MKDocsConfig = match serde_yaml::from_reader(mkdocs_yaml_file) {
        Ok(mkdocs) => mkdocs,
        Err(err) => {
            return Err(format!("Failed to parse mkdocs yaml file: {}", err));
        }
    };

    return Ok(config);
}

pub fn get_routes(file_path: &PathBuf) -> Result<Routes, String> {
    let routes_file = match std::fs::File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            return Err(format!("Failed to open routes file: {}", err));
        }
    };
    let routes: Routes = match serde_yaml::from_reader(routes_file) {
        Ok(routes) => routes,
        Err(err) => {
            return Err(format!("Failed to parse routes file: {}", err));
        }
    };

    return Ok(routes);
}

#[tauri::command]
pub fn load_token(token: &str, app: tauri::AppHandle) -> Result<(), String> {
    let store = match app.store("store.json") {
        Ok(store) => store,
        Err(err) => {
            return Err(format!("Failed to load store: {}", err));
        }
    };
    store.set("token", token);

    match app.emit("token-loaded", ()) {
        Ok(_) => {}
        Err(err) => {
            let error = format!("Failed to emit token-loaded event: {}", err);
            logger::write_log(
                &app.path().app_data_dir().unwrap().join("token_logs"),
                logger::LogLevel::Error,
                &error,
            );
        }
    };

    let window = app.get_webview_window("github-access-request");
    if let Some(window) = window {
        if let Err(err) = window.close() {
            let error = format!("Failed to close window: {}", err);
            logger::write_log(
                &app.path().app_data_dir().unwrap().join("token_logs"),
                logger::LogLevel::Error,
                &error,
            );
        }
    }

    Ok(())
}

pub fn update_mkdocs_yaml(
    wiki_name: &str,
    base_path: &PathBuf,
    mkdocs_config: &MKDocsConfig,
) -> Result<(), String> {
    let mkdocs_yaml_file_path = base_path.join(wiki_name).join("dist").join("mkdocs.yml");

    if let Err(err) = fs::write(
        mkdocs_yaml_file_path,
        serde_yaml::to_string(&mkdocs_config).unwrap(),
    ) {
        let message = format!("{wiki_name}: Failed to update mkdocs yaml file: {err}");
        write_log(&base_path, LogLevel::Error, &message);
        return Err(message);
    }
    Ok(())
}

pub fn create_docs_file(
    wiki_name: &str,
    base_path: &PathBuf,
    file_name: &str,
) -> Result<File, String> {
    let file = match File::create(
        base_path
            .join(wiki_name)
            .join("dist")
            .join("docs")
            .join(format!("{}.md", file_name)),
    ) {
        Ok(file) => file,
        Err(err) => {
            let message = format!("{wiki_name}: Failed to create {file_name} file: {err}");
            write_log(&base_path, LogLevel::Error, &message);
            return Err(message);
        }
    };

    Ok(file)
}

pub fn remove_docs_file(
    wiki_name: &str,
    base_path: &PathBuf,
    file_name: &str,
) -> Result<(), String> {
    let file_path = base_path
        .join(wiki_name)
        .join("dist")
        .join("docs")
        .join(format!("{}.md", file_name));

    if let Err(err) = fs::remove_file(file_path) {
        let message = format!("{wiki_name}: Failed to remove {file_name} file: {err}");
        write_log(&base_path, LogLevel::Error, &message);
        return Err(message);
    }
    Ok(())
}

pub fn page_exists_in_mkdocs(mut mkdocs_config: MKDocsConfig, page_title: &str) -> (bool, usize) {
    let nav_entries = mkdocs_config.nav.as_sequence_mut().unwrap();
    let mut item_page_exists = false;
    let mut page_index = 0;
    for (index, entry) in nav_entries.iter_mut().enumerate() {
        let map_entries = entry.as_mapping_mut().unwrap();
        if let Some(_) = map_entries.get_mut(serde_yaml::Value::String(page_title.to_string())) {
            item_page_exists = true;
            page_index = index;
            break;
        }
    }

    (item_page_exists, page_index)
}
