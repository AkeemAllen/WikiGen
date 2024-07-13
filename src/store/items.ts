import { writable } from "svelte/store";

export type Item = {
  id: number;
  name: string;
  effect: string;
  is_modified: number;
  is_new: number;
};

export type SearchItem = {
  id: number;
  name: string;
};

export type ModifiedItems = {
  [key: string]: ModifiedItem;
};

export type ModifiedItem = {
  original: { effect: string; sprite: string };
  modified: { effect: string; sprite: string };
  is_new_item: boolean;
};

export let items = writable<Items>();
export let itemsList = writable<string[]>([]);
export let modifiedItems = writable<ModifiedItems>({});

export let dbItemsList = writable<[number, string][]>([]);
