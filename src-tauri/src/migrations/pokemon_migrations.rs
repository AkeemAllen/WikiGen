use std::{collections::HashMap, fs::File, path::PathBuf, str::FromStr};

use sqlx::{migrate::MigrateDatabase, prelude::FromRow, Pool, Sqlite, SqlitePool};

use crate::structs::pokemon_structs::DBPokemon;
use serde::{Deserialize, Serialize};

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

pub async fn check_if_pokemon_already_migrated(marker_pokemon: &str, conn: &Pool<Sqlite>) -> bool {
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
    WHERE pokemon.name = '{}' LIMIT 1",
        marker_pokemon
    );
    let missing_pokemon_exists = match sqlx::query_as::<_, DBPokemon>(&pokemon_query)
        .fetch_optional(conn)
        .await
    {
        Ok(existence) => match existence {
            Some(_) => true,
            None => false,
        },
        Err(_) => false,
    };

    return missing_pokemon_exists;
}

pub async fn add_all_missing_pokemon(
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
                    evolves_into,
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
            .bind(pokemon_db_data.evolves_into)
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
