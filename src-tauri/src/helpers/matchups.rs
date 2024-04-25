use std::{collections::HashMap, fs::File};

use tauri::AppHandle;

use crate::structs::matchup_models::{
    NumberTypes, GENERATION_ONE, GENERATION_TWO, GEN_DEFAULT, POKEMON_TYPES_ARRAY,
};

pub fn generate_matchup_map(
    wiki_name: &str,
    generation: &str,
    app_handle: AppHandle,
) -> Result<String, String> {
    let mut data = GEN_DEFAULT;

    if generation == "gen1" {
        data = GENERATION_ONE;
    } else if generation == "gen2" {
        data = GENERATION_TWO
    }

    let mut matchup_map: HashMap<String, f32> = HashMap::new();

    for (row_index, row_value) in data.iter().enumerate() {
        for (col_index, col_value) in row_value.iter().enumerate() {
            let type_one = POKEMON_TYPES_ARRAY[row_index];
            let type_two = POKEMON_TYPES_ARRAY[col_index];
            let key = format!("{} > {}", type_one, type_two);
            let value = match col_value {
                NumberTypes::I32(int_val) => *int_val as f32,
                NumberTypes::F32(float_val) => *float_val,
            };

            matchup_map.insert(key, value);
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
