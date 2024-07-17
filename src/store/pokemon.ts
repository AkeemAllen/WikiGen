import { writable } from "svelte/store";

export type SearchPokemon = {
  id: number;
  name: string;
};

export type Pokemon = {
  id: number;
  dex_number: number;
  name: string;
  type_1: string | null;
  type_2: string | null;
  ability_1: number | null;
  ability_2: number | null;
  hp: number;
  attack: number;
  defense: number;
  sp_attack: number;
  sp_defense: number;
  speed: number;
  evolution_change: number | null;
};

export type Moveset = {
  id: number;
  name: string;
  learn_method: string;
  level_learned: number;
};

export type EvolutionChange = {
  id: number;
  level: number | null;
  item: number | null;
  other: string | null;
  evolved_pokemon: number | null;
  method: "level_up" | "item" | "other" | "no_change";
};

export type PokemonMoveSet = {
  [key: string]: PokemonMove;
};

export type PokemonMove = {
  level_learned: number;
  learn_method: string[];
};

export const PokemonTypes = [
  null,
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
];

export let pokemonList = writable<[number, string][]>([]);
