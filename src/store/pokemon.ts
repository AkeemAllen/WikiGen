import { writable } from "svelte/store";

export type Pokemon = {
  pokemon: { [key: number]: PokemonData };
};

export type PokemonData = {
  id: number;
  name: string;
  types: string[];
  abilities: string[];
  stats: Stats;
  moves: Moves;
  sprite: string;
  evolution: Evolution;
};

export type Stats = {
  hp: number;
  attack: number;
  defense: number;
  sp_attack: number;
  sp_defense: number;
  speed: number;
};

export type Evolution = {
  level: number | null;
  item: string | null;
  other: string | null;
  evolves_to: string | null;
  method: string;
};

export type Moves = {
  [key: string]: Move;
};

export type Move = {
  level_learned: number;
  learn_method: string;
};

export const PokemonTypes = [
  "none",
  "normal",
  "fire",
  "water",
  "electric",
  "grass",
  "ice",
  "fighting",
  "poison",
  "ground",
  "flying",
  "psychic",
  "bug",
  "rock",
  "ghost",
  "dragon",
  "dark",
  "steel",
  "fairy", 
]

export let pokemon = writable<Pokemon>();
export let pokemonList = writable<[string, number][]>([]);
