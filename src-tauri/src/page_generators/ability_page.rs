use std::io::Write;

use serde_yaml::{Mapping, Value};
use sqlx::FromRow;
use tauri::{AppHandle, Manager};

use crate::{
    database::{
        create_docs_file, get_mkdocs_config, get_sqlite_connection, page_exists_in_mkdocs,
        remove_docs_file, update_mkdocs_yaml,
    },
    helpers::{capitalize_and_remove_hyphens, FALSE, TRUE},
    logger::{self, write_log, LogLevel},
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
    let base_path = app_handle.path().app_data_dir().unwrap();

    let sqlite_file_path = base_path.join(wiki_name).join(format!("{}.db", wiki_name));
    let conn = match get_sqlite_connection(sqlite_file_path).await {
        Ok(conn) => conn,
        Err(err) => {
            logger::write_log(&base_path.join(wiki_name), logger::LogLevel::Error, &err);
            return Err(err);
        }
    };

    let abilities = match sqlx::query_as::<_, Ability>("SELECT * FROM abilities")
        .fetch_all(&conn)
        .await
    {
        Ok(abilities) => abilities,
        Err(err) => {
            let message = format!("Failed to get abilities: {}", err);
            logger::write_log(
                &base_path.join(wiki_name),
                logger::LogLevel::Error,
                &message,
            );
            return Err(message);
        }
    };

    return generate_ability_page(wiki_name, &abilities, &base_path);
}

pub fn generate_ability_page(
    wiki_name: &str,
    abilities: &[Ability],
    base_path: &std::path::PathBuf,
) -> Result<String, String> {
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

    let mkdocs_yaml_file_path = base_path.join(wiki_name).join("dist").join("mkdocs.yml");
    let mut mkdocs_config = match get_mkdocs_config(&mkdocs_yaml_file_path) {
        Ok(config) => config,
        Err(err) => {
            logger::write_log(&base_path.join(wiki_name), logger::LogLevel::Error, &err);
            return Err(err);
        }
    };

    let (page_exists, page_index) = page_exists_in_mkdocs(mkdocs_config.clone(), "Ability Changes");

    if ability_changes_markdown.is_empty() {
        if !page_exists {
            return Ok("No Ability changes to generate".to_string());
        }

        remove_docs_file(wiki_name, base_path, "ability_changes.md")?;

        mkdocs_config
            .nav
            .as_sequence_mut()
            .unwrap()
            .remove(page_index);

        update_mkdocs_yaml(wiki_name, base_path, &mkdocs_config)?;

        return Ok("No Ability changes to generate. Ability Changes page removed".to_string());
    }

    let mut ability_changes_file = create_docs_file(wiki_name, base_path, "ability_changes.md")?;

    if let Err(err) =
        ability_changes_file.write_all(format!("{}", ability_changes_markdown).as_bytes())
    {
        let message = format!("{wiki_name}: Failed to write ability changes file: {err}");
        write_log(&base_path, LogLevel::Error, &message);
        return Err(message);
    }

    if page_exists {
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

    update_mkdocs_yaml(wiki_name, base_path, &mkdocs_config)?;

    Ok("Abilities Page Generated".to_string())
}
