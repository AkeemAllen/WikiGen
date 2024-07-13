use std::{collections::HashMap, fs::File};

use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tauri::AppHandle;

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
