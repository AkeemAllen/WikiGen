use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
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
    // TODO: Update field to be Vec<Ability> where Ability contains name and effect
    pub abilities: Vec<String>,
    pub stats: Stats,
    pub moves: HashMap<String, Move>,
    pub sprite: String,
    pub evolution: Evolution,
    pub forms: HashMap<String, PokemonForm>,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EvolvedPokemon {
    pub id: usize,
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
