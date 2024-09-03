use std::path::PathBuf;

use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};

pub mod ability_page;
// pub mod evolution_page;
pub mod game_routes;
pub mod item_page;
pub mod nature_page;
mod pokemon_page_generator_functions;
pub mod pokemon_pages;
// pub mod type_page;

pub async fn get_sqlite_connection(sql_lite_file_path: PathBuf) -> Result<Pool<Sqlite>, String> {
    let sqlite_connection_string = format!("sqlite:{}", sql_lite_file_path.to_str().unwrap());
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
