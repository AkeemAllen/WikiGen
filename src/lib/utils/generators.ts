import { invoke } from "@tauri-apps/api";
import { ToastType, getToastSettings } from "./toasts";
import type { Routes } from "../../store/gameRoutes";
import { BaseDirectory, writeTextFile } from "@tauri-apps/api/fs";

export async function generatePokemonPages(
  pokemonIds: number[],
  wikiName: string,
) {
  return await invoke("generate_pokemon_pages_from_list", {
    pokemonIds: pokemonIds,
    wikiName,
  });
}

export async function removePokemonPage(
  wikiName: string,
  pokemonName: string,
  oldDexNumber: number,
) {
  return await invoke("remove_pokemon_page_with_old_dex_number", {
    wikiName,
    oldDexNumber,
    pokemonName,
  });
}

export async function generateRoutePages(
  routeNames: string[],
  wikiName: string,
) {
  return await invoke("generate_route_pages_with_handle", {
    routeNames,
    wikiName,
  });
}

export async function updateRoutes(routes: Routes, wikiName: string) {
  return writeTextFile(`${wikiName}/data/routes.json`, JSON.stringify(routes), {
    dir: BaseDirectory.AppData,
  }).catch((err) => {
    return err;
  });
}
