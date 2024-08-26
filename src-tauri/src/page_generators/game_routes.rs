use std::{
    collections::HashMap,
    fs::{self, read_to_string, File},
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
    pub encounter_areas: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RouteProperties {
    pub position: usize,
    pub trainers: IndexMap<String, TrainerInfo>,
    pub wild_encounters: IndexMap<String, Vec<WildEncounter>>,
    pub wild_encounter_area_levels: IndexMap<String, String>,
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
    pub encounter_area: String,
    pub route: String,
    pub special_note: String,
}

#[tauri::command]
pub async fn generate_route_pages_with_handle(
    wiki_name: &str,
    route_names: Vec<&str>,
    app_handle: AppHandle,
) -> Result<String, String> {
    let base_path = app_handle.path_resolver().app_data_dir().unwrap();
    let resources_path = app_handle.path_resolver().resource_dir().unwrap();
    return generate_route_pages(
        &wiki_name,
        base_path.clone(),
        resources_path.clone(),
        route_names,
    );
}

pub fn generate_route_pages(
    wiki_name: &str,
    base_path: PathBuf,
    resources_path: PathBuf,
    route_names: Vec<&str>,
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

    let template = match read_to_string(
        resources_path
            .join("resources")
            .join("generator_assets")
            .join("templates")
            .join("route_page_template.md"),
    ) {
        Ok(template) => template,
        Err(err) => {
            return Err(format!("Failed to read template file: {}", err));
        }
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
    for route_name in route_names {
        let route_properties = routes.routes.get(route_name).unwrap();

        let mut page_entry_exists = false;
        let mut page_position = 0;
        for (index, page_entry) in mkdocs_routes.iter_mut().enumerate() {
            if page_entry.as_mapping().unwrap().contains_key(route_name) {
                page_entry_exists = true;
                page_position = index;
                break;
            }
        }

        let mut markdown_file =
            match File::create(docs_path.join("routes").join(format!("{}.md", route_name))) {
                Ok(file) => file,
                Err(e) => {
                    println!("Error creating file: {:?}", e);
                    continue;
                }
            };

        let route_page_markdown =
            generate_route_page_from_template(wiki_name, route_properties, template.clone());

        match markdown_file.write_all(route_page_markdown.as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                println!("Error writing to file: {:?}", e);
                continue;
            }
        }

        let mut route_page_entry = Mapping::new();
        route_page_entry.insert(
            Value::String(route_name.to_string()),
            Value::String(format!("routes/{}.md", route_name)),
        );

        if !page_entry_exists {
            mkdocs_routes.push(Value::Mapping(route_page_entry));
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

fn generate_route_page_from_template(
    wiki_name: &str,
    route_properties: &RouteProperties,
    template: String,
) -> String {
    let mut wild_encounter_tab = String::new();
    let mut wild_encounters = String::new();

    if !route_properties.wild_encounters.is_empty() {
        wild_encounter_tab.push_str("=== \"Wild Encounters\"");
        let wild_encounter_markdown = generate_wild_encounter_markdown(
            wiki_name,
            &route_properties.wild_encounters,
            &route_properties.wild_encounter_area_levels,
        );
        wild_encounters.push_str(&wild_encounter_markdown);
    }

    let mut trainer_encounter_tab = String::new();
    let mut trainer_encounters = String::new();

    if !route_properties.trainers.is_empty() {
        trainer_encounter_tab.push_str("=== \"Trainer Encounters\"");
        let trainer_table = create_trainer_table(wiki_name, &route_properties.trainers);
        trainer_encounters.push_str(&trainer_table);
    }

    let route_image = String::new();

    let result = template
        .replace("{{route_image}}", &route_image)
        .replace("{{wild_encounter_tab}}", &wild_encounter_tab)
        .replace("{{wild_encounters}}", &wild_encounters)
        .replace("{{trainer_encounter_tab}}", &trainer_encounter_tab)
        .replace("{{trainer_encounters}}", &trainer_encounters);

    return result;
}

fn generate_wild_encounter_markdown(
    wiki_name: &str,
    encounters: &IndexMap<String, Vec<WildEncounter>>,
    encounter_areas_levels: &IndexMap<String, String>,
) -> String {
    let mut markdown_encounters = String::new();
    for (encounter_area, pokemon_encounter_list) in encounters {
        let mut pokemon_entries = String::new();
        for pokemon in pokemon_encounter_list.iter() {
            let entry = format!(
                "<div style=\"display: grid; justify-items: center\">
                    {}
                </div>",
                get_markdown_entry_for_pokemon(wiki_name, &pokemon)
            );
            pokemon_entries.push_str(&entry);
        }

        let encounter_area_entry = match encounter_areas_levels.get(&encounter_area.clone()) {
            Some(area_level) => {
                if area_level.is_empty() {
                    encounter_area.to_string()
                } else {
                    format!(
                        "{} Lv. {}",
                        capitalize_and_remove_hyphens(&encounter_area),
                        area_level
                    )
                }
            }
            None => encounter_area.to_string(),
        };
        let encounter_area = capitalize_and_remove_hyphens(&encounter_area_entry);
        let encounters = format!(
            "<div style=\"display: grid; grid-template-columns: 1fr 1fr 1fr 1fr 1fr 1fr;\">{}</div>",
            pokemon_entries
        );
        let encounter_entry = format!("\n\n\t???+ note \"{}\"\n\t\t{}", encounter_area, encounters);
        markdown_encounters.push_str(&encounter_entry);
    }

    return markdown_encounters;
}

fn create_trainer_table(wiki_name: &str, trainers: &IndexMap<String, TrainerInfo>) -> String {
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
    return format!("{}", markdown_trainers);
}

fn get_markdown_entry_for_pokemon(wiki_name: &str, pokemon: &WildEncounter) -> String {
    let dex_number_file_name = get_pokemon_dex_formatted_name(pokemon.id.try_into().unwrap());
    let encounter_rate = match pokemon.encounter_rate {
        0 => "".to_string(),
        _ => format!("{}%", &pokemon.encounter_rate),
    };
    return format!(
        "![{}](../../img/pokemon/{}.png) [{}](/{}/pokemon/{}) {}",
        pokemon.name,
        pokemon.name,
        capitalize(&pokemon.name),
        wiki_name,
        format!("{}-{}", dex_number_file_name, pokemon.name),
        encounter_rate
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
