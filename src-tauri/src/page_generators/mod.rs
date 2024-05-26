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

pub mod evolution_page;
pub mod game_routes;
pub mod pokemon_pages;
pub mod type_page;
