import type Database from "tauri-plugin-sql-api";
import { writable } from "svelte/store";

export let db = writable<Database>({} as Database);
