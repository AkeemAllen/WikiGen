<script lang="ts">
import { BaseDirectory, readTextFile } from "@tauri-apps/api/fs";
import { selectedWiki, wikis, type Wiki } from "../../store";
import { routes } from "../../store/gameRoutes";
import { moveList, moves } from "../../store/moves";
import { pokemon, pokemonList } from "../../store/pokemon";

async function loadWikiData(wiki: Wiki) {
  $selectedWiki = wiki;
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
  routes.set(JSON.parse(routesFromFile));
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
