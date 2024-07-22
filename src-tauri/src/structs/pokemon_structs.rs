use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::{collections::HashMap, fmt, str::FromStr};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pokemon {
    pub pokemon: IndexMap<u32, PokemonData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PokemonData {
    pub id: u32,
    pub name: String,
    pub types: Vec<String>,
    pub abilities: Vec<String>,
    pub stats: Stats,
    pub moves: HashMap<String, Move>,
    pub sprite: String,
    pub evolution: Evolution,
    pub forms: HashMap<String, PokemonForm>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct DBPokemon {
    pub id: u32,
    pub dex_number: u32,
    pub name: String,
    pub types: String,
    pub ability_1: Option<String>,
    pub ability_2: Option<String>,
    pub hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub sp_attack: u32,
    pub sp_defense: u32,
    pub speed: u32,
    pub a1_effect: Option<String>,
    pub a2_effect: Option<String>,
    pub evolution_method: String,
    pub evolution_level: Option<u32>,
    pub evolution_item: Option<String>,
    pub evolution_other: Option<String>,
    pub evolved_pokemon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct PokemonMove {
    pub pokemon: u32,
    pub move_id: u32,
    pub learn_method: String,
    pub level_learned: Option<u32>,
    pub move_name: String,
    pub move_type: Option<String>,
    pub power: Option<u32>,
    pub accuracy: Option<u32>,
    pub pp: Option<u32>,
    pub damage_class: String,
    pub machine_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EvolutionChange {
    pub id: u32,
    pub level: Option<u32>,
    pub item: Option<String>,
    pub other: Option<String>,
    pub evolved_pokemon: EvolvedPokemon,
    pub method: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ability {
    pub effect: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PokemonForm {
    pub types: Vec<String>,
    pub abilities: Vec<String>,
    pub stats: Stats,
    pub sprite: String,
    pub moves: HashMap<String, Move>,
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
    pub level: u32,
    pub item: String,
    pub other: String,
    pub evolves_to: EvolvedPokemon,
    pub method: EvolutionMethod,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct EvolvedPokemon {
    pub id: u32,
    pub pokemon_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EvolutionMethod {
    LevelUp,
    Item,
    Other,
    NoChange,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum PokemonTypesEnum {
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy,
    None,
}

impl fmt::Display for PokemonTypesEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for PokemonTypesEnum {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "normal" => Ok(PokemonTypesEnum::Normal),
            "fire" => Ok(PokemonTypesEnum::Fire),
            "water" => Ok(PokemonTypesEnum::Water),
            "electric" => Ok(PokemonTypesEnum::Electric),
            "grass" => Ok(PokemonTypesEnum::Grass),
            "ice" => Ok(PokemonTypesEnum::Ice),
            "fighting" => Ok(PokemonTypesEnum::Fighting),
            "poison" => Ok(PokemonTypesEnum::Poison),
            "ground" => Ok(PokemonTypesEnum::Ground),
            "flying" => Ok(PokemonTypesEnum::Flying),
            "psychic" => Ok(PokemonTypesEnum::Psychic),
            "bug" => Ok(PokemonTypesEnum::Bug),
            "rock" => Ok(PokemonTypesEnum::Rock),
            "ghost" => Ok(PokemonTypesEnum::Ghost),
            "dragon" => Ok(PokemonTypesEnum::Dragon),
            "dark" => Ok(PokemonTypesEnum::Dark),
            "steel" => Ok(PokemonTypesEnum::Steel),
            "fairy" => Ok(PokemonTypesEnum::Fairy),
            "none" => Ok(PokemonTypesEnum::None),
            &_ => todo!(),
        }
    }
}

pub const POKEMON_TYPES_ARRAY: [PokemonTypesEnum; 18] = [
    PokemonTypesEnum::Normal,
    PokemonTypesEnum::Fire,
    PokemonTypesEnum::Water,
    PokemonTypesEnum::Electric,
    PokemonTypesEnum::Grass,
    PokemonTypesEnum::Ice,
    PokemonTypesEnum::Fighting,
    PokemonTypesEnum::Poison,
    PokemonTypesEnum::Ground,
    PokemonTypesEnum::Flying,
    PokemonTypesEnum::Psychic,
    PokemonTypesEnum::Bug,
    PokemonTypesEnum::Rock,
    PokemonTypesEnum::Ghost,
    PokemonTypesEnum::Dragon,
    PokemonTypesEnum::Dark,
    PokemonTypesEnum::Steel,
    PokemonTypesEnum::Fairy,
];
