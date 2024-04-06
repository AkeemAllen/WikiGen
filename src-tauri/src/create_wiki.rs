use crate::yaml_declaration;
use serde::{Deserialize, Serialize};
use std::{fs, io, path::Path};

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
    let _ = copy_recursively(starting_data_folder, wiki_data_folder);

    // Create dist folder
    fs::create_dir_all(format!("{}{}/dist", dir, wiki_name)).unwrap();

    let wiki_dist_folder = format!("{}{}/dist", dir, wiki_name);
    let _ = copy_recursively(
        format!("{}generator_assets/items", resource_dir),
        format!("{}/docs/img/items", wiki_dist_folder),
    );
    let _ = copy_recursively(
        format!("{}generator_assets/types", resource_dir),
        format!("{}/docs/img/types", wiki_dist_folder),
    );
    fs::create_dir_all(format!("{}/docs/img/pokemon", wiki_dist_folder)).unwrap();

    fs::create_dir_all(format!("{}/docs/pokemon", wiki_dist_folder)).unwrap();
    fs::write(
        format!("{}/docs/pokemon/test_pokemon.md", wiki_dist_folder),
        "# Placeholder Pokemon",
    )
    .unwrap();

    fs::create_dir_all(format!("{}/docs/routes/Test_route", wiki_dist_folder)).unwrap();
    fs::write(
        format!(
            "{}/docs/routes/Test_route/wild_encounters.md",
            wiki_dist_folder
        ),
        "# Wild Encounters",
    )
    .unwrap();

    fs::write(format!("{}/docs/index.md", wiki_dist_folder), "# Index").unwrap();

    let wiki_config = WikiConfig {
        use_side_menu: true,
        line_breaks: "gfm".to_string(),
        anchor_character: "#".to_string(),
        title: wiki_name.to_string(),
    };

    fs::write(
        format!("{}/docs/config.json", wiki_dist_folder),
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

    fs::write(
        format!("{}/mkdocs.yml", wiki_dist_folder),
        serde_yaml::to_string(&mkdocs_config).unwrap(),
    )
    .unwrap();

    return format!("{} Wiki created and initialized", wiki_name);
}

/// Copy files from source to destination recursively.
pub fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
