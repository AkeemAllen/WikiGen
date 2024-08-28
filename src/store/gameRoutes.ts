import { writable } from "svelte/store";

export type Routes = {
  routes: { [key: string]: RouteProperties };
  encounter_areas: string[];
};

export type RouteProperties = {
  render: boolean;
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
  types: string[];
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
  encounter_area: string;
  route: string;
  special_note: string;
};

export let routes = writable<Routes>();
export let wildEncounters = writable<WildEncounter[]>();
// export let routesList = writable<>()
