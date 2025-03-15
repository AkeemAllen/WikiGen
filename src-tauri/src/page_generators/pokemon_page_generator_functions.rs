use crate::{
    helpers::{capitalize, capitalize_and_remove_hyphens},
    structs::pokemon_structs::{DBPokemon, PokemonMove},
};

use super::game_routes::WildEncounter;

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

pub fn create_locations_table(wild_encounters: &[WildEncounter]) -> String {
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
