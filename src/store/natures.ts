import { writable } from "svelte/store";

export type Nature = {
  id: number;
  name: string;
  increased_stat: string | undefined;
  decreased_stat: string | undefined;
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
