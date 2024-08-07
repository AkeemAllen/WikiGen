use std::{
    fs::{self, File},
    io::Write,
};

use serde_yaml::{Mapping, Value};
use sqlx::{migrate::MigrateDatabase, FromRow, Sqlite, SqlitePool};
use tauri::AppHandle;

use crate::{
    helpers::{capitalize, capitalize_and_remove_hyphens, FALSE, TRUE},
    structs::mkdocs_structs::MKDocsConfig,
};

#[derive(Debug, Clone, FromRow)]
struct Nature {
    name: String,
    increased_stat: Option<String>,
    decreased_stat: Option<String>,
    is_modified: i32,
    is_new: i32,
}

// enum Stat {
//     Attack,
//     Defense,
//     SpecialAttack,
//     SpecialDefense,
//     Speed,
// }

#[tauri::command]
pub async fn generate_nature_page(
    wiki_name: &str,
    app_handle: AppHandle,
) -> Result<String, String> {
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

    let natures = sqlx::query_as::<_, Nature>("SELECT * FROM natures")
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

    let mut nature_changes_file = match File::create(
        base_path
            .join(wiki_name)
            .join("dist")
            .join("docs")
            .join("nature_changes.md"),
    ) {
        Ok(file) => file,
        Err(err) => return Err(format!("Failed to create nature changes file: {}", err)),
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

        return Ok("No Nature changes to generate. Nature Changes page removed".to_string());
    }

    nature_changes_file
        .write_all(format!("{}", nature_changes_markdown).as_bytes())
        .unwrap();

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
        Err(err) => return Err(format!("Failed to update mkdocs yaml file: {}", err)),
    }

    Ok("Natures Page Generated".to_string())
}
