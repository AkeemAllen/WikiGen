import { writable } from "svelte/store";

export let types = writable<string[]>([]);
