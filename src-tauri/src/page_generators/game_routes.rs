use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::{
    helpers::{capitalize::capitalize, get_pokemon_dex_formatted_name},
    structs::{
        mkdocs_structs::{MKDocsConfig, Navigation},
        pokemon_structs::Pokemon,
    },
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
    pub sprite: String,
    pub versions: Vec<String>,
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
pub async fn generate_route_page_with_handle(
    wiki_name: &str,
    app_handle: AppHandle,
) -> Result<String, String> {
    let base_path = app_handle.path_resolver().app_data_dir().unwrap();
    return generate_route_pages(&wiki_name, base_path);
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
            create_encounter_table(
                wiki_name,
                route_name,
                &routes_directory,
                &route_properties.wild_encounters,
                &route_properties.wild_encounter_area_levels,
            )
            .unwrap();
            let mut wild_encounters_entry = HashMap::new();
            wild_encounters_entry.insert(
                "Wild Encounter".to_string(),
                Navigation::String(format!("routes/{}/wild_encounters.md", route_name)),
            );
            entries.push(Navigation::Map(wild_encounters_entry));
        }
        if !route_properties.trainers.is_empty() {
            create_trainer_table(
                wiki_name,
                route_name,
                &routes_directory,
                &route_properties.trainers,
            )
            .unwrap();
            let mut trainers_entry = HashMap::new();
            trainers_entry.insert(
                "Trainers".to_string(),
                Navigation::String(format!("routes/{}/trainers.md", route_name)),
            );
            entries.push(Navigation::Map(trainers_entry));
        }

        if entries.is_empty() {
            continue;
        }

        route_entry.insert(formatted_route_name, Navigation::Array(entries));

        mkdoc_routes.push(Navigation::Map(route_entry));
    }

    let paths = fs::read_dir(&docs_path.join("routes")).unwrap();
    for path in paths {
        let path_name = path
            .as_ref()
            .ok()
            .unwrap()
            .file_name()
            .into_string()
            .unwrap();

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

fn create_encounter_table(
    wiki_name: &str,
    route_name: &str,
    routes_directory: &PathBuf,
    encounters: &HashMap<String, Vec<WildEncounter>>,
    encounter_areas_levels: &HashMap<String, String>,
) -> Result<(), String> {
    let mut encounters_markdown_file =
        File::create(routes_directory.join("wild_encounters.md")).unwrap();

    encounters_markdown_file
        .write_all(format!("# {}\n\n", capitalize(route_name)).as_bytes())
        .unwrap();

    let mut markdown_encounters = String::new();
    for (encounter_type, pokemon_encounter_list) in encounters {
        let mut pokemon_entries = String::new();
        for (index, pokemon) in pokemon_encounter_list.iter().enumerate() {
            let mut entry = format!("| {} ", get_markdown_entry_for_pokemon(wiki_name, &pokemon));
            // It's possible for a single route to have more than 6 pokemon
            // We want break the entries into a new line at that 6th position.
            if index != 0 && index % 6 == 0 {
                entry = format!(
                    "\n| | {} ",
                    get_markdown_entry_for_pokemon(wiki_name, &pokemon)
                );
            }
            pokemon_entries.push_str(&entry);
        }
        pokemon_entries.push_str("|");

        let encounter_type_entry = match encounter_areas_levels.get(&encounter_type.clone()) {
            Some(area_level) => format!("{}<br/> lv {}", &encounter_type, area_level),
            None => encounter_type.to_string(),
        };
        let encounter_entry = format!("| {} {}\n", encounter_type_entry, pokemon_entries);
        markdown_encounters.push_str(&encounter_entry);
    }

    encounters_markdown_file
        .write_all(
            format!(
                "| Area | Pokemon | | | | | |
        | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
        {}
        ",
                markdown_encounters
            )
            .as_bytes(),
        )
        .unwrap();
    Ok(())
}

fn create_trainer_table(
    wiki_name: &str,
    route_name: &str,
    routes_directory: &PathBuf,
    trainers: &HashMap<String, TrainerInfo>,
) -> Result<(), String> {
    let mut trainers_markdown_file = File::create(routes_directory.join("trainers.md")).unwrap();

    trainers_markdown_file
        .write_all(format!("# {}\n\n", capitalize(route_name)).as_bytes())
        .unwrap();

    let mut markdown_trainers = String::new();
    for (name, trainer_info) in trainers {
        let mut pokemon_team = format!("\n| {}", get_trainer_sprite(name, &trainer_info.sprite));
        let mut header_divider = format!("| :--: ");
        let mut levels = format!("| <strong>Level</stong> ");
        let mut items = format!("| <strong>Item</stong> ");
        let mut natures = format!("| <strong>Nature</stong> ");
        let mut abilities = format!("| <strong>Ability</stong> ");

        for pokemon in &trainer_info.pokemon_team {
            let pokemon_entry = format!(
                "| {} ",
                get_markdown_entry_for_trainer_pokemon(wiki_name, pokemon)
            );
            let level_entry = format!("| {} ", pokemon.level);
            let item_entry = format!("| {} ", extract_pokemon_attribute(&pokemon.item));
            let nature_entry = format!("| {} ", extract_pokemon_attribute(&pokemon.nature));
            let ability_entry = format!("| {} ", extract_pokemon_attribute(&pokemon.ability));

            pokemon_team.push_str(&pokemon_entry);
            header_divider.push_str("| :--: ");
            levels.push_str(&level_entry);
            items.push_str(&item_entry);
            natures.push_str(&nature_entry);
            abilities.push_str(&ability_entry);
        }
        pokemon_team.push_str("|");
        header_divider.push_str("|");
        levels.push_str("|");
        items.push_str("|");
        natures.push_str("|");
        abilities.push_str("|");

        let trainer_entry = format!(
            "{}
            {}
            {}
            {}
            {}
            {}
            ",
            &pokemon_team, &header_divider, &levels, &items, &natures, &abilities
        );
        markdown_trainers.push_str(&trainer_entry);
    }
    trainers_markdown_file
        .write_all(
            format!(
                "{}
                ",
                markdown_trainers
            )
            .as_bytes(),
        )
        .unwrap();
    Ok(())
}

fn get_markdown_entry_for_pokemon(wiki_name: &str, pokemon: &WildEncounter) -> String {
    let dex_number_file_name = get_pokemon_dex_formatted_name(pokemon.id);
    return format!(
        "![{}](../../img/pokemon/{}.png)<br/> [{}](/{}/pokemon/{})<br/> {}%",
        pokemon.name,
        dex_number_file_name,
        capitalize(&pokemon.name),
        wiki_name,
        dex_number_file_name,
        pokemon.encounter_rate
    );
}

fn get_markdown_entry_for_trainer_pokemon(wiki_name: &str, pokemon: &TrainerPokemon) -> String {
    let dex_number_file_name = get_pokemon_dex_formatted_name(pokemon.id);
    return format!(
        "![{}](../../img/pokemon/{}.png)<br/> [{}](/{}/pokemon/{})<br/> Lv. {}",
        pokemon.name,
        dex_number_file_name,
        capitalize(&pokemon.name),
        wiki_name,
        dex_number_file_name,
        pokemon.level
    );
}

fn get_trainer_sprite(name: &str, sprite: &str) -> String {
    if sprite == "".to_string() {
        return name.to_string();
    }
    return format!(
        "{}<br/> ![{}](https://play.pokemonshowdown.com/sprites/trainers/{}.png)",
        name, name, sprite
    );
}

fn extract_pokemon_attribute(attribute: &Option<String>) -> String {
    match attribute.clone() {
        Some(attribute) => {
            if attribute == "" {
                return "-".to_string();
            } else {
                return attribute;
            }
        }
        None => return "-".to_string(),
    }
}
