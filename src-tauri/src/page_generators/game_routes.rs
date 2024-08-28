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
    pub types: Vec<String>,
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

static mut TRAINER_POKEMON_TEMPLATE: String = String::new();

#[tauri::command]
pub async fn generate_route_pages_with_handle(
    wiki_name: &str,
    route_names: Vec<&str>,
    app_handle: AppHandle,
) -> Result<String, String> {
    let base_path = app_handle.path_resolver().app_data_dir().unwrap();
    let resources_path = app_handle.path_resolver().resource_dir().unwrap();

    unsafe {
        TRAINER_POKEMON_TEMPLATE = match read_to_string(
            resources_path
                .join("resources")
                .join("generator_assets")
                .join("templates")
                .join("trainer_pokemon_template.md"),
        ) {
            Ok(template) => template,
            Err(err) => {
                return Err(format!("Failed to read template file: {}", err));
            }
        };
    }

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
        let trainer_sprite = get_trainer_sprite(name, &trainer_info.sprite);
        let mut trainer_entry: String = String::new();

        if trainer_info.versions.is_empty() {
            trainer_entry = format!(
                "<div class=\"trainer-pokemon-container\">\n{}</div>",
                generate_trainer_entry(wiki_name, name, trainer_info, "")
            );
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

                let version_title = format!("\n\n\t\t=== \"{}\"", version);
                let entry = format!(
                    "\t<div class=\"trainer-pokemon-container\">\n{}</div>",
                    generate_trainer_entry(wiki_name, name, trainer_info, version)
                );
                trainer_entry.push_str(&format!("{}\n\t\t{}", version_title, entry));
            }
        }
        markdown_trainers.push_str(&format!(
            "\n\t{}\n\t???+ note \"{}\"\n\t\t{}",
            trainer_sprite, name, trainer_entry
        ));
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
        return "".to_string();
    }
    return format!(
        "![{}](https://play.pokemonshowdown.com/sprites/trainers/{}.png)",
        name, sprite
    );
}

fn extract_move(_move: Option<&String>) -> String {
    match _move.clone() {
        Some(_move) => format!(
            "<div class=\"trainer-pokemon-move\">{}</div>",
            capitalize_and_remove_hyphens(_move)
        ),
        None => return "<div class=\"trainer-pokemon-move\">-</div>".to_string(),
    }
}

fn evaluate_attribute(attribute: &str) -> String {
    match attribute {
        "" => return "-".to_string(),
        _ => return capitalize_and_remove_hyphens(attribute),
    }
}

fn generate_trainer_entry(
    wiki_name: &str,
    name: &str,
    trainer_info: &TrainerInfo,
    version: &str,
) -> String {
    let mut pokemon_team = String::new();
    // if version == "" {
    //     pokemon_team = format!("| {}", get_trainer_sprite(name, &trainer_info.sprite))
    // }
    for pokemon in &trainer_info.pokemon_team {
        if !pokemon.trainer_versions.contains(&version.to_string()) && version != "" {
            continue;
        }
        let mut pokemon_entry = String::new();
        let mut item_image = String::new();
        if !pokemon.item.is_empty() {
            item_image.push_str(&format!(
                "<img src=\"../../img/items/{}.png\" alt={} style=\"width: 25px;\"/>",
                pokemon.item, pokemon.item
            ))
        } else {
            item_image.push_str("<div></div>");
        }
        let mut ability = String::new();
        if pokemon.ability != "" {
            ability = capitalize_and_remove_hyphens(&pokemon.ability);
        } else {
            ability = "-".to_string();
        }
        let mut nature = String::new();
        if pokemon.nature != "" {
            nature = capitalize_and_remove_hyphens(&pokemon.nature);
        } else {
            nature = "-".to_string();
        }

        let type_one = format!(
            "<img src=\"../../img/types/{}.png\" alt={} style=\"width: 50px;\"/>",
            pokemon.types.get(0).unwrap(),
            pokemon.types.get(0).unwrap()
        );

        let type_two: String;

        if pokemon.types.len() > 1 {
            type_two = format!(
                "<img src=\"../../img/types/{}.png\" alt={} style=\"width: 50px;\"/>",
                pokemon.types.get(1).unwrap(),
                pokemon.types.get(1).unwrap()
            );
        } else {
            type_two = "<div></div>".to_string();
        }

        unsafe {
            pokemon_entry = TRAINER_POKEMON_TEMPLATE
                .clone()
                .replace("{{pokemon_name}}", &pokemon.name)
                .replace("{{cap_pokemon_name}}", &capitalize(&pokemon.name))
                .replace(
                    "{{page_title}}",
                    &format!(
                        "{}-{}",
                        get_pokemon_dex_formatted_name(u32::try_from(pokemon.id).unwrap()),
                        pokemon.name
                    ),
                )
                .replace("{{level}}", pokemon.level.to_string().as_str())
                .replace("{{ability}}", &ability)
                .replace("{{nature}}", &nature)
                .replace("{{item_image}}", &item_image)
                .replace("{{type_one}}", &type_one)
                .replace("{{type_two}}", &type_two)
                .replace("{{item_name}}", &evaluate_attribute(&pokemon.item))
                .replace("{{move_1}}", &extract_move(pokemon.moves.get(0)))
                .replace("{{move_2}}", &extract_move(pokemon.moves.get(1)))
                .replace("{{move_3}}", &extract_move(pokemon.moves.get(2)))
                .replace("{{move_4}}", &extract_move(pokemon.moves.get(3)));
        };
        let mut tabs = "\t\t";
        if !version.is_empty() {
            tabs = "\t\t\t\t";
        }
        let indented_lines: Vec<String> = pokemon_entry
            .lines()
            .map(|line| format!("{}{}", tabs, line))
            .collect();
        let indented_pokemon_entry = indented_lines.join("\n");

        pokemon_team.push_str(&format!("{}", &indented_pokemon_entry))
    }

    return pokemon_team;
}
