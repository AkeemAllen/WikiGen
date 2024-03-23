import { writable } from "svelte/store";

export type Pokemon = {
    [key: string]: PokemonData;
}

export type PokemonData = {
  id: number;
  name: string;
  types: string[];
  abilities: string[];
  stats: Stats;
  moves: Moves;
  sprite: string;
  evolution?: Evolution;
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
    level?: number;
    item?: string;
    other?: string;
    evolves_to: string;
};

export type Moves = {
    [key: string]: Move;
}

export type Move = {
    level_learned: number;
    learn_method: string;
}

export let pokemon = writable<Pokemon>();