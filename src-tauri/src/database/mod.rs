use std::path::PathBuf;

use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};

use crate::{page_generators::game_routes::Routes, structs::mkdocs_structs::MKDocsConfig};

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
