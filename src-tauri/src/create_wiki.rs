use crate::utils::copy_recursively;
use crate::yaml_declaration;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WikiConfig {
    use_side_menu: bool,
    line_breaks: String,
    anchor_character: String,
    title: String,
}

#[tauri::command]
pub fn create_wiki(
    wiki_name: &str,
    wiki_description: &str,
    wiki_author: &str,
    site_name: &str,
    dir: &str,
    resource_dir: &str,
) -> String {
    let base_path: String = format!("{}{}", dir, wiki_name);
    let path_exists: bool = Path::new(&base_path).exists();

    if path_exists {
        return format!("{} already exists", wiki_name);
    }

    fs::create_dir_all(&base_path).unwrap();

    // Copy Starting Data to new wiki
    let wiki_data_folder = format!("{}{}/data", dir, wiki_name);
    let starting_data_folder = format!("{}generator_assets/starting_data", resource_dir);
    let _ = copy_recursively(
        Path::new(&starting_data_folder),
        Path::new(&wiki_data_folder),
    );

    // Create dist folder
    let dist_folder = format!("{}{}/dist", dir, wiki_name);
    fs::create_dir_all(Path::new(&dist_folder)).unwrap();

    let wiki_dist_folder = format!("{}{}/dist", dir, wiki_name);
    let items_folder = format!("{}generator_assets/items", resource_dir);
    let dist_items_folder = format!("{}/docs/img/items", wiki_dist_folder);
    let _ = copy_recursively(Path::new(&items_folder), Path::new(&dist_items_folder));

    let types_folder = format!("{}generator_assets/types", resource_dir);
    let dist_types_folder = format!("{}/docs/img/types", wiki_dist_folder);
    let _ = copy_recursively(Path::new(&types_folder), Path::new(&dist_types_folder));
    let pokemon_images_folder = format!("{}/docs/img/pokemon", wiki_dist_folder);
    fs::create_dir_all(Path::new(&pokemon_images_folder)).unwrap();

    let pokemon_data_folder = format!("{}/docs/pokemon", wiki_dist_folder);
    fs::create_dir_all(Path::new(&pokemon_data_folder)).unwrap();
    let test_pokemon_file_path = format!("{}/docs/pokemon/test_pokemon.md", wiki_dist_folder);
    fs::write(Path::new(&test_pokemon_file_path), "# Placeholder Pokemon").unwrap();

    let test_route_file_path = format!("{}/docs/routes/Test_route", wiki_dist_folder);
    fs::create_dir_all(Path::new(&test_route_file_path)).unwrap();

    let wild_enounters_file_path = format!(
        "{}/docs/routes/Test_route/wild_encounters.md",
        wiki_dist_folder
    );
    fs::write(Path::new(&wild_enounters_file_path), "# Wild Encounters").unwrap();

    let index_file_path = format!("{}/docs/index.md", wiki_dist_folder);
    fs::write(Path::new(&index_file_path), "# Index").unwrap();

    let wiki_config = WikiConfig {
        use_side_menu: true,
        line_breaks: "gfm".to_string(),
        anchor_character: "#".to_string(),
        title: wiki_name.to_string(),
    };

    let config_file_path = format!("{}/docs/config.json", wiki_dist_folder);
    fs::write(
        Path::new(&config_file_path),
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

    let mkdocs_file_path = format!("{}/mkdocs.yml", wiki_dist_folder);
    fs::write(
        Path::new(&mkdocs_file_path),
        serde_yaml::to_string(&mkdocs_config).unwrap(),
    )
    .unwrap();

    return format!("{} Wiki created and initialized", wiki_name);
}
