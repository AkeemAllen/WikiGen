import { writable } from "svelte/store";

export type Move = {
  id: number;
  name: string;
  accuracy: number | null;
  pp: number | null;
  power: number | null;
  type: string;
  damage_class: string;
  machine_name: string | null;
  is_modified: number;
  is_new: number;
};

export type SearchMove = {
  id: number;
  name: string;
};

export let moveList = writable<[number, string][]>([]);
