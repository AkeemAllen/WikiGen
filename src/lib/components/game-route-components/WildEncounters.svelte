<script lang="ts">
import {
  getToastStore,
  popup,
  type AutocompleteOption,
} from "@skeletonlabs/skeleton";
import { BaseDirectory, writeTextFile } from "@tauri-apps/api/fs";
import _ from "lodash";
import { selectedWiki } from "../../../store";
import { routes, type WildEncounter } from "../../../store/gameRoutes";
import { pokemonList, pokemon } from "../../../store/pokemon";
import Button from "../Button.svelte";
import NumberInput from "../NumberInput.svelte";
import SelectInput from "../SelectInput.svelte";
import { IconTrash } from "@tabler/icons-svelte";
import AutoComplete from "../AutoComplete.svelte";
import { invoke } from "@tauri-apps/api";
import TextInput from "../TextInput.svelte";
import BaseModal from "../BaseModal.svelte";
import { shortcut } from "@svelte-put/shortcut";

export let routeName: string = "";
let pokemonName: string = "";
let encounterType: string = "grass-normal";
let currentWildEncounterIndex: number;
let currentEncounterType: string;
let editEncounterModalOpen: boolean = false;
let encounterRate: number = 0;
let areaLevels = _.cloneDeep(
  $routes.routes[routeName].wild_encounter_area_levels,
);
let originalAreaLevels = _.cloneDeep(areaLevels);
let routeWildEncounters = _.cloneDeep(
  $routes.routes[routeName].wild_encounters,
);
let originalRouteWildEncounters = _.cloneDeep(routeWildEncounters);
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
  routeWildEncounters = {
    ...routeWildEncounters,
    [encounterType]: [
      ...(routeWildEncounters[encounterType] ?? []),
      {
        id: $pokemonList.find(
          ([name, _]) => name === pokemonName,
        )?.[1] as number,
        name: pokemonName,
        encounter_rate: encounterRate,
      },
    ],
  };
  routeWildEncounters[encounterType]
    .sort(
      (encounter1, encounter2) =>
        encounter1.encounter_rate - encounter2.encounter_rate,
    )
    .reverse();
}

async function deleteEncounter(pokemonName: string, encounterType: string) {
  editEncounterModalOpen = false;
  let updatedEncounters = {
    ...routeWildEncounters,
  };
  updatedEncounters[encounterType] = updatedEncounters[encounterType].filter(
    (encounter) => encounter.name !== pokemonName,
  );
  if (updatedEncounters[encounterType].length === 0) {
    delete updatedEncounters[encounterType];
  }

  if (updatedEncounters[encounterType] !== undefined) {
    updatedEncounters[encounterType]
      .sort(
        (encounter1, encounter2) =>
          encounter1.encounter_rate - encounter2.encounter_rate,
      )
      .reverse();
  }

  routeWildEncounters = updatedEncounters;
}

async function saveChanges() {
  $routes.routes[routeName].wild_encounters = routeWildEncounters;
  $routes.routes[routeName].wild_encounter_area_levels = areaLevels;

  routeWildEncounters = _.cloneDeep($routes.routes[routeName].wild_encounters);
  originalRouteWildEncounters = _.cloneDeep(routeWildEncounters);
  areaLevels = _.cloneDeep(
    $routes.routes[routeName].wild_encounter_area_levels,
  );
  originalAreaLevels = _.cloneDeep(areaLevels);

  await writeTextFile(
    `${$selectedWiki.name}/data/routes.json`,
    JSON.stringify($routes),
    { dir: BaseDirectory.AppData },
  ).then(() => {
    invoke("generate_single_route_page_with_handle", {
      wikiName: $selectedWiki.name,
      routeName,
    })
      .then(() => {
        toastStore.trigger({
          message: "Changes saved successfully",
          timeout: 3000,
          background: "variant-filled-success",
        });
      })
      .catch((e) => {
        console.error(e);
      });
  });
}
</script>

<BaseModal bind:open={editEncounterModalOpen}>
  <NumberInput
    label="Encounter Rate"
    bind:value={routeWildEncounters[currentEncounterType][currentWildEncounterIndex].encounter_rate}
    class="w-32"
    max={100}
  />
  <Button
    title="Save Changes"
    onClick={() => {
      routeWildEncounters[currentEncounterType].sort((encounter1, encounter2) => encounter1.encounter_rate - encounter2.encounter_rate).reverse();
            saveChanges();
            editEncounterModalOpen = false;
        }}
    disabled={_.isEqual(routeWildEncounters, originalRouteWildEncounters) && _.isEqual(areaLevels, originalAreaLevels)}
  />
</BaseModal>

<div
  class="sticky top-0 z-50 flex flex-row gap-x-5 rounded-md bg-white pb-1 shadow-sm"
>
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
    showChevron={false}
  />
  <NumberInput
    label="Encounter Rate"
    bind:value={encounterRate}
    class="w-32"
    max={100}
  />
  <Button
    class="mt-8 w-32"
    title="Add Encounter"
    disabled={pokemonName === "" || encounterRate === 0}
    onClick={addEncounter}
  />
  <Button
    class="mt-8 w-32"
    title="Save Changes"
    disabled={_.isEqual(routeWildEncounters, originalRouteWildEncounters) && _.isEqual(areaLevels, originalAreaLevels)}
    onClick={saveChanges}
  />
</div>

<div class="mt-5 flex flex-col gap-y-5">
  {#each Object.entries(routeWildEncounters) as [_encounterType, encounters]}
    <div>
      <strong>
        {_.capitalize(_encounterType)} Encounters
      </strong>
      <TextInput
        bind:value={areaLevels[_encounterType]}
        placeholder="Lv."
        class="w-20"
      />
      <div class="mt-2 grid grid-cols-6 gap-5">
        {#each encounters as encounter, index}
          <button
            class="group card relative grid !bg-transparent p-2 shadow-md transition ease-in-out hover:scale-110 hover:cursor-pointer"
            on:click={() => {
                editEncounterModalOpen = true;
                currentEncounterType = _encounterType;
                currentWildEncounterIndex = index;
              }}
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
              class="invisible absolute right-2 top-2 z-20 rounded-md bg-red-200 p-1 hover:scale-110 group-hover:visible"
              on:click={(e) => {e.stopPropagation() ;deleteEncounter(encounter.name, _encounterType)}}
            >
              <IconTrash size={16} color="grey" />
            </button>
          </button>
        {/each}
      </div>
      <div></div>
    </div>
  {/each}
</div>

<svelte:window
  use:shortcut={{
    trigger: {
      key: 'Enter',
      modifier: ["ctrl", "meta"],
      callback: () => {
        if (_.isEqual(routeWildEncounters, originalRouteWildEncounters) && _.isEqual(areaLevels, originalAreaLevels)) {
          return;
        }
        saveChanges();
      }
    },
  }}
/>
