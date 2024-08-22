<script lang="ts">
  import { DataHandler, Pagination } from "@vincjo/datatables";
  import ThSort from "$lib/components/ThSort.svelte";
  import SelectInput from "$lib/components/SelectInput.svelte";
  import TextInput from "$lib/components/TextInput.svelte";
  import Button from "$lib/components/Button.svelte";
  import BaseModal from "$lib/components/BaseModal.svelte";
  import AutoComplete from "$lib/components/AutoComplete.svelte";
  import { IconEdit, IconTrash } from "@tabler/icons-svelte";
  import { type WildEncounter, routes } from "../../store/gameRoutes";
  import NumberInput from "./NumberInput.svelte";
  import { getToastStore } from "@skeletonlabs/skeleton";
  import { BaseDirectory, writeTextFile } from "@tauri-apps/api/fs";
  import { invoke } from "@tauri-apps/api";
  import { selectedWiki } from "../../store";
  import { cloneDeep } from "$lib/utils/cloneDeep";

  const toastStore = getToastStore();

  let searchValue: string = "";
  let addLocationModalOpen: boolean = false;
  export let pokemonId: number;
  export let pokemonName: string;
  export let pokemonLocations: WildEncounter[] = [];

  let newLocation: WildEncounter = {
    id: 0,
    name: "",
    encounter_type: "",
    encounter_rate: 0,
    route: "",
    special_note: "",
  };

  let routeListOptions = Object.keys($routes.routes).map((route) => ({
    label: route,
    value: route,
  }));

  let encounterTypeOptions = $routes.encounter_types.map((encounter_type) => ({
    label: encounter_type,
    value: encounter_type,
  }));

  const rowsPerPageOptions = [
    { label: "5", value: 5 },
    { label: "10", value: 10 },
    { label: "20", value: 20 },
    { label: "50", value: 50 },
    { label: "100", value: 100 },
  ];

  $: handler = new DataHandler(pokemonLocations, {
    rowsPerPage: 5,
  });
  $: rows = handler.getRows();
  $: rowsPerPage = handler.getRowsPerPage();

  async function addPokemonToLocation() {
    newLocation.id = pokemonId;
    newLocation.name = pokemonName;

    let location = cloneDeep(newLocation);

    if (!Object.keys($routes.routes).includes(location.route)) {
      getToastStore().trigger({
        message: "Route is required",
        background: "variant-filled-error",
      });
      return;
    }

    if (
      $routes.routes[location.route].wild_encounters[
        location.encounter_type
      ]?.find((encounter) => encounter.name === location.name)
    ) {
      toastStore.trigger({
        message: "This Pokemon is already in this location",
        background: "variant-filled-error",
      });
      return;
    }

    $routes.routes[location.route].wild_encounters = {
      ...$routes.routes[location.route].wild_encounters,
      [location.encounter_type]: [
        ...($routes.routes[location.route].wild_encounters[
          location.encounter_type
        ] ?? []),
        location,
      ],
    };

    $routes.routes[location.route].wild_encounters[location.encounter_type]
      .sort((a, b) => a.encounter_rate - b.encounter_rate)
      .reverse();

    await writeTextFile(
      `${$selectedWiki.name}/data/routes.json`,
      JSON.stringify($routes),
      { dir: BaseDirectory.AppData },
    ).then(() => {
      invoke("generate_single_route_page_with_handle", {
        wikiName: $selectedWiki.name,
        routeName: location.route,
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
      pokemonLocations = [...pokemonLocations, { ...location }];
      addLocationModalOpen = false;
      newLocation = {
        id: 0,
        name: "",
        encounter_type: "",
        encounter_rate: 0,
        route: "",
        special_note: "",
      };
    });
  }

  async function deletePokemonFromLocation(
    routeName: string,
    encounterType: string,
    id: number,
  ) {
    let updatedEncounters = {
      ...$routes.routes[routeName].wild_encounters,
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

    $routes.routes[routeName].wild_encounters = { ...updatedEncounters };

    await writeTextFile(
      `${$selectedWiki.name}/data/routes.json`,
      JSON.stringify($routes),
      { dir: BaseDirectory.AppData },
    ).then(() => {
      invoke("generate_single_route_page_with_handle", {
        wikiName: $selectedWiki.name,
        routeName: newLocation.route,
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
      let updatedLocations: WildEncounter[] = [];
      for (let location of pokemonLocations) {
        if (location.route !== routeName) {
          updatedLocations.push(location);
          continue;
        }
        if (location.encounter_type !== encounterType) {
          updatedLocations.push(location);
          continue;
        }
      }
      pokemonLocations = [...updatedLocations];
    });
  }
</script>

<BaseModal bind:open={addLocationModalOpen} class="w-[20rem]">
  <h2 class="text-lg font-medium leading-6 text-gray-900">New Location</h2>
  <AutoComplete
    bind:value={newLocation.route}
    label="Routes"
    popupId="newLocationPopup"
    class="z-10 w-full text-sm"
    options={routeListOptions}
    onSelection={(e) => {
      newLocation.route = e.detail.value;
    }}
  />
  <SelectInput
    bind:value={newLocation.encounter_type}
    label="Encounter Type"
    options={encounterTypeOptions}
  />
  <NumberInput
    bind:value={newLocation.encounter_rate}
    label="Encounter Rate"
    placeholder="Encounter Rate"
    max={100}
  />
  <TextInput
    bind:value={newLocation.special_note}
    label="Special Note"
    placeholder="Special Note"
  />
  <Button
    title="Add Location"
    disabled={newLocation.route === "" ||
      newLocation.encounter_rate <= 0 ||
      newLocation.encounter_type === ""}
    onClick={addPokemonToLocation}
  />
</BaseModal>

<div class="mt-4 space-y-4 overflow-x-auto px-4">
  <header class="flex items-center justify-between gap-4">
    <div class="flex gap-x-3">
      <TextInput
        id="route-name"
        bind:value={searchValue}
        inputHandler={() => handler.search(searchValue)}
        placeholder="Search route name..."
      />
      <Button
        title="Add Route"
        class="mt-2"
        onClick={() => (addLocationModalOpen = true)}
      />
    </div>
    <aside class="flex items-center gap-x-3">
      <p class="mt-2">Show</p>
      <SelectInput bind:value={$rowsPerPage} options={rowsPerPageOptions} />
    </aside>
  </header>
  <table class="table table-hover table-compact w-full table-auto bg-white">
    <thead>
      <tr class="bg-white">
        <ThSort {handler} orderBy="route">Route</ThSort>
        <ThSort {handler} orderBy="encounter_type">Encounter Type</ThSort>
        <ThSort {handler} orderBy="encounter_rate">Encounter Rate</ThSort>
        <ThSort {handler} orderBy="special_note">Special Note</ThSort>
      </tr>
    </thead>
    <tbody>
      {#each $rows as row}
        <tr>
          <td>{row.route}</td>
          <td>{row.encounter_type}</td>
          <td>{row.encounter_rate}</td>
          <td>{row.special_note}</td>
          <td
            class="w-5 rounded-sm hover:cursor-pointer hover:bg-gray-300"
            on:click={() => {}}
          >
            <IconEdit size={18} class="text-gray-500" />
          </td>
          <td
            class="w-5 rounded-sm hover:cursor-pointer hover:bg-gray-300"
            on:click={() => {
              deletePokemonFromLocation(row.route, row.encounter_type, row.id);
            }}
          >
            <IconTrash size={18} class="text-gray-500" />
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
  <footer class="flex">
    <Pagination {handler} />
  </footer>
</div>
