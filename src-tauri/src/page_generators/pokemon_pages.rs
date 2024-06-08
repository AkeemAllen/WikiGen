use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use indexmap::IndexMap;
use serde_yaml::{Mapping, Value};
use tauri::AppHandle;

use crate::{
    helpers::{capitalize, capitalize_and_remove_hyphens},
    page_generators::pokemon_page_generator_functions::{
        create_ability_table, create_defenses_table, create_evolution_table,
        create_learnable_moves_table, create_level_up_moves_table, create_stats_table,
        create_type_table,
    },
    structs::{
        matchup_models::TypeEffectiveness,
        mkdocs_structs::MKDocsConfig,
        move_structs::Moves,
        pokemon_structs::{EvolutionMethod, Pokemon, PokemonData, PokemonForm},
    },
};

use super::{evolution_page::generate_evolution_page, type_page::generate_type_page};

#[tauri::command]
pub async fn generate_pokemon_pages_from_list(
    wiki_name: &str,
    dex_numbers: Vec<usize>,
    app_handle: AppHandle,
) -> Result<String, String> {
    let base_path = app_handle.path_resolver().app_data_dir().unwrap();
    let result = generate_pokemon_pages(dex_numbers, wiki_name, base_path.clone());
    generate_type_page(wiki_name, base_path.clone()).unwrap();
    generate_evolution_page(wiki_name, base_path).unwrap();
    return result;
}

#[tauri::command]
pub async fn generate_pokemon_pages_from_range(
    wiki_name: &str,
    range_start: usize,
    range_end: usize,
    app_handle: AppHandle,
) -> Result<String, String> {
    let base_path = app_handle.path_resolver().app_data_dir().unwrap();
    let mut dex_numbers = Vec::new();
    for number in range_start..=range_end {
        dex_numbers.push(number);
    }
    let result = generate_pokemon_pages(dex_numbers, wiki_name, base_path.clone());
    generate_type_page(wiki_name, base_path.clone()).unwrap();
    generate_evolution_page(wiki_name, base_path).unwrap();
    return result;
}

pub fn generate_pokemon_pages(
    dex_numbers: Vec<usize>,
    wiki_name: &str,
    base_path: PathBuf,
) -> Result<String, String> {
    let docs_path = base_path.join(wiki_name).join("dist").join("docs");

    let mut pokemon: Pokemon = Pokemon {
        pokemon: IndexMap::new(),
    };

    for i in 1..=10 {
        let shard_json_file_path = base_path
            .join(wiki_name)
            .join("data")
            .join("pokemon_data")
            .join(format!("shard_{}.json", i));
        let shard_file = File::open(&shard_json_file_path).unwrap();
        let shard: Pokemon = match serde_json::from_reader(shard_file) {
            Ok(shard) => shard,
            Err(err) => {
                println!("Error {}", err);
                return Err("Something failed".to_string());
            }
        };

        pokemon.pokemon.extend(shard.pokemon.into_iter());
    }

    let moves_json_file_path = base_path.join(wiki_name).join("data").join("moves.json");
    let moves_file = File::open(&moves_json_file_path).unwrap();
    let moves: Moves = serde_json::from_reader(moves_file).unwrap();

    let mkdocs_yaml_file_path = base_path.join(wiki_name).join("dist").join("mkdocs.yml");
    let mkdocs_yaml_file = File::open(&mkdocs_yaml_file_path).unwrap();
    let mut mkdocs_config: MKDocsConfig = serde_yaml::from_reader(mkdocs_yaml_file).unwrap();

    let calculated_defenses_path = base_path
        .join(wiki_name)
        .join("data")
        .join("calculated_defenses.json");
    let calculated_defenses_json_file = File::open(&calculated_defenses_path).unwrap();
    let mut calculated_defenses: HashMap<String, TypeEffectiveness> =
        serde_json::from_reader(calculated_defenses_json_file).unwrap();

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

    for dex_number in dex_numbers {
        let pokemon_data = match pokemon.pokemon.get(&(dex_number as u32)) {
            Some(pokemon_data) => pokemon_data,
            None => {
                println!("Pokemon Data not found for dex number: {:?}", dex_number);
                continue;
            }
        };

        let mut pokedex_markdown_file_name = format!("00{}", dex_number);
        if dex_number >= 10 {
            pokedex_markdown_file_name = format!("0{}", dex_number);
        }
        if dex_number >= 100 {
            pokedex_markdown_file_name = format!("{}", dex_number);
        }

        let mut markdown_file = match File::create(
            docs_path
                .join("pokemon")
                .join(format!("{}.md", pokedex_markdown_file_name)),
        ) {
            Ok(file) => file,
            Err(e) => {
                println!("Error creating file: {:?}", e);
                continue;
            }
        };

        let initial_form = PokemonForm {
            types: pokemon_data.types.clone(),
            abilities: pokemon_data.abilities.clone(),
            stats: pokemon_data.stats.clone(),
            moves: pokemon_data.moves.clone(),
            sprite: pokemon_data.sprite.clone(),
        };
        let mut forms_to_render: IndexMap<String, PokemonForm> = IndexMap::new();
        forms_to_render.insert(pokemon_data.name.clone(), initial_form);

        for form in pokemon_data.forms.clone() {
            forms_to_render.insert(form.0, form.1);
        }

        let mut tab_string = String::new();
        for (form_name, form_details) in forms_to_render.iter() {
            let mut tabbed = false;
            if forms_to_render.len() > 1 {
                tab_string.push_str(&format!(
                    "\n=== \"{}\"\n",
                    capitalize_and_remove_hyphens(form_name)
                ));
                tabbed = true;
            }

            let pokemon_markdown_string = generate_pokemon_page(
                wiki_name,
                &base_path,
                &mut calculated_defenses,
                form_name,
                form_details,
                &pokedex_markdown_file_name,
                moves.clone(),
                pokemon_data,
                tabbed,
            );

            tab_string.push_str(&pokemon_markdown_string);
        }

        markdown_file
            .write_all(format!("{}", tab_string).as_bytes())
            .unwrap();

        let mut pokemon_page_entry = Mapping::new();
        let entry_key = format!(
            "{} - {}",
            pokedex_markdown_file_name,
            capitalize(&pokemon_data.name)
        );
        pokemon_page_entry.insert(
            Value::String(entry_key.clone()),
            Value::String(format!("pokemon/{}.md", pokedex_markdown_file_name)),
        );

        let mut page_entry_exists = false;
        for page_entry in mkdocs_pokemon.clone() {
            if page_entry.as_mapping().unwrap().contains_key(&entry_key) {
                page_entry_exists = true;
                break;
            }
        }

        if !page_entry_exists {
            mkdocs_pokemon.push(Value::Mapping(pokemon_page_entry));
        }
    }

    fs::write(
        mkdocs_yaml_file_path,
        serde_yaml::to_string(&mkdocs_config).unwrap(),
    )
    .unwrap();

    fs::write(
        calculated_defenses_path,
        serde_json::to_string(&calculated_defenses).unwrap(),
    )
    .unwrap();

    return Ok("Pokemon Pages Generated".to_string());
}

