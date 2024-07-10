use std::{
    fs::{self, File},
    io::Write,
};

use serde_yaml::{Mapping, Value};
use tauri::AppHandle;

use crate::{helpers::capitalize_and_remove_hyphens, structs::mkdocs_structs::MKDocsConfig};

use super::ModifiedItem;

#[tauri::command]
pub fn generate_item_page(wiki_name: &str, app_handle: AppHandle) -> Result<String, String> {
    let base_path = app_handle.path_resolver().app_data_dir().unwrap();

    let modified_items_natures_abilities_json_file_path = base_path
        .join(wiki_name)
        .join("data")
        .join("modifications")
        .join("modified_items_natures_abilities.json");
    let modified_items_natures_abilities_file =
        File::open(&modified_items_natures_abilities_json_file_path).unwrap();
    let modified_items_natures_abilities: super::ModifiedItemsNaturesAbilities =
        match serde_json::from_reader(modified_items_natures_abilities_file) {
            Ok(file) => file,
            Err(err) => {
                return Err(format!(
                    "Failed to read modified items, natures, and abilities json file: {}",
                    err
                ))
            }
        };
    let modified_items = modified_items_natures_abilities.items;

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

    for (item_name, item_details) in modified_items.iter() {
        let entry = format!(
            "| {} | {} |\n",
            format!(
                "{}<br/>{}",
                get_item_image(item_name, item_details),
                capitalize_and_remove_hyphens(item_name)
            ),
            item_details.modified.effect.replace("\n", "")
        );

        if item_details.is_new_item {
            if item_new.is_empty() {
                item_new.push_str(&format!(
                    "| New Items | Effect |
                    | :--: | :-- |
                    "
                ))
            }
            item_new.push_str(&entry);
        } else {
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

fn get_item_image(item_name: &str, item_details: &ModifiedItem) -> String {
    if item_details.is_new_item {
        return format!("![{}]({})", item_name, item_details.modified.sprite);
    }
    return format!("![{}](img/items/{}.png)", item_name, item_name);
}
