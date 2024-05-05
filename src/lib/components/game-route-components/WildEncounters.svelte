<script lang="ts">
import {
  Autocomplete,
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
  let encounterTypesObjects = {
    ...$routes.routes[routeName].wild_encounters,
  };

  $routes.routes[routeName].wild_encounters = {
    ...$routes.routes[routeName].wild_encounters,
    [encounterType]: [
      ...(encounterTypesObjects[encounterType] ?? []),
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
  ).then(() => {
    toastStore.trigger({
      message: "Encounter Added",
      background: "variant-filled-success",
    });
  });
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
  <div class="w-60">
    <label
      for="pokemon-name"
      class="block text-sm font-medium leading-6 text-gray-900"
    >
      Pokemon for current encounter type
    </label>
    <input
      id="pokemon-name"
      type="text"
      placeholder="Pokemon Name"
      class="mt-2 block w-full rounded-md border-0 py-1.5 pl-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 disabled:bg-gray-100 disabled:text-gray-400 sm:text-sm sm:leading-6"
      bind:value={pokemonName}
      use:popup={{
        event: "focus-click",
        target: "popupAutoComplete",
        placement: "bottom",
      }}
    />
    <div
      data-popup="popupAutoComplete"
      class="card mt-2 w-60 overflow-y-auto rounded-sm bg-white"
      tabindex="-1"
    >
      <Autocomplete
        bind:input={pokemonName}
        options={pokemonListOptions}
        limit={5}
        on:selection={onPokemonNameSelected}
        class="w-full rounded-md border bg-white p-2 text-sm"
      />
    </div>
  </div>
  <NumberInput label="Encounter Rate" bind:value={encounterRate} />
  <div class="mt-8 w-32">
    <Button
      title="Add Encounter"
      disabled={pokemonName === "" || encounterRate === 0}
      onClick={addEncounter}
    />
  </div>
</div>

<div class="mt-5 flex flex-col gap-y-5">
  {#each Object.entries($routes.routes[routeName].wild_encounters ?? {}) as [_encounterType, encounters]}
    <div>
      <strong>
        {_.capitalize(_encounterType)} Encounters
      </strong>
      <div class="mt-2 grid grid-cols-6 gap-5">
        {#each encounters as encounter}
          <div class="card grid !bg-transparent p-2 shadow-md">
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
          </div>
        {/each}
      </div>
      <div></div>
    </div>
  {/each}
</div>
