use std::{
    fs::{self, read_to_string, File},
    io::Write,
    path::PathBuf,
};

use serde_yaml::{Mapping, Value};
use sqlx::Sqlite;
use tauri::AppHandle;

use crate::{
    database::{get_mkdocs_config, get_routes, get_sqlite_connection},
    helpers::{capitalize, get_pokemon_dex_formatted_name},
    logger,
    page_generators::pokemon_page_generator_functions::{
        create_evolution_table, create_learnable_moves_table, create_level_up_moves_table,
    },
    structs::pokemon_structs::{DBPokemon, PokemonMove},
};

use super::{game_routes::WildEncounter, pokemon_page_generator_functions::create_locations_table};

#[tauri::command]
pub async fn remove_pokemon_page_with_old_dex_number(
    wiki_name: &str,
    pokemon_name: &str,
    old_dex_number: u32,
    app_handle: AppHandle,
) -> Result<String, String> {
    let base_path = app_handle.path_resolver().app_data_dir().unwrap();

    let mkdocs_yaml_file_path = base_path.join(wiki_name).join("dist").join("mkdocs.yml");
    let mut mkdocs_config = match get_mkdocs_config(&mkdocs_yaml_file_path) {
        Ok(config) => config,
        Err(err) => {
            logger::write_log(&base_path.join(wiki_name), logger::LogLevel::Error, &err);
            return Err(err);
        }
    };

    let mut mkdocs_pokemon: &mut Vec<Value> = &mut Vec::new();

    let nav_entries = mkdocs_config.nav.as_sequence_mut().unwrap();
    for entry in nav_entries {
        let map_entries = entry.as_mapping_mut().unwrap();
        match map_entries.get_mut(Value::String("Pokemon".to_string())) {
            Some(map_entry) => {
                mkdocs_pokemon = map_entry.as_sequence_mut().unwrap();
            }
            None => {}
        }
    }

    let pokedex_markdown_file_name = get_pokemon_dex_formatted_name(old_dex_number);
    let entry_key = format!(
        "{} - {}",
        &pokedex_markdown_file_name,
        capitalize(pokemon_name)
    );

    let mut page_entry_exists = false;
    let mut page_position = 0;
    for (index, page_entry) in mkdocs_pokemon.iter_mut().enumerate() {
        if page_entry.as_mapping().unwrap().contains_key(&entry_key) {
            page_entry_exists = true;
            page_position = index;
            break;
        }
    }

    if !page_entry_exists {
        return Ok("Page with old dex number not present".to_string());
    }
    mkdocs_pokemon.remove(page_position);
    let pokemon_page_path = base_path
        .join(wiki_name)
        .join("dist")
        .join("docs")
        .join("pokemon")
        .join(format!(
            "{}-{}.md",
            &pokedex_markdown_file_name, pokemon_name
        ));
    if pokemon_page_path.try_exists().unwrap_or(false) {
        match fs::remove_file(pokemon_page_path) {
            Ok(_) => {}
            Err(err) => {
                let message = format!("Failed to remove pokemon page: {err}");
                logger::write_log(
                    &base_path.join(wiki_name),
                    logger::LogLevel::Error,
                    &message,
                );
                return Err(message);
            }
        }
    }
    match fs::write(
        &mkdocs_yaml_file_path,
        serde_yaml::to_string(&mut mkdocs_config).unwrap(),
    ) {
        Ok(_) => {}
        Err(err) => {
            let message = format!("Failed to update mkdocs yaml: {err}");
            logger::write_log(
                &base_path.join(wiki_name),
                logger::LogLevel::Error,
                &message,
            );
            return Err(message);
        }
    };
    Ok("".to_string())
}

// TODO: Remove this function
#[tauri::command]
pub async fn generate_pokemon_pages_from_list(
    wiki_name: &str,
    pokemon_ids: Vec<usize>,
    app_handle: AppHandle,
) -> Result<String, String> {
    let base_path = app_handle.path_resolver().app_data_dir().unwrap();
    let resources_path = app_handle.path_resolver().resource_dir().unwrap();

    let sqlite_file_path = base_path.join(wiki_name).join(format!("{}.db", wiki_name));
    let conn = match get_sqlite_connection(sqlite_file_path).await {
        Ok(conn) => conn,
        Err(err) => {
            logger::write_log(&base_path.join(wiki_name), logger::LogLevel::Error, &err);
            return Err(err);
        }
    };
    let (pokemon_list, movesets) = match get_pokemon_list_and_movesets(&conn, &pokemon_ids).await {
        Ok((pokemon_list, movesets)) => (pokemon_list, movesets),
        Err(err) => {
            logger::write_log(&base_path.join(wiki_name), logger::LogLevel::Error, &err);
            return Err(err);
        }
    };

    return generate_pokemon_pages(
        wiki_name,
        &pokemon_list,
        &movesets,
        &base_path,
        &resources_path,
    );
}

