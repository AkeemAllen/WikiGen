import { writable } from "svelte/store";

export type Natures = {
  [key: string]: Nature;
};
export type Nature = {
  increased_stat: string | null;
  decreased_stat: string | null;
};

export type ModifiedNatures = {
  [key: string]: ModifiedNature;
};

export type ModifiedNature = {
  original: Nature;
  modified: Nature;
  is_new_nature: boolean;
};

export let natures = writable<Natures>();
export let naturesList = writable<string[]>([]);
export let modifiedNatures = writable<ModifiedNatures>({});
