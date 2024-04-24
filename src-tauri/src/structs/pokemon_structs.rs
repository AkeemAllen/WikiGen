use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pokemon {
    pub pokemon: IndexMap<u32, PokemonData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PokemonData {
    pub id: u32,
    pub name: String,
    pub types: Vec<String>,
    // TODO: Update field to be Vec<Ability> where Ability contains name and effect
    pub abilities: Vec<String>,
    pub stats: Stats,
    pub moves: HashMap<String, Move>,
    pub sprite: String,
    pub evolution: Evolution,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Stats {
    pub hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub sp_attack: u32,
    pub sp_defense: u32,
    pub speed: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Move {
    pub level_learned: u32,
    pub learn_method: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evolution {
    pub level: Option<u32>,
    pub item: Option<String>,
    pub other: Option<String>,
    pub evolves_to: Option<String>,
    pub method: EvolutionMethod,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EvolutionMethod {
    LevelUp,
    Item,
    Other,
    NoChange,
}
