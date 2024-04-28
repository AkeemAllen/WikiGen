use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    str::FromStr,
};

use indexmap::IndexMap;
use tauri::AppHandle;

use crate::{
    helpers::{capitalize, matchups::get_defensive_matchups},
    structs::{
        mkdocs_structs::{MKDocsConfig, Navigation},
        move_structs::{LearnMethodDetail, MoveSetMove, Moves},
        pokemon_structs::{Evolution, EvolutionMethod, Move, Pokemon, PokemonTypesEnum, Stats},
    },
};

#[tauri::command]
pub async fn generate_pokemon_pages_in_range(
    range_start: usize,
    range_end: usize,
    wiki_name: &str,
    app_handle: AppHandle,
) -> Result<String, String> {
    let base_path = app_handle.path_resolver().app_data_dir().unwrap();
    let docs_path = base_path.join(wiki_name).join("dist").join("docs");

    let pokemon_json_file_path = base_path.join(wiki_name).join("data").join("pokemon.json");
    let pokemon_file = File::open(&pokemon_json_file_path).unwrap();
    let pokemon: Pokemon = serde_json::from_reader(pokemon_file).unwrap();

    let moves_json_file_path = base_path.join(wiki_name).join("data").join("moves.json");
    let moves_file = File::open(&moves_json_file_path).unwrap();
    let moves: Moves = serde_json::from_reader(moves_file).unwrap();

    let mkdocs_yaml_file_path = base_path.join(wiki_name).join("dist").join("mkdocs.yml");
    let mkdocs_yaml_file = File::open(&mkdocs_yaml_file_path).unwrap();
    let mut mkdocs_config: MKDocsConfig = serde_yaml::from_reader(mkdocs_yaml_file).unwrap();

    let mut specific_changes = HashMap::new();

    /*
    Collecting the hashmap with the Specific Changes key here so it's easier to
    check for existing values further down.

    Note: this is probably not the most elegant way of collecting this value. However,
    because the navigation object in the mkdocs_config is so complex, I may not have a choice
    here.
     */
    if let Some(pokemon_nav) = mkdocs_config.nav[1].get("Pokemon") {
        if let Navigation::Array(pokemon_nav) = pokemon_nav {
            if let Navigation::Map(pokemon_nav) = &pokemon_nav[0] {
                specific_changes = pokemon_nav.clone();
            }
        }
    }

    for dex_number in range_start..=range_end {
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

        // Adding sprite
        markdown_file
            .write_all(
                format!(
                    "![{}](../img/pokemon/{}.png)\n\n",
                    pokemon_data.name, pokedex_markdown_file_name
                )
                .as_bytes(),
            )
            .unwrap();

        // Add Type Table
        markdown_file.write_all(b"## Types\n\n").unwrap();
        markdown_file
            .write_all(create_type_table(&pokemon_data.types).as_bytes())
            .unwrap();

        // Add Defensive Matchups Table
        markdown_file.write_all(b"\n\n##Defenses\n\n").unwrap();
        markdown_file
            .write_all(
                create_defenses_table(&pokemon_data.types, wiki_name, app_handle.clone())
                    .as_bytes(),
            )
            .unwrap();

        // Add Abilities Table
        markdown_file.write_all(b"\n\n## Abilities\n\n").unwrap();
        markdown_file
            .write_all(create_ability_table(&pokemon_data.abilities).as_bytes())
            .unwrap();

        // Add Stats Table
        markdown_file.write_all(b"\n\n## Stats\n\n").unwrap();
        markdown_file
            .write_all(create_stats_table(&pokemon_data.stats).as_bytes())
            .unwrap();

        // Add Evolution Table
        if &pokemon_data.evolution.method != &EvolutionMethod::NoChange {
            // Add Stats Table
            markdown_file
                .write_all(b"\n\n## Evolution Change\n\n")
                .unwrap();
            markdown_file
                .write_all(create_evolution_table(pokemon_data.evolution.clone()).as_bytes())
                .unwrap();
        }

        // Add Level Up Moves
        markdown_file
            .write_all(b"\n\n## Level Up Moves\n\n")
            .unwrap();
        markdown_file
            .write_all(
                create_level_up_moves_table(pokemon_data.moves.clone(), moves.clone()).as_bytes(),
            )
            .unwrap();

        let mut specific_change_entry = HashMap::new();
        let entry_key = format!(
            "{} - {}",
            pokedex_markdown_file_name,
            capitalize::capitalize(&pokemon_data.name)
        );
        specific_change_entry.insert(
            entry_key.clone(),
            Navigation::String(format!("pokemon/{}.md", pokedex_markdown_file_name)),
        );

        /*
        This block of code check if the specific_change_entry exists in
        specific changes already. If not, then we push it to the array.

        Note: Similar to gathering the specific changes above, the complexity
        of the nesting requires all of the steps here.
         */
        if let Some(change) = specific_changes.get_mut("Specific Changes") {
            if let Navigation::Array(entries) = change {
                let mut entry_exists = false;
                for entry in &mut *entries {
                    if let Navigation::Map(entry_map) = entry {
                        if entry_map.contains_key(&entry_key.clone()) {
                            entry_exists = true;
                            break;
                        }
                    }
                }
                if !entry_exists {
                    entries.push(Navigation::Map(specific_change_entry))
                }
            }
        }
    }

    if let Some(pokemon_nav) = mkdocs_config.nav[1].get_mut("Pokemon") {
        *pokemon_nav = Navigation::Array(vec![Navigation::Map(specific_changes)]);
    };

    fs::write(
        mkdocs_yaml_file_path,
        serde_yaml::to_string(&mkdocs_config).unwrap(),
    )
    .unwrap();

    return Ok("Pokemon Pages Generated".to_string());
}

