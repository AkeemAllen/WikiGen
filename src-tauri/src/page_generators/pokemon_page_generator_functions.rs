use std::{collections::HashMap, path::PathBuf};

use indexmap::IndexMap;

use crate::{
    helpers::{capitalize, matchups::get_defensive_matchups},
    structs::{
        matchup_models::TypeEffectiveness,
        move_structs::{LearnMethodDetail, MoveSetMove, Moves},
        pokemon_structs::{Evolution, EvolutionMethod, Move, Stats},
    },
};

pub fn create_type_table(types: &Vec<String>) -> String {
    let type_images: Vec<String> = types
        .iter()
        .map(|_type| format!("![{}](../img/types/{}.png)", _type, _type))
        .filter(|_type| !_type.contains("none"))
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
pub fn create_defenses_table(
    types: &Vec<String>,
    wiki_name: &str,
    calculated_defenses: &HashMap<String, TypeEffectiveness>,
    base_path: &PathBuf,
) -> String {
    // Get Defensive Matchups from file before calculating them
    let defensive_matchups = match calculated_defenses.get(&types.join("-").to_string()) {
        Some(matchup) => matchup.0.clone(),
        // May be able to remove this functionality since I'm only working with latest gen matchups
        // But leaving for now.
        None => get_defensive_matchups(&types, wiki_name, base_path),
    };

    let immunities = get_markdown_for_effectiveness(&defensive_matchups, "0");
    let quarter_strong_resist = get_markdown_for_effectiveness(&defensive_matchups, "0.25");
    let half_strong_resist = get_markdown_for_effectiveness(&defensive_matchups, "0.5");
    let normal_resists = get_markdown_for_effectiveness(&defensive_matchups, "1");
    let double_weak_resists = get_markdown_for_effectiveness(&defensive_matchups, "2");
    let quad_weak_resists = get_markdown_for_effectiveness(&defensive_matchups, "4");

    return format!(
        "| Immune x0 | Resistant ×¼ | Resistant ×½ | Normal x1 | Weak x2 | Weak x4 |
        | :--: | :--: | :--: | :--: | :--: | :--: |
        | {} | {} | {} | {} | {} | {} |
        ",
        immunities,
        quarter_strong_resist,
        half_strong_resist,
        normal_resists,
        double_weak_resists,
        quad_weak_resists
    );
}

pub fn create_ability_table(abilities: &Vec<String>) -> String {
    let placeholder_effect = "Effect";
    let ability_entries: Vec<String> = abilities
        .iter()
        .map(|ability| format!("[{}](\" {} \")", capitalize(ability), placeholder_effect))
        .filter(|ability| !ability.contains("None"))
        .collect();

    return format!(
        "| Version | Ability |
        | :--: | ---: |
        | All | {} |
        ",
        ability_entries.join("/")
    );
}

pub fn create_stats_table(stats: &Stats) -> String {
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

pub fn create_evolution_table(evolution: Evolution) -> String {
    let no_change = "".to_string();

    let item_level_note = match evolution.method {
        EvolutionMethod::Item => evolution.item,
        EvolutionMethod::LevelUp => evolution.level.to_string(),
        EvolutionMethod::Other => evolution.other,
        EvolutionMethod::NoChange => no_change,
    };

    return format!(
        "| Method | Item/Level/Note | Evolved Pokemon |
        | :--: | :--: | :--: |
        | {:?} | {} | {} |
        ",
        evolution.method, item_level_note, &evolution.evolves_to.pokemon_name
    );
}

pub fn create_level_up_moves_table(
    moves: HashMap<String, Move>,
    moves_from_file: Moves,
    tabbed: bool,
) -> String {
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

        let mut tab = "";
        if tabbed {
            tab = "\t";
        }
        let table_entry = format!(
            "{}| {} | {} | {} | {} | {} | {} | {} |\n",
            tab,
            level_learned,
            capitalize(&move_name),
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

pub fn create_learnable_moves_table(
    moves: HashMap<String, Move>,
    moves_from_file: Moves,
    tabbed: bool,
) -> String {
    let mut _moves_data: IndexMap<String, MoveSetMove> = IndexMap::new();

    for (move_name, details) in moves {
        if !details.learn_method.contains(&"machine".to_string()) {
            continue;
        }
        match moves_from_file.moves.get(&move_name) {
            Some(file_move) => {
                _moves_data.insert(
                    move_name,
                    MoveSetMove {
                        learn_method_detail: LearnMethodDetail::MachineName(
                            file_move.machine_name.clone(),
                        ),
                        power: file_move.power,
                        pp: file_move.pp,
                        accuracy: file_move.accuracy,
                        _type: file_move._type.clone(),
                        damage_class: file_move.damage_class.clone(),
                    },
                );
            }
            None => {
                println!("Issue getting move from file");
                continue;
            }
        };
    }

    _moves_data.sort_by(|_, value1, _, value2| {
        let new_string = String::new();
        let machine_name1 = match &value1.learn_method_detail {
            LearnMethodDetail::MachineName(name) => name,
            LearnMethodDetail::LevelLearned(_) => &new_string,
        };

        let machine_name2 = match &value2.learn_method_detail {
            LearnMethodDetail::MachineName(name) => name,
            LearnMethodDetail::LevelLearned(_) => &new_string,
        };
        machine_name1.cmp(&machine_name2)
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

        let mut machine_name = String::new();
        if let LearnMethodDetail::MachineName(name) = movesetmove.learn_method_detail {
            machine_name = name;
        }

        if machine_name.is_empty() {
            continue;
        }

        // Capitalizing the first two characters of the machine name
        if let Some(chars) = machine_name.get_mut(0..2) {
            chars.make_ascii_uppercase();
        }

        let mut tab = "";
        if tabbed {
            tab = "\t";
        }
        let table_entry = format!(
            "{}| {} | {} | {} | {} | {} | {} | {} |\n",
            tab,
            machine_name,
            capitalize(&move_name),
            power,
            accuracy,
            movesetmove.pp,
            get_markdown_image_for_type(&movesetmove._type),
            get_markdown_image_for_type(&movesetmove.damage_class)
        );
        markdown_moves.push_str(&table_entry); // markdown_moves.
    }

    return format!(
        "| Machine | Name | Power | Accuracy | PP | Type | Damage Class |
        | -- | -- | -- | -- | -- | -- | -- |
        {}
        ",
        markdown_moves
    );
}

fn get_markdown_image_for_type(_type: &String) -> String {
    return format!(
        "![{}](../img/types/{}.png)",
        _type.to_lowercase(),
        _type.to_lowercase()
    );
}

fn get_markdown_for_effectiveness(
    matchups: &HashMap<String, Vec<String>>,
    effectiveness: &str,
) -> String {
    if let Some(resistances) = matchups.get(effectiveness) {
        let resistance_values: Vec<String> = resistances
            .iter()
            .map(|resistance| get_markdown_image_for_type(resistance))
            .collect();
        return resistance_values.join("<br/>").to_string();
    }
    return String::new();
}
