<script lang="ts">
import { BaseDirectory, readTextFile } from "@tauri-apps/api/fs";
import { selectedWiki, wikis, type Wiki } from "../../store";
import { routes } from "../../store/gameRoutes";
import { moveList, moves } from "../../store/moves";
import { modifiedPokemon, pokemon, pokemonList } from "../../store/pokemon";
import { sortRoutesByPosition } from "$lib/utils";
import { abilities, abilitiesList } from "../../store/abilities";
import { natures, naturesList } from "../../store/natures";
import { items, itemsList } from "../../store/items";

async function loadWikiData(wiki: Wiki) {
  $selectedWiki = wiki;

  const abilitiesFromFile = await readTextFile(
    `${$selectedWiki.name}/data/abilities.json`,
    { dir: BaseDirectory.AppData },
  );
  abilities.set(JSON.parse(abilitiesFromFile));
  abilitiesList.set(Object.keys($abilities));

  const naturesFromFile = await readTextFile(
    `${$selectedWiki.name}/data/natures.json`,
    { dir: BaseDirectory.AppData },
  );
  natures.set(JSON.parse(naturesFromFile));
  naturesList.set(Object.keys($natures));

  const itemsFromFile = await readTextFile(
    `${$selectedWiki.name}/data/items.json`,
    { dir: BaseDirectory.AppData },
  );
  items.set(JSON.parse(itemsFromFile));
  itemsList.set(Object.keys($items));

  const pokemonFromFile = await readTextFile(
    `${$selectedWiki.name}/data/pokemon.json`,
    { dir: BaseDirectory.AppData },
  );
  pokemon.set(JSON.parse(pokemonFromFile));
  pokemonList.set(
    Object.entries($pokemon.pokemon).map(([key, value]) => [
      value.name,
      parseInt(key),
    ]),
  );
  const modifiedPokemonFromFile = await readTextFile(
    `${$selectedWiki.name}/data/modifications/modified_pokemon.json`,
    { dir: BaseDirectory.AppData },
  );
  modifiedPokemon.set(JSON.parse(modifiedPokemonFromFile));

  const movesFromFile = await readTextFile(
    `${$selectedWiki.name}/data/moves.json`,
    { dir: BaseDirectory.AppData },
  );
  moves.set(JSON.parse(movesFromFile));
  moveList.set(Object.keys($moves.moves));

  const routesFromFile = await readTextFile(
    `${$selectedWiki.name}/data/routes.json`,
    { dir: BaseDirectory.AppData },
  );
  routes.set(sortRoutesByPosition(JSON.parse(routesFromFile)));
}
</script>

<div
  class="card max-w-52 grid-cols-1 p-4 shadow-xl"
  data-popup="wikiSelectPopup"
>
  {#if Object.keys($wikis).length !== 0}
    <p class="mb-1 text-xs text-slate-400">Wikis</p>
    {#each Object.entries($wikis) as [_, value]}
      <button
        on:click={() => loadWikiData(value)}
        class="w-full rounded-md p-2 text-left text-sm hover:bg-slate-300"
        >{value.site_name}</button
      >
    {/each}
  {/if}
  <p class="mb-1 text-xs text-slate-400">Actions</p>
  <a href="/create-wiki">
    <button class="w-full rounded-md p-2 text-left text-sm hover:bg-slate-300"
      >Create New Wiki</button
    >
  </a>
</div>
