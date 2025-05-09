use std::{
    fs::{self, File},
    io::Write,
};

use serde_yaml::{Mapping, Value};
use sqlx::FromRow;
use tauri::{AppHandle, Manager};

use crate::{
    database::{
        create_docs_file, get_mkdocs_config, get_sqlite_connection, page_exists_in_mkdocs,
        remove_docs_file, update_mkdocs_yaml,
    },
    helpers::{capitalize, capitalize_and_remove_hyphens, FALSE, TRUE},
    logger::{self, write_log, LogLevel},
};

#[derive(Debug, Clone, FromRow)]
pub struct Nature {
    pub name: String,
    pub increased_stat: Option<String>,
    pub decreased_stat: Option<String>,
    pub is_modified: i32,
    pub is_new: i32,
}

#[tauri::command]
pub async fn generate_nature_page_with_handle(
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

    let natures = match sqlx::query_as::<_, Nature>("SELECT * FROM natures")
        .fetch_all(&conn)
        .await
    {
        Ok(natures) => natures,
        Err(err) => {
            let message = format!("Failed to get natures: {}", err);
            logger::write_log(
                &base_path.join(wiki_name),
                logger::LogLevel::Error,
                &message,
            );
            return Err(message);
        }
    };

    return generate_nature_page(wiki_name, &natures, &base_path);
}

pub fn generate_nature_page(
    wiki_name: &str,
    natures: &[Nature],
    base_path: &std::path::PathBuf,
) -> Result<String, String> {
    let mut nature_changes_markdown = String::new();
    let mut nature_new = String::new();
    let mut nature_modified = String::new();

    for nature in natures {
        if nature.is_new == FALSE && nature.is_modified == FALSE {
            continue;
        }

        let increased_stat = match nature.increased_stat.clone() {
            Some(stat) => capitalize_and_remove_hyphens(&stat),
            None => "None".to_string(),
        };
        let decreased_stat = match nature.decreased_stat.clone() {
            Some(stat) => capitalize_and_remove_hyphens(&stat),
            None => "None".to_string(),
        };

        let entry = format!(
            "| {} | {} | {} |\n",
            capitalize(&nature.name),
            increased_stat,
            decreased_stat
        );

        if nature.is_new == TRUE {
            if nature_new.is_empty() {
                nature_new.push_str(&format!(
                    "| New Natures | Increased Stat | Decreased Stat |
                    | :--: | :-- | :-- |
                    "
                ))
            }
            nature_new.push_str(&entry);
        }

        if nature.is_modified == TRUE {
            if nature_modified.is_empty() {
                nature_modified.push_str(&format!(
                    "| Modified Natures | Increased Stat | Decreased Stat |
                    | :--: | :-- | :-- |
                    "
                ))
            }
            nature_modified.push_str(&entry);
        }
    }

    if !nature_new.is_empty() {
        let entry = format!("{}\n", nature_new);
        nature_changes_markdown.push_str(&entry);
    }

    if !nature_modified.is_empty() {
        let entry = format!("{}\n", nature_modified);
        nature_changes_markdown.push_str(&entry);
    }

    let mkdocs_yaml_file_path = base_path.join(wiki_name).join("dist").join("mkdocs.yml");
    let mut mkdocs_config = match get_mkdocs_config(&mkdocs_yaml_file_path) {
        Ok(config) => config,
        Err(err) => {
            logger::write_log(&base_path.join(wiki_name), logger::LogLevel::Error, &err);
            return Err(err);
        }
    };

    let (page_exists, page_index) = page_exists_in_mkdocs(mkdocs_config.clone(), "Nature Changes");

    if nature_changes_markdown.is_empty() {
        if !page_exists {
            return Ok("No Nature changes to generate".to_string());
        }

        remove_docs_file(wiki_name, base_path, "nature_changes.md")?;

        mkdocs_config
            .nav
            .as_sequence_mut()
            .unwrap()
            .remove(page_index);

        update_mkdocs_yaml(wiki_name, base_path, &mkdocs_config)?;

        return Ok("No Nature changes to generate. Nature Changes page removed".to_string());
    }

    let mut nature_changes_file = create_docs_file(wiki_name, base_path, "nature_changes.md")?;

    if let Err(err) =
        nature_changes_file.write_all(format!("{}", nature_changes_markdown).as_bytes())
    {
        let message = format!("{wiki_name}: Failed to write nature changes file: {err}");
        write_log(&base_path, LogLevel::Error, &message);
        return Err(message);
    }

    if page_exists {
        return Ok("Nature Changes Page Updated".to_string());
    }

    let mut nature_changes = Mapping::new();
    nature_changes.insert(
        Value::String("Nature Changes".to_string()),
        Value::String("nature_changes.md".to_string()),
    );

    mkdocs_config
        .nav
        .as_sequence_mut()
        .unwrap()
        .insert(1, Value::Mapping(nature_changes));

    update_mkdocs_yaml(wiki_name, base_path, &mkdocs_config)?;

    Ok("Natures Page Generated".to_string())
}
