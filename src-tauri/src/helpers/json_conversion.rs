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
