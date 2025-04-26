use std::{
    collections::HashMap,
    fs::{self, File},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Sqlite};
use tauri::{AppHandle, Manager};

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

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Migration {
    pub name: String,
    pub app_version: String,
    pub execution_order: i32,
    pub sql: String,
}

#[tauri::command]
pub async fn check_and_run_migrations(app_handle: AppHandle) -> Result<String, String> {
    let base_path = app_handle.path().app_data_dir().unwrap();
    let resources_path = app_handle.path().resource_dir().unwrap();

    let migrations = gather_migrations(&base_path, &resources_path)?;
    run_migrations(migrations, &base_path).await?;

    Ok("Migration Completed".to_string())
}

pub fn gather_migrations(
    base_path: &PathBuf,
    resources_path: &PathBuf,
) -> Result<Vec<Migration>, String> {
    let migrations_folder = resources_path.join("resources").join("migrations");
    if !migrations_folder.exists() || !migrations_folder.is_dir() {
        return Err("No migrations folder found".to_string());
    }

    let mut migrations: Vec<Migration> = Vec::new();

    let migrations_from_file = fs::read_dir(migrations_folder).unwrap();

    for migration in migrations_from_file {
        let migration_path = migration.unwrap().path();
        if migration_path.is_file() && migration_path.extension().unwrap() == "json" {
            logger::write_log(
                base_path,
                logger::LogLevel::Debug,
                &format!("Reading migration file: {}", migration_path.display()),
            );
            let migration_content = fs::read_to_string(migration_path).unwrap();
            let migration: Migration = match serde_json::from_str(&migration_content) {
                Ok(m) => m,
                Err(e) => {
                    println!("Error: {}", e);
                    logger::write_log(
                        base_path,
                        logger::LogLevel::MigrationError,
                        &format!("Failed to parse migration: {}", e),
                    );
                    continue;
                }
            };
            migrations.push(migration);
        }
    }

    migrations.sort_by(|a, b| a.execution_order.cmp(&b.execution_order));

    return Ok(migrations);
}

pub async fn run_migrations(
    migrations: Vec<Migration>,
    base_path: &PathBuf,
) -> Result<String, String> {
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

        let sqlite_file_path = wiki_path.join(format!("{}.db", wiki_name));

        // Create backup
        if let Err(err) = std::fs::copy(
            sqlite_file_path.clone(),
            wiki_path.join(format!("{}.db.bak", wiki_name)),
        ) {
            logger::write_log(
                &wiki_path,
                logger::LogLevel::MigrationError,
                &format!("Failed to create backup: {}", err),
            );
            continue;
        };

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

        let existing_migrations = match sqlx::query_as::<_, Migration>("SELECT * FROM migrations")
            .fetch_all(&conn)
            .await
        {
            Ok(mut migrations) => {
                migrations.sort_by(|a, b| a.execution_order.cmp(&b.execution_order));
                let existing_migration_names = migrations
                    .iter()
                    .map(|m| m.name.clone())
                    .collect::<Vec<String>>();
                existing_migration_names
            }
            Err(err) => {
                if err.to_string().contains("no such table") {
                    create_migrations_table(&conn).await?;
                    let existing_migration_names = Vec::new();
                    existing_migration_names
                } else {
                    logger::write_log(
                        &wiki_path,
                        logger::LogLevel::MigrationError,
                        &format!("Failed to fetch existing migrations: {}", err),
                    );
                    continue;
                }
            }
        };

        for migration in migrations.iter() {
            // We check if the migration has already been executed
            // If it has, we skip it
            if existing_migrations.contains(&migration.name) {
                continue;
            }

            if let Err(err) = execute_migration(&wiki_path, &conn, migration).await {
                logger::write_log(
                    &wiki_path,
                    logger::LogLevel::MigrationError,
                    &format!("Failed to execute migration {}: {}", migration.name, err),
                );
                continue;
            }
        }

        logger::write_log(
            &wiki_path,
            logger::LogLevel::MigrationSuccess,
            &format!("Migrations successful"),
        );
    }

    Ok("Migrations Successful".to_string())
}

async fn execute_migration(
    wiki_path: &PathBuf,
    conn: &Pool<Sqlite>,
    migration: &Migration,
) -> Result<(), String> {
    logger::write_log(
        &wiki_path,
        logger::LogLevel::Debug,
        &format!("Executing migration: {}", migration.name),
    );
    if let Err(err) = sqlx::query(&migration.sql).execute(conn).await {
        return Err(err.to_string());
    }
    insert_migration(conn, &migration).await?;

    Ok(())
}

async fn create_migrations_table(conn: &Pool<Sqlite>) -> Result<(), String> {
    let migration: Migration = Migration {
        name: "create_migrations_table".to_string(),
        app_version: "1.7.5".to_string(),
        execution_order: 1,
        sql: "CREATE TABLE IF NOT EXISTS migrations (
                    id INTEGER PRIMARY KEY,
                    name TEXT NOT NULL,
                    app_version TEXT NOT NULL,
                    execution_order INTEGER NOT NULL,
                    sql TEXT NOT NULL
                )"
        .to_string(),
    };

    if let Err(err) = sqlx::query(&migration.sql).execute(conn).await {
        return Err(err.to_string());
    }
    insert_migration(conn, &migration).await?;

    Ok(())
}

async fn insert_migration(conn: &Pool<Sqlite>, migration: &Migration) -> Result<(), String> {
    match sqlx::query(
        "INSERT INTO migrations (name, app_version, execution_order, sql) VALUES (?, ?, ?, ?)",
    )
    .bind(&migration.name)
    .bind(&migration.app_version)
    .bind(&migration.execution_order)
    .bind(&migration.sql)
    .execute(conn)
    .await
    {
        Ok(_) => {}
        Err(err) => return Err(format!("Failed to insert migration: {}", err)),
    }

    Ok(())
}
