use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    str::FromStr,
};

use serde::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, Executor, FromRow, Pool, Sqlite, SqlitePool};

use crate::{helpers::copy_recursively, structs::pokemon_structs::DBPokemon};

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

pub async fn run_db_migrations(
    base_path: &PathBuf,
    resources_path: &PathBuf,
) -> Result<(), String> {
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

        let wiki_backup_file_path = wiki_path.join(format!("{}_pre_regional_forms.db", wiki_name));
        if !wiki_backup_file_path.exists() {
            fs::copy(
                wiki_path.join(format!("{}.db", wiki_name)),
                wiki_path.join(format!("{}_pre_regional_forms.db", wiki_name)),
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
        let pokemon_query = format!(
            "SELECT
            pokemon.*,
            a1.effect as a1_effect,
            a2.effect as a2_effect,
            h3.effect as h3_effect
        FROM pokemon
        LEFT JOIN abilities a1 on a1.name = pokemon.ability_1
        LEFT JOIN abilities a2 on a2.name = pokemon.ability_2
        LEFT JOIN abilities h3 on h3.name = pokemon.hidden_ability
        WHERE pokemon.name = 'meowth-alolan' LIMIT 1",
        );
        let missing_pokemon_exists = match sqlx::query_as::<_, DBPokemon>(&pokemon_query)
            .fetch_optional(&conn)
            .await
        {
            Ok(existence) => match existence {
                Some(_) => true,
                None => false,
            },
            Err(_) => false,
        };
        println!("Missing pokemon exists: {}", missing_pokemon_exists);
        if !missing_pokemon_exists {
            match add_all_missing_pokemon(&conn, resources_path).await {
                Ok(_) => {
                    let wiki_sprite_dir = wiki_path
                        .join("dist")
                        .join("docs")
                        .join("img")
                        .join("pokemon");
                    fs::remove_dir_all(&wiki_sprite_dir).unwrap();
                    copy_recursively(
                        resources_path
                            .join("resources")
                            .join("generator_assets")
                            .join("pokemon_sprites"),
                        &wiki_sprite_dir,
                    )
                    .expect("Failed to copy sprites");
                }
                Err(err) => {
                    wiki_error_file
                        .write(format!("Failed to add missing pokemon: {}", err).as_bytes())
                        .expect("Failed to write error to file");
                    continue;
                }
            }
        }
        conn.close().await;
    }
    Ok(())
}
