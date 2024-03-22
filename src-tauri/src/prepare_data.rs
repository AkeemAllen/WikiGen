use std::{collections::HashMap, fs::File};

struct Pokemon {
    pokemon: HashMap<String, PokemonData>,
}

struct PokemonData {
    id: u32,
    name: String,
    types: Vec<String>,
    abilities: Vec<String>,
    stats: Stats,
    moves: Moves,
    sprite: String,
    evolution: Option<Evolution>,
}

struct Stats {
    hp: u32,
    attack: u32,
    defense: u32,
    sp_attack: u32,
    sp_defense: u32,
    speed: u32,
}

struct Moves {
    moves: HashMap<String, Move>,
}

struct Move {
    level_learned: u32,
    learn_method: String,
}

struct Evolution {
    level: Option<u32>,
    item: Option<String>,
    other: Option<String>,
    evolves_to: String,
}

#[tauri::command]
pub fn download_pokemon_data(wiki_name: &str, range_start: u32, range_end: u32, dir: &str) {
    let base_path: String = format!("{}{}", dir, wiki_name);
    let pokemon_path = format!("{}/data/pokemon.json", base_path);

    println!("{}/data", base_path);
    // let pokemon_file = File::open(pokemon_path).unwrap();
    // if (pokemon_file.) {
    //     return "File exists";
    // }
    // return;
}
