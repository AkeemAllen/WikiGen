<script lang="ts">
import {
  BaseDirectory,
  readTextFile,
  removeDir,
  writeTextFile,
} from "@tauri-apps/api/fs";
import { selectedWiki, wikis, type Wiki } from "../../store";
import { routes } from "../../store/gameRoutes";
import { moveList, moves } from "../../store/moves";
import { modifiedPokemon, pokemon, pokemonList } from "../../store/pokemon";
import { sortRoutesByPosition } from "$lib/utils";
import { abilities, abilitiesList } from "../../store/abilities";
import { natures, naturesList } from "../../store/natures";
import { items, itemsList } from "../../store/items";
import BaseModal from "./BaseModal.svelte";
import MultiSelect from "svelte-multiselect";
import Button from "./Button.svelte";
import { getToastStore } from "@skeletonlabs/skeleton";

const toastStore = getToastStore();

let deleteWikiModalOpen: boolean = false;
let wikisToDelete: string[] = [];
let directoriesRemoved: boolean = false;
let wikiJsonUpdated: boolean = false;

$: wikiListOptions = Object.keys($wikis).filter(
  (wiki) => wiki !== $selectedWiki.name,
);

async function loadPokemonData() {
  for (let i = 1; i <= 10; i++) {
    const shard = await readTextFile(
      `${$selectedWiki.name}/data/pokemon_data/shard_${i}.json`,
      { dir: BaseDirectory.AppData },
    );
    let parsedShard = JSON.parse(shard);
    $pokemon.pokemon = {
      ...$pokemon.pokemon,
      ...parsedShard.pokemon,
    };
  }

  pokemonList.set(
    Object.entries($pokemon.pokemon).map(([key, value]) => [
      value.name,
      parseInt(key),
    ]),
  );
}

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

  loadPokemonData();

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

async function deleteWikis() {
  for (const wiki of wikisToDelete) {
    await removeDir(wiki, { dir: BaseDirectory.AppData, recursive: true }).then(
      () => {
        directoriesRemoved = true;
      },
    );
    $wikis = Object.fromEntries(
      Object.entries($wikis).filter(([key, _]) => key !== wiki),
    );
  }
  await writeTextFile("wikis.json", JSON.stringify($wikis), {
    dir: BaseDirectory.AppData,
  }).then(() => {
    wikiJsonUpdated = true;
  });
  if (directoriesRemoved && wikiJsonUpdated) {
    toastStore.trigger({
      message: "Wikis Deleted Successfully",
      background: "variant-filled-success",
    });
    directoriesRemoved = false;
    wikiJsonUpdated = false;
  }
  deleteWikiModalOpen = false;
  wikisToDelete = [];
}
</script>

<BaseModal bind:open={deleteWikiModalOpen}>
  <div>
    <label for="wikis" class="block text-sm font-medium leading-6 text-gray-900"
      >Wikis To Delete</label
    >
    <MultiSelect
      id="wikis"
      bind:selected={wikisToDelete}
      options={wikiListOptions}
      style="height: 36px; border-color: rgb(209 213 219); border-radius: 0.375rem; box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05); font-size: 0.875rem;"
    />
  </div>
  <Button onClick={() => deleteWikis()} title="Delete Selected Wikis" />
</BaseModal>

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
  <button
    class="w-full rounded-md p-2 text-left text-sm text-red-500 hover:bg-slate-300"
    on:click={() => deleteWikiModalOpen = true}>Delete A Wiki</button
  >
</div>
