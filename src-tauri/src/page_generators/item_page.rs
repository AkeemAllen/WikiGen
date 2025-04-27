use std::{
    collections::HashSet,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use serde_yaml::{Mapping, Value};
use sqlx::FromRow;
use tauri::{AppHandle, Manager};

use crate::{
    database::{get_mkdocs_config, get_sqlite_connection},
    helpers::{capitalize_and_remove_hyphens, FALSE, TRUE},
    logger,
};

#[derive(Debug, Clone, FromRow)]
pub struct Item {
    pub name: String,
    pub effect: String,
    pub category: String,
    pub is_modified: i32,
    pub is_new: i32,
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
pub async fn generate_items_page_with_handle(
    wiki_name: &str,
    app_handle: AppHandle,
) -> Result<String, String> {
    let base_path = app_handle.path().app_data_dir().unwrap();
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

    return generate_items_page(wiki_name, &base_path, &items, &item_locations);
}

pub fn generate_items_page(
    wiki_name: &str,
    base_path: &PathBuf,
    items: &[Item],
    item_locations: &[ItemLocation],
) -> Result<String, String> {
    let mkdocs_yaml_file_path = base_path.join(wiki_name).join("dist").join("mkdocs.yml");
    let mut mkdocs_config = match get_mkdocs_config(&mkdocs_yaml_file_path) {
        Ok(config) => config,
        Err(err) => {
            logger::write_log(&base_path.join(wiki_name), logger::LogLevel::Error, &err);
            return Err(err);
        }
    };
    let mut item_information_file = match File::create(
        base_path
            .join(wiki_name)
            .join("dist")
            .join("docs")
            .join("item_information.md"),
    ) {
        Ok(file) => file,
        Err(err) => {
            let message = format!("Failed to create item information file: {err}");
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
        if let Some(_) = map_entries.get_mut(Value::String("Item Information".to_string())) {
            item_page_exists = true;
            page_index = index;
            break;
        }
    }

    let mut items_markdown = String::new();
    let item_changes_markdown = generate_item_modifications(&items);
    // let item_modifications_markdown = generate_item_modifications(&items);
    let item_locations_markdown = generate_item_locations(&item_locations);

    if !item_changes_markdown.is_empty() {
        items_markdown.push_str(&item_changes_markdown);
    }
    if !item_locations_markdown.is_empty() {
        items_markdown.push_str(&format!("\n{}", &item_locations_markdown));
    }

    if items_markdown.is_empty() {
        if !item_page_exists {
            return Ok("No Item Information to generate".to_string());
        }

        match fs::remove_file(
            base_path
                .join(wiki_name)
                .join("dist")
                .join("docs")
                .join("item_information.md"),
        ) {
            Ok(file) => file,
            Err(err) => {
                logger::write_log(
                    &base_path.join(wiki_name),
                    logger::LogLevel::Error,
                    &format!("Failed to remove item information page: {}", err),
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

        return Ok("No Item information to generate. Item Information page removed".to_string());
    }

    match item_information_file.write_all(format!("{}", items_markdown).as_bytes()) {
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
        return Ok("Item Page Updated".to_string());
    }

    let mut item_changes = Mapping::new();
    item_changes.insert(
        Value::String("Item Information".to_string()),
        Value::String("item_information.md".to_string()),
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

pub fn generate_item_modifications(items: &[Item]) -> String {
    let categories = items
        .iter()
        .map(|item| item.category.clone())
        .collect::<Vec<String>>();
    let mut unique_categories = categories
        .into_iter()
        .collect::<HashSet<String>>()
        .into_iter()
        .collect::<Vec<String>>();
    let mut item_modifications_markdown = String::new();
    let mut category_collection = String::new();
    item_modifications_markdown.push_str("### Modifications\n");

    unique_categories.sort();

    for category in unique_categories {
        let mut category_items = items
            .iter()
            .filter(|item| item.category == category)
            .collect::<Vec<&Item>>();

        // Sorting so that new items are listed first
        category_items.sort_by(|a, b| b.is_new.cmp(&a.is_new));

        // Collecting the number of new items so I can split the list with a new line
        // once we start going through modified entries
        let num_of_new_items = category_items
            .iter()
            .filter(|item| item.is_new == TRUE)
            .count();

        let mut item_entries = String::new();

        for (index, item) in category_items.iter().enumerate() {
            if item.is_new == FALSE && item.is_modified == FALSE {
                continue;
            }

            if index == num_of_new_items && num_of_new_items > 0 {
                item_entries.push('\n');
            }

            let item_entry = format!(
                "\t| {} | {} |\n",
                format!(
                    "{}<br/>{}",
                    format!("![{}](img/items/{}.png)", &item.name, &item.name),
                    capitalize_and_remove_hyphens(&item.name)
                ),
                &item.effect.replace("\n", "")
            );

            item_entries.push_str(&item_entry);
        }

        let item_entries_div = format!("\n<div class=\"item-entries\">{item_entries}</div>");

        category_collection.push_str(&format!("\n???+ note \"{category}\"\n{item_entries_div}"));
    }
    item_modifications_markdown.push_str(&category_collection);

    return item_modifications_markdown;
}

pub fn generate_item_changes(items: &[Item]) -> String {
    let mut item_changes_markdown = String::new();
    let mut item_new = String::new();
    let mut item_modified = String::new();

    item_changes_markdown.push_str("### Modifications\n");

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

    if item_new.is_empty() && item_modified.is_empty() {
        return String::new();
    }

    if !item_new.is_empty() {
        let entry = format!("{}\n", item_new);
        item_changes_markdown.push_str(&entry);
    }
    if !item_modified.is_empty() {
        let entry = format!("{}\n", item_modified);
        item_changes_markdown.push_str(&entry);
    }

    return item_changes_markdown;
}

pub fn generate_item_locations(item_locations: &[ItemLocation]) -> String {
    let mut item_locations_markdown = String::new();
    let mut item_location_entries = String::new();

    item_locations_markdown.push_str("### Locations\n");

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
        return String::new();
    }

    item_locations_markdown.push_str(&format!(
        "| Item Name | Route | Specific Location | Method | Requirements |
            | :--- | :--- | :--- | :--- | :--- |
            {}
            ",
        item_location_entries
    ));

    return item_locations_markdown;
}
