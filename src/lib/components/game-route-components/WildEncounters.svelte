<script lang="ts">
  import {
    getToastStore,
    popup,
    type AutocompleteOption,
  } from "@skeletonlabs/skeleton";
  import {
    BaseDirectory,
    readBinaryFile,
    writeTextFile,
  } from "@tauri-apps/api/fs";
  import { selectedWiki } from "../../../store";
  import { routes, type WildEncounter } from "../../../store/gameRoutes";
  import { pokemonList } from "../../../store/pokemon";
  import Button from "../Button.svelte";
  import NumberInput from "../NumberInput.svelte";
  import SelectInput from "../SelectInput.svelte";
  import { IconTrash } from "@tabler/icons-svelte";
  import AutoComplete from "../AutoComplete.svelte";
  import { invoke } from "@tauri-apps/api";
  import TextInput from "../TextInput.svelte";
  import BaseModal from "../BaseModal.svelte";
  import { shortcut } from "@svelte-put/shortcut";
  import { cloneDeep } from "$lib/utils/cloneDeep";
  import capitalizeWords from "$lib/utils/capitalizeWords";
  import isEqual from "$lib/utils/isEqual";

  export let routeName: string = "";
  let pokemonName: string = "";
  let encounterType: string = "grass-normal";
  let currentWildEncounterIndex: number;
  let currentEncounterType: string;
  let editEncounterModalOpen: boolean = false;
  let encounterRate: number = 0;
  let areaLevels = cloneDeep(
    $routes.routes[routeName].wild_encounter_area_levels,
  );
  let originalAreaLevels = cloneDeep(areaLevels);
  let routeWildEncounters: { [key: string]: WildEncounter[] } = cloneDeep(
    $routes.routes[routeName].wild_encounters,
  );
  let originalRouteWildEncounters = cloneDeep(routeWildEncounters);
  let pokemonListOptions: AutocompleteOption<string | number>[] =
    $pokemonList.map(([id, _, name]) => ({ label: name, value: id }));

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
    let searchedPokemon = $pokemonList.find(
      ([_, __, name]) => name === pokemonName.toLowerCase(),
    );

    if (searchedPokemon === undefined) {
      toastStore.trigger({
        message: "Pokemon not found",
        timeout: 3000,
        background: "variant-filled-error",
      });
      return;
    }

    routeWildEncounters = {
      ...routeWildEncounters,
      [encounterType]: [
        ...(routeWildEncounters[encounterType] ?? []),
        {
          id: searchedPokemon[0],
          name: pokemonName.toLowerCase(),
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

    routeWildEncounters = cloneDeep($routes.routes[routeName].wild_encounters);
    originalRouteWildEncounters = cloneDeep(routeWildEncounters);
    areaLevels = cloneDeep(
      $routes.routes[routeName].wild_encounter_area_levels,
    );
    originalAreaLevels = cloneDeep(areaLevels);

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

  async function getSpriteImage(pokemonName: string): Promise<string> {
    let sprite = "";
    await readBinaryFile(
      `${$selectedWiki.name}/dist/docs/img/pokemon/${pokemonName}.png`,
      { dir: BaseDirectory.AppData },
    )
      .then((res) => {
        const blob = new Blob([res], { type: "image/png" });
        sprite = URL.createObjectURL(blob);
      })
      .catch((err) => {
        console.log(err);
        if (err.includes("No such file or directory")) {
          sprite = "Image Not Found";
        }
        sprite = "Error loading image";
      });
    return sprite;
  }

  $: console.log(getSpriteImage(pokemonName));
</script>

<BaseModal bind:open={editEncounterModalOpen}>
  <NumberInput
    label="Encounter Rate"
    bind:value={routeWildEncounters[currentEncounterType][
      currentWildEncounterIndex
    ].encounter_rate}
    class="w-32"
    max={100}
  />
  <Button
    title="Save Changes"
    onClick={() => {
      routeWildEncounters[currentEncounterType]
        .sort(
          (encounter1, encounter2) =>
            encounter1.encounter_rate - encounter2.encounter_rate,
        )
        .reverse();
      saveChanges();
      editEncounterModalOpen = false;
    }}
    disabled={isEqual(routeWildEncounters, originalRouteWildEncounters) &&
      isEqual(areaLevels, originalAreaLevels)}
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
    disabled={isEqual(routeWildEncounters, originalRouteWildEncounters) &&
      isEqual(areaLevels, originalAreaLevels)}
    onClick={saveChanges}
  />
</div>

<div class="mt-5 flex flex-col gap-y-5">
  {#each Object.entries(routeWildEncounters) as [_encounterType, encounters]}
    <div>
      <strong>
        {capitalizeWords(_encounterType)} Encounters
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
            {#await getSpriteImage(encounter.name) then spriteUrl}
              <img
                src={spriteUrl}
                alt={encounter.name}
                class="m-0 justify-self-center"
              />
            {/await}
            <div class="w-full rounded-md border-2">
              <p class="text-center">
                {capitalizeWords(encounter.name)}
              </p>
              <p class="text-center">
                {encounter.encounter_rate}%
              </p>
            </div>
            <button
              class="invisible absolute right-2 top-2 z-20 rounded-md bg-red-200 p-1 hover:scale-110 group-hover:visible"
              on:click={(e) => {
                e.stopPropagation();
                deleteEncounter(encounter.name, _encounterType);
              }}
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
      key: "Enter",
      modifier: ["ctrl", "meta"],
      callback: () => {
        if (
          isEqual(routeWildEncounters, originalRouteWildEncounters) &&
          isEqual(areaLevels, originalAreaLevels)
        ) {
          return;
        }
        saveChanges();
      },
    },
  }}
/>
