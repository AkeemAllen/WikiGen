use std::{
    collections::HashMap,
    fs::{self, File},
};

use serde::{Deserialize, Serialize};
use tauri::AppHandle;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Routes {
    pub routes: HashMap<String, RouteProperties>,
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
pub async fn create_new_route(
    wiki_name: &str,
    route_name: &str,
    app_handle: AppHandle,
) -> Result<Routes, String> {
    let base_path = app_handle.path_resolver().app_data_dir().unwrap();
    let routes_path = base_path.join(wiki_name).join("data").join("routes.json");
    let routes_file = File::open(&routes_path).unwrap();
    let mut routes: Routes = serde_json::from_reader(routes_file).unwrap();

    let position = routes.routes.keys().len();
    routes.routes.insert(
        (&route_name).to_string(),
        RouteProperties {
            position,
            trainers: None,
            wild_encounters: None,
            wild_encouter_area_levels: None,
        },
    );

    fs::write(routes_path, serde_json::to_string(&routes).unwrap()).unwrap();

    Ok(routes)
}
