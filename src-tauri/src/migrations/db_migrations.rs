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

pub type PokemonMoves = HashMap<String, HashMap<String, Move>>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Move {
    pub level_learned: u32,
    pub learn_method: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct DBMove {
    pub id: i32,
}

async fn add_all_missing_pokemon(
    wiki_db_conn: &Pool<Sqlite>,
    resources_path: &PathBuf,
) -> Result<(), String> {
    // connect to initial.db
    let sqlite_path = resources_path
        .join("resources")
        .join("generator_assets")
        .join("initial.db");
    let sqlite_connection_string = format!(
        "sqlite:{}",
        sqlite_path.to_str().unwrap().replace("\\\\?\\", "")
    );
    println!("Connecting to {}", &sqlite_connection_string);
    if !Sqlite::database_exists(&sqlite_connection_string)
        .await
        .unwrap_or(false)
    {
        return Err("Database does not exist".to_string());
    }
    let initia_db_conn = match SqlitePool::connect(&sqlite_connection_string).await {
        Ok(conn) => conn,
        Err(err) => {
            return Err(format!("Failed to connect to database: {}", err));
        }
    };

    let moves_file_path = resources_path
        .join("resources")
        .join("generator_assets")
        .join("pokemon_moves.json");
    let proper_move_file_path =
        PathBuf::from_str(&moves_file_path.to_str().unwrap().replace("\\\\?\\", "")).unwrap();
    let pokemon_moves_file = match File::open(proper_move_file_path) {
        Ok(file) => file,
        Err(err) => return Err(format!("Failed to read moves file: {}", err)),
    };
    let pokemon_moves: PokemonMoves = match serde_json::from_reader(pokemon_moves_file) {
        Ok(pokemon_moves) => pokemon_moves,
        Err(err) => return Err(format!("Failed to parse moves file: {}", err)),
    };

    for (pokemon, moves) in pokemon_moves {
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
        WHERE pokemon.name = \"{}\" LIMIT 1",
            &pokemon
        );
        let pokemon_db_data = match sqlx::query_as::<_, DBPokemon>(&pokemon_query)
            .fetch_one(&initia_db_conn)
            .await
        {
            Ok(pokemon_db_data) => pokemon_db_data,
            Err(err) => {
                return Err(format!("Failed to fetch pokemon from database: {}", err));
            }
        };

        let pokemon_insert_query = format!(
            "INSERT INTO pokemon
                (
                    dex_number,
                    name,
                    types,
                    ability_1,
                    ability_2,
                    hidden_ability,
                    hp,
                    attack,
                    defense,
                    sp_attack,
                    sp_defense,
                    speed,
                    evolution_method,
                    evolution_level,
                    evolution_item,
                    evolution_other,
                    evolved_pokemon,
                    render
                )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)"
        );
        let id = match sqlx::query(&pokemon_insert_query)
            .bind(pokemon_db_data.dex_number)
            .bind(pokemon_db_data.name)
            .bind(pokemon_db_data.types)
            .bind(pokemon_db_data.ability_1)
            .bind(pokemon_db_data.ability_2)
            .bind(pokemon_db_data.hidden_ability)
            .bind(pokemon_db_data.hp)
            .bind(pokemon_db_data.attack)
            .bind(pokemon_db_data.defense)
            .bind(pokemon_db_data.sp_attack)
            .bind(pokemon_db_data.sp_defense)
            .bind(pokemon_db_data.speed)
            .bind(pokemon_db_data.evolution_method)
            .bind(pokemon_db_data.evolution_level)
            .bind(pokemon_db_data.evolution_item)
            .bind(pokemon_db_data.evolution_other)
            .bind(pokemon_db_data.evolved_pokemon)
            .bind(pokemon_db_data.render)
            .execute(wiki_db_conn)
            .await
        {
            Ok(id) => id.last_insert_rowid(),
            Err(err) => {
                let err = err.to_string();
                print!("Insert Error: {}", err);
                if !err.contains("UNIQUE constraint failed: pokemon.name") {
                    return Err(format!(
                        "Failed to insert pokemon moveset into database: {}",
                        err
                    ));
                }
                i64::try_from(pokemon_db_data.id).unwrap()
            }
        };

        for (move_name, move_data) in moves {
            let move_query = format!(
                "SELECT id FROM moves WHERE name = \"{}\" LIMIT 1",
                &move_name
            );
            let move_db_data = match sqlx::query_as::<_, DBMove>(&move_query)
                .fetch_one(wiki_db_conn)
                .await
            {
                Ok(move_db_data) => move_db_data,
                Err(err) => {
                    println!("Failed to fetch move {} from database: {}", move_name, err);
                    continue;
                }
            };
            let moveset_query = format!(
                "INSERT INTO pokemon_movesets (pokemon, move, level_learned, learn_method) VALUES ({}, {}, {}, \"{}\")",
                id, move_db_data.id, move_data.level_learned, move_data.learn_method.join(", ")
            );
            sqlx::query(&moveset_query)
                .execute(wiki_db_conn)
                .await
                .unwrap();
        }
    }

    Ok(())
}
