use std::{collections::HashMap, fs::File, path::PathBuf};

use serde::{Deserialize, Serialize};
use sqlx::Executor;
use tauri::AppHandle;

use crate::{database::get_sqlite_connection, logger};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Manifest {
    pub body: String,
    pub data: String,
    pub version: String,
}

#[tauri::command]
pub async fn run_migrations(app_handle: AppHandle) -> Result<(), String> {
    // check for migrations folder for current app version. If not present or folder empty, return early.
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

    for (wiki_name, _) in wikis.iter() {
        if wiki_name != "migration-tests" {
            continue;
        }
        let wiki_path = base_path.join(wiki_name);

        if !wiki_path.exists() {
            return Err(format!("Wiki path does not exist: {:?}", wiki_path));
        }

        // Create versioned backups

        let sqlite_file_path = wiki_path.join(format!("{}.db", wiki_name));
        let conn = match get_sqlite_connection(sqlite_file_path).await {
            Ok(conn) => conn,
            Err(err) => {
                logger::write_log(
                    &wiki_path,
                    logger::LogLevel::MigrationError,
                    &format!("Failed to connect to database: {}", err),
                );
                continue;
            }
        };
        let result = match sqlx::query(
            "INSERT INTO abilities (name, effect) VALUES ('migration-test-3', 'migration-test-3')",
        )
        .execute(&conn)
        .await
        {
            Ok(result) => result,
            Err(err) => {
                logger::write_log(
                    &wiki_path,
                    logger::LogLevel::MigrationError,
                    &format!("Failed to execute migration: {}", err),
                );
                continue;
            }
        };

        logger::write_log(
            &wiki_path,
            logger::LogLevel::MigrationSuccess,
            &format!("Migration successful: {:?}", result),
        );
    }
    Ok(())
}
