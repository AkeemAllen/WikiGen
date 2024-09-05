use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::PathBuf,
};

use sqlx::{
    migrate::MigrateDatabase, sqlite::SqliteQueryResult, Error, Executor, Pool, Sqlite,
    SqliteConnection, SqlitePool,
};
use tauri::AppHandle;

use super::file_migrations::Wikis;

async fn get_sqlite_connection(sql_lite_file_path: PathBuf) -> Result<Pool<Sqlite>, String> {
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

pub async fn run_db_migrations(base_path: &PathBuf) -> Result<(), String> {
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
        let wiki_path = base_path.join(wiki_name);

        if !wiki_path
            .join("migration_errors.txt")
            .try_exists()
            .unwrap_or(false)
        {
            fs::File::create(wiki_path.join("migration_errors.txt"))
                .expect("Failed to create migration_errors.txt");
        }
        let mut wiki_error_file = OpenOptions::new()
            .append(true)
            .open(wiki_path.join("migration_errors.txt"))
            .expect("cannot open file");

        let wiki_backup_file_path = wiki_path.join(format!("{}_pre_hidden_ability.db", wiki_name));
        if !wiki_backup_file_path.exists() {
            fs::copy(
                wiki_path.join(format!("{}.db", wiki_name)),
                wiki_path.join(format!("{}_pre_hidden_ability.db", wiki_name)),
            )
            .expect("Failed to backup wiki's database");
        }

        let sqlite_file_path = wiki_path.join(format!("{}.db", wiki_name));
        let conn = match get_sqlite_connection(sqlite_file_path).await {
            Ok(conn) => conn,
            Err(err) => {
                wiki_error_file
                    .write(format!("Failed to connect to database: {}", err).as_bytes())
                    .expect("Failed to write connection error to file");
                continue;
            }
        };

        match create_item_location_table(&conn).await {
            Ok(_) => {}
            Err(err) => {
                wiki_error_file
                    .write(
                        format!(
                            "Failed to create item_location table in wiki {}: {}",
                            wiki_name, err
                        )
                        .as_bytes(),
                    )
                    .expect("Failed to write connection error to file");
                continue;
            }
        };

        match add_hidden_ability_column_pokemon(&conn).await {
            Ok(_) => {},
            Err(err) => {
                println!("Error: {}", err);
                wiki_error_file
                    .write(
                        format!(
                            "Failed to create item_location table in wiki {}: {}",
                            wiki_name, err
                        )
                        .as_bytes(),
                    )
                    .expect("Failed to write connection error to file");
                continue;
            }
        };

        let mut hidden_ability_column_exists = false;
        match conn.execute("
           SELECT hidden_ability FROM pokemon;
           ").await {
               Ok(_) => hidden_ability_column_exists = true,
               Err(err) => hidden_ability_column_exists = false,
           }
        if !hidden_ability_column_exists {
           match add_hidden_ability_column_pokemon(&conn).await {
                Ok(_) => {}
                Err(err) => {
                     wiki_error_file
                          .write(
                            format!(
                                 "Failed to add hidden_ability column to pokemon table in wiki {}: {}",
                                 wiki_name, err
                            )
                            .as_bytes(),
                          )
                          .expect("Failed to write connection error to file");
                     continue;
                }
           }
        }

        conn.close().await;
    }
    Ok(())
}

async fn create_item_location_table(conn: &Pool<Sqlite>) -> Result<SqliteQueryResult, Error> {
    return conn
        .execute(
            "
            CREATE TABLE IF NOT EXISTS item_location (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                item_name TEXT NOT NULL,
                route TEXT NOT NULL,
                specific_location TEXT,
                method TEXT,
                requirements TEXT
            );
        ",
        )
        .await;
}

async fn add_hidden_ability_column_pokemon(conn: &Pool<Sqlite>) -> Result<SqliteQueryResult, Error> {
    return conn
        .execute(
            "
            ALTER TABLE pokemon
            ADD COLUMN hidden_ability TEXT;
        ",
        )
        .await;
}
