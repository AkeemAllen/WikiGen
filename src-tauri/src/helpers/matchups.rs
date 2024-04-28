use std::{collections::HashMap, fs::File};

use tauri::AppHandle;

use crate::structs::{
    matchup_models::GEN_DEFAULT,
    pokemon_structs::{PokemonTypesEnum, POKEMON_TYPES_ARRAY},
};

// May need this code in the function in the future to generate other matchup maps
// For now it is unused because what it generates is already stored in the
// matchup_map.json file in the wiki's data folder
pub fn generate_matchup_map(wiki_name: &str, app_handle: AppHandle) -> Result<String, String> {
    let data = GEN_DEFAULT;

    let mut matchup_map: HashMap<String, f32> = HashMap::new();

    for (row_index, row_value) in data.iter().enumerate() {
        for (col_index, col_value) in row_value.iter().enumerate() {
            let type_one = POKEMON_TYPES_ARRAY[row_index];
            let type_two = POKEMON_TYPES_ARRAY[col_index];
            let key = format!("{} > {}", type_one, type_two);
            let value = col_value;

            matchup_map.insert(key, *value);
        }
    }

    let base_path = app_handle.path_resolver().app_data_dir().unwrap();
    let matchup_map_path = base_path
        .join(wiki_name)
        .join("data")
        .join("matchup_map.json");

    let matchup_map_json_file = File::create(matchup_map_path).unwrap();
    serde_json::to_writer(matchup_map_json_file, &matchup_map).unwrap();

    Ok("Matchup Map Generated".to_string())
}

fn get_matchup_map(wiki_name: &str, app_handle: AppHandle) -> HashMap<String, f32> {
    let base_path = app_handle.path_resolver().app_data_dir().unwrap();
    let matchup_map_path = base_path
        .join(wiki_name)
        .join("data")
        .join("matchup_map.json");
    let matchup_file = File::open(matchup_map_path).unwrap();

    let matchup_map: HashMap<String, f32> = match serde_json::from_reader(matchup_file) {
        Ok(map_file) => map_file,
        Err(error) => {
            println!("{}", error);
            return HashMap::new();
        }
    };

    return matchup_map;
}

fn matchup_for_pair(
    wiki_name: &str,
    defense_type: &str,
    offense_type: &str,
    app_handle: AppHandle,
) -> f32 {
    let matchup_map = get_matchup_map(wiki_name, app_handle);
    let key = format!("{} > {}", offense_type, defense_type);
    let value = matchup_map.get(&key).unwrap();
    return *value;
}

fn matchup_for(
    wiki_name: &str,
    defense_types: Vec<PokemonTypesEnum>,
    offense_type: &PokemonTypesEnum,
    app_handle: AppHandle,
) -> f32 {
    let filtered_defense_types = defense_types
        .iter()
        .filter(|defense_type| !matches!(defense_type, PokemonTypesEnum::None));

    let mapped_defense_types = filtered_defense_types.map(|defense_type| {
        matchup_for_pair(
            wiki_name,
            &defense_type.to_string(),
            &offense_type.to_string(),
            app_handle.clone(),
        )
    });

    return mapped_defense_types.fold(1.0, |x, y| x * y);
}

#[derive(Debug)]
struct DefensiveMatchups {
    pokemon_type: String,
    effectiveness: f32,
}

fn generate_defensive_matchups(
    wiki_name: &str,
    defense_types: Vec<PokemonTypesEnum>,
    pokemon_type: &PokemonTypesEnum,
    app_handle: AppHandle,
) -> DefensiveMatchups {
    let effectiveness = matchup_for(wiki_name, defense_types, pokemon_type, app_handle);

    return DefensiveMatchups {
        pokemon_type: pokemon_type.to_string(),
        effectiveness: effectiveness,
    };
}

fn defensive_matchups(
    wiki_name: &str,
    defense_types: Vec<PokemonTypesEnum>,
    app_handle: AppHandle,
) -> Vec<DefensiveMatchups> {
    let matchups: Vec<DefensiveMatchups> = POKEMON_TYPES_ARRAY
        .iter()
        .map(|pokemon_type| {
            generate_defensive_matchups(
                wiki_name,
                defense_types.clone(),
                pokemon_type,
                app_handle.clone(),
            )
        })
        .collect();

    return matchups;
}

fn group_matchups_by_effectiveness(
    matchups: &Vec<DefensiveMatchups>,
    effectiveness: f32,
) -> Vec<String> {
    let filtered_matchups: Vec<&DefensiveMatchups> = matchups
        .iter()
        .filter(|matchup| matchup.effectiveness == effectiveness)
        .collect();

    let mapped_matchups = filtered_matchups
        .iter()
        .map(|&matchup| matchup.pokemon_type.to_string())
        .collect();

    return mapped_matchups;
}

pub fn get_defensive_matchups(
    types: Vec<PokemonTypesEnum>,
    wiki_name: &str,
    app_handle: AppHandle,
) -> HashMap<String, Vec<String>> {
    // Consdier removing some of the effectiveness levels since they may never be used
    let effectiveness_levels = [8.0, 4.0, 2.0, 1.0, 0.5, 0.25, 0.125, 0.0];

    let matchups = defensive_matchups(wiki_name, types, app_handle);

    let mut matchups_by_effectiveness = HashMap::new();

    for effectiveness in effectiveness_levels {
        let grouped_matchups = group_matchups_by_effectiveness(&matchups, effectiveness);
        matchups_by_effectiveness.insert(effectiveness.to_string(), grouped_matchups);
    }

    let mut filtered_matchups_by_effectiveness = HashMap::new();
    for (key, value) in matchups_by_effectiveness {
        if value.len() != 0 {
            filtered_matchups_by_effectiveness.insert(key, value);
        }
    }

    return filtered_matchups_by_effectiveness;
}
