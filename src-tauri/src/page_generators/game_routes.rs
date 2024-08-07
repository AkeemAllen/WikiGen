use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_yaml::{Mapping, Value};
use tauri::AppHandle;

use crate::{
    helpers::{capitalize, capitalize_and_remove_hyphens, get_pokemon_dex_formatted_name},
    structs::mkdocs_structs::MKDocsConfig,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Routes {
    pub routes: IndexMap<String, RouteProperties>,
    pub encounter_types: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RouteProperties {
    pub position: usize,
    pub trainers: IndexMap<String, TrainerInfo>,
    pub wild_encounters: HashMap<String, Vec<WildEncounter>>,
    pub wild_encounter_area_levels: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrainerInfo {
    pub position: usize,
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
    pub moves: Vec<String>,
    pub item: String,
    pub nature: String,
    pub ability: String,
    pub trainer_versions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WildEncounter {
    pub id: usize,
    pub name: String,
    pub encounter_rate: usize,
}

#[tauri::command]
pub async fn generate_route_pages_with_handle(
    wiki_name: &str,
    app_handle: AppHandle,
) -> Result<String, String> {
    let base_path = app_handle.path_resolver().app_data_dir().unwrap();
    return generate_route_pages(&wiki_name, base_path);
}

#[tauri::command]
pub async fn generate_single_route_page_with_handle(
    wiki_name: &str,
    route_name: &str,
    app_handle: AppHandle,
) -> Result<String, String> {
    let base_path = app_handle.path_resolver().app_data_dir().unwrap();
    return generate_single_route(route_name, wiki_name, base_path);
}

pub fn generate_single_route(
    route_name: &str,
    wiki_name: &str,
    base_path: PathBuf,
) -> Result<String, String> {
    let docs_path = base_path.join(wiki_name).join("dist").join("docs");

    let routes_json_file_path = base_path.join(wiki_name).join("data").join("routes.json");
    let routes_file = match File::open(&routes_json_file_path) {
        Ok(file) => file,
        Err(err) => return Err(format!("Failed to read routes file: {}", err)),
    };
    let routes: Routes = match serde_json::from_reader(routes_file) {
        Ok(routes) => routes,
        Err(err) => return Err(format!("Failed to parse routes file: {}", err)),
    };

    let mkdocs_yaml_file_path = base_path.join(wiki_name).join("dist").join("mkdocs.yml");
    let mkdocs_yaml_file = match File::open(&mkdocs_yaml_file_path) {
        Ok(mkdocs) => mkdocs,
        Err(err) => return Err(format!("Failed to read Mkdocs yaml file: {}", err)),
    };
    let mut mkdocs_config: MKDocsConfig = match serde_yaml::from_reader(mkdocs_yaml_file) {
        Ok(config) => config,
        Err(err) => return Err(format!("Failed to parse Mkdocs yaml file: {}", err)),
    };

    let mut mkdocs_routes: &mut Vec<Value> = &mut Vec::new();

    let nav_entries = mkdocs_config.nav.as_sequence_mut().unwrap();
    for entry in nav_entries {
        let map_entries = entry.as_mapping_mut().unwrap();
        match map_entries.get_mut(Value::String("Routes".to_string())) {
            Some(map_entry) => {
                mkdocs_routes = map_entry.as_sequence_mut().unwrap();
            }
            None => {}
        }
    }

    let mut page = &mut Mapping::new();
    let mut page_position = 0;
    for (index, page_entry) in mkdocs_routes.iter_mut().enumerate() {
        if page_entry.as_mapping().unwrap().contains_key(&route_name) {
            page = page_entry.as_mapping_mut().unwrap();
            page_position = index;
            break;
        }
    }

    let routes_directory = docs_path.join("routes").join(route_name);
    let route_properties = routes.routes.get(route_name).unwrap();

    if route_properties.trainers.is_empty() && route_properties.wild_encounters.is_empty() {
        if page.is_empty() {
            return Ok("Route is empty".to_string());
        }
        mkdocs_routes.remove(page_position);
        match fs::remove_dir_all(&routes_directory) {
            Ok(_) => {}
            Err(err) => return Err(format!("Failed to deleted routes directory: {}", err)),
        };
        match fs::write(
            mkdocs_yaml_file_path,
            serde_yaml::to_string(&mkdocs_config).unwrap(),
        ) {
            Ok(_) => {}
            Err(err) => return Err(format!("Failed to update Mkdocs file: {}", err)),
        };
        return Ok("Route is empty. Removed empty route".to_string());
    }

    fs::create_dir_all(&routes_directory).unwrap();

    let route_entry =
        generate_route_entry(wiki_name, route_name, &routes_directory, route_properties);

    if page.is_empty() {
        mkdocs_routes.push(Value::Mapping(route_entry.clone()))
    } else {
        *page = Value::Mapping(route_entry.clone())
            .as_mapping()
            .unwrap()
            .clone();
    }

    match fs::write(
        mkdocs_yaml_file_path,
        serde_yaml::to_string(&mkdocs_config).unwrap(),
    ) {
        Ok(_) => {}
        Err(err) => return Err(format!("Failed to update Mkdocs file: {}", err)),
    };

    Ok("".to_string())
}

pub fn generate_route_pages(wiki_name: &str, base_path: PathBuf) -> Result<String, String> {
    let docs_path = base_path.join(wiki_name).join("dist").join("docs");

    let routes_json_file_path = base_path.join(wiki_name).join("data").join("routes.json");
    let routes_file = match File::open(&routes_json_file_path) {
        Ok(file) => file,
        Err(err) => return Err(format!("Failed to read routes file: {}", err)),
    };
    let routes: Routes = match serde_json::from_reader(routes_file) {
        Ok(routes) => routes,
        Err(err) => return Err(format!("Failed to parse routes file: {}", err)),
    };

    let mkdocs_yaml_file_path = base_path.join(wiki_name).join("dist").join("mkdocs.yml");
    let mkdocs_yaml_file = match File::open(&mkdocs_yaml_file_path) {
        Ok(mkdocs) => mkdocs,
        Err(err) => return Err(format!("Failed to read Mkdocs yaml file: {}", err)),
    };
    let mut mkdocs_config: MKDocsConfig = match serde_yaml::from_reader(mkdocs_yaml_file) {
        Ok(config) => config,
        Err(err) => return Err(format!("Failed to parse Mkdocs yaml file: {}", err)),
    };

    let mut mkdocs_routes: &mut Vec<Value> = &mut Vec::new();

    let nav_entries = mkdocs_config.nav.as_sequence_mut().unwrap();
    for entry in nav_entries {
        let map_entries = entry.as_mapping_mut().unwrap();
        match map_entries.get_mut(Value::String("Routes".to_string())) {
            Some(map_entry) => {
                mkdocs_routes = map_entry.as_sequence_mut().unwrap();
            }
            None => {}
        }
    }

    mkdocs_routes.clear();
    for (route_name, route_properties) in routes.routes.iter() {
        let routes_directory = docs_path.join("routes").join(route_name);

        match fs::create_dir_all(&routes_directory) {
            Ok(_) => {}
            Err(err) => return Err(format!("Failed to create routes directory: {}", err)),
        };

        let route_entry =
            generate_route_entry(wiki_name, route_name, &routes_directory, route_properties);

        if route_entry.is_empty() {
            continue;
        }

        mkdocs_routes.push(Value::Mapping(route_entry));
    }

    let paths = match fs::read_dir(&docs_path.join("routes")) {
        Ok(directories) => directories,
        Err(err) => return Err(format!("Failed to read routes directory: {}", err)),
    };
    for path in paths {
        let path_name = path
            .as_ref()
            .ok()
            .unwrap()
            .file_name()
            .into_string()
            .unwrap();

        if !routes.routes.contains_key(&path_name) {
            match fs::remove_dir_all(&path.unwrap().path()) {
                Ok(_) => {}
                Err(err) => return Err(format!("Failed to move directory: {}", err)),
            };
        }
    }

    match fs::write(
        mkdocs_yaml_file_path,
        serde_yaml::to_string(&mkdocs_config.clone()).unwrap(),
    ) {
        Ok(_) => {}
        Err(err) => return Err(format!("Failed to update mkdocs yaml: {}", err)),
    };

    Ok("Generating Routes".to_string())
}

fn generate_route_entry(
    wiki_name: &str,
    route_name: &str,
    routes_directory: &PathBuf,
    route_properties: &RouteProperties,
) -> Mapping {
    let mut entries: Vec<Value> = Vec::new();

    if !route_properties.wild_encounters.is_empty() {
        create_encounter_table(
            wiki_name,
            route_name,
            &routes_directory,
            &route_properties.wild_encounters,
            &route_properties.wild_encounter_area_levels,
        )
        .unwrap();
        let mut wild_encounters_entry = Mapping::new();
        wild_encounters_entry.insert(
            Value::String("Wild Encounter".to_string()),
            Value::String(format!("routes/{}/wild_encounters.md", route_name)),
        );
        entries.push(Value::Mapping(wild_encounters_entry));
    }
    if !route_properties.trainers.is_empty() {
        create_trainer_table(
            wiki_name,
            route_name,
            &routes_directory,
            &route_properties.trainers,
        )
        .unwrap();
        let mut trainers_entry = Mapping::new();
        trainers_entry.insert(
            Value::String("Trainers".to_string()),
            Value::String(format!("routes/{}/trainers.md", route_name)),
        );
        entries.push(Value::Mapping(trainers_entry));
    }

    let mut route_entry = Mapping::new();

    if entries.is_empty() {
        return route_entry;
    }

    route_entry.insert(
        Value::String(route_name.to_string()),
        Value::Sequence(entries),
    );
    return route_entry;
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
            Some(area_level) => format!(
                "{}<br/> Lv. {}",
                capitalize_and_remove_hyphens(&encounter_type),
                area_level
            ),
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
    trainers: &IndexMap<String, TrainerInfo>,
) -> Result<(), String> {
    let mut trainers_markdown_file = File::create(routes_directory.join("trainers.md")).unwrap();

    trainers_markdown_file
        .write_all(format!("# {}\n\n", capitalize(route_name)).as_bytes())
        .unwrap();

    let mut markdown_trainers = String::new();
    for (name, trainer_info) in trainers {
        if trainer_info.versions.is_empty() {
            let trainer_entry = generate_trainer_entry(wiki_name, name, trainer_info, "");
            markdown_trainers.push_str(&format!("\n{}", trainer_entry));
        } else {
            for version in &trainer_info.versions {
                // This is to prevent rendering a trainer version that doesn't have a pokemon
                // This is probably not the cleanest/most effecient way to do this.
                // May revisit later...maybe
                let mut version_has_one_pokemon = false;
                for pokemon in &trainer_info.pokemon_team {
                    if pokemon.trainer_versions.contains(version) {
                        version_has_one_pokemon = true;
                    }
                }
                if !version_has_one_pokemon {
                    continue;
                }
                markdown_trainers.push_str(&format!("\n=== \"{}\"", version));
                let trainer_entry = generate_trainer_entry(wiki_name, name, trainer_info, version);
                markdown_trainers.push_str(&format!("\t{}", trainer_entry));
            }
        }
        markdown_trainers.push_str("<br/>");
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
    let dex_number_file_name = get_pokemon_dex_formatted_name(pokemon.id.try_into().unwrap());
    return format!(
        "![{}](../../img/pokemon/{}.png)<br/> [{}](/{}/pokemon/{})<br/> {}%",
        pokemon.name,
        pokemon.name,
        capitalize(&pokemon.name),
        wiki_name,
        format!("{}-{}", dex_number_file_name, pokemon.name),
        pokemon.encounter_rate
    );
}

fn get_markdown_entry_for_trainer_pokemon(wiki_name: &str, pokemon: &TrainerPokemon) -> String {
    let dex_number_file_name = get_pokemon_dex_formatted_name(pokemon.id.try_into().unwrap());
    return format!(
        "![{}](../../img/pokemon/{}.png)<br/> [{}](/{}/pokemon/{})",
        pokemon.name,
        pokemon.name,
        capitalize(&pokemon.name),
        wiki_name,
        format!("{}-{}", dex_number_file_name, pokemon.name),
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

fn extract_move(_move: Option<&String>) -> String {
    match _move.clone() {
        Some(_move) => _move.to_string(),
        None => return "-".to_string(),
    }
}

fn evaluate_attribute(attribute: &str) -> String {
    match attribute {
        "" => return "-".to_string(),
        _ => return attribute.to_string(),
    }
}

fn generate_trainer_entry(
    wiki_name: &str,
    name: &str,
    trainer_info: &TrainerInfo,
    version: &str,
) -> String {
    let mut pokemon_team = format!("\n\t| {}", get_trainer_sprite(name, &trainer_info.sprite));
    if version == "" {
        pokemon_team = format!("\n| {}", get_trainer_sprite(name, &trainer_info.sprite))
    }
    let mut header_divider = format!("| :-- ");
    let mut levels = format!("| <strong>Level</stong> ");
    let mut items = format!("| <strong>Item</stong> ");
    let mut natures = format!("| <strong>Nature</stong> ");
    let mut abilities = format!("| <strong>Ability</stong> ");
    let mut move1 = format!("| <strong>Move 1</stong> ");
    let mut move2 = format!("| <strong>Move 2</stong> ");
    let mut move3 = format!("| <strong>Move 3</stong> ");
    let mut move4 = format!("| <strong>Move 4</stong> ");

    for pokemon in &trainer_info.pokemon_team {
        if !pokemon.trainer_versions.contains(&version.to_string()) && version != "" {
            continue;
        }

        let pokemon_entry = format!(
            "| {} ",
            get_markdown_entry_for_trainer_pokemon(wiki_name, pokemon)
        );
        let level_entry = format!("| {} ", pokemon.level);
        let item_entry = format!("| {} ", evaluate_attribute(&pokemon.item));
        let nature_entry = format!("| {} ", evaluate_attribute(&pokemon.nature));
        let ability_entry = format!("| {} ", evaluate_attribute(&pokemon.ability));
        let move1_entry = format!("| {} ", extract_move(pokemon.moves.get(0)));
        let move2_entry = format!("| {} ", extract_move(pokemon.moves.get(1)));
        let move3_entry = format!("| {} ", extract_move(pokemon.moves.get(2)));
        let move4_entry = format!("| {} ", extract_move(pokemon.moves.get(3)));

        pokemon_team.push_str(&pokemon_entry);
        header_divider.push_str("| :-- ");
        levels.push_str(&level_entry);
        items.push_str(&item_entry);
        natures.push_str(&nature_entry);
        abilities.push_str(&ability_entry);
        move1.push_str(&move1_entry);
        move2.push_str(&move2_entry);
        move3.push_str(&move3_entry);
        move4.push_str(&move4_entry);
    }
    pokemon_team.push_str("|");
    header_divider.push_str("|");
    levels.push_str("|");
    items.push_str("|");
    natures.push_str("|");
    abilities.push_str("|");
    move1.push_str("|");
    move2.push_str("|");
    move3.push_str("|");
    move4.push_str("|");

    return format!(
        "{}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        ",
        &pokemon_team,
        &header_divider,
        &levels,
        &items,
        &natures,
        &abilities,
        &move1,
        &move2,
        &move3,
        &move4
    );
}
