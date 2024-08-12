use crate::helpers::{capitalize, copy_recursively};
use crate::wiki_preparation::yaml_declaration;
use serde::{Deserialize, Serialize};
use std::fs::{self};
use tauri::AppHandle;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WikiConfig {
    use_side_menu: bool,
    line_breaks: String,
    anchor_character: String,
    title: String,
}

#[tauri::command]
pub async fn create_wiki(
    wiki_name: &str,
    wiki_description: &str,
    wiki_author: &str,
    site_name: &str,
    app_handle: AppHandle,
) -> Result<String, String> {
    let data_dir = app_handle.path_resolver().app_data_dir().unwrap();
    let base_path = data_dir.join(wiki_name);
    if base_path.exists() {
        return Err(format!("{} already exists", wiki_name));
    }

    let resource_dir = app_handle.path_resolver().resource_dir().unwrap();
    let resource_path = resource_dir.join("resources");

    match fs::create_dir_all(&base_path) {
        Ok(_) => {}
        Err(err) => {
            return Err(format!(
                "Failed to create {} wiki's base path: {:?}",
                wiki_name, err
            ));
        }
    }

    // Create dist folder
    let dist_folder = base_path.join("dist");
    let docs_folder = dist_folder.join("docs");

    match fs::create_dir_all(&docs_folder) {
        Ok(_) => {}
        Err(err) => {
            return Err(format!("Failed to create dist directory path: {:?}", err));
        }
    }

    let pokemon_data_folder = docs_folder.join("pokemon");
    match fs::create_dir_all(&pokemon_data_folder) {
        Ok(_) => {}
        Err(err) => {
            return Err(format!(
                "Failed to create pokemon directory path: {:?}",
                err
            ));
        }
    }

    let routes_folder = docs_folder.join("routes");
    match fs::create_dir_all(&routes_folder) {
        Ok(_) => {}
        Err(err) => {
            return Err(format!("Failed to create routes directory path: {:?}", err));
        }
    }

    // Copy Starting Data to new wiki
    let wiki_data_folder = base_path.join("data");
    let starting_data_folder = resource_path.join("generator_assets").join("starting_data");
    let _ = copy_recursively(starting_data_folder, wiki_data_folder);

    let generator_assets_path = resource_path.join("generator_assets");

    let sqlite_db_path = generator_assets_path.join("initial.db");
    match fs::copy(sqlite_db_path, base_path.join(format!("{}.db", wiki_name))) {
        Ok(_) => {}
        Err(err) => {
            return Err(format!("Failed to copy initial database: {:?}", err));
        }
    }

    let items_folder = generator_assets_path.join("items");
    let dist_items_folder = docs_folder.join("img").join("items");
    let _ = copy_recursively(items_folder, dist_items_folder);

    let types_folder = generator_assets_path.join("types");
    let dist_types_folder = docs_folder.join("img").join("types");
    let _ = copy_recursively(types_folder, dist_types_folder);

    let stylesheets_folder = generator_assets_path.join("stylesheets");
    let dist_stylesheets_folder = docs_folder.join("stylesheets");
    let _ = copy_recursively(stylesheets_folder, dist_stylesheets_folder);

    let templates_folder = generator_assets_path.join("templates");
    let dist_templates_folder = docs_folder.join("templates");
    let _ = copy_recursively(templates_folder, dist_templates_folder);

    let pokemon_sprites_folder = generator_assets_path.join("pokemon_sprites");
    let pokemon_images_folder = docs_folder.join("img").join("pokemon");
    let _ = copy_recursively(pokemon_sprites_folder, pokemon_images_folder);

    let index_file_path = docs_folder.join("index.md");
    match fs::write(index_file_path, "# Index") {
        Ok(_) => {}
        Err(err) => {
            return Err(format!("Failed to create index/Homepage file: {:?}", err));
        }
    }

    let wiki_config = WikiConfig {
        use_side_menu: true,
        line_breaks: "gfm".to_string(),
        anchor_character: "#".to_string(),
        title: wiki_name.to_string(),
    };

    let config_file_path = docs_folder.join("config.json");
    match fs::write(
        config_file_path,
        serde_json::to_string(&wiki_config).unwrap(),
    ) {
        Ok(_) => {}
        Err(err) => {
            return Err(format!("Failed to create mkdocs config file: {:?}", err));
        }
    }

    let repo_url = format!("https://github.com/{}/{}", wiki_author, wiki_name);
    let site_url = format!(
        "https://{}.github.io/{}",
        wiki_author.to_lowercase(),
        wiki_name
    );

    let mkdocs_config = yaml_declaration::get_yaml(
        site_name,
        &site_url,
        wiki_description,
        wiki_author,
        &repo_url,
    );

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

    return Ok(format!(
        "{} Wiki created and initialized",
        capitalize(wiki_name)
    ));
}
