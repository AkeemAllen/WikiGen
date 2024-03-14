import { writable } from "svelte/store";

type Wikis = {
    [key: string]: Wiki;
}

type Wiki = {
    name: string;
    description: string;
}

export let wikis = writable<Wikis>({} as Wikis);
export let selectedWiki = writable<Wiki>({name: "", description: ""} as Wiki);