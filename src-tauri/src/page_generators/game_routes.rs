use std::{
    collections::HashMap,
    fs::{self, File},
};

use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::{helpers::capitalize::capitalize, structs::mkdocs_structs::Navigation};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Routes {
    pub routes: HashMap<String, RouteProperties>,
    pub encounter_types: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RouteProperties {
    pub position: usize,
    pub trainers: Option<HashMap<String, TrainerInfo>>,
    pub wild_encounters: Option<HashMap<String, Vec<WildEncounter>>>,
    pub wild_encouter_area_levels: Option<HashMap<String, String>>,
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

#[tauri::command]
pub async fn generate_route_pages(
    wiki_name: &str,
    app_handle: AppHandle,
) -> Result<String, String> {
    let base_path = app_handle.path_resolver().app_data_dir().unwrap();
    let docs_path = base_path.join(wiki_name).join("dist").join("docs");

    let routes_json_file_path = base_path.join(wiki_name).join("data").join("routes.json");
    let routes_file = File::open(&routes_json_file_path).unwrap();
    let routes: Routes = serde_json::from_reader(routes_file).unwrap();

    for (route_name, route_properties) in routes.routes.iter() {
        let routes_directory = docs_path.join("routes").join(route_name);
        fs::create_dir_all(&routes_directory).unwrap();

        let formatted_route_name = capitalize(&route_name);

        let mut route_entry = HashMap::new();

        let mut wild_encounters_entry = HashMap::new();
        if let Some(wild_encounters) = &route_properties.wild_encounters {
            println!("{:?}", wild_encounters);
            wild_encounters_entry.insert(
                "Wild Encounters".to_string(),
                Navigation::String(format!("routes/{}/wild_encounters.md", route_name)),
            );
        }

        route_entry.insert(
            formatted_route_name,
            Navigation::Array(vec![Navigation::Map(wild_encounters_entry)]),
        );
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
    Ok("Generating Routes".to_string())
}
