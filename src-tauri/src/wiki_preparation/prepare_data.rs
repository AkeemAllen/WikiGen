use reqwest;
use serde_json::Value;
use std::fs;
use std::io::Cursor;
use std::path::PathBuf;
use std::{collections::HashMap, fs::File};
use tauri::AppHandle;

use crate::helpers::round_up_to_nearest_100;
use crate::structs::pokemon_structs::{
    Evolution, EvolutionMethod, EvolvedPokemon, Move, Pokemon, PokemonData, Stats,
};

#[derive(Debug)]
struct Shard {
    data: Pokemon,
    shard_index: usize,
}

#[tauri::command]
pub async fn download_and_prep_pokemon_data(
    wiki_name: &str,
    range_start: u32,
    range_end: u32,
    app_handle: AppHandle,
) -> Result<String, String> {
    let base_path = app_handle.path_resolver().app_data_dir().unwrap();

    let mut shards: Vec<Shard> = Vec::new();

    let rounded_range_start = round_up_to_nearest_100(range_start);
    let rounded_range_end = round_up_to_nearest_100(range_end);

    // I'm using math here to selectively load in data from separate pokemon files.
    // This way I don'e have to read in all of them.
    for i in (rounded_range_start..=rounded_range_end).step_by(100) {
        if i == 100 {
            shards.push(get_pokemon_data_from_file(wiki_name, &base_path, 1));
            continue;
        }
        if i == 200 {
            shards.push(get_pokemon_data_from_file(wiki_name, &base_path, 2));
            continue;
        }
        if i == 300 {
            shards.push(get_pokemon_data_from_file(wiki_name, &base_path, 3));
            continue;
        }
        if i == 400 {
            shards.push(get_pokemon_data_from_file(wiki_name, &base_path, 4));
            continue;
        }
        if i == 500 {
            shards.push(get_pokemon_data_from_file(wiki_name, &base_path, 5));
            continue;
        }
        if i == 600 {
            shards.push(get_pokemon_data_from_file(wiki_name, &base_path, 6));
            continue;
        }
        if i == 700 {
            shards.push(get_pokemon_data_from_file(wiki_name, &base_path, 7));
            continue;
        }
        if i == 800 {
            shards.push(get_pokemon_data_from_file(wiki_name, &base_path, 8));
            continue;
        }
        if i == 900 {
            shards.push(get_pokemon_data_from_file(wiki_name, &base_path, 9));
            continue;
        }
        if i == 1000 {
            shards.push(get_pokemon_data_from_file(wiki_name, &base_path, 10));
            continue;
        }
    }

    for i in range_start..=range_end {
        let response = match reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/{}", i)).await
        {
            Ok(res) => res.json::<Value>().await,
            Err(_) => {
                println!("Trouble processing pokemon with dex_number {}", i);
                continue;
            }
        };
        let mut pokemon_response_body = response.ok().unwrap();

        let mut types = Vec::new();
        for t in pokemon_response_body["types"].as_array().unwrap() {
            types.push(t["type"]["name"].as_str().unwrap().to_string());
        }

        if types.len() == 1 {
            types.push("none".to_string());
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
                level: 0,
                item: "".to_string(),
                other: "".to_string(),
                evolves_to: EvolvedPokemon {
                    id: 0,
                    pokemon_name: "".to_string(),
                },
                method: EvolutionMethod::NoChange,
            },
        };
        // Each shard (or files) contains 100 pokemon entries.
        // I have to use this to select and modify the correct file's data.
        if i <= 100 {
            update_shard(&mut shards, 1, pokemon_data);
            continue;
        }
        if i <= 200 {
            update_shard(&mut shards, 2, pokemon_data);
            continue;
        }
        if i <= 300 {
            update_shard(&mut shards, 3, pokemon_data);
            continue;
        }
        if i <= 400 {
            update_shard(&mut shards, 4, pokemon_data);
            continue;
        }
        if i <= 500 {
            update_shard(&mut shards, 5, pokemon_data);
            continue;
        }
        if i <= 600 {
            update_shard(&mut shards, 6, pokemon_data);
            continue;
        }
        if i <= 700 {
            update_shard(&mut shards, 7, pokemon_data);
            continue;
        }
        if i <= 800 {
            update_shard(&mut shards, 8, pokemon_data);
            continue;
        }
        if i <= 900 {
            update_shard(&mut shards, 9, pokemon_data);
            continue;
        }
        if i <= 1025 {
            update_shard(&mut shards, 10, pokemon_data);
            continue;
        }
    }

    write_pokemon_data_to_files(wiki_name, base_path, shards);

    return Ok("Pokemon Saved".to_string());
}

