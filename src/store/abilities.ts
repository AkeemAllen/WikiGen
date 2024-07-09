import { writable } from "svelte/store";

export type Abilities = {
  [key: string]: Ability;
};

export type Ability = {
  effect: string;
};

export type ModifiedAbilities = {
  [key: string]: ModifiedAbility;
};

export type ModifiedAbility = {
  original: { effect: string };
  modified: { effect: string };
  is_new_ability: boolean;
};

export let abilities = writable<Abilities>();
export let abilitiesList = writable<string[]>([]);
export let modifiedAbilities = writable<ModifiedAbilities>({});
