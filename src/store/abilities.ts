import { writable } from "svelte/store";

export type Abilities = {
  [key: string]: Ability;
};

export type Ability = {
  effect: string;
};

export let abilities = writable<Abilities>();
export let abilitiesList = writable<string[]>([]);
