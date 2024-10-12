use std::{
    collections::HashMap,
    fs::{self, File},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};
use sqlx::{Error, Executor, Pool, Sqlite};
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Migration {
    pub name: String,
    pub app_version: String,
    pub sql: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct MigrationJson {
    migrations_present: bool,
}

#[tauri::command]
pub async fn run_migrations(app_handle: AppHandle) -> Result<String, String> {
    // check for migrations folder for current app version. If not present or folder empty, return early.
    let base_path = app_handle.path_resolver().app_data_dir().unwrap();
    let resources_path = app_handle.path_resolver().resource_dir().unwrap();

    let migrations_file_path = resources_path
        .join("resources")
        .join("migrations")
        .join("migration.json");
    let migration_file = match File::open(&migrations_file_path) {
        Ok(file) => file,
        Err(err) => return Err(format!("Failed to read migrations file: {}", err)),
    };
    let mut migration_json: MigrationJson = match serde_json::from_reader(migration_file) {
        Ok(migration_json) => migration_json,
        Err(err) => return Err(format!("Failed to parse migrations file: {}", err)),
    };

    if migration_json.migrations_present == false {
        return Ok("skipping".to_string());
    }

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

        match create_migrations_table(&conn).await {
            Ok(_) => {
                println!("Migrations table created: {wiki_name}")
            }
            Err(err) => {
                logger::write_log(
                    &wiki_path,
                    logger::LogLevel::MigrationError,
                    &format!("Failed to create migrations table: {}", err),
                );
                continue;
            }
        };

        match update_mega_sharpedo_typo(&conn).await {
            Ok(_) => {}
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
            &format!("Migrations successful"),
        );
    }

    migration_json.migrations_present = false;
    match fs::write(
        &migrations_file_path,
        serde_json::to_string(&mut migration_json).unwrap(),
    ) {
        Ok(_) => {}
        Err(err) => {
            let message = format!("Failed to update migration json: {err}");
            logger::write_log(&base_path, logger::LogLevel::Error, &message);
            return Err(message);
        }
    };

    Ok("Migrations Successful".to_string())
}

async fn create_migrations_table(conn: &Pool<Sqlite>) -> Result<(), Error> {
    let migration: Migration = Migration {
        name: "create_migrations_table".to_string(),
        app_version: "1.7.5".to_string(),
        sql: "CREATE TABLE IF NOT EXISTS migrations (
                    id INTEGER PRIMARY KEY,
                    name TEXT NOT NULL,
                    app_version TEXT NOT NULL,
                    sql TEXT NOT NULL
                )"
        .to_string(),
    };
    match sqlx::query(&migration.sql).execute(conn).await {
        Ok(_) => insert_migration(conn, &migration)
            .await
            .expect("Failed to Insert Migration"),
        Err(err) => return Err(err),
    };

    Ok(())
}

async fn update_mega_sharpedo_typo(conn: &Pool<Sqlite>) -> Result<(), Error> {
    let migration: Migration = Migration {
        name: "update_mega_sharpedo_typo".to_string(),
        app_version: "1.7.5".to_string(),
        sql: "UPDATE pokemon SET name = 'mega-sharpedo' WHERE name = 'meag-sharpedo' AND id = 1095"
            .to_string(),
    };

    match sqlx::query(&migration.sql).execute(conn).await {
        Ok(_) => insert_migration(conn, &migration)
            .await
            .expect("Failed to Insert Migration"),
        Err(err) => return Err(err),
    };

    Ok(())
}

async fn insert_migration(conn: &Pool<Sqlite>, migration: &Migration) -> Result<(), String> {
    match sqlx::query("INSERT INTO migrations (name, app_version, sql) VALUES (?, ?, ?)")
        .bind(&migration.name)
        .bind(&migration.app_version)
        .bind(&migration.sql)
        .execute(conn)
        .await
    {
        Ok(_) => {}
        Err(err) => return Err(format!("Failed to insert migration: {}", err)),
    };

    Ok(())
}
