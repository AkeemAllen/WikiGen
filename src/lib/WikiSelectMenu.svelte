<script lang="ts">
  import { BaseDirectory, readTextFile } from "@tauri-apps/api/fs";
  import { selectedWiki, wikis, type Wiki } from "../store";
  import { pokemon, pokemonList } from "../store/pokemon";

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
  }
</script>

<div class="card p-4 w-40 shadow-xl flex flex-col" data-popup="wikiSelectPopup">
  {#if Object.keys($wikis).length !== 0}
    <p class="text-xs text-slate-400 mb-1">Wikis</p>
  {/if}
  {#each Object.entries($wikis) as [_, value]}
    <button
      on:click={() => loadWikiData(value)}
      class="w-30 text-sm text-left p-2 hover:bg-slate-300 rounded-md"
      >{value.site_name}</button
    >
  {/each}
  <p class="text-xs text-slate-400 mb-1">Actions</p>
  <a
    href="/create-wiki"
    class="w-30 text-sm text-left p-2 hover:bg-slate-300 rounded-md"
    >Create New Wiki</a
  >
</div>