async fn get_pokemon_list_and_movesets(
    conn: &sqlx::Pool<Sqlite>,
    pokemon_ids: &[usize],
) -> Result<(Vec<DBPokemon>, Vec<PokemonMove>), String> {
    let id_list = &pokemon_ids
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",");

    let pokemon_query = format!(
        "SELECT
            pokemon.*,
            a1.effect as a1_effect,
            a2.effect as a2_effect,
            h3.effect as h3_effect
        FROM pokemon
        LEFT JOIN abilities a1 on a1.name = pokemon.ability_1
        LEFT JOIN abilities a2 on a2.name = pokemon.ability_2
        LEFT JOIN abilities h3 on h3.name = pokemon.hidden_ability
        WHERE pokemon.id IN ({}) ORDER BY dex_number ASC",
        id_list
    );
    let pokemon_list = match sqlx::query_as::<_, DBPokemon>(&pokemon_query)
        .fetch_all(conn)
        .await
    {
        Ok(pokemon_list) => pokemon_list,
        Err(err) => {
            return Err(format!("Failed to fetch pokemon from database: {}", err));
        }
    };

    let moveset_query = format!(
        "SELECT
            pokemon_movesets.*,
            pokemon_movesets.move as move_id,
            m.name as move_name, m.type as move_type,
            m.power as power, m.accuracy as accuracy, m.pp as pp,
            m.damage_class as damage_class, m.machine_name as machine_name
        FROM pokemon_movesets
        LEFT JOIN moves m on m.id = pokemon_movesets.move
        WHERE pokemon_movesets.pokemon IN ({})",
        id_list
    );

    let movesets = match sqlx::query_as::<_, PokemonMove>(&moveset_query)
        .fetch_all(conn)
        .await
    {
        Ok(pokemon_movesets) => pokemon_movesets,
        Err(err) => {
            return Err(format!(
                "Failed to fetch pokemon movesets from database: {}",
                err
            ));
        }
    };
    return Ok((pokemon_list, movesets));
}

