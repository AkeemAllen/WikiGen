import { writable } from "svelte/store";

export type Nature = {
  id: number;
  name: string;
  increased_stat: Stat | null;
  decreased_stat: Stat | null;
  is_modified: number;
  is_new: number;
};

export enum Stat {
  "attack",
  "defense",
  "special-attack",
  "special-defense",
  "speed",
}

export type SearchNature = {
  id: number;
  name: string;
};

export let naturesList = writable<[number, string][]>([]);
