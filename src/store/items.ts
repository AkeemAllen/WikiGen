import { writable } from "svelte/store";

export type Items = {
  [key: string]: Item;
};

export type Item = {
  effect: string;
  sprite: string;
};

export type ModifiedItems = {
  [key: string]: ModifiedItem;
};

export type ModifiedItem = {
  original: { effect: string };
  modified: { effect: string };
};

export let items = writable<Items>();
export let itemsList = writable<string[]>([]);
export let modifiedItems = writable<ModifiedItems>({});