pub fn generate_pokemon_pages(
    wiki_name: &str,
    pokemon_list: &[DBPokemon],
    movesets: &[PokemonMove],
    base_path: &PathBuf,
    resources_path: &PathBuf,
) -> Result<String, String> {
    let docs_path = base_path.join(wiki_name).join("dist").join("docs");

    let routes_json_file_path = base_path.join(wiki_name).join("data").join("routes.json");
    let routes = get_routes(&routes_json_file_path)?;

    let dex_numbers = pokemon_list
        .iter()
        .map(|p| usize::try_from(p.dex_number).unwrap())
        .collect::<Vec<_>>();

    // Gather all wild encounters for the selected pokemon
    let mut wild_encounters: Vec<WildEncounter> = Vec::new();
    for (_, properties) in &routes.routes {
        for (_, _wild_encounters) in &properties.wild_encounters {
            for wild_encounter in _wild_encounters {
                if dex_numbers.contains(&wild_encounter.id) {
                    wild_encounters.push(wild_encounter.clone());
                }
            }
        }
    }

    let mkdocs_yaml_file_path = base_path.join(wiki_name).join("dist").join("mkdocs.yml");
    let mut mkdocs_config = match get_mkdocs_config(&mkdocs_yaml_file_path) {
        Ok(config) => config,
        Err(err) => {
            logger::write_log(&base_path.join(wiki_name), logger::LogLevel::Error, &err);
            return Err(err);
        }
    };

    let template = match read_to_string(
        resources_path
            .join("resources")
            .join("generator_assets")
            .join("templates")
            .join("pokemon_page_template.md"),
    ) {
        Ok(template) => template,
        Err(err) => {
            let message = format!("Failed to read template file: {err}");
            logger::write_log(
                &base_path.join(wiki_name),
                logger::LogLevel::Error,
                &message,
            );
            return Err(message);
        }
    };

    let mut mkdocs_pokemon: &mut Vec<Value> = &mut Vec::new();

    let nav_entries = mkdocs_config.nav.as_sequence_mut().unwrap();
    for entry in nav_entries {
        let map_entries = entry.as_mapping_mut().unwrap();
        match map_entries.get_mut(Value::String("Pokemon".to_string())) {
            Some(map_entry) => {
                mkdocs_pokemon = map_entry.as_sequence_mut().unwrap();
            }
            None => {}
        }
    }

    for pokemon in pokemon_list {
        let pokedex_markdown_file_name = get_pokemon_dex_formatted_name(pokemon.dex_number);
        let entry_key = format!(
            "{} - {}",
            pokedex_markdown_file_name,
            capitalize(&pokemon.name)
        );
        let mut page_entry_exists = false;
        let mut page_position = 0;
        for (index, page_entry) in mkdocs_pokemon.iter_mut().enumerate() {
            if page_entry.as_mapping().unwrap().contains_key(&entry_key) {
                page_entry_exists = true;
                page_position = index;
                break;
            }
        }
        if pokemon.render == "false" && page_entry_exists {
            mkdocs_pokemon.remove(page_position);
            continue;
        }

        if pokemon.render == "false" {
            continue;
        }

        let mut markdown_file = match File::create(docs_path.join("pokemon").join(format!(
            "{}-{}.md",
            pokedex_markdown_file_name, &pokemon.name
        ))) {
            Ok(file) => file,
            Err(e) => {
                logger::write_log(
                    &base_path.join(wiki_name),
                    logger::LogLevel::Error,
                    &format!("Error creating markdown file for {}: {:?}", pokemon.name, e),
                );
                continue;
            }
        };

        let current_pokemon_movset = movesets
            .iter()
            .cloned()
            .filter(|m| m.pokemon == pokemon.id)
            .collect::<Vec<_>>();

        let current_pokemon_locations = wild_encounters
            .iter()
            .cloned()
            .filter(|w| {
                w.id == usize::try_from(pokemon.dex_number).unwrap() && w.name == pokemon.name
            })
            .collect::<Vec<_>>();

        let pokemon_markdown_string = generate_page_from_template(
            &template,
            &pokemon,
            &current_pokemon_movset,
            &current_pokemon_locations,
        );

        match markdown_file.write_all(format!("{pokemon_markdown_string}").as_bytes()) {
            Err(err) => {
                let message = format!(
                    "Error writing to markdown file for {}: {}",
                    pokemon.name, err
                );
                logger::write_log(
                    &base_path.join(wiki_name),
                    logger::LogLevel::Error,
                    &message,
                );
                return Err(message);
            }
            _ => {}
        };

        let mut pokemon_page_entry = Mapping::new();
        pokemon_page_entry.insert(
            Value::String(entry_key.clone()),
            Value::String(format!(
                "pokemon/{}-{}.md",
                pokedex_markdown_file_name, &pokemon.name
            )),
        );

        if !page_entry_exists {
            mkdocs_pokemon.push(Value::Mapping(pokemon_page_entry));

            // Sort pokemon entries so new ones don't appear out of order
            // in the navigation
            mkdocs_pokemon.sort_by(|a, b| {
                let first = a.as_mapping().unwrap().keys().next().unwrap();
                let second = b.as_mapping().unwrap().keys().next().unwrap();

                extract_pokemon_id(first.as_str()).cmp(&extract_pokemon_id(second.as_str()))
            })
        }
    }

    match fs::write(
        &mkdocs_yaml_file_path,
        serde_yaml::to_string(&mut mkdocs_config).unwrap(),
    ) {
        Ok(_) => {}
        Err(err) => {
            let message = format!("Failed to update mkdocs yaml: {err}");
            logger::write_log(
                &base_path.join(wiki_name),
                logger::LogLevel::Error,
                &message,
            );
            return Err(message);
        }
    };

    return Ok("Pokemon Pages Generated".to_string());
}

fn extract_pokemon_id(key: Option<&str>) -> i32 {
    // This long chain is just meant to get, format and trim dex number
    return key
        .unwrap()
        .split_once("-")
        .unwrap()
        .0
        .trim()
        .parse::<i32>()
        .unwrap();
}

