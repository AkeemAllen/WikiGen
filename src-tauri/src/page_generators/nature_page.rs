use std::{
    fs::{self, File},
    io::Write,
};

use serde_yaml::{Mapping, Value};
use sqlx::{FromRow, Sqlite};
use tauri::AppHandle;

use crate::{
    database::{get_mkdocs_config, get_sqlite_connection},
    helpers::{capitalize, capitalize_and_remove_hyphens, FALSE, TRUE},
    logger,
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
    let base_path = app_handle.path_resolver().app_data_dir().unwrap();
    let sqlite_path = base_path.join(wiki_name).join(format!("{}.db", wiki_name));
    let conn = match get_sqlite_connection(sqlite_path).await {
        Ok(conn) => conn,
        Err(err) => {
            logger::write_log(&base_path.join(wiki_name), logger::LogLevel::Error, &err);
            return Err(err);
        }
    };

    let natures = match sqlx::query_as::<_, Nature>("SELECT * FROM natures")
        .fetch_all(conn)
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
    let mkdocs_yaml_file_path = base_path.join(wiki_name).join("dist").join("mkdocs.yml");
    let mut mkdocs_config = match get_mkdocs_config(&mkdocs_yaml_file_path) {
        Ok(config) => config,
        Err(err) => {
            logger::write_log(&base_path.join(wiki_name), logger::LogLevel::Error, &err);
            return Err(err);
        }
    };

    let mut nature_changes_file = match File::create(
        base_path
            .join(wiki_name)
            .join("dist")
            .join("docs")
            .join("nature_changes.md"),
    ) {
        Ok(file) => file,
        Err(err) => {
            let message = format!("Failed to create nature changes file: {err}");
            logger::write_log(
                &base_path.join(wiki_name),
                logger::LogLevel::Error,
                &message,
            );
            return Err(message);
        }
    };

    let nav_entries = mkdocs_config.nav.as_sequence_mut().unwrap();
    let mut nature_page_exists = false;
    let mut page_index = 0;
    for (index, entry) in nav_entries.iter_mut().enumerate() {
        let map_entries = entry.as_mapping_mut().unwrap();
        if let Some(_) = map_entries.get_mut(Value::String("Nature Changes".to_string())) {
            nature_page_exists = true;
            page_index = index;
            break;
        }
    }

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

    if nature_changes_markdown.is_empty() {
        if !nature_page_exists {
            return Ok("No Nature changes to generate".to_string());
        }

        match fs::remove_file(
            base_path
                .join(wiki_name)
                .join("dist")
                .join("docs")
                .join("nature_changes.md"),
        ) {
            Ok(file) => file,
            Err(err) => {
                let message = format!("Failed to remove nature changes file: {err}");
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
                let message = format!("Failed to update mkdocs yaml file: {err}");
                logger::write_log(
                    &base_path.join(wiki_name),
                    logger::LogLevel::Error,
                    &message,
                );
                return Err(message);
            }
        }

        return Ok("No Nature changes to generate. Nature Changes page removed".to_string());
    }

    match nature_changes_file.write_all(format!("{}", nature_changes_markdown).as_bytes()) {
        Ok(_) => {}
        Err(err) => {
            let message = format!("Failed to write to nature changes file: {err}");
            logger::write_log(
                &base_path.join(wiki_name),
                logger::LogLevel::Error,
                &message,
            );
            return Err(message);
        }
    }

    if nature_page_exists {
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

    Ok("Natures Page Generated".to_string())
}
