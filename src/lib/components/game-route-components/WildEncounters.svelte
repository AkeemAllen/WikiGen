<script lang="ts">
import {
  getToastStore,
  popup,
  type AutocompleteOption,
} from "@skeletonlabs/skeleton";
import { BaseDirectory, writeTextFile } from "@tauri-apps/api/fs";
import _ from "lodash";
import { selectedWiki } from "../../../store";
import { routes, type RouteProperties } from "../../../store/gameRoutes";
import { pokemonList, pokemon } from "../../../store/pokemon";
import Button from "../Button.svelte";
import NumberInput from "../NumberInput.svelte";
import SelectInput from "../SelectInput.svelte";
import { IconTrash } from "@tabler/icons-svelte";
import AutoComplete from "../AutoComplete.svelte";

export let routeName: string = "";
let pokemonName: string = "";
let encounterType: string = "grass-normal";
let encounterRate: number = 0;
let pokemonListOptions: AutocompleteOption<string | number>[] =
  $pokemonList.map(([name, id]) => ({ label: name, value: id }));

const encounterTypes = $routes.encounter_types.map((type) => ({
  label: type,
  value: type,
}));

const toastStore = getToastStore();
function onPokemonNameSelected(
  event: CustomEvent<AutocompleteOption<string | number>>,
): void {
  pokemonName = event.detail.label;
}

async function addEncounter() {
  $routes.routes[routeName].wild_encounters = {
    ...$routes.routes[routeName].wild_encounters,
    [encounterType]: [
      ...($routes.routes[routeName].wild_encounters[encounterType] ?? []),
      {
        id: $pokemonList.find(
          ([name, _]) => name === pokemonName,
        )?.[1] as number,
        name: pokemonName,
        encounter_rate: encounterRate,
      },
    ],
  };
  await writeTextFile(
    `${$selectedWiki.name}/data/routes.json`,
    JSON.stringify($routes),
    { dir: BaseDirectory.AppData },
  );
}

async function deleteEncounter(pokemonName: string, encounterType: string) {
  let updatedEncounters = {
    ...$routes.routes[routeName].wild_encounters,
  };
  updatedEncounters[encounterType] = updatedEncounters[encounterType].filter(
    (encounter) => encounter.name !== pokemonName,
  );
  if (updatedEncounters[encounterType].length === 0) {
    delete updatedEncounters[encounterType];
  }

  $routes.routes[routeName].wild_encounters = updatedEncounters;

  await writeTextFile(
    `${$selectedWiki.name}/data/routes.json`,
    JSON.stringify($routes),
    { dir: BaseDirectory.AppData },
  );
}
</script>

<div class="flex flex-row gap-x-5">
  <div class="w-40">
    <SelectInput
      id="encounter-type"
      label="Encounter Type"
      bind:value={encounterType}
      options={encounterTypes}
    />
  </div>

  <AutoComplete
    label="Pokemon for current encounter type"
    placeholder="Pokemon Name"
    bind:value={pokemonName}
    options={pokemonListOptions}
    popupId="popupAutoComplete"
    onSelection={onPokemonNameSelected}
  />
  <NumberInput label="Encounter Rate" bind:value={encounterRate} />
  <Button
    class="mt-8 w-32"
    title="Add Encounter"
    disabled={pokemonName === "" || encounterRate === 0}
    onClick={addEncounter}
  />
</div>

<div class="mt-5 flex flex-col gap-y-5">
  {#each Object.entries($routes.routes[routeName].wild_encounters) as [_encounterType, encounters]}
    <div>
      <strong>
        {_.capitalize(_encounterType)} Encounters
      </strong>
      <div class="mt-2 grid grid-cols-6 gap-5">
        {#each encounters as encounter}
          <div
            class="group card relative grid !bg-transparent p-2 shadow-md hover:cursor-pointer"
          >
            <img
              src={$pokemon.pokemon[encounter.id].sprite}
              alt={encounter.name}
              class="m-0 justify-self-center"
            />
            <div class="w-full rounded-md border-2">
              <p class="text-center">
                {_.capitalize(encounter.name)}
              </p>
              <p class="text-center">
                {encounter.encounter_rate}%
              </p>
            </div>
            <button
              class="invisible absolute right-2 top-2 group-hover:visible"
              on:click={() => deleteEncounter(encounter.name, _encounterType)}
            >
              <IconTrash size={16} color="grey" />
            </button>
          </div>
        {/each}
      </div>
      <div></div>
    </div>
  {/each}
</div>
