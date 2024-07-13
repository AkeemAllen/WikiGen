use std::{
    fs::{self, File},
    io::Write,
};

use serde_yaml::{Mapping, Value};
use sqlx::{migrate::MigrateDatabase, FromRow, Sqlite, SqlitePool};
use tauri::AppHandle;

use crate::{
    helpers::{capitalize_and_remove_hyphens, FALSE, TRUE},
    structs::mkdocs_structs::MKDocsConfig,
};

#[derive(Debug, Clone, FromRow)]
struct Item {
    name: String,
    effect: String,
    is_modified: i32,
    is_new: i32,
}

#[tauri::command]
pub async fn generate_item_page(wiki_name: &str, app_handle: AppHandle) -> Result<String, String> {
    let base_path = app_handle.path_resolver().app_data_dir().unwrap();

    let sqlite_path = base_path.join(wiki_name).join(format!("{}.db", wiki_name));
    let sqlite_connection_string = format!("sqlite:{}", sqlite_path.to_str().unwrap());
    if !Sqlite::database_exists(&sqlite_connection_string)
        .await
        .unwrap_or(false)
    {
        return Err("Database does not exist".to_string());
    }
    let conn = match SqlitePool::connect(&sqlite_connection_string).await {
        Ok(conn) => conn,
        Err(err) => {
            return Err(format!("Failed to connect to database: {}", err));
        }
    };

    let items = sqlx::query_as::<_, Item>("SELECT * FROM items")
        .fetch_all(&conn)
        .await
        .unwrap();

    conn.close().await;

    let mkdocs_yaml_file_path = base_path.join(wiki_name).join("dist").join("mkdocs.yml");
    let mkdocs_yaml_file = File::open(&mkdocs_yaml_file_path).unwrap();
    let mut mkdocs_config: MKDocsConfig = match serde_yaml::from_reader(mkdocs_yaml_file) {
        Ok(file) => file,
        Err(err) => return Err(format!("Failed to read mkdocs yaml file: {}", err)),
    };

    let mut item_changes_file = match File::create(
        base_path
            .join(wiki_name)
            .join("dist")
            .join("docs")
            .join("item_changes.md"),
    ) {
        Ok(file) => file,
        Err(err) => return Err(format!("Failed to create item changes file: {}", err)),
    };

    let nav_entries = mkdocs_config.nav.as_sequence_mut().unwrap();
    let mut item_page_exists = false;
    let mut page_index = 0;
    for (index, entry) in nav_entries.iter_mut().enumerate() {
        let map_entries = entry.as_mapping_mut().unwrap();
        if let Some(_) = map_entries.get_mut(Value::String("Item Changes".to_string())) {
            item_page_exists = true;
            page_index = index;
            break;
        }
    }

    let mut item_changes_markdown = String::new();
    let mut item_new = String::new();
    let mut item_modified = String::new();

    for item in items {
        if item.is_new == FALSE && item.is_modified == FALSE {
            continue;
        }

        let entry = format!(
            "| {} | {} |\n",
            format!(
                "{}<br/>{}",
                format!("![{}](img/items/{}.png)", &item.name, &item.name),
                capitalize_and_remove_hyphens(&item.name)
            ),
            &item.effect.replace("\n", "")
        );

        if item.is_new == TRUE {
            if item_new.is_empty() {
                item_new.push_str(&format!(
                    "| New Items | Effect |
                    | :--: | :-- |
                    "
                ))
            }
            item_new.push_str(&entry);
        }

        if item.is_modified == TRUE {
            if item_modified.is_empty() {
                item_modified.push_str(&format!(
                    "| Modified Items | Effect |
                    | :--: | :-- |
                    "
                ))
            }
            item_modified.push_str(&entry);
        }
    }

    if !item_new.is_empty() {
        let entry = format!("{}\n", item_new);
        item_changes_markdown.push_str(&entry);
    }
    if !item_modified.is_empty() {
        let entry = format!("{}\n", item_modified);
        item_changes_markdown.push_str(&entry);
    }

    if item_changes_markdown.is_empty() {
        if !item_page_exists {
            return Ok("No Item changes to generate".to_string());
        }

        match fs::remove_file(
            base_path
                .join(wiki_name)
                .join("dist")
                .join("docs")
                .join("item_changes.md"),
        ) {
            Ok(file) => file,
            Err(err) => {
                println!("Failed to remove item changes file: {}", err);
            }
        }

        mkdocs_config
            .nav
            .as_sequence_mut()
            .unwrap()
            .remove(page_index);
        match fs::write(
            &mkdocs_yaml_file_path,
            serde_yaml::to_string(&mkdocs_config).unwrap(),
        ) {
            Ok(file) => file,
            Err(err) => return Err(format!("Failed to update mkdocs yaml file: {}", err)),
        }

        return Ok("No Item changes to generate. Item Changes page removed".to_string());
    }

    item_changes_file
        .write_all(format!("{}", item_changes_markdown).as_bytes())
        .unwrap();

    if item_page_exists {
        return Ok("Item Changes Page Updated".to_string());
    }

    let mut item_changes = Mapping::new();
    item_changes.insert(
        Value::String("Item Changes".to_string()),
        Value::String("item_changes.md".to_string()),
    );

    mkdocs_config
        .nav
        .as_sequence_mut()
        .unwrap()
        .insert(1, Value::Mapping(item_changes));

    match fs::write(
        mkdocs_yaml_file_path,
        serde_yaml::to_string(&mkdocs_config).unwrap(),
    ) {
        Ok(_) => {}
        Err(err) => return Err(format!("Failed to update mkdocs yaml file: {}", err)),
    }

    Ok("Items Page Generated".to_string())
}
