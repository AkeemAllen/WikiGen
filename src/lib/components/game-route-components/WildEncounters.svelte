<script lang="ts">
import {
  getToastStore,
  popup,
  type AutocompleteOption,
} from "@skeletonlabs/skeleton";
import { BaseDirectory, writeTextFile } from "@tauri-apps/api/fs";
import _ from "lodash";
import { selectedWiki } from "../../../store";
import { routes } from "../../../store/gameRoutes";
import { pokemonList, pokemon } from "../../../store/pokemon";
import Button from "../Button.svelte";
import NumberInput from "../NumberInput.svelte";
import SelectInput from "../SelectInput.svelte";
import { IconTrash } from "@tabler/icons-svelte";
import AutoComplete from "../AutoComplete.svelte";
import { invoke } from "@tauri-apps/api";
import TextInput from "../TextInput.svelte";

export let routeName: string = "";
let pokemonName: string = "";
let encounterType: string = "grass-normal";
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
}

async function deleteEncounter(pokemonName: string, encounterType: string) {
  let updatedEncounters = {
    ...routeWildEncounters,
  };
  updatedEncounters[encounterType] = updatedEncounters[encounterType].filter(
    (encounter) => encounter.name !== pokemonName,
  );
  if (updatedEncounters[encounterType].length === 0) {
    delete updatedEncounters[encounterType];
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
