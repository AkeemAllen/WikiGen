import { writable } from "svelte/store";

export type Items = {
  [key: string]: Item;
};

export type Item = {
  effect: string;
  sprite: string;
};

export let items = writable<Items>();
export let itemsList = writable<string[]>([]);