fn generate_pokemon_page(
    wiki_name: &str,
    base_path: &PathBuf,
    mut calculated_defenses: &mut HashMap<String, TypeEffectiveness>,
    pokemon_name: &str,
    pokemon_details: &PokemonForm,
    file_name: &str,
    moves: Moves,
    pokemon_data: &PokemonData,
    tabbed: bool,
) -> String {
    let mut markdown_string = String::new();
    let mut tab = "";
    if tabbed {
        tab = "\t"
    }
    markdown_string.push_str(&format!(
        "{}![{}](../img/pokemon/{}.png)\n\n",
        tab, &pokemon_name, file_name
    ));

    markdown_string.push_str(&format!("{}## Types\n\n", tab));
    markdown_string.push_str(&format!(
        "{}{}",
        tab,
        create_type_table(&pokemon_details.types)
    ));
    markdown_string.push_str("\n\n");

    markdown_string.push_str(&format!("{}## Defenses\n\n", tab));
    markdown_string.push_str(&format!(
        "{}{}",
        tab,
        create_defenses_table(
            &pokemon_details.types,
            wiki_name,
            &mut calculated_defenses,
            &base_path,
        )
    ));
    markdown_string.push_str("\n\n");

    markdown_string.push_str(&format!("{}## Abilities\n\n", tab));
    markdown_string.push_str(&format!(
        "{}{}",
        tab,
        create_ability_table(&pokemon_details.abilities)
    ));
    markdown_string.push_str("\n\n");

    markdown_string.push_str(&format!("{}## Stats\n\n", tab));
    markdown_string.push_str(&format!(
        "{}{}",
        tab,
        create_stats_table(&pokemon_details.stats)
    ));
    markdown_string.push_str("\n\n");

    if &pokemon_data.evolution.method != &EvolutionMethod::NoChange
        && &pokemon_name == &pokemon_data.name
    {
        markdown_string.push_str(&format!("{}## Evolution\n\n", tab));
        markdown_string.push_str(&format!(
            "{}{}",
            tab,
            create_evolution_table(pokemon_data.evolution.clone())
        ));
        markdown_string.push_str("\n\n");
    }

    markdown_string.push_str(&format!("{}## Level Up Moves\n\n", tab));
    markdown_string.push_str(&format!(
        "{}{}",
        tab,
        create_level_up_moves_table(pokemon_details.moves.clone(), moves.clone(), tabbed)
    ));
    markdown_string.push_str("\n\n");

    markdown_string.push_str(&format!("{}## Learnable Moves\n\n", tab));
    markdown_string.push_str(&format!(
        "{}{}",
        tab,
        create_learnable_moves_table(pokemon_details.moves.clone(), moves.clone(), tabbed)
    ));
    markdown_string.push_str("\n\n");
    return markdown_string;
}
