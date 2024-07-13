use std::{collections::HashMap, fs::File};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, SqlitePool};
use tauri::AppHandle;

use crate::structs::pokemon_structs::Pokemon;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Item {
    effect: String,
    sprite: Option<String>,
}

type Items = HashMap<String, Item>;

#[tauri::command]
pub async fn convert_items_to_sqlite(
    wiki_name: &str,
    app_handle: AppHandle,
) -> Result<String, String> {
    let base_path = app_handle.path_resolver().app_data_dir().unwrap();

    let json_items_file_path = base_path.join(wiki_name).join("data").join("items.json");
    let json_items_file = File::open(&json_items_file_path).unwrap();
    let json_items: Items = match serde_json::from_reader(json_items_file) {
        Ok(file) => file,
        Err(err) => return Err(format!("Failed to read items json file: {}", err)),
    };

    let sqlite_path = base_path.join(wiki_name).join(format!("{}.db", wiki_name));
    let sqlite_connection_string = format!("sqlite:{}", sqlite_path.to_str().unwrap());

    let conn = match SqlitePool::connect(&sqlite_connection_string).await {
        Ok(conn) => conn,
        Err(err) => {
            return Err(format!("Failed to connect to database: {}", err));
        }
    };

    for (name, item) in json_items {
        let effect = &item.effect;
        let result = sqlx::query("INSERT INTO items (name, effect) VALUES (?, ?)")
            .bind(&name)
            .bind(effect)
            .execute(&conn)
            .await
            .unwrap();

        println!("{:?}", result);
    }

    conn.close().await;
    Ok("Success".to_string())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Ability {
    effect: String,
}

type Abilties = HashMap<String, Ability>;

#[tauri::command]
pub async fn convert_abilities_to_sqlite(
    wiki_name: &str,
    app_handle: AppHandle,
) -> Result<String, String> {
    let base_path = app_handle.path_resolver().app_data_dir().unwrap();

    let json_abilities_file_path = base_path
        .join(wiki_name)
        .join("data")
        .join("abilities.json");
    let json_abilities_file = File::open(&json_abilities_file_path).unwrap();
    let json_abilities: Abilties = match serde_json::from_reader(json_abilities_file) {
        Ok(file) => file,
        Err(err) => return Err(format!("Failed to read abilities json file: {}", err)),
    };

    let sqlite_path = base_path.join(wiki_name).join(format!("{}.db", wiki_name));
    let sqlite_connection_string = format!("sqlite:{}", sqlite_path.to_str().unwrap());

    let conn = match SqlitePool::connect(&sqlite_connection_string).await {
        Ok(conn) => conn,
        Err(err) => {
            return Err(format!("Failed to connect to database: {}", err));
        }
    };

    for (name, ability) in json_abilities {
        let effect = &ability.effect;
        let result = sqlx::query("INSERT INTO abilities (name, effect) VALUES (?, ?)")
            .bind(&name)
            .bind(effect)
            .execute(&conn)
            .await
            .unwrap();

        println!("{:?}", result);
    }

    conn.close().await;
    Ok("Success".to_string())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Nature {
    increased_stat: Option<String>,
    decreased_stat: Option<String>,
}

type Natures = HashMap<String, Nature>;

#[tauri::command]
pub async fn convert_natures_to_sqlite(
    wiki_name: &str,
    app_handle: AppHandle,
) -> Result<String, String> {
    let base_path = app_handle.path_resolver().app_data_dir().unwrap();

    let json_natures_file_path = base_path.join(wiki_name).join("data").join("natures.json");
    let json_natures_file = File::open(&json_natures_file_path).unwrap();
    let json_natures: Natures = match serde_json::from_reader(json_natures_file) {
        Ok(file) => file,
        Err(err) => return Err(format!("Failed to read natures json file: {}", err)),
    };

    let sqlite_path = base_path.join(wiki_name).join(format!("{}.db", wiki_name));
    let sqlite_connection_string = format!("sqlite:{}", sqlite_path.to_str().unwrap());

    let conn = match SqlitePool::connect(&sqlite_connection_string).await {
        Ok(conn) => conn,
        Err(err) => {
            return Err(format!("Failed to connect to database: {}", err));
        }
    };

    for (name, nature) in json_natures {
        let increased_stat = nature.increased_stat.clone();
        let decreased_stat = nature.decreased_stat.clone();

        let result = sqlx::query(
            "INSERT INTO natures (name, increased_stat, decreased_stat) VALUES (?, ?, ?)",
        )
        .bind(&name)
        .bind(increased_stat)
        .bind(decreased_stat)
        .execute(&conn)
        .await
        .unwrap();

        println!("{:?}", result);
    }

    conn.close().await;
    Ok("Success".to_string())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Move {
    power: Option<i32>,
    accuracy: Option<i32>,
    pp: Option<i32>,
    #[serde(rename = "type")]
    type_: String,
    damage_class: String,
    machine_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct JsonMoves {
    moves: HashMap<String, Move>,
}

#[tauri::command]
pub async fn convert_moves_to_sqlite(
    wiki_name: &str,
    app_handle: AppHandle,
) -> Result<String, String> {
    let base_path = app_handle.path_resolver().app_data_dir().unwrap();

    let json_moves_file_path = base_path.join(wiki_name).join("data").join("moves.json");
    let json_moves_file = File::open(&json_moves_file_path).unwrap();
    let json_moves: JsonMoves = match serde_json::from_reader(json_moves_file) {
        Ok(file) => file,
        Err(err) => return Err(format!("Failed to read moves json file: {}", err)),
    };

    let sqlite_path = base_path.join(wiki_name).join(format!("{}.db", wiki_name));
    let sqlite_connection_string = format!("sqlite:{}", sqlite_path.to_str().unwrap());

    let conn = match SqlitePool::connect(&sqlite_connection_string).await {
        Ok(conn) => conn,
        Err(err) => {
            return Err(format!("Failed to connect to database: {}", err));
        }
    };

    for (name, _move) in json_moves.moves {
        let result = sqlx::query(
            "INSERT INTO moves (name, power, accuracy, pp, type, damage_class, machine_name) VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&name)
        .bind(_move.power)
        .bind(_move.accuracy)
        .bind(_move.pp)
        .bind(_move.type_)
        .bind(_move.damage_class)
        .bind(_move.machine_name)
        .execute(&conn)
        .await
        .unwrap();

        println!("{:?}", result);
    }

    conn.close().await;
    Ok("Success".to_string())
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
struct DBAbility {
    id: i32,
    name: String,
    effect: String,
    is_modified: i32,
    is_new: i32,
}

// enum ExistingAbility {
//     ID(i32),
//     NONE(None),
// }

#[tauri::command]
pub async fn convert_pokemon_to_sqlite(
    wiki_name: &str,
    app_handle: AppHandle,
) -> Result<String, String> {
    let base_path = app_handle.path_resolver().app_data_dir().unwrap();

    let mut pokemon: Pokemon = Pokemon {
        pokemon: IndexMap::new(),
    };
    for i in 1..=10 {
        let shard_json_file_path = base_path
            .join(wiki_name)
            .join("data")
            .join("pokemon_data")
            .join(format!("shard_{}.json", i));
        let shard_file = match File::open(&shard_json_file_path) {
            Ok(file) => file,
            Err(err) => {
                return Err(format!("Failed to open shard {} file: {}", i, err));
            }
        };
        let shard: Pokemon = match serde_json::from_reader(shard_file) {
            Ok(shard) => shard,
            Err(err) => {
                return Err(format!("Failed to parse shard {}: {}", i, err));
            }
        };

        pokemon.pokemon.extend(shard.pokemon.into_iter());
    }

    let sqlite_path = base_path.join(wiki_name).join(format!("{}.db", wiki_name));
    let sqlite_connection_string = format!("sqlite:{}", sqlite_path.to_str().unwrap());

    let conn = match SqlitePool::connect(&sqlite_connection_string).await {
        Ok(conn) => conn,
        Err(err) => {
            return Err(format!("Failed to connect to database: {}", err));
        }
    };

    for (name, pokemon) in pokemon.pokemon {
        // Check forms and add them too

        let ability_1_id = match pokemon.abilities.get(0) {
            Some(ability) => {
                match sqlx::query_as::<_, DBAbility>("SELECT * FROM abilities WHERE name = ?")
                    .bind(ability)
                    .fetch_one(&conn)
                    .await
                {
                    Ok(ability) => ability.id,
                    Err(err) => {
                        println!("Error fetching ability 1 for {}: {}", pokemon.name, err);
                        0
                    }
                }
            }
            None => 0,
        };

        let ability_2_id = match pokemon.abilities.get(1) {
            Some(ability) => {
                match sqlx::query_as::<_, DBAbility>("SELECT * FROM abilities WHERE name = ?")
                    .bind(ability)
                    .fetch_one(&conn)
                    .await
                {
                    Ok(ability) => ability.id,
                    Err(err) => {
                        println!("Error fetching ability 2 for {}: {}", pokemon.name, err);
                        0
                    }
                }
            }
            None => 0,
        };

        let result = sqlx::query(
            "INSERT INTO pokemon (
                dex_number,
                name,
                type_1,
                type_2,
                ability_1,
                ability_2,
                attack,
                defense,
                sp_attack,
                sp_defense,
                speed
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(pokemon.id)
        .bind(&pokemon.name)
        .bind(&pokemon.types[0])
        .bind(&pokemon.types[1])
        .bind(ability_1_id)
        .bind(ability_2_id)
        .bind(pokemon.stats.attack)
        .bind(pokemon.stats.defense)
        .bind(pokemon.stats.sp_attack)
        .bind(pokemon.stats.sp_defense)
        .bind(pokemon.stats.speed)
        .execute(&conn)
        .await
        .unwrap();

        for (name, form) in pokemon.forms {
            let form_ability_1_id = match form.abilities.get(0) {
                Some(ability) => {
                    match sqlx::query_as::<_, DBAbility>("SELECT * FROM abilities WHERE name = ?")
                        .bind(ability)
                        .fetch_one(&conn)
                        .await
                    {
                        Ok(ability) => ability.id,
                        Err(err) => {
                            println!("Error fetching ability 1 for form {}: {}", &name, err);
                            0
                        }
                    }
                }
                None => 0,
            };

            let form_ability_2_id = match pokemon.abilities.get(1) {
                Some(ability) => {
                    match sqlx::query_as::<_, DBAbility>("SELECT * FROM abilities WHERE name = ?")
                        .bind(ability)
                        .fetch_one(&conn)
                        .await
                    {
                        Ok(ability) => ability.id,
                        Err(err) => {
                            println!("Error fetching ability 2 for form {}: {}", &name, err);
                            0
                        }
                    }
                }
                None => 0,
            };

            let form_result = sqlx::query(
                "INSERT INTO pokemon (
                            dex_number,
                            name,
                            type_1,
                            type_2,
                            ability_1,
                            ability_2,
                            attack,
                            defense,
                            sp_attack,
                            sp_defense,
                            speed
                        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(pokemon.id)
            .bind(&name)
            .bind(&form.types[0])
            .bind(&form.types[1])
            .bind(form_ability_1_id)
            .bind(form_ability_2_id)
            .bind(form.stats.attack)
            .bind(form.stats.defense)
            .bind(form.stats.sp_attack)
            .bind(form.stats.sp_defense)
            .bind(form.stats.speed)
            .execute(&conn)
            .await
            .unwrap();

            println!("Form Result: {:?}", form_result)
        }

        println!("{:?}", result);
    }

    conn.close().await;
    Ok("Success".to_string())
}
