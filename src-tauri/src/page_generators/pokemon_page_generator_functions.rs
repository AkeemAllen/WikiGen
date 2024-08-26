use std::collections::HashMap;

use crate::{
    helpers::{capitalize, capitalize_and_remove_hyphens},
    structs::{
        matchup_models::TypeEffectiveness,
        pokemon_structs::{DBPokemon, PokemonMove},
    },
};

use super::game_routes::WildEncounter;

#[allow(dead_code)]
pub fn create_type_table(types: String) -> String {
    let type_images: Vec<String> = types
        .split(",")
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
#[allow(dead_code)]
pub fn create_defenses_table(
    types: String,
    calculated_defenses: &HashMap<String, TypeEffectiveness>,
) -> String {
    let mut types = types.split(",").collect::<Vec<&str>>();
    if types.len() == 1 {
        types.push("none")
    }
    // Get Defensive Matchups from file before calculating them
    let defensive_matchups = match calculated_defenses.get(&types.join("-").to_string()) {
        Some(matchup) => matchup.0.clone(),
        None => return "Unable to parse defensive matchups".to_string(),
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

#[allow(dead_code)]
pub fn create_ability_table(pokemon: &DBPokemon) -> String {
    let mut abilities_string = String::new();
    if pokemon.ability_1.is_some() {
        abilities_string.push_str(&format!(
            "[{}](\" {} \")",
            capitalize(&pokemon.ability_1.as_ref().unwrap()),
            &pokemon.a1_effect.as_ref().unwrap()
        ));
    }

    if pokemon.ability_2.is_some() {
        abilities_string.push_str(&format!(
            "/[{}](\" {} \")",
            capitalize(pokemon.ability_2.as_ref().unwrap()),
            &pokemon.a2_effect.as_ref().unwrap()
        ));
    }

    if pokemon.ability_1.is_none() && pokemon.ability_2.is_none() {
        abilities_string.push_str("None");
    }

    return format!(
        "| Version | Ability |
        | :--: | ---: |
        | All | {} |
        ",
        abilities_string
    );
}

#[allow(dead_code)]
pub fn create_stats_table(pokemon: &DBPokemon) -> String {
    let base_stat_total: u32 = [
        pokemon.hp,
        pokemon.attack,
        pokemon.defense,
        pokemon.sp_attack,
        pokemon.sp_defense,
        pokemon.speed,
    ]
    .iter()
    .sum();

    return format!(
        "| Version | HP | Atk | Def | SAtk | SDef | Spd | BST |
        | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
        | All | {} | {} | {} | {} | {} | {} | {} |
        ",
        pokemon.hp,
        pokemon.attack,
        pokemon.defense,
        pokemon.sp_attack,
        pokemon.sp_defense,
        pokemon.speed,
        base_stat_total
    );
}

pub fn create_evolution_table(pokemon: &DBPokemon) -> String {
    let no_change = "";

    let method: String;

    if &pokemon.evolution_method == "no_change" {
        return "".to_string();
    }

    if &pokemon.evolution_method == "level_up" {
        method = "Level Up".to_string();
    } else {
        method = pokemon.evolution_method.to_string();
    };

    let level = match pokemon.evolution_level {
        Some(level) => level.to_string(),
        None => 0.to_string(),
    };
    let item_level_note = match pokemon.evolution_method.as_str() {
        "item" => pokemon.evolution_item.as_ref().unwrap(),
        "level_up" => &level,
        "other" => pokemon.evolution_other.as_ref().unwrap(),
        _ => no_change,
    };

    let evolved_pokemon = match &pokemon.evolved_pokemon {
        Some(evolved_pokemon) => evolved_pokemon,
        None => "Evolution Missing",
    };

    return format!(
        "##Evolution Change\n| Method | Item/Level/Note | Evolved Pokemon |
        | :--: | :--: | :--: |
        | {} | {} | {} |
        ",
        capitalize(&method),
        item_level_note,
        &evolved_pokemon
    );
}

pub fn create_level_up_moves_table(mut level_up_moveset: Vec<PokemonMove>) -> String {
    // Setting default value of unwrapped level to 0
    // just in case the level is somehow missing
    level_up_moveset.sort_by_key(|_move| _move.level_learned.unwrap_or(0));

    let mut markdown_moves = String::new();
    for _move in level_up_moveset {
        let power = match _move.power {
            Some(power) => power.to_string(),
            None => "-".to_string(),
        };
        let accuracy = match _move.accuracy {
            Some(accuracy) => accuracy.to_string(),
            None => "-".to_string(),
        };

        let table_entry = format!(
            "\t| {} | {} | {} | {} | {} | {} | {} |\n",
            _move.level_learned.unwrap_or(0),
            capitalize(&_move.move_name),
            power,
            accuracy,
            _move.pp.unwrap_or(0),
            get_markdown_image_for_type(&_move.move_type.unwrap_or("normal".to_string())),
            get_markdown_image_for_type(&_move.damage_class)
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

pub fn create_learnable_moves_table(learnable_moveset: Vec<PokemonMove>) -> String {
    let mut markdown_moves = String::new();
    for _move in learnable_moveset {
        let power = match _move.power {
            Some(power) => power.to_string(),
            None => "-".to_string(),
        };
        let accuracy = match _move.accuracy {
            Some(accuracy) => accuracy.to_string(),
            None => "-".to_string(),
        };

        let mut machine_name = match &_move.machine_name {
            Some(machine_name) => machine_name.clone(),
            None => "".to_string(),
        };

        if machine_name.is_empty() {
            continue;
        }

        // Capitalizing the first two characters of the machine name
        if let Some(chars) = machine_name.get_mut(0..2) {
            chars.make_ascii_uppercase();
        }

        let table_entry = format!(
            "\t| {} | {} | {} | {} | {} | {} | {} |\n",
            &machine_name,
            capitalize(&_move.move_name),
            power,
            accuracy,
            _move.pp.unwrap_or(0),
            get_markdown_image_for_type(&_move.move_type.unwrap_or("normal".to_string())),
            get_markdown_image_for_type(&_move.damage_class)
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

pub fn create_locations_table(wild_encounters: Vec<WildEncounter>) -> String {
    if wild_encounters.is_empty() {
        return "".to_string();
    }
    let mut markdown_locations = String::new();
    for encounter in wild_encounters {
        let table_entry = format!(
            "\t| {} | {} | {} | {} |\n",
            encounter.route,
            capitalize_and_remove_hyphens(&encounter.encounter_area),
            encounter.encounter_rate,
            encounter.special_note
        );
        markdown_locations.push_str(&table_entry); // markdown_locations.
    }

    return format!(
        "## Locations\n| Route | Area | Encounter Rate | Extra Instructions |
        | -- | -- | -- | -- |
        {}
        ",
        markdown_locations
    );
}

fn get_markdown_image_for_type(_type: &String) -> String {
    return format!(
        "![{}](../img/types/{}.png)",
        _type.to_lowercase(),
        _type.to_lowercase()
    );
}

#[allow(dead_code)]
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