fn create_type_table(types: &Vec<String>) -> String {
    let type_images: Vec<String> = types
        .iter()
        .map(|_type| format!("![{}](../img/types/{}.png)", _type, _type))
        .collect();

    return format!(
        "| Version | Type |
            | :--: | ----: |
            | Classic | {} |
        ",
        type_images.join(" ")
    );
}

// Find better way to implement this function
fn create_defenses_table(types: &Vec<String>, wiki_name: &str, app_handle: AppHandle) -> String {
    let defense_types = types
        .iter()
        .map(|_type| PokemonTypesEnum::from_str(&_type).unwrap())
        .collect();
    let defensive_matchups = get_defensive_matchups(defense_types, wiki_name, app_handle);

    let mut immunity_markdown = "".to_string();
    if let Some(immunities) = defensive_matchups.get("0") {
        let markdown_immunity: Vec<String> = immunities
            .iter()
            .map(|immunity| get_markdown_image_for_type(immunity))
            .collect();
        immunity_markdown = markdown_immunity.join("<br/>").to_string();
    }

    let mut normal_resists_markdown = "".to_string();
    if let Some(normal_resists) = defensive_matchups.get("1") {
        let markdown_normal: Vec<String> = normal_resists
            .iter()
            .map(|normal_resist| get_markdown_image_for_type(normal_resist))
            .collect();
        normal_resists_markdown = markdown_normal.join("<br/>").to_string();
    }

    let mut double_weak_resists_markdown = "".to_string();
    if let Some(double_resists) = defensive_matchups.get("2") {
        let markdown_double: Vec<String> = double_resists
            .iter()
            .map(|double_resist| get_markdown_image_for_type(double_resist))
            .collect();
        double_weak_resists_markdown = markdown_double.join("<br/>").to_string();
    }

    let mut quad_weak_resists_markdown = "".to_string();
    if let Some(quad_weak_resists) = defensive_matchups.get("4") {
        let markdown_quad: Vec<String> = quad_weak_resists
            .iter()
            .map(|quad_weak_resist| get_markdown_image_for_type(quad_weak_resist))
            .collect();
        quad_weak_resists_markdown = markdown_quad.join("<br/>").to_string();
    }

    let mut half_weak_resists_markdown = "".to_string();
    if let Some(half_weak_resists) = defensive_matchups.get("0.5") {
        let markdown_half_weak: Vec<String> = half_weak_resists
            .iter()
            .map(|half_weak_resist| get_markdown_image_for_type(half_weak_resist))
            .collect();
        half_weak_resists_markdown = markdown_half_weak.join("<br/>").to_string();
    }

    let mut quarter_weak_resists_markdown = "".to_string();
    if let Some(quarter_resists) = defensive_matchups.get("0.25") {
        let markdown_quarter_weak: Vec<String> = quarter_resists
            .iter()
            .map(|quarter_weak_resist| get_markdown_image_for_type(quarter_weak_resist))
            .collect();
        quarter_weak_resists_markdown = markdown_quarter_weak.join("<br/>").to_string();
    }

    return format!(
        "| Immune x0 | Resistant ×¼ | Resistant ×½ | Normal x1 | Weak x2 | Weak x4 |
        | :--: | :--: | :--: | :--: | :--: | :--: |
        | {} | {} | {} | {} | {} | {} |
        ",
        immunity_markdown,
        quarter_weak_resists_markdown,
        half_weak_resists_markdown,
        normal_resists_markdown,
        double_weak_resists_markdown,
        quad_weak_resists_markdown
    );
}

