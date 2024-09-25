use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use serde_yaml::{Mapping, Value};
use sqlx::FromRow;
use tauri::AppHandle;

use crate::{
    database::{get_mkdocs_config, get_sqlite_connection},
    helpers::{capitalize_and_remove_hyphens, FALSE, TRUE},
    logger,
};

#[derive(Debug, Clone, FromRow)]
pub struct Item {
    pub name: String,
    pub effect: String,
    pub is_modified: i32,
    pub is_new: i32,
}

#[tauri::command]
pub async fn generate_item_changes_page_with_handle(
    wiki_name: &str,
    app_handle: AppHandle,
) -> Result<String, String> {
    let base_path = app_handle.path_resolver().app_data_dir().unwrap();
    let sqlite_path = base_path.join(wiki_name).join(format!("{}.db", wiki_name));
    let conn = match get_sqlite_connection(sqlite_path).await {
        Ok(conn) => conn,
        Err(err) => {
            logger::write_log(&base_path.join(wiki_name), logger::LogLevel::Error, &err);
            return Err(err);
        }
    };

    let items = match sqlx::query_as::<_, Item>("SELECT * FROM items")
        .fetch_all(&conn)
        .await
    {
        Ok(items) => items,
        Err(err) => {
            let message = format!("Failed to get items: {}", err);
            logger::write_log(
                &base_path.join(wiki_name),
                logger::LogLevel::Error,
                &message,
            );
            return Err(message);
        }
    };

    return generate_item_changes_page(wiki_name, &items, &base_path);
}

pub fn generate_item_changes_page(
    wiki_name: &str,
    items: &[Item],
    base_path: &PathBuf,
) -> Result<String, String> {
    let mkdocs_yaml_file_path = base_path.join(wiki_name).join("dist").join("mkdocs.yml");
    let mut mkdocs_config = match get_mkdocs_config(&mkdocs_yaml_file_path) {
        Ok(config) => config,
        Err(err) => {
            logger::write_log(&base_path.join(wiki_name), logger::LogLevel::Error, &err);
            return Err(err);
        }
    };
    let mut item_changes_file = match File::create(
        base_path
            .join(wiki_name)
            .join("dist")
            .join("docs")
            .join("item_changes.md"),
    ) {
        Ok(file) => file,
        Err(err) => {
            let message = format!("Failed to create item changes file: {err}");
            logger::write_log(
                &base_path.join(wiki_name),
                logger::LogLevel::Error,
                &message,
            );
            return Err(message);
        }
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
                logger::write_log(
                    &base_path.join(wiki_name),
                    logger::LogLevel::Error,
                    &format!("Failed to remove item changes file: {}", err),
                );
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
            Err(err) => {
                let message = format!("Failed to update mkdocs yaml file: {err}");
                logger::write_log(
                    &base_path.join(wiki_name),
                    logger::LogLevel::Error,
                    &message,
                );
                return Err(message);
            }
        }

        return Ok("No Item changes to generate. Item Changes page removed".to_string());
    }

    match item_changes_file.write_all(format!("{}", item_changes_markdown).as_bytes()) {
        Ok(_) => {}
        Err(err) => {
            let message = format!("Failed to write item changes file: {err}");
            logger::write_log(
                &base_path.join(wiki_name),
                logger::LogLevel::Error,
                &message,
            );
            return Err(message);
        }
    }

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
        Err(err) => {
            let message = format!("Failed to update mkdocs yaml file: {err}");
            logger::write_log(
                &base_path.join(wiki_name),
                logger::LogLevel::Error,
                &message,
            );
            return Err(message);
        }
    }

    Ok("Items Page Generated".to_string())
}

#[derive(Debug, Clone, FromRow)]
pub struct ItemLocation {
    pub item_name: String,
    pub route: String,
    pub specific_location: Option<String>,
    pub method: Option<String>,
    pub requirements: Option<String>,
}

#[tauri::command]
pub async fn generate_item_location_page_with_handle(
    wiki_name: &str,
    app_handle: AppHandle,
) -> Result<String, String> {
    let base_path = app_handle.path_resolver().app_data_dir().unwrap();

    let sqlite_path = base_path.join(wiki_name).join(format!("{}.db", wiki_name));
    let conn = match get_sqlite_connection(sqlite_path).await {
        Ok(conn) => conn,
        Err(err) => {
            logger::write_log(&base_path.join(wiki_name), logger::LogLevel::Error, &err);
            return Err(err);
        }
    };

    let item_locations =
        match sqlx::query_as::<_, ItemLocation>("SELECT * FROM item_location ORDER BY item_name;")
            .fetch_all(&conn)
            .await
        {
            Ok(item_locations) => item_locations,
            Err(err) => {
                let message = format!("Failed to fetch item locations: {err}");
                logger::write_log(
                    &base_path.join(wiki_name),
                    logger::LogLevel::Error,
                    &message,
                );
                return Err(message);
            }
        };

    return generate_item_location_page(wiki_name, &item_locations, &base_path);
}

