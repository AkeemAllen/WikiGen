use std::{
    fs::{self, File},
    io::Write,
};

use serde_yaml::{Mapping, Value};
use sqlx::{FromRow, Sqlite};
use tauri::AppHandle;

use crate::{
    database::{get_mkdocs_config, get_sqlite_connection},
    helpers::{capitalize_and_remove_hyphens, FALSE, TRUE}
};

#[derive(Debug, Clone, FromRow)]
pub struct Ability {
    pub name: String,
    pub effect: String,
    pub is_modified: i32,
    pub is_new: i32,
}

#[tauri::command]
pub async fn generate_ability_page_with_handle(
    wiki_name: &str,
    app_handle: AppHandle,
) -> Result<String, String> {
    let base_path = app_handle.path_resolver().app_data_dir().unwrap();

    let sqlite_file_path = base_path.join(wiki_name).join(format!("{}.db", wiki_name));
    let conn = get_sqlite_connection(sqlite_file_path).await?;

    let abilities = get_abilities(&conn).await?;

    let result = generate_ability_page(wiki_name, &abilities, &base_path);
    return result;
}

async fn get_abilities(conn: &sqlx::Pool<Sqlite>) -> Result<Vec<Ability>, String> {
    let abilities = match sqlx::query_as::<_, Ability>("SELECT * FROM abilities")
        .fetch_all(conn)
        .await
    {
        Ok(abilities) => abilities,
        Err(err) => return Err(format!("Failed to get abilities: {}", err)),
    };

    return Ok(abilities);
}

pub fn generate_ability_page(
    wiki_name: &str,
    abilities: &[Ability],
    base_path: &std::path::PathBuf,
) -> Result<String, String> {
    let mkdocs_yaml_file_path = base_path.join(wiki_name).join("dist").join("mkdocs.yml");
    let mut mkdocs_config = get_mkdocs_config(&mkdocs_yaml_file_path)?;

    let mut ability_changes_file = match File::create(
        base_path
            .join(wiki_name)
            .join("dist")
            .join("docs")
            .join("ability_changes.md"),
    ) {
        Ok(file) => file,
        Err(err) => return Err(format!("Failed to create ability changes file: {}", err)),
    };

    let nav_entries = mkdocs_config.nav.as_sequence_mut().unwrap();
    let mut ability_page_exists = false;
    let mut page_index = 0;
    for (index, entry) in nav_entries.iter_mut().enumerate() {
        let map_entries = entry.as_mapping_mut().unwrap();
        if let Some(_) = map_entries.get_mut(Value::String("Ability Changes".to_string())) {
            ability_page_exists = true;
            page_index = index;
            break;
        }
    }

    let mut ability_changes_markdown = String::new();
    let mut ability_new = String::new();
    let mut ability_modified = String::new();

    for ability in abilities {
        if ability.is_new == FALSE && ability.is_modified == FALSE {
            continue;
        }

        let entry = format!(
            "| {} | {} |\n",
            capitalize_and_remove_hyphens(&ability.name),
            &ability.effect.replace("\n", "")
        );

        if ability.is_new == TRUE {
            if ability_new.is_empty() {
                ability_new.push_str(&format!(
                    "| New Abilities | Effect |
                    | :--: | :-- |
                    "
                ))
            }
            ability_new.push_str(&entry);
        }

        if ability.is_modified == TRUE {
            if ability_modified.is_empty() {
                ability_modified.push_str(&format!(
                    "| Modified Abilities | Effect |
                    | :--: | :-- |
                    "
                ))
            }
            ability_modified.push_str(&entry);
        }
    }

    if !ability_new.is_empty() {
        let entry = format!("{}\n", ability_new);
        ability_changes_markdown.push_str(&entry);
    }
    if !ability_modified.is_empty() {
        let entry = format!("{}\n", ability_modified);
        ability_changes_markdown.push_str(&entry);
    }

    if ability_changes_markdown.is_empty() {
        if !ability_page_exists {
            return Ok("No Ability changes to generate".to_string());
        }

        match fs::remove_file(
            base_path
                .join(wiki_name)
                .join("dist")
                .join("docs")
                .join("ability_changes.md"),
        ) {
            Ok(file) => file,
            Err(err) => {
                println!("Failed to remove ability changes file: {}", err);
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

        return Ok("No Ability changes to generate. Ability Changes page removed".to_string());
    }

    ability_changes_file
        .write_all(format!("{}", ability_changes_markdown).as_bytes())
        .unwrap();

    if ability_page_exists {
        return Ok("Ability Changes Page Updated".to_string());
    }

    let mut ability_changes = Mapping::new();
    ability_changes.insert(
        Value::String("Ability Changes".to_string()),
        Value::String("ability_changes.md".to_string()),
    );

    mkdocs_config
        .nav
        .as_sequence_mut()
        .unwrap()
        .insert(1, Value::Mapping(ability_changes));

    match fs::write(
        mkdocs_yaml_file_path,
        serde_yaml::to_string(&mkdocs_config).unwrap(),
    ) {
        Ok(_) => {}
        Err(err) => return Err(format!("Failed to update mkdocs yaml file: {}", err)),
    }

    Ok("Abilities Page Generated".to_string())
}
