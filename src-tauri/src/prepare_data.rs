use indexmap::IndexMap;
use reqwest;
use serde_json::Value;
use std::fs;
use std::{collections::HashMap, fs::File};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Pokemon {
    pokemon: IndexMap<String, PokemonData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PokemonData {
    id: u32,
    name: String,
    types: Vec<String>,
    abilities: Vec<String>,
    stats: Stats,
    moves: HashMap<String, Move>,
    sprite: String,
    evolution: Option<Evolution>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Stats {
    hp: u32,
    attack: u32,
    defense: u32,
    sp_attack: u32,
    sp_defense: u32,
    speed: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Move {
    level_learned: u32,
    learn_method: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Evolution {
    level: Option<u32>,
    item: Option<String>,
    other: Option<String>,
    evolves_to: String,
}

#[tauri::command]
pub fn download_and_prep_pokemon_data(
    wiki_name: &str,
    range_start: u32,
    range_end: u32,
    dir: &str,
) {
    let base_path: String = format!("{}{}", dir, wiki_name);
    let pokemon_path = format!("{}/data/pokemon.json", base_path);

    let pokemon_file = File::open(pokemon_path.clone()).unwrap();
    let mut pokemon: Pokemon = serde_json::from_reader(pokemon_file).unwrap();

    for i in range_start..=range_end {
        let response = reqwest::blocking::get(format!("https://pokeapi.co/api/v2/pokemon/{}", i))
            .unwrap()
            .json::<Value>();
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
            let learn_method = m["version_group_details"][0]["move_learn_method"]["name"]
                .as_str()
                .unwrap()
                .to_string();

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
            evolution: None,
        };
        pokemon
            .pokemon
            .insert(pokemon_data.name.clone(), pokemon_data.clone());
    }
    let string_pokemon_data = serde_json::to_string(&pokemon).unwrap();
    fs::write(pokemon_path.clone(), string_pokemon_data).unwrap();
}