pub fn generate_item_location_page(
    wiki_name: &str,
    item_locations: &[ItemLocation],
    base_path: &PathBuf,
) -> Result<String, String> {
    let mkdocs_yaml_file_path = base_path.join(wiki_name).join("dist").join("mkdocs.yml");
    let mut mkdocs_config = match get_mkdocs_config(&mkdocs_yaml_file_path) {
        Ok(config) => config,
        Err(err) => {
            logger::write_log(&base_path.join(wiki_name), logger::LogLevel::Error, &err);
            return Err(err);
        }
    };

    let mut item_locations_file = match File::create(
        base_path
            .join(wiki_name)
            .join("dist")
            .join("docs")
            .join("item_locations.md"),
    ) {
        Ok(file) => file,
        Err(err) => {
            let message = format!("Failed to create item locations file: {}", err);
            logger::write_log(
                &base_path.join(wiki_name),
                logger::LogLevel::Error,
                &message,
            );
            return Err(message);
        }
    };

    let nav_entries = mkdocs_config.nav.as_sequence_mut().unwrap();
    let mut item_location_page = false;
    let mut page_index = 0;
    for (index, entry) in nav_entries.iter_mut().enumerate() {
        let map_entries = entry.as_mapping_mut().unwrap();
        if let Some(_) = map_entries.get_mut(Value::String("Item Locations".to_string())) {
            item_location_page = true;
            page_index = index;
            break;
        }
    }

    let mut item_locations_markdown = String::new();
    let mut item_location_entries = String::new();

    for item_location in item_locations {
        let specific_location = match &item_location.specific_location {
            Some(location) => location,
            None => "",
        };
        let method = match &item_location.method {
            Some(method) => method,
            None => "",
        };
        let requirements = match &item_location.requirements {
            Some(requirements) => requirements,
            None => "",
        };
        let entry = format!(
            "| {} | {} | {} | {} | {} |\n",
            format!(
                "{}<br/>{}",
                format!(
                    "![{}](img/items/{}.png)",
                    item_location.item_name, item_location.item_name
                ),
                capitalize_and_remove_hyphens(&item_location.item_name)
            ),
            item_location.route,
            specific_location,
            method,
            requirements
        );
        item_location_entries.push_str(&entry);
    }

    if item_location_entries.is_empty() {
        if !item_location_page {
            return Ok("No Item changes to generate".to_string());
        }

        match fs::remove_file(
            base_path
                .join(wiki_name)
                .join("dist")
                .join("docs")
                .join("item_locations.md"),
        ) {
            Ok(file) => file,
            Err(err) => {
                let message = format!("Failed to remove item locations file: {}", err);
                logger::write_log(
                    &base_path.join(wiki_name),
                    logger::LogLevel::Error,
                    &message,
                );
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
            Err(err) => {
                let message = format!("Failed to update mkdocs yaml file: {}", err);
                logger::write_log(
                    &base_path.join(wiki_name),
                    logger::LogLevel::Error,
                    &message,
                );
                return Err(message);
            }
        }

        return Ok("No Item Locations to generate. Item Locations page removed".to_string());
    }

    item_locations_markdown.push_str(&format!(
        "| Item Name | Route | Specific Location | Method | Requirements |
            | :--- | :--- | :--- | :--- | :--- |
            {}
            ",
        item_location_entries
    ));

    match item_locations_file.write_all(format!("{}", item_locations_markdown).as_bytes()) {
        Ok(_) => {}
        Err(err) => {
            let message = format!("Failed to write item locations file: {}", err);
            logger::write_log(
                &base_path.join(wiki_name),
                logger::LogLevel::Error,
                &message,
            );
            return Err(message);
        }
    }

    if item_location_page {
        return Ok("Item Changes Page Updated".to_string());
    }

    let mut item_locations = Mapping::new();
    item_locations.insert(
        Value::String("Item Locations".to_string()),
        Value::String("item_locations.md".to_string()),
    );

    mkdocs_config
        .nav
        .as_sequence_mut()
        .unwrap()
        .insert(1, Value::Mapping(item_locations));

    match fs::write(
        mkdocs_yaml_file_path,
        serde_yaml::to_string(&mkdocs_config).unwrap(),
    ) {
        Ok(_) => {}
        Err(err) => {
            let message = format!("Failed to update mkdocs yaml file: {}", err);
            logger::write_log(
                &base_path.join(wiki_name),
                logger::LogLevel::Error,
                &message,
            );
            return Err(message);
        }
    }
    Ok("Item Location Page Generated".to_string())
}
