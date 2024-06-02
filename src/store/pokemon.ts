import { writable } from "svelte/store";

export type Pokemon = {
  pokemon: { [key: number]: PokemonDetails };
};

export type PokemonDetails = {
  id: number;
  name: string;
  types: string[];
  abilities: string[];
  stats: Stats;
  moves: PokemonMoveSet;
  sprite: string;
  evolution: Evolution;
  forms: {
    [key: string]: PokemonForm;
  };
};

export type PokemonForm = {
  types: string[];
  abilities: string[];
  stats: Stats;
  sprite: string;
};

export type ModifiedPokemon = {
  [key: string]: {
    id: number;
    evolution: Evolution;
    types: {
      original: string[];
      modified: string[];
    };
  };
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
  level: number;
  item: string;
  other: string;
  evolves_to: {
    id: number;
    pokemon_name: string;
  };
  method: "no_change" | "level_up" | "item" | "other";
};

export type PokemonMoveSet = {
  [key: string]: PokemonMove;
};

export type PokemonMove = {
  level_learned: number;
  learn_method: string[];
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
];

export let pokemon = writable<Pokemon>({ pokemon: {} });
export let pokemonList = writable<[string, number][]>([]);
export let modifiedPokemon = writable<ModifiedPokemon>({});
