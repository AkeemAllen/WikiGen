import { writable } from "svelte/store";

export type Natures = {
  [key: string]: Nature;
};
export type Nature = {
  increased_stat: string | null;
  decreased_stat: string | null;
};

export let natures = writable<Natures>();
export let naturesList = writable<string[]>([]);
