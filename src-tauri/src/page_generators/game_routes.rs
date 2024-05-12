use std::{
    collections::HashMap,
    fs::{self, File},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::{
    helpers::capitalize::capitalize,
    structs::mkdocs_structs::{MKDocsConfig, Navigation},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Routes {
    pub routes: HashMap<String, RouteProperties>,
    pub encounter_types: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RouteProperties {
    pub position: usize,
    pub trainers: HashMap<String, TrainerInfo>,
    pub wild_encounters: HashMap<String, Vec<WildEncounter>>,
    pub wild_encounter_area_levels: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrainerInfo {
    pub pokemon_team: Vec<TrainerPokemon>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrainerPokemon {
    pub id: usize,
    pub unique_id: String,
    pub name: String,
    pub level: usize,
    pub moves: Option<[String; 4]>,
    pub item: Option<String>,
    pub nature: Option<String>,
    pub ability: Option<String>,
    pub tainer_version: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WildEncounter {
    pub id: usize,
    pub name: String,
    pub encounter_rate: usize,
}

pub fn generate_route_pages(wiki_name: &str, base_path: PathBuf) -> Result<String, String> {
    let docs_path = base_path.join(wiki_name).join("dist").join("docs");

    let routes_json_file_path = base_path.join(wiki_name).join("data").join("routes.json");
    let routes_file = File::open(&routes_json_file_path).unwrap();
    let routes: Routes = serde_json::from_reader(routes_file).unwrap();

    let mkdocs_yaml_file_path = base_path.join(wiki_name).join("dist").join("mkdocs.yml");
    let mkdocs_yaml_file = File::open(&mkdocs_yaml_file_path).unwrap();
    let mut mkdocs_config: MKDocsConfig = serde_yaml::from_reader(mkdocs_yaml_file).unwrap();

    let mut mkdoc_routes = Vec::new();

    for (route_name, route_properties) in routes.routes.iter() {
        let routes_directory = docs_path.join("routes").join(route_name);
        fs::create_dir_all(&routes_directory).unwrap();

        let formatted_route_name = capitalize(&route_name);

        let mut route_entry = HashMap::new();
        let mut entries = Vec::new();

        if !route_properties.wild_encounters.is_empty() {
            let mut wild_encounters_entry = HashMap::new();
            wild_encounters_entry.insert(
                "Wild Encounter".to_string(),
                Navigation::String(format!("routes/{}/wild_encounters.md", route_name)),
            );
            entries.push(Navigation::Map(wild_encounters_entry));
        }
        if !route_properties.trainers.is_empty() {
            let mut trainers_entry = HashMap::new();
            trainers_entry.insert(
                "Trainers".to_string(),
                Navigation::String(format!("routes/{}/trainers.md", route_name)),
            );
            entries.push(Navigation::Map(trainers_entry));
        }

        route_entry.insert(formatted_route_name, Navigation::Array(entries));

        mkdoc_routes.push(Navigation::Map(route_entry));
    }

    let paths = fs::read_dir(&docs_path.join("routes")).unwrap();
    for path in paths {
        let path_name = capitalize(
            &path
                .as_ref()
                .ok()
                .unwrap()
                .file_name()
                .into_string()
                .unwrap(),
        );

        if !routes.routes.contains_key(&path_name) {
            fs::remove_dir_all(&path.unwrap().path()).unwrap();
        }
    }

    if let Some(nav_routes) = mkdocs_config.nav[2].get_mut("Routes") {
        *nav_routes = Navigation::Array(mkdoc_routes);
    }

    fs::write(
        mkdocs_yaml_file_path,
        serde_yaml::to_string(&mkdocs_config).unwrap(),
    )
    .unwrap();

    Ok("Generating Routes".to_string())
}

#[tauri::command]
pub async fn generate_route_page_with_handle(
    wiki_name: &str,
    app_handle: AppHandle,
) -> Result<String, String> {
    let base_path = app_handle.path_resolver().app_data_dir().unwrap();
    return generate_route_pages(wiki_name, base_path);
}