fn update_shard(shards: &mut Vec<Shard>, index: usize, pokemon_data: PokemonData) {
    for shard in shards {
        if shard.shard_index == index {
            shard
                .data
                .pokemon
                .insert(pokemon_data.id.clone(), pokemon_data.clone());
            break;
        }
    }
}

fn write_pokemon_data_to_files(wiki_name: &str, base_path: PathBuf, shards: Vec<Shard>) {
    for shard in shards {
        let pokemon_path = base_path
            .join(wiki_name)
            .join("data")
            .join("pokemon_data")
            .join(format!("shard_{}.json", shard.shard_index));

        let string_pokemon_data = serde_json::to_string(&shard.data).unwrap();
        fs::write(pokemon_path.clone(), string_pokemon_data).unwrap();
    }
}

fn get_pokemon_data_from_file(wiki_name: &str, base_path: &PathBuf, shard_index: usize) -> Shard {
    let pokemon_path = base_path
        .join(wiki_name)
        .join("data")
        .join("pokemon_data")
        .join(format!("shard_{}.json", shard_index));
    let pokemon_file = File::open(&pokemon_path).unwrap();
    let pokemon: Pokemon = serde_json::from_reader(pokemon_file).unwrap();

    return Shard {
        data: pokemon,
        shard_index,
    };
}

#[tauri::command]
pub async fn download_pokemon_sprites(
    wiki_name: &str,
    range_start: u32,
    range_end: u32,
    app_handle: AppHandle,
) -> Result<String, String> {
    let base_path = app_handle.path_resolver().app_data_dir().unwrap();
    let docs_path = base_path.join(wiki_name).join("dist").join("docs");

    let pokemon_path = base_path.join(wiki_name).join("data").join("pokemon.json");

    let pokemon_file = File::open(&pokemon_path).unwrap();

    let pokemon: Pokemon = serde_json::from_reader(pokemon_file).unwrap();

    for dex_number in range_start..=range_end {
        let mut pokedex_img_file_name = format!("00{}", dex_number);
        if dex_number >= 10 {
            pokedex_img_file_name = format!("0{}", dex_number);
        }
        if dex_number >= 100 {
            pokedex_img_file_name = format!("{}", dex_number);
        }

        let file_path = docs_path
            .join("img")
            .join("pokemon")
            .join(format!("{}.png", pokedex_img_file_name));

        if file_path.try_exists().unwrap() {
            continue;
        }

        let pokemon_data = match pokemon.pokemon.get(&(dex_number as u32)) {
            Some(pokemon_data) => pokemon_data,
            None => {
                println!(
                    "Error finding pokemon data for pokemon with dex number {}",
                    dex_number
                );
                continue;
            }
        };

        let response = reqwest::get(&pokemon_data.sprite)
            .await
            .unwrap()
            .bytes()
            .await;

        let response_body = response.ok();

        let sprite_data = match response_body {
            Some(response_body) => response_body,
            None => {
                println!("Error retrieving sprite for {}", pokemon_data.name);
                continue;
            }
        };

        let sprite_image = image::io::Reader::new(Cursor::new(sprite_data))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();
        let mut image_file = File::create(file_path).unwrap();
        sprite_image
            .write_to(&mut image_file, image::ImageFormat::Png)
            .unwrap();
        // image::load_from_memory(&sprite_data).unwrap();
        // image::save_buffer(file_path, &sprite_data, 200, 200, image::R);
        // fs::write(file_path, sprite_image).unwrap();
    }
    Ok("Image Downloaded".to_string())
}
