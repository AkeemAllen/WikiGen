use crate::helpers::copy_recursively;
use crate::wiki_preparation::yaml_declaration;
use serde::{Deserialize, Serialize};
use std::fs;
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
    let resource_dir = app_handle.path_resolver().resource_dir().unwrap();
    let resource_path = resource_dir.join("resources");

    if base_path.exists() {
        return Err(format!("{} already exists", wiki_name));
    }

    fs::create_dir_all(&base_path).unwrap();

    // Copy Starting Data to new wiki
    let wiki_data_folder = base_path.join("data");
    let starting_data_folder = resource_path.join("generator_assets").join("starting_data");
    let _ = copy_recursively(starting_data_folder, wiki_data_folder);

    // Create dist folder
    let dist_folder = base_path.join("dist");
    let docs_folder = dist_folder.join("docs");
    fs::create_dir_all(&docs_folder).unwrap();

    let generator_assets_path = resource_path.join("generator_assets");

    let items_folder = generator_assets_path.join("items");
    let dist_items_folder = docs_folder.join("img").join("items");
    let _ = copy_recursively(items_folder, dist_items_folder);

    let types_folder = generator_assets_path.join("types");
    let dist_types_folder = docs_folder.join("img").join("types");
    let _ = copy_recursively(types_folder, dist_types_folder);

    let stylesheets_folder = generator_assets_path.join("stylesheets");
    let dist_stylesheets_folder = docs_folder.join("stylesheets");
    let _ = copy_recursively(stylesheets_folder, dist_stylesheets_folder);

    let pokemon_images_folder = docs_folder.join("img").join("pokemon");
    fs::create_dir_all(&pokemon_images_folder).unwrap();

    let pokemon_data_folder = docs_folder.join("pokemon");
    fs::create_dir_all(pokemon_data_folder).unwrap();

    let test_pokemon_file_path = docs_folder.join("pokemon").join("test_pokemon.md");
    fs::write(test_pokemon_file_path, "# Placeholder Pokemon").unwrap();

    let test_route_file_path = docs_folder.join("routes").join("Test_route");
    fs::create_dir_all(&test_route_file_path).unwrap();

    let wild_enounters_file_path = docs_folder
        .join("routes")
        .join("Test_route")
        .join("wild_encounters.md");
    fs::write(wild_enounters_file_path, "# Wild Encounters").unwrap();

    let index_file_path = docs_folder.join("index.md");
    fs::write(index_file_path, "# Index").unwrap();

    let wiki_config = WikiConfig {
        use_side_menu: true,
        line_breaks: "gfm".to_string(),
        anchor_character: "#".to_string(),
        title: wiki_name.to_string(),
    };

    let config_file_path = docs_folder.join("config.json");
    fs::write(
        config_file_path,
        serde_json::to_string(&wiki_config).unwrap(),
    )
    .unwrap();

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
    fs::write(
        mkdocs_file_path,
        serde_yaml::to_string(&mkdocs_config).unwrap(),
    )
    .unwrap();

    return Ok(format!("{} Wiki created and initialized", wiki_name));
}
