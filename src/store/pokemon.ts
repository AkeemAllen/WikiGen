import { writable } from "svelte/store";

export type SearchPokemon = {
  id: number;
  dex_number: number;
  name: string;
  types: string;
};

export type Pokemon = {
  id: number;
  dex_number: number;
  name: string;
  types: string;
  ability_1: string | null;
  ability_2: string | null;
  hidden_ability: string | null;
  hp: number;
  attack: number;
  defense: number;
  sp_attack: number;
  sp_defense: number;
  speed: number;
  evolution_method: "level_up" | "item" | "other" | "no_change";
  evolution_level: number | null;
  evolution_item: string | null;
  evolution_other: string | null;
  evolves_into: string | null;
  render: "true" | "false";
};

export type Ability = {
  id: number;
  name: string;
};

export type PokemonMove = {
  id: number;
  name: string;
  learn_method: string;
  level_learned: number;
};

export type MoveSetChange = {
  id: number;
  operation: string;
  move: string;
  method: string;
  level: number;
  secondaryMoveId: number | null | undefined;
  secondaryMove: string;
};

export type LearnMethod = "level-up" | "machine" | "tutor" | "egg";

export enum Operation {
  ADD = "add",
  SHIFT = "shift",
  DELETE = "delete",
  REPLACE_MOVE = "replace_move",
  // REPLACE_BY_LEVEL = "replace_by_level",
  // SWAP_MOVES = "swap_moves",
}

export type EvolutionChange = {};

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

export let pokemonList = writable<[number, number, string, string][]>([]);
export let pokemonUnderMoveModification = writable<{
  [key: string]: {
    currentMoves: PokemonMove[];
    sprite: string;
    movesToAdd: PokemonMove[];
  };
}>({});
