<script lang="ts">
import { IconEdit, IconTrash } from "@tabler/icons-svelte";
import { type TrainerPokemon } from "../../store/gameRoutes";
import { pokemon as storePokemon } from "../../store/pokemon";
import _ from "lodash";
import BaseModal from "./BaseModal.svelte";
import AutoComplete from "./AutoComplete.svelte";
import MultiSelect from "svelte-multiselect";
import { BaseDirectory, writeTextFile } from "@tauri-apps/api/fs";
import { selectedWiki } from "../../store";
import { routes } from "../../store/gameRoutes";
import { abilitiesList } from "../../store/abilities";
import { naturesList } from "../../store/natures";
import { itemsList } from "../../store/items";
import { moveList } from "../../store/moves";
import { sortTrainersByPosition } from "$lib/utils";
import { invoke } from "@tauri-apps/api/tauri";

export let pokemon: TrainerPokemon;
export let trainerVersions: string[];
export let trainerName: string;
export let routeName: string;

export let editModalOpen: boolean = false;
export let deletePokemon = (id: string, name: string) => {};

let abilityListOptions = $abilitiesList.map((ability) => ({
  label: ability,
  value: ability,
}));

let natureListOptions = $naturesList.map((nature) => ({
  label: nature,
  value: nature,
}));

let itemListOptions = $itemsList.map((item) => ({
  label: item,
  value: item,
}));

async function generateRoutePage() {
  await invoke("generate_single_route_page_with_handle", {
    wikiName: $selectedWiki.name,
    routeName,
  }).catch((e) => {
    console.error(e);
  });
}

async function writeRouteToFile() {
  let sortedTrainers = sortTrainersByPosition($routes, routeName);
  $routes.routes[routeName].trainers = sortedTrainers;

  await writeTextFile(
    `${$selectedWiki.name}/data/routes.json`,
    JSON.stringify($routes),
    { dir: BaseDirectory.AppData },
  ).then(() => {
    generateRoutePage();
  });
}
</script>

<BaseModal bind:open={editModalOpen} class="grid w-[40rem] grid-cols-2 gap-5">
  <AutoComplete
    label="Ability"
    bind:value={pokemon.ability}
    options={abilityListOptions}
    popupId="ability-popup"
    onSelection={async (e) => {pokemon.ability = e.detail.value; writeRouteToFile()}}
  />
  <AutoComplete
    label="Nature"
    bind:value={pokemon.nature}
    options={natureListOptions}
    popupId="nature-popup"
    onSelection={async (e) => {pokemon.nature = e.detail.value; writeRouteToFile()}}
  />
  <AutoComplete
    label="Item"
    bind:value={pokemon.item}
    options={itemListOptions}
    popupId="item-popup"
    onSelection={async (e) => {pokemon.item = e.detail.value; writeRouteToFile()}}
  />
  <div>
    <label
      for="versions"
      class="mb-2 block text-sm font-medium leading-6 text-gray-900"
      >Trainer Versions</label
    >
    <MultiSelect
      id="versions"
      bind:selected={pokemon.trainer_versions}
      options={trainerVersions}
      on:change={async (e) => {writeRouteToFile()}}
      style="height: 36px; border-color: rgb(209 213 219); border-radius: 0.375rem; box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05); font-size: 0.875rem;"
    />
  </div>
  <div class="col-span-2">
    <label
      for="moveSet"
      class="mb-2 block text-sm font-medium leading-6 text-gray-900"
      >Moves</label
    >
    <MultiSelect
      id="moveSet"
      bind:selected={pokemon.moves}
      options={$moveList}
      maxSelect={4}
      on:change={async (e) => {writeRouteToFile()}}
      style="height: 36px; border-color: rgb(209 213 219); border-radius: 0.375rem; box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05); font-size: 0.875rem;"
    />
  </div>
</BaseModal>
<div
  class="group card relative grid !bg-transparent p-2 shadow-md hover:cursor-pointer"
>
  <img
    src={$storePokemon.pokemon[pokemon.id].sprite}
    alt={pokemon.name}
    class="m-0 justify-self-center"
  />
  <div class="w-full rounded-md border-2">
    <p class="text-center">
      {_.capitalize(pokemon.name)}
    </p>
    <p class="text-center">
      {pokemon.level}
    </p>
  </div>
  <button
    class="invisible absolute right-8 top-2 group-hover:visible"
    on:click={() => {
          editModalOpen = true;
        }}
  >
    <IconEdit size={16} color="grey" />
  </button>
  <button
    class="invisible absolute right-2 top-2 group-hover:visible"
    on:click={() => deletePokemon(pokemon.unique_id, trainerName)}
  >
    <IconTrash size={16} color="grey" />
  </button>
</div>
