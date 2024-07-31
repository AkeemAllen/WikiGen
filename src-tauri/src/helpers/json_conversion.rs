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

    for (_, pokemon) in pokemon.pokemon {
        // Check forms and add them too
        let mut types = pokemon.types[0].to_string();
        if pokemon.types.len() > 1 && pokemon.types[1] != "none" {
            types.push_str(&format!(",{}", pokemon.types[1]));
        }

        let mut ability_2 = pokemon.abilities.get(1);

        if ability_2.is_some() {
            if ability_2.unwrap() == "none" || ability_2.unwrap() == "" {
                ability_2 = None;
            }
        }

        let update_result_1 =
            sqlx::query("UPDATE pokemon SET ability_1 = ?, ability_2 = ? WHERE name = ?")
                .bind(pokemon.abilities.get(0))
                .bind(ability_2)
                .bind(&pokemon.name)
                .execute(&conn)
                .await
                .unwrap();

        // let result = sqlx::query(
        //     "INSERT INTO pokemon (
        //         dex_number,
        //         name,
        //         type_1,
        //         type_2,
        //         ability_1,
        //         ability_2,
        //         hp,
        //         attack,
        //         defense,
        //         sp_attack,
        //         sp_defense,
        //         speed
        //     ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        // )
        // .bind(pokemon.id)
        // .bind(&pokemon.name)
        // .bind(&pokemon.types[0])
        // .bind(&pokemon.types[1])
        // .bind(ability_1_id)
        // .bind(ability_2_id)
        // .bind(pokemon.stats.hp)
        // .bind(pokemon.stats.attack)
        // .bind(pokemon.stats.defense)
        // .bind(pokemon.stats.sp_attack)
        // .bind(pokemon.stats.sp_defense)
        // .bind(pokemon.stats.speed)
        // .execute(&conn)
        // .await
        // .unwrap();

        for (name, form) in pokemon.forms {
            let mut types = form.types[0].to_string();
            if form.types.len() > 1 && form.types[1] != "none" {
                types.push_str(&format!(",{}", form.types[1]));
            }

            let mut ability_2 = pokemon.abilities.get(1);

            if ability_2.is_some() {
                if ability_2.unwrap() == "none" || ability_2.unwrap() == "" {
                    ability_2 = None;
                }
            }

            let update_result =
                sqlx::query("UPDATE pokemon SET ability_1 = ?, ability_2 = ? WHERE name = ?")
                    .bind(pokemon.abilities.get(0))
                    .bind(ability_2)
                    .bind(&name)
                    .execute(&conn)
                    .await
                    .unwrap();

            // let form_result = sqlx::query(
            //     "INSERT INTO pokemon (
            //                 dex_number,
            //                 name,
            //                 type_1,
            //                 type_2,
            //                 ability_1,
            //                 ability_2,
            //                 hp,
            //                 attack,
            //                 defense,
            //                 sp_attack,
            //                 sp_defense,
            //                 speed
            //             ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            // )
            // .bind(pokemon.id)
            // .bind(&name)
            // .bind(&form.types[0])
            // .bind(&form.types[1])
            // .bind(form_ability_1_id)
            // .bind(form_ability_2_id)
            // .bind(form.stats.hp)
            // .bind(form.stats.attack)
            // .bind(form.stats.defense)
            // .bind(form.stats.sp_attack)
            // .bind(form.stats.sp_defense)
            // .bind(form.stats.speed)
            // .execute(&conn)
            // .await
            // .unwrap();

            println!("Form Result: {:?}", update_result)
        }

        println!("{:?}", update_result_1);

        // Change none types to null
    }
    // let update_to_null_result =
    //     sqlx::query("UPDATE pokemon SET type_2 = NULL WHERE type_2 = 'none'")
    //         .execute(&conn)
    //         .await
    //         .unwrap();

    // println!("Update none types to null{:?}", update_to_null_result);

    // let remove_empty_ab2_results =
    //     sqlx::query("UPDATE pokemon SET ability_2 = NULL WHERE ability_2 = 0")
    //         .execute(&conn)
    //         .await
    //         .unwrap();
    // println!("Remove empty ability 2 {:?}", remove_empty_ab2_results);

    // // When considering migration, insert None ability into abilities table
    // // then remove it here
    // let delete_none_ability = sqlx::query("DELETE FROM abilities WHERE name = 'None'")
    //     .execute(&conn)
    //     .await
    //     .unwrap();

    // println!("Deleteing None Ability {:?}", delete_none_ability);

    conn.close().await;
    Ok("Success".to_string())
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
struct DBMove {
    id: i32,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
struct DBPokemon {
    id: i32,
    name: String,
}

#[tauri::command]
pub async fn convert_pokemon_movesets_to_sqlite(
    wiki_name: &str,
    app_handle: AppHandle,
) -> Result<String, String> {
    let base_path = app_handle.path_resolver().app_data_dir().unwrap();

    let sqlite_path = base_path.join(wiki_name).join(format!("{}.db", wiki_name));
    let sqlite_connection_string = format!("sqlite:{}", sqlite_path.to_str().unwrap());

    let conn = match SqlitePool::connect(&sqlite_connection_string).await {
        Ok(conn) => conn,
        Err(err) => {
            return Err(format!("Failed to connect to database: {}", err));
        }
    };

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

    for (_, pokemon) in pokemon.pokemon {
        println!("Processing {}: {}", &pokemon.name, pokemon.id);
        for (move_name, move_details) in pokemon.moves {
            let move_id = match sqlx::query_as::<_, DBMove>("SELECT * FROM moves WHERE name = ?")
                .bind(&move_name)
                .fetch_one(&conn)
                .await
            {
                Ok(_move) => _move.id,
                Err(err) => {
                    println!("Error fetching move {}: {}", move_name, err);
                    continue;
                }
            };

            let pokemon_id =
                match sqlx::query_as::<_, DBPokemon>("SELECT * FROM pokemon WHERE name = ?")
                    .bind(&pokemon.name)
                    .fetch_one(&conn)
                    .await
                {
                    Ok(pokemon) => pokemon.id,
                    Err(err) => {
                        println!("Error fetching pokemon {}: {}", &pokemon.name, err);
                        continue;
                    }
                };

            let mut learn_methods = move_details.learn_method.get(0).unwrap().clone();

            if move_details.learn_method.len() > 1 {
                learn_methods = format!(
                    "{}, {}",
                    learn_methods,
                    move_details.learn_method.get(1).unwrap()
                );
            }

            let result = sqlx::query(
                "INSERT INTO pokemon_movesets_ (
                            pokemon,
                            move,
                            learn_method,
                            level_learned
                        ) VALUES (?, ?, ?, ?)",
            )
            .bind(pokemon_id)
            .bind(move_id)
            .bind(learn_methods)
            .bind(move_details.level_learned)
            .execute(&conn)
            .await
            .unwrap();
            println!("{:?}", result);
        }

        for (name, form) in pokemon.forms {
            let form_id =
                match sqlx::query_as::<_, DBPokemon>("SELECT * FROM pokemon WHERE name = ?")
                    .bind(&name)
                    .fetch_one(&conn)
                    .await
                {
                    Ok(_form) => _form.id,
                    Err(err) => {
                        println!("Error fetching form {}: {}", name, err);
                        continue;
                    }
                };

            println!("Processing {}: {}", name, form_id);
            for (move_name, move_details) in form.moves {
                let move_id =
                    match sqlx::query_as::<_, DBMove>("SELECT * FROM moves WHERE name = ?")
                        .bind(&move_name)
                        .fetch_one(&conn)
                        .await
                    {
                        Ok(_move) => _move.id,
                        Err(err) => {
                            println!("Error fetching move {}: {}", move_name, err);
                            continue;
                        }
                    };

                let mut learn_methods = move_details.learn_method.get(0).unwrap().clone();

                if move_details.learn_method.len() > 1 {
                    learn_methods = format!(
                        "{}, {}",
                        learn_methods,
                        move_details.learn_method.get(1).unwrap()
                    );
                }

                let result = sqlx::query(
                    "INSERT INTO pokemon_movesets_ (
                                pokemon,
                                move,
                                learn_method,
                                level_learned
                            ) VALUES (?, ?, ?, ?)",
                )
                .bind(form_id)
                .bind(move_id)
                .bind(learn_methods)
                .bind(move_details.level_learned)
                .execute(&conn)
                .await
                .unwrap();
                println!("{:?}", result);
            }
        }
    }

    Ok("Success".to_string())
}
