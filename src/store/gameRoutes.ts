import { writable } from "svelte/store";

export type Routes = {
  routes: { [key: string]: RouteProperties };
  encounter_types: string[];
};

export type RouteProperties = {
  position: number;
  trainers: {
    [key: string]: TrainerInfo;
  };
  wild_encounters: {
    [key: string]: WildEncounter[];
  };
  wild_encounter_area_levels: {
    [key: string]: string;
  };
};

export type TrainerInfo = {
  position: number;
  pokemon_team: TrainerPokemon[];
  sprite: string;
  versions: string[];
};

export type TrainerPokemon = {
  id: number;
  unique_id: string;
  name: string;
  level: number;
  moves: string[];
  item: string;
  nature: string;
  ability: string;
  trainer_versions: string[];
};

export type WildEncounter = {
  id: number;
  name: string;
  encounter_rate: number;
};

export let routes = writable<Routes>();
// export let routesList = writable<>()
