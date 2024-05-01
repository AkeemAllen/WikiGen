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

  async function writeRouteChangeToFile() {
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
    await writeRouteChangeToFile();
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
      class="block w-full pl-2 mt-2 rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6 disabled:bg-gray-100 disabled:text-gray-400"
      bind:value={pokemonName}
      use:popup={{
        event: "focus-click",
        target: "popupAutoComplete",
        placement: "bottom",
      }}
    />
    <div
      data-popup="popupAutoComplete"
      class="card w-60 mt-2 overflow-y-auto bg-white rounded-sm"
      tabindex="-1"
    >
      <Autocomplete
        bind:input={pokemonName}
        options={pokemonListOptions}
        limit={5}
        on:selection={onPokemonNameSelected}
        class="bg-white w-full text-sm border rounded-md p-2"
      />
    </div>
  </div>
  <NumberInput label="Encounter Rate" bind:value={encounterRate} />
  <div class="w-32 mt-8">
    <Button
      title="Add Encounter"
      disabled={pokemonName === "" || encounterRate === 0}
      onClick={addEncounter}
    />
  </div>
</div>

<div class="flex flex-col gap-y-5 mt-5">
  {#each Object.entries($routes.routes[routeName].wild_encounters ?? {}) as [_encounterType, encounters]}
    <div>
      <strong>
        {_.capitalize(_encounterType)} Encounters
      </strong>
      <div class="grid grid-cols-6 gap-5 mt-2">
      {#each encounters as encounter}
        <div class="grid card shadow-md p-2 !bg-transparent">
          <img src={$pokemon.pokemon[encounter.id].sprite} alt={encounter.name} class="m-0 justify-self-center"/>
          <div class="border-2 w-full rounded-md">
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
