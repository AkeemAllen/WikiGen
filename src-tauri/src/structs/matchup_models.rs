use serde::{Deserialize, Serialize};

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

pub const POKEMON_TYPES_ARRAY: [&str; 18] = [
    "normal", "fire", "water", "electric", "grass", "ice", "fighting", "poison", "ground",
    "flying", "psychic", "bug", "rock", "ghost", "dragon", "dark", "steel", "fairy",
];

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum NumberTypes {
    I32(i32),
    F32(f32),
}
// const N: i32 = 1;
// const H: i32 = 1 / 2;
// const X: f32 = f32::NAN;
const N: NumberTypes = NumberTypes::I32(1);
const H: NumberTypes = NumberTypes::I32(1 / 2);
const X: NumberTypes = NumberTypes::F32(f32::NAN);
const D: NumberTypes = NumberTypes::I32(2);
const Z: NumberTypes = NumberTypes::I32(0);

pub const GEN_DEFAULT: [[NumberTypes; 18]; 18] = [
    [N, N, N, N, N, N, N, N, N, N, N, N, H, Z, N, N, H, N],
    [N, H, H, N, D, D, N, N, N, N, N, D, H, N, H, N, D, N],
    [N, D, H, N, H, N, N, N, D, N, N, N, D, N, H, N, N, N],
    [N, N, D, H, H, N, N, N, Z, D, N, N, N, N, H, N, N, N],
    [N, H, D, N, H, N, N, H, D, H, N, H, D, N, H, N, H, N],
    [N, H, H, N, D, H, N, N, D, D, N, N, N, N, D, N, H, N],
    [D, N, N, N, N, D, N, H, N, H, H, H, D, Z, N, D, D, H],
    [N, N, N, N, D, N, N, H, H, N, N, N, H, H, N, N, Z, D],
    [N, D, N, D, H, N, N, D, N, Z, N, H, D, N, N, N, D, N],
    [N, N, N, H, D, N, D, N, N, N, N, D, H, N, N, N, H, N],
    [N, N, N, N, N, N, D, D, N, N, H, N, N, N, N, Z, H, N],
    [N, H, N, N, D, N, H, H, N, H, D, N, N, H, N, D, H, H],
    [N, D, N, N, N, D, H, N, H, D, N, D, N, N, N, N, H, N],
    [Z, N, N, N, N, N, N, N, N, N, D, N, N, D, N, H, N, N],
    [N, N, N, N, N, N, N, N, N, N, N, N, N, N, D, N, H, Z],
    [N, N, N, N, N, N, H, N, N, N, D, N, N, D, N, H, N, H],
    [N, H, H, H, N, D, N, N, N, N, N, N, D, N, N, N, H, D],
    [N, H, N, N, N, N, D, H, N, N, N, N, N, N, D, D, H, N],
];

pub const GENERATION_ONE: [[NumberTypes; 18]; 18] = [
    [N, N, N, N, N, N, N, N, N, N, N, N, H, Z, N, X, X, X],
    [N, H, H, N, D, D, N, N, N, N, N, D, H, N, H, X, X, X],
    [N, D, H, N, H, N, N, N, D, N, N, N, D, N, H, X, X, X],
    [N, N, D, H, H, N, N, N, Z, D, N, N, N, N, H, X, X, X],
    [N, H, D, N, H, N, N, H, D, H, N, H, D, N, H, X, X, X],
    [N, N, H, N, D, H, N, N, D, D, N, N, N, N, D, X, X, X],
    [D, N, N, N, N, D, N, H, N, H, H, H, D, Z, N, X, X, X],
    [N, N, N, N, D, N, N, H, H, N, N, D, H, H, N, X, X, X],
    [N, D, N, D, H, N, N, D, N, Z, N, H, D, N, N, X, X, X],
    [N, N, N, H, D, N, D, N, N, N, N, D, H, N, N, X, X, X],
    [N, N, N, N, N, N, D, D, N, N, H, N, N, N, N, X, X, X],
    [N, H, N, N, D, N, H, D, N, H, D, N, N, H, N, X, X, X],
    [N, D, N, N, N, D, H, N, H, D, N, D, N, N, N, X, X, X],
    [Z, N, N, N, N, N, N, N, N, N, Z, N, N, D, N, X, X, X],
    [N, N, N, N, N, N, N, N, N, N, N, N, N, N, D, X, X, X],
    [X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X],
    [X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X],
    [X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X],
];

pub const GENERATION_TWO: [[NumberTypes; 18]; 18] = [
    [N, N, N, N, N, N, N, N, N, N, N, N, H, Z, N, N, H, X],
    [N, H, H, N, D, D, N, N, N, N, N, D, H, N, H, N, D, X],
    [N, D, H, N, H, N, N, N, D, N, N, N, D, N, H, N, N, X],
    [N, N, D, H, H, N, N, N, Z, D, N, N, N, N, H, N, N, X],
    [N, H, D, N, H, N, N, H, D, H, N, H, D, N, H, N, H, X],
    [N, H, H, N, D, H, N, N, D, D, N, N, N, N, D, N, H, X],
    [D, N, N, N, N, D, N, H, N, H, H, H, D, Z, N, D, D, X],
    [N, N, N, N, D, N, N, H, H, N, N, N, H, H, N, N, Z, X],
    [N, D, N, D, H, N, N, D, N, Z, N, H, D, N, N, N, D, X],
    [N, N, N, H, D, N, D, N, N, N, N, D, H, N, N, N, H, X],
    [N, N, N, N, N, N, D, D, N, N, H, N, N, N, N, Z, H, X],
    [N, H, N, N, D, N, H, H, N, H, D, N, N, H, N, D, H, X],
    [N, D, N, N, N, D, H, N, H, D, N, D, N, N, N, N, H, X],
    [Z, N, N, N, N, N, N, N, N, N, D, N, N, D, N, H, H, X],
    [N, N, N, N, N, N, N, N, N, N, N, N, N, N, D, N, H, X],
    [N, N, N, N, N, N, H, N, N, N, D, N, N, D, N, H, H, X],
    [N, H, H, H, N, D, N, N, N, N, N, N, D, N, N, N, H, X],
    [X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X],
];
