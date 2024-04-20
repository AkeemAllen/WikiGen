use reqwest;
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::{collections::HashMap, fs::File};

use crate::structs::pokemon_structs::{
    Evolution, EvolutionMethod, Move, Pokemon, PokemonData, Stats,
};

#[tauri::command]
pub async fn download_and_prep_pokemon_data(
    wiki_name: &str,
    range_start: u32,
    range_end: u32,
    dir: &str,
) -> Result<String, String> {
    let base_path = Path::new(dir).join(wiki_name);
    let pokemon_path = base_path.join("data").join("pokemon.json");

    let pokemon_file = File::open(&pokemon_path).unwrap();

    let mut pokemon: Pokemon = serde_json::from_reader(pokemon_file).unwrap();

    for i in range_start..=range_end {
        let response = reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/{}", i))
            .await
            .unwrap()
            .json::<Value>()
            .await;
        let mut pokemon_response_body = response.ok().unwrap();

        let mut types = Vec::new();
        for t in pokemon_response_body["types"].as_array().unwrap() {
            types.push(t["type"]["name"].as_str().unwrap().to_string());
        }

        let mut abilities = Vec::new();
        for a in pokemon_response_body["abilities"].as_array().unwrap() {
            abilities.push(a["ability"]["name"].as_str().unwrap().to_string());
        }

        // Not sure if it's guaranteed that the stats from the
        // will match the order of the stats from the API
        let stats = Stats {
            hp: pokemon_response_body["stats"][0]["base_stat"]
                .as_u64()
                .unwrap() as u32,
            attack: pokemon_response_body["stats"][1]["base_stat"]
                .as_u64()
                .unwrap() as u32,
            defense: pokemon_response_body["stats"][2]["base_stat"]
                .as_u64()
                .unwrap() as u32,
            sp_attack: pokemon_response_body["stats"][3]["base_stat"]
                .as_u64()
                .unwrap() as u32,
            sp_defense: pokemon_response_body["stats"][4]["base_stat"]
                .as_u64()
                .unwrap() as u32,
            speed: pokemon_response_body["stats"][5]["base_stat"]
                .as_u64()
                .unwrap() as u32,
        };

        // TODO: Figure out a way to set moves based on the version group of the hack
        // May need to create my own repository of pokemon move data to accomplish this
        // It's too difficult to parse with pokeapis current API
        let mut moves = HashMap::new();
        for m in pokemon_response_body["moves"].as_array().unwrap() {
            let move_name = m["move"]["name"].as_str().unwrap().to_string();
            let level_learned = m["version_group_details"][0]["level_learned_at"]
                .as_u64()
                .unwrap() as u32;
            let learn_method = vec![m["version_group_details"][0]["move_learn_method"]["name"]
                .as_str()
                .unwrap()
                .to_string()];

            moves.insert(
                move_name,
                Move {
                    level_learned,
                    learn_method,
                },
            );
        }

        pokemon_response_body["sprites"] =
            pokemon_response_body["sprites"]["front_default"].clone();

        let pokemon_data = PokemonData {
            id: i,
            name: pokemon_response_body["name"].as_str().unwrap().to_string(),
            types,
            abilities,
            stats,
            moves,
            sprite: pokemon_response_body["sprites"]
                .as_str()
                .unwrap()
                .to_string(),
            evolution: Evolution {
                level: None,
                item: None,
                other: None,
                evolves_to: None,
                method: EvolutionMethod::NoChange,
            },
        };
        pokemon
            .pokemon
            .insert(pokemon_data.id.clone(), pokemon_data.clone());
    }
    let string_pokemon_data = serde_json::to_string(&pokemon).unwrap();
    fs::write(pokemon_path.clone(), string_pokemon_data).unwrap();

    return Ok("Pokemon Saved".to_string());
}
