import { writable } from "svelte/store";

export type Ability = {
  id: number;
  name: string;
  effect: string;
  is_modified: number;
  is_new: number;
};

export type SearchAbility = {
  id: number;
  name: string;
};

export let abilitiesList = writable<[number, string][]>([]);
