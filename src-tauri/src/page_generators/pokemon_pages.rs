use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use serde_yaml::{Mapping, Value};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use tauri::AppHandle;

use crate::{
    helpers::capitalize,
    page_generators::pokemon_page_generator_functions::{
        create_ability_table, create_defenses_table, create_evolution_table,
        create_learnable_moves_table, create_level_up_moves_table, create_stats_table,
        create_type_table,
    },
    structs::{
        matchup_models::TypeEffectiveness,
        mkdocs_structs::MKDocsConfig,
        pokemon_structs::{DBPokemon, PokemonMove},
    },
};

#[tauri::command]
pub async fn generate_pokemon_pages_from_list(
    wiki_name: &str,
    pokemon_ids: Vec<usize>,
    app_handle: AppHandle,
) -> Result<String, String> {
    let base_path = app_handle.path_resolver().app_data_dir().unwrap();

    let result = generate_pokemon_pages(pokemon_ids, wiki_name, base_path.clone()).await;
    // generate_type_page(wiki_name, base_path.clone())?;
    // generate_evolution_page(wiki_name, base_path)?;
    return result;
}

pub async fn generate_pokemon_pages(
    pokemon_ids: Vec<usize>,
    wiki_name: &str,
    base_path: PathBuf,
) -> Result<String, String> {
    let docs_path = base_path.join(wiki_name).join("dist").join("docs");

    let sqlite_path = base_path.join(wiki_name).join(format!("{}.db", wiki_name));
    let sqlite_connection_string = format!("sqlite:{}", sqlite_path.to_str().unwrap());
    if !Sqlite::database_exists(&sqlite_connection_string)
        .await
        .unwrap_or(false)
    {
        return Err("Database does not exist".to_string());
    }
    let conn = match SqlitePool::connect(&sqlite_connection_string).await {
        Ok(conn) => conn,
        Err(err) => {
            return Err(format!("Failed to connect to database: {}", err));
        }
    };

    let id_list = &pokemon_ids
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",");

    let pokemon_query = format!(
        "SELECT
            pokemon.*,
            a1.effect as a1_effect,
            a2.effect as a2_effect
        FROM pokemon
        LEFT JOIN abilities a1 on a1.name = pokemon.ability_1
        LEFT JOIN abilities a2 on a2.name = pokemon.ability_2
        WHERE pokemon.id IN ({}) ORDER BY dex_number ASC",
        id_list
    );
    let pokemon_list = match sqlx::query_as::<_, DBPokemon>(&pokemon_query)
        .fetch_all(&conn)
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
        .fetch_all(&conn)
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

    let mkdocs_yaml_file_path = base_path.join(wiki_name).join("dist").join("mkdocs.yml");
    let mkdocs_yaml_file = match File::open(&mkdocs_yaml_file_path) {
        Ok(file) => file,
        Err(err) => {
            return Err(format!("Failed to open mkdocs yaml file: {}", err));
        }
    };
    let mut mkdocs_config: MKDocsConfig = match serde_yaml::from_reader(mkdocs_yaml_file) {
        Ok(mkdocs) => mkdocs,
        Err(err) => {
            return Err(format!("Failed to parse mkdocs yaml file: {}", err));
        }
    };

    let calculated_defenses_path = base_path
        .join(wiki_name)
        .join("data")
        .join("calculated_defenses.json");
    let calculated_defenses_json_file = match File::open(&calculated_defenses_path) {
        Ok(file) => file,
        Err(err) => {
            return Err(format!("Failed to open calculated defenses file: {}", err));
        }
    };
    let calculated_defenses: HashMap<String, TypeEffectiveness> =
        match serde_json::from_reader(calculated_defenses_json_file) {
            Ok(calc_defenses) => calc_defenses,
            Err(err) => {
                return Err(format!("Failed to parse calculated defenses file: {}", err));
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
        let mut pokedex_markdown_file_name = format!("00{}", pokemon.dex_number);
        if pokemon.dex_number >= 10 {
            pokedex_markdown_file_name = format!("0{}", pokemon.dex_number);
        }
        if pokemon.dex_number >= 100 {
            pokedex_markdown_file_name = format!("{}", pokemon.dex_number);
        }
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
            println!("Skipping rendering page for {}", &pokemon.name);
            continue;
        }

        println!("Rendering page for {}", &pokemon.name);

        let mut markdown_file = match File::create(docs_path.join("pokemon").join(format!(
            "{}-{}.md",
            pokedex_markdown_file_name, &pokemon.name
        ))) {
            Ok(file) => file,
            Err(e) => {
                println!("Error creating file: {:?}", e);
                continue;
            }
        };

        let current_pokemon_movset = movesets
            .iter()
            .cloned()
            .filter(|m| m.pokemon == pokemon.id)
            .collect::<Vec<_>>();

        let pokemon_markdown_string =
            generate_pokemon_page(&calculated_defenses, &pokemon, current_pokemon_movset);

        match markdown_file.write_all(format!("{}", pokemon_markdown_string).as_bytes()) {
            Ok(_) => {}
            Err(err) => {
                return Err(format!(
                    "Failed to write to pokemon markdown file for {}: {}",
                    pokemon.dex_number, err
                ))
            }
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
        Err(err) => return Err(format!("Failed to update mkdocs yaml: {}", err)),
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

pub fn generate_pokemon_page(
    calculated_defenses: &HashMap<String, TypeEffectiveness>,
    pokemon: &DBPokemon,
    movesets: Vec<PokemonMove>,
) -> String {
    let mut markdown_string = String::new();
    markdown_string.push_str(&format!(
        "![{}](../img/pokemon/{}.png)\n\n",
        &pokemon.name, &pokemon.name
    ));

    markdown_string.push_str(&format!("## Types\n\n"));
    markdown_string.push_str(&format!("{}", create_type_table(pokemon.types.clone())));
    markdown_string.push_str("\n\n");

    markdown_string.push_str(&format!("## Defenses\n\n"));
    markdown_string.push_str(&format!(
        "{}",
        create_defenses_table(pokemon.types.clone(), &calculated_defenses,)
    ));
    markdown_string.push_str("\n\n");

    markdown_string.push_str(&format!("## Abilities\n\n"));
    markdown_string.push_str(&format!("{}", create_ability_table(pokemon)));
    markdown_string.push_str("\n\n");

    markdown_string.push_str(&format!("## Stats\n\n"));
    markdown_string.push_str(&format!("{}", create_stats_table(pokemon)));
    markdown_string.push_str("\n\n");

    if pokemon.evolution_method != "no_change" {
        markdown_string.push_str(&format!("## Evolution\n\n"));
        markdown_string.push_str(&format!("{}", create_evolution_table(pokemon)));
        markdown_string.push_str("\n\n");
    }

    let level_up_moveset = movesets
        .iter()
        .cloned()
        .filter(|m| m.learn_method == "level-up")
        .collect::<Vec<_>>();

    markdown_string.push_str(&format!("## Level Up Moves\n\n"));
    markdown_string.push_str(&format!(
        "{}",
        create_level_up_moves_table(level_up_moveset)
    ));
    markdown_string.push_str("\n\n");

    let learnable_moveset = movesets
        .iter()
        .cloned()
        .filter(|m| m.learn_method == "machine")
        .collect::<Vec<_>>();

    markdown_string.push_str(&format!("## Learnable Moves\n\n"));
    markdown_string.push_str(&format!(
        "{}",
        create_learnable_moves_table(learnable_moveset)
    ));
    markdown_string.push_str("\n\n");
    return markdown_string;
}
