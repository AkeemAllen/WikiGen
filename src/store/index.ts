import { writable } from "svelte/store";

type Wikis = {
  [key: string]: Wiki;
};

export type Wiki = {
  name: string;
  description: string;
  author: string;
  site_name: string;
  repo_url: string;
  site_url: string;
  settings: WikiSettings;
};

export type WikiSettings = {
  deployment_url: string;
};

export type User = {
  userName: string;
  avatarUrl: string;
  isConnected: boolean;
};

export let wikis = writable<Wikis>({} as Wikis);
export let selectedWiki = writable<Wiki>({ name: "" } as Wiki);
export let token = writable<string>("");
export let user = writable<User>({
  userName: "",
  avatarUrl: "",
  isConnected: false,
});
