<script lang="ts">
import { IconEdit, IconTrash } from "@tabler/icons-svelte";
import { type TrainerPokemon } from "../../store/gameRoutes";
import { pokemon as storePokemon } from "../../store/pokemon";
import _ from "lodash";
import BaseModal from "./BaseModal.svelte";
import TextInput from "./TextInput.svelte";
import AutoComplete from "./AutoComplete.svelte";
import MultiSelect from "svelte-multiselect";
import { BaseDirectory, writeTextFile } from "@tauri-apps/api/fs";
import { selectedWiki } from "../../store";
import { routes } from "../../store/gameRoutes";
import { abilitiesList } from "../../store/abilities";
import { naturesList } from "../../store/natures";
import { itemsList } from "../../store/items";

export let pokemon: TrainerPokemon;
export let trainerVersions: string[];
export let trainerName: string;

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

async function writeRouteToFile() {
  await writeTextFile(
    `${$selectedWiki.name}/data/routes.json`,
    JSON.stringify($routes),
    { dir: BaseDirectory.AppData },
  );
}
</script>

<BaseModal bind:open={editModalOpen} class="grid grid-cols-2 gap-5">
  <AutoComplete
    label="Ability"
    bind:value={pokemon.ability}
    placeholder="Ability"
    options={abilityListOptions}
    popupId="ability-popup"
    onSelection={async (e) => {pokemon.ability = e.detail.value; writeRouteToFile()}}
  />
  <AutoComplete
    label="Nature"
    bind:value={pokemon.nature}
    placeholder="Nature"
    options={natureListOptions}
    popupId="nature-popup"
    onSelection={async (e) => {pokemon.nature = e.detail.value; writeRouteToFile()}}
  />
  <AutoComplete
    label="Item"
    bind:value={pokemon.item}
    placeholder="Item"
    options={itemListOptions}
    popupId="item-popup"
    onSelection={async (e) => {pokemon.item = e.detail.value; writeRouteToFile()}}
  />
  <MultiSelect
    bind:selected={pokemon.trainer_versions}
    options={trainerVersions}
    on:change={async (e) => {await writeTextFile(
      `${$selectedWiki.name}/data/routes.json`,
      JSON.stringify($routes),
      { dir: BaseDirectory.AppData },
    )}}
  />
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
