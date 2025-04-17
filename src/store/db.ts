import type Database from "@tauri-apps/plugin-sql";
import { writable } from "svelte/store";

export let db = writable<Database>({} as Database);
