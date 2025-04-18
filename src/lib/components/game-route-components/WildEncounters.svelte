<script lang="ts">
  import {
    getToastStore,
    type AutocompleteOption,
  } from "@skeletonlabs/skeleton";
  import { BaseDirectory, readFile } from "@tauri-apps/plugin-fs";
  import { selectedWiki } from "../../../store";
  import { routes, type WildEncounter } from "../../../store/gameRoutes";
  import { pokemonList } from "../../../store/pokemon";
  import Button from "../Button.svelte";
  import NumberInput from "../NumberInput.svelte";
  import SelectInput from "../SelectInput.svelte";
  import IconTrash from "@tabler/icons-svelte/icons/trash";
  import AutoComplete from "../AutoComplete.svelte";
  import TextInput from "../TextInput.svelte";
  import BaseModal from "../BaseModal.svelte";
  import { shortcut } from "@svelte-put/shortcut";
  import { cloneDeep } from "$lib/utils/cloneDeep";
  import capitalizeWords from "$lib/utils/capitalizeWords";
  import isEqual from "$lib/utils/isEqual";
  import WildEncounterAreaMenu from "../modals/WildEncounterAreaMenu.svelte";
  import { generateRoutePages, updateRoutes } from "$lib/utils/generators";
  import { getToastSettings, ToastType } from "$lib/utils/toasts";

  interface Props {
    routeName?: string;
  }

  let { routeName = $bindable("") }: Props = $props();
  let pokemonName: string = $state("");
  let encounterArea: string = $state("grass");
  let currentWildEncounterIndex: number = $state(0);
  let currentEncounterType: string = $state("");
  let editEncounterModalOpen: boolean = $state(false);
  let encounterRate: number = $state(0);
  let areaLevels = $state(
    cloneDeep($routes.routes[routeName].wild_encounter_area_levels),
  );
  let originalAreaLevels = $state(
    cloneDeep($routes.routes[routeName].wild_encounter_area_levels),
  );
  let routeWildEncounters: { [key: string]: WildEncounter[] } = $state(
    cloneDeep($routes.routes[routeName].wild_encounters),
  );
  let originalRouteWildEncounters = $state(
    cloneDeep($routes.routes[routeName].wild_encounters),
  );
  let pokemonListOptions: AutocompleteOption<string | number>[] =
    $pokemonList.map(([id, _, name]) => ({
      label: capitalizeWords(name),
      value: id,
    }));

  const encounterAreas = $routes.encounter_areas.map((type) => ({
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
      ([_, __, name]) =>
        name === pokemonName.toLowerCase().replaceAll(" ", "-"),
    );

    if (searchedPokemon === undefined) {
      toastStore.trigger(
        getToastSettings(ToastType.ERROR, "Pokemon not found"),
      );
      return;
    }

    routeWildEncounters = {
      ...routeWildEncounters,
      [encounterArea]: [
        ...(routeWildEncounters[encounterArea] ?? []),
        {
          id: searchedPokemon[1],
          name: pokemonName.toLowerCase().replaceAll(" ", "-"),
          encounter_rate: encounterRate,
          encounter_area: encounterArea,
          special_note: "",
          route: routeName,
        },
      ],
    };

    routeWildEncounters[encounterArea]
      .sort(
        (encounter1, encounter2) =>
          encounter1.encounter_rate - encounter2.encounter_rate,
      )
      .reverse();
  }

  async function deleteEncounter(pokemonName: string, encounterArea: string) {
    editEncounterModalOpen = false;
    let updatedEncounters = {
      ...routeWildEncounters,
    };
    updatedEncounters[encounterArea] = updatedEncounters[encounterArea].filter(
      (encounter) => encounter.name !== pokemonName,
    );
    if (updatedEncounters[encounterArea].length === 0) {
      delete updatedEncounters[encounterArea];
    }

    if (updatedEncounters[encounterArea] !== undefined) {
      updatedEncounters[encounterArea]
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

    await updateRoutes($routes, $selectedWiki.name)
      .then(() => {
        generateRoutePages([routeName], $selectedWiki.name)
          .then((res) => {
            toastStore.trigger(
              getToastSettings(ToastType.SUCCESS, res as string),
            );
          })
          .catch((e) => {
            toastStore.trigger(getToastSettings(ToastType.ERROR, e as string));
          });
      })
      .catch((e) => {
        toastStore.trigger(getToastSettings(ToastType.ERROR, e as string));
      });
  }

  async function getSpriteImage(pokemonName: string): Promise<string> {
    let sprite = "";
    let spriteName = pokemonName.toLowerCase().replaceAll(" ", "-");
    await readFile(
      `${$selectedWiki.name}/dist/docs/img/pokemon/${spriteName}.png`,
      { baseDir: BaseDirectory.AppData },
    )
      .then((res) => {
        const blob = new Blob([res], { type: "image/png" });
        sprite = URL.createObjectURL(blob);
      })
      .catch((err) => {
        if (err.includes("No such file or directory")) {
          sprite = "Image Not Found";
        }
        sprite = "Error loading image";
      });
    return sprite;
  }
</script>

<BaseModal bind:open={editEncounterModalOpen}>
  <NumberInput
    label="Encounter Rate"
    bind:value={
      routeWildEncounters[currentEncounterType][currentWildEncounterIndex]
        .encounter_rate
    }
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
      id="encounter-area"
      label="Encounter Area"
      bind:value={encounterArea}
      options={encounterAreas}
    />
  </div>

  <AutoComplete
    label="Pokemon for current encounter area"
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
    disabled={pokemonName === ""}
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
  {#each Object.entries(routeWildEncounters) as [_encounterType, encounters], index}
    <div>
      <strong class="flex flex-row items-center gap-x-4">
        {capitalizeWords(_encounterType)} Encounters
        <WildEncounterAreaMenu
          {index}
          encounterArea={_encounterType}
          {routeName}
        />
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
            onclick={() => {
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
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <a
              class="invisible absolute right-2 top-2 z-20 rounded-md bg-red-200 p-1 hover:scale-110 group-hover:visible"
              type="button"
              onclick={(e) => {
                e.stopPropagation();
                deleteEncounter(encounter.name, _encounterType);
              }}
            >
              <IconTrash size={16} color="grey" />
            </a>
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
