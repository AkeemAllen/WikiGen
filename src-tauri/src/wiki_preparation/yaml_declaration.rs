use std::{
    collections::HashMap,
    fs::{self, File},
};

use serde_yaml::{Mapping, Value};
use tauri::{AppHandle, Manager};

use crate::structs::mkdocs_structs::{
    MKDocsConfig, MarkdownExtension, Palette, Plugin, PymdownxTabbed, PymdownxTaskList, Search,
    Theme, Toggle,
};

// Since I'm new to rust, almost everything in here is likely poorly done, but it works for now.
pub fn get_yaml(
    site_name: &str,
    site_url: &str,
    site_description: &str,
    site_author: &str,
    repo_url: &str,
) -> MKDocsConfig {
    let mut custom_checkbox = HashMap::new();
    custom_checkbox.insert("custom_checkbox".to_string(), true);

    let mut alternate_style = HashMap::new();
    alternate_style.insert("alternate_style".to_string(), true);

    let mut home_map = Mapping::new();
    home_map.insert(
        Value::String("Home".to_string()),
        Value::String("index.md".to_string()),
    );

    let mut pokemon_map = Mapping::new();
    pokemon_map.insert(
        Value::String("Pokemon".to_string()),
        Value::Sequence(vec![]),
    );

    let mut routes_map = Mapping::new();
    routes_map.insert(Value::String("Routes".to_string()), Value::Sequence(vec![]));

    let mkdocs_config = MKDocsConfig {
        site_name: site_name.to_string(),
        site_url: site_url.to_string(),
        site_description: site_description.to_string(),
        site_author: site_author.to_string(),
        repo_url: repo_url.to_string(),
        theme: Theme {
            name: "material".to_string(),
            features: vec![
                "content.tabls.links".to_string(),
                "content.tooltips".to_string(),
            ],
            favicon: "img/items/poke-ball.png".to_string(),
            palette: [
                Palette {
                    media: "(prefers-color-scheme: light)".to_string(),
                    primary: "black".to_string(),
                    scheme: "default".to_string(),
                    toggle: Toggle {
                        icon: "material/eye-outline".to_string(),
                        name: "Switch to dark mode".to_string(),
                    },
                },
                Palette {
                    media: "(prefers-color-scheme: dark)".to_string(),
                    primary: "black".to_string(),
                    scheme: "default".to_string(),
                    toggle: Toggle {
                        icon: "material/eye".to_string(),
                        name: "Switch to light mode".to_string(),
                    },
                },
            ],
        },
        nav: Value::Sequence(vec![
            Value::Mapping(home_map),
            Value::Mapping(pokemon_map),
            Value::Mapping(routes_map),
        ]),
        plugins: vec![Plugin {
            search: Search {
                lang: "en".to_string(),
            },
        }],
        markdown_extensions: [
            MarkdownExtension::String("admonition".to_string()),
            MarkdownExtension::String("abbr".to_string()),
            MarkdownExtension::String("attr_list".to_string()),
            MarkdownExtension::String("pymdownx.snippets".to_string()),
            MarkdownExtension::String("pymdownx.superfences".to_string()),
            MarkdownExtension::String("pymdownx.details".to_string()),
            MarkdownExtension::PymdownxTaskList(PymdownxTaskList {
                custom_checkbox: true,
            }),
            MarkdownExtension::PymdownxTabbed(PymdownxTabbed {
                alternate_style: true,
            }),
        ]
        .to_vec(),
        extra_css: vec!["stylesheets/extra.css".to_string()],
    };

    return mkdocs_config;
}

#[tauri::command]
pub fn update_yaml(wiki_name: &str, app_handle: AppHandle) -> Result<String, String> {
    let data_dir = app_handle.path().app_data_dir().unwrap();
    let dist_folder = data_dir.join(wiki_name).join("dist");

    let mkdocs_yaml_file_path = dist_folder.join("mkdocs.yml");
    let mkdocs_yaml_file = match File::open(&mkdocs_yaml_file_path) {
        Ok(file) => file,
        Err(err) => {
            return Err(format!("Failed to open mkdocs yaml file: {}", err));
        }
    };
    let mut mkdocs_config: MKDocsConfig = match serde_yaml::from_reader(mkdocs_yaml_file) {
        Ok(mkdocs) => mkdocs,
        Err(err) => {
            return Err(format!("Failed to parse mkdocs yaml file: {}", err));
        }
    };

    if !mkdocs_config
        .markdown_extensions
        .contains(&MarkdownExtension::String("pymdownx.details".to_string()))
    {
        mkdocs_config.markdown_extensions = [
            MarkdownExtension::String("admonition".to_string()),
            MarkdownExtension::String("abbr".to_string()),
            MarkdownExtension::String("attr_list".to_string()),
            MarkdownExtension::String("pymdownx.snippets".to_string()),
            MarkdownExtension::String("pymdownx.superfences".to_string()),
            MarkdownExtension::String("pymdownx.details".to_string()),
            MarkdownExtension::PymdownxTaskList(PymdownxTaskList {
                custom_checkbox: true,
            }),
            MarkdownExtension::PymdownxTabbed(PymdownxTabbed {
                alternate_style: true,
            }),
        ]
        .to_vec();
        let mkdocs_file_path = dist_folder.join("mkdocs.yml");
        match fs::write(
            mkdocs_file_path,
            serde_yaml::to_string(&mkdocs_config).unwrap(),
        ) {
            Ok(_) => {}
            Err(err) => {
                return Err(format!("Failed to create mkdocs yaml file: {:?}", err));
            }
        }
        return Ok("Mkdocs yml updated".to_string());
    }

    Ok("".to_string())
}
