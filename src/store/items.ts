import { writable } from "svelte/store";

export type Item = {
  id: number;
  name: string;
  effect: string;
  category: string;
  is_modified: number;
  is_new: number;
};

export type ItemLocation = {
  id: number;
  item_name: string;
  route: string;
  specific_location: string | null;
  method: string | null;
  requirements: string | null;
};

export type SearchItem = {
  id: number;
  name: string;
};

export let itemsList = writable<[number, string][]>([]);
export let itemCategories = writable<string[]>([]);
