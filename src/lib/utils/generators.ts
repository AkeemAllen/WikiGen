import { invoke } from "@tauri-apps/api";
import { ToastType, getToastSettings } from "./toasts";

export async function generatePokemonPages(
  pokemonIds: number[],
  wikiName: string,
) {
  return await invoke("generate_pokemon_pages_from_list", {
    pokemonIds: pokemonIds,
    wikiName,
  });
}