pub fn generate_page_from_template(
    template: &str,
    pokemon: &DBPokemon,
    movesets: &[PokemonMove],
    locations: &[WildEncounter],
) -> String {
    let type_images: Vec<String> = pokemon
        .types
        .clone()
        .split(",")
        .map(|_type| {
            format!(
                "<img src='../../img/types/{}.png' style='width: 77px; height: 26px;'/>",
                _type
            )
        })
        .filter(|_type| !_type.contains("none"))
        .collect();

    let type_1_image = type_images.get(0).unwrap();
    let mut type_2_image = String::new();
    if type_images.len() > 1 {
        type_2_image.push_str(type_images.get(1).unwrap());
    }

    let mut ability_1 = String::new();
    let mut ability_2 = String::new();
    let mut hidden_ability = String::new();

    if pokemon.ability_1.is_some() {
        ability_1.push_str(
            format!(
                "<a href='' title=\"{}\">{}</a>",
                &pokemon.a1_effect.as_ref().unwrap(),
                capitalize(&pokemon.ability_1.as_ref().unwrap())
            )
            .as_str(),
        );
    }

    if pokemon.ability_2.is_some() {
        ability_2.push_str(
            format!(
                "/<a href='' title=\"{}\">{}</a>",
                &pokemon.a2_effect.as_ref().unwrap(),
                capitalize(&pokemon.ability_2.as_ref().unwrap())
            )
            .as_str(),
        );
    }

    let mut display_hidden_ability_section = "none";
    if pokemon.hidden_ability.is_some() {
        display_hidden_ability_section = "grid";
        hidden_ability.push_str(
            format!(
                "<a href='' title=\"{}\">{}</a>",
                &pokemon.h3_effect.as_ref().unwrap(),
                capitalize(&pokemon.hidden_ability.as_ref().unwrap())
            )
            .as_str(),
        );
    }

    let level_up_moveset = movesets
        .iter()
        .cloned()
        .filter(|m| m.learn_method.contains("level-up"))
        .collect::<Vec<_>>();

    let level_up_moves = create_level_up_moves_table(level_up_moveset);

    let learnable_moveset = movesets
        .iter()
        .cloned()
        .filter(|m| m.learn_method.contains("machine"))
        .collect::<Vec<_>>();

    let learnable_moves = create_learnable_moves_table(learnable_moveset);

    let location_table = create_locations_table(locations);

    let evolution_change = create_evolution_table(&pokemon);

    let result = template
        .replace("{{pokemon_name}}", &pokemon.name)
        .replace("{{type_1_image}}", &type_1_image)
        .replace("{{type_2_image}}", &type_2_image)
        .replace("{{ability_1}}", &ability_1)
        .replace("{{ability_2}}", &ability_2)
        .replace(
            "{{display_hidden_ability}}",
            &display_hidden_ability_section,
        )
        .replace("{{hidden_ability}}", &hidden_ability)
        .replace("{{hp}}", pokemon.hp.to_string().as_str())
        .replace(
            "{{hp_width}}",
            calculate_bar_width(pokemon.hp).to_string().as_str(),
        )
        .replace(
            "{{hp_rank}}",
            calculate_bar_rank(pokemon.hp).to_string().as_str(),
        )
        .replace("{{attack}}", pokemon.attack.to_string().as_str())
        .replace(
            "{{atk_width}}",
            calculate_bar_width(pokemon.attack).to_string().as_str(),
        )
        .replace(
            "{{atk_rank}}",
            calculate_bar_rank(pokemon.attack).to_string().as_str(),
        )
        .replace("{{defense}}", pokemon.defense.to_string().as_str())
        .replace(
            "{{def_width}}",
            calculate_bar_width(pokemon.defense).to_string().as_str(),
        )
        .replace(
            "{{def_rank}}",
            calculate_bar_rank(pokemon.defense).to_string().as_str(),
        )
        .replace("{{special_attack}}", pokemon.sp_attack.to_string().as_str())
        .replace(
            "{{sp_atk_width}}",
            calculate_bar_width(pokemon.sp_attack).to_string().as_str(),
        )
        .replace(
            "{{sp_atk_rank}}",
            calculate_bar_rank(pokemon.sp_attack).to_string().as_str(),
        )
        .replace(
            "{{special_defense}}",
            pokemon.sp_defense.to_string().as_str(),
        )
        .replace(
            "{{sp_def_width}}",
            calculate_bar_width(pokemon.sp_defense).to_string().as_str(),
        )
        .replace(
            "{{sp_def_rank}}",
            calculate_bar_rank(pokemon.sp_defense).to_string().as_str(),
        )
        .replace("{{speed}}", pokemon.speed.to_string().as_str())
        .replace(
            "{{speed_width}}",
            calculate_bar_width(pokemon.speed).to_string().as_str(),
        )
        .replace(
            "{{speed_rank}}",
            calculate_bar_rank(pokemon.speed).to_string().as_str(),
        )
        .replace("{{evolution_change}}", &evolution_change)
        .replace("{{locations}}", &location_table)
        .replace("{{level_up_moves}}", &level_up_moves)
        .replace("{{machine_moves}}", &learnable_moves);

    let mut evolution_change_string = String::new();
    if pokemon.evolution_method != "no_change" {
        evolution_change_string.push_str(&format!("## Evolution\n\n"));
        evolution_change_string.push_str(&format!("{}", create_evolution_table(pokemon)));
    }
    let result = result.replace("{{evolution_change}}", &evolution_change_string);
    return result;
}

fn calculate_bar_width(stat: u32) -> u32 {
    return ((stat as f64 / 255f64) * 100f64) as u32;
}
fn calculate_bar_rank(stat: u32) -> u32 {
    let rank = ((stat as f64 / 255f64) * 10f64).ceil() as u32;
    if rank > 6 {
        return 6;
    }
    return rank;
}
