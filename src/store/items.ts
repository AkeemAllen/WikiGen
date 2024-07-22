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

export let itemsList = writable<[number, string][]>([]);