fn create_ability_table(abilities: &Vec<String>) -> String {
    let placeholder_effect = "Effect";
    let ability_entries: Vec<String> = abilities
        .iter()
        .map(|ability| {
            format!(
                "[{}](\" {} \")",
                capitalize::capitalize(ability),
                placeholder_effect
            )
        })
        .collect();

    return format!(
        "| Version | Ability |
        | :--: | ---: |
        | All | {} |
        ",
        ability_entries.join("/")
    );
}

fn create_stats_table(stats: &Stats) -> String {
    let base_stat_total: u32 = [
        stats.hp,
        stats.attack,
        stats.defense,
        stats.sp_attack,
        stats.sp_defense,
        stats.speed,
    ]
    .iter()
    .sum();

    return format!(
        "| Version | HP | Atk | Def | SAtk | SDef | Spd | BST |
        | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
        | All | {} | {} | {} | {} | {} | {} | {} |
        ",
        stats.hp,
        stats.attack,
        stats.defense,
        stats.sp_attack,
        stats.sp_defense,
        stats.speed,
        base_stat_total
    );
}

fn create_level_up_moves_table(moves: HashMap<String, Move>, moves_from_file: Moves) -> String {
    let mut _moves_data: IndexMap<String, MoveSetMove> = IndexMap::new();

    for (move_name, details) in moves {
        if !details.learn_method.contains(&"level-up".to_string()) {
            continue;
        }
        match moves_from_file.moves.get(&move_name) {
            Some(file_move) => _moves_data.insert(
                move_name,
                MoveSetMove {
                    learn_method_detail: LearnMethodDetail::LevelLearned(details.level_learned),
                    power: file_move.power,
                    pp: file_move.pp,
                    accuracy: file_move.accuracy,
                    _type: file_move._type.clone(),
                    damage_class: file_move.damage_class.clone(),
                },
            ),
            None => {
                println!("Issue getting move from file");
                continue;
            }
        };
    }

    _moves_data.sort_by(|_, value1, _, value2| {
        let level = match value1.learn_method_detail {
            LearnMethodDetail::LevelLearned(level) => level,
            LearnMethodDetail::MachineName(_) => 0,
        };

        let level2 = match value2.learn_method_detail {
            LearnMethodDetail::LevelLearned(level) => level,
            LearnMethodDetail::MachineName(_) => 0,
        };
        level.cmp(&level2)
    });

    let mut markdown_moves = String::new();
    for (move_name, movesetmove) in _moves_data {
        let power = match movesetmove.power {
            Some(power) => power.to_string(),
            None => "-".to_string(),
        };
        let accuracy = match movesetmove.accuracy {
            Some(accuracy) => accuracy.to_string(),
            None => "-".to_string(),
        };

        let mut level_learned = 0;
        if let LearnMethodDetail::LevelLearned(level) = movesetmove.learn_method_detail {
            level_learned = level;
        }

        let table_entry = format!(
            "| {} | {} | {} | {} | {} | {} | {} |\n",
            level_learned,
            capitalize::capitalize(&move_name),
            power,
            accuracy,
            movesetmove.pp,
            get_markdown_image_for_type(&movesetmove._type),
            get_markdown_image_for_type(&movesetmove.damage_class)
        );
        markdown_moves.push_str(&table_entry); // markdown_moves.
    }

    return format!(
        "| Level | Name | Power | Accuracy | PP | Type | Damage Class |
        | -- | -- | -- | -- | -- | -- | -- |
        {}
        ",
        markdown_moves
    );
}

fn create_evolution_table(evolution: Evolution) -> String {
    let no_change = "".to_string();

    let item_level_note = match evolution.method {
        EvolutionMethod::Item => evolution.item.unwrap(),
        EvolutionMethod::LevelUp => evolution.level.unwrap().to_string(),
        EvolutionMethod::Other => evolution.other.unwrap(),
        EvolutionMethod::NoChange => no_change,
    };

    return format!(
        "| Method | Item/Level/Note | Evolved Pokemon |
        | :--: | :--: | :--: |
        | {:?} | {} | {} |
        ",
        evolution.method,
        item_level_note,
        &evolution.evolves_to.unwrap()
    );
}

fn get_markdown_image_for_type(_type: &String) -> String {
    return format!(
        "![{}](../img/types/{}.png)",
        _type.to_lowercase(),
        _type.to_lowercase()
    );
}
