use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::structs::pokemon_structs::Evolution;

type ModifiedPokemon = HashMap<String, ModifiedPokemonDetails>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModifiedPokemonDetails {
    pub id: usize,
    pub types: Types,
    pub evolution: Evolution,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Types {
    original: Vec<String>,
    modified: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ModifiedItemsNaturesAbilities {
    items: HashMap<String, ModifiedItem>,
    natures: HashMap<String, ModifiedNature>,
    abilities: HashMap<String, ModifiedAbility>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModifiedItem {
    pub original: Item,
    pub modified: Item,
    pub is_new_item: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModifiedAbility {
    pub original: Effect,
    pub modified: Effect,
    pub is_new_ability: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModifiedNature {
    pub original: Nature,
    pub modified: Nature,
    pub is_new_nature: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Nature {
    pub increased_stat: Option<String>,
    pub decreased_stat: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {
    pub effect: String,
    pub sprite: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Effect {
    pub effect: String,
}

pub mod ability_page;
pub mod evolution_page;
pub mod game_routes;
pub mod item_page;
pub mod nature_page;
mod pokemon_page_generator_functions;
pub mod pokemon_pages;
pub mod type_page;
