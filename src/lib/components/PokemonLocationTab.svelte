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
  import { generatePokemonPages } from "$lib/utils/generators";
  import { getToastSettings, ToastType } from "$lib/utils/toasts";

  const toastStore = getToastStore();

  let searchValue: string = $state("");
  let addLocationModalOpen: boolean = $state(false);
  let editLocationModalOpen: boolean = $state(false);
  interface Props {
    pokemonId: number;
    pokemonDexNumber: number;
    pokemonName: string;
    pokemonLocations?: WildEncounter[];
  }

  let {
    pokemonId,
    pokemonDexNumber,
    pokemonName,
    pokemonLocations = $bindable([])
  }: Props = $props();

  let newLocation: WildEncounter = $state({
    id: 0,
    name: "",
    encounter_area: "",
    encounter_rate: 0,
    route: "",
    special_note: "",
  });

  let locationToEdit: WildEncounter = $state({
    id: 0,
    name: "",
    encounter_area: "",
    encounter_rate: 0,
    route: "",
    special_note: "",
  });

  let routeListOptions = Object.keys($routes.routes).map((route) => ({
    label: route,
    value: route,
  }));

  let encounterTypeOptions = $routes.encounter_areas.map((encounter_area) => ({
    label: encounter_area,
    value: encounter_area,
  }));

  const rowsPerPageOptions = [
    { label: "5", value: 5 },
    { label: "10", value: 10 },
    { label: "20", value: 20 },
    { label: "50", value: 50 },
    { label: "100", value: 100 },
  ];

  let handler = $derived(new DataHandler(pokemonLocations, {
    rowsPerPage: 5,
  }));
  let rows = $derived(handler.getRows());
  let rowsPerPage = $derived(handler.getRowsPerPage());

  async function generateRoutePage(routeName: string) {
    invoke("generate_route_pages_with_handle", {
      wikiName: $selectedWiki.name,
      routeNames: [routeName],
    }).catch((e) => {
      getToastSettings(ToastType.ERROR, e);
    });
  }

  async function addPokemonToLocation() {
    newLocation.id = pokemonDexNumber;
    newLocation.name = pokemonName;

    let location = cloneDeep(newLocation);

    if (!Object.keys($routes.routes).includes(location.route)) {
      toastStore.trigger(
        getToastSettings(ToastType.ERROR, "Route is required"),
      );
      return;
    }

    if (
      $routes.routes[location.route].wild_encounters[
        location.encounter_area
      ]?.find((encounter) => encounter.name === location.name)
    ) {
      toastStore.trigger(
        getToastSettings(
          ToastType.ERROR,
          "This Pokemon is already in this location",
        ),
      );
      return;
    }

    $routes.routes[location.route].wild_encounters = {
      ...$routes.routes[location.route].wild_encounters,
      [location.encounter_area]: [
        ...($routes.routes[location.route].wild_encounters[
          location.encounter_area
        ] ?? []),
        location,
      ],
    };

    $routes.routes[location.route].wild_encounters[location.encounter_area]
      .sort((a, b) => a.encounter_rate - b.encounter_rate)
      .reverse();

    await writeTextFile(
      `${$selectedWiki.name}/data/routes.json`,
      JSON.stringify($routes),
      { dir: BaseDirectory.AppData },
    )
      .then(() => {
        generateRoutePage(location.route);
        pokemonLocations = [...pokemonLocations, { ...location }];
        addLocationModalOpen = false;
        newLocation = {
          id: 0,
          name: "",
          encounter_area: "",
          encounter_rate: 0,
          route: "",
          special_note: "",
        };
      })
      .then(() => {
        generatePokemonPages([pokemonId], $selectedWiki.name)
          .then(() => {
            toastStore.trigger(
              getToastSettings(ToastType.SUCCESS, "Pokemon page regenerated"),
            );
          })
          .catch((e) => {
            toastStore.trigger(getToastSettings(ToastType.ERROR, e));
          });
      });
  }

  async function editPokemonLocation() {
    let location = cloneDeep(locationToEdit);

    let index = $routes.routes[location.route].wild_encounters[
      location.encounter_area
    ].findIndex((encounter) => encounter.id === location.id);
    $routes.routes[location.route].wild_encounters[
      location.encounter_area
    ].splice(index, 1, cloneDeep(location));

    $routes.routes[location.route].wild_encounters[location.encounter_area]
      .sort((a, b) => a.encounter_rate - b.encounter_rate)
      .reverse();

    await writeTextFile(
      `${$selectedWiki.name}/data/routes.json`,
      JSON.stringify($routes),
      { dir: BaseDirectory.AppData },
    )
      .then(() => {
        generateRoutePage(location.route);
        let updatedLocations = [...pokemonLocations];
        let locationIndex = updatedLocations.findIndex(
          (loc) =>
            loc.route === location.route &&
            loc.encounter_area === location.encounter_area,
        );
        console.log(locationIndex);
        updatedLocations[locationIndex] = cloneDeep(location);

        pokemonLocations = [...updatedLocations];
        editLocationModalOpen = false;
        locationToEdit = {
          id: 0,
          name: "",
          encounter_area: "",
          encounter_rate: 0,
          route: "",
          special_note: "",
        };
      })
      .then(() => {
        generatePokemonPages([pokemonId], $selectedWiki.name)
          .then(() => {
            toastStore.trigger(
              getToastSettings(ToastType.SUCCESS, "Pokemon page regenerated"),
            );
          })
          .catch((e) => {
            toastStore.trigger(getToastSettings(ToastType.ERROR, e));
          });
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
      generateRoutePage(routeName);
      let updatedLocations: WildEncounter[] = [];
      for (let location of pokemonLocations) {
        if (location.route !== routeName) {
          updatedLocations.push(location);
          continue;
        }
        if (location.encounter_area !== encounterType) {
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
    bind:value={newLocation.encounter_area}
    label="Encounter Area"
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
      newLocation.encounter_area === ""}
    onClick={addPokemonToLocation}
  />
</BaseModal>

<BaseModal bind:open={editLocationModalOpen} class="w-[20rem]">
  <h2 class="text-lg font-medium leading-6 text-gray-900">Edit Location</h2>
  <AutoComplete
    bind:value={locationToEdit.route}
    label="Routes"
    popupId="newLocationPopup"
    class="z-10 w-full text-sm"
    disabled={true}
    options={routeListOptions}
    onSelection={(e) => {
      locationToEdit.route = e.detail.value;
    }}
  />
  <SelectInput
    bind:value={locationToEdit.encounter_area}
    label="Encounter Area"
    disabled={true}
    options={encounterTypeOptions}
  />
  <NumberInput
    bind:value={locationToEdit.encounter_rate}
    label="Encounter Rate"
    placeholder="Encounter Rate"
    max={100}
  />
  <TextInput
    bind:value={locationToEdit.special_note}
    label="Special Note"
    placeholder="Special Note"
  />
  <Button
    title="Save Changes"
    disabled={locationToEdit.route === "" ||
      locationToEdit.encounter_rate <= 0 ||
      locationToEdit.encounter_area === ""}
    onClick={editPokemonLocation}
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
        <ThSort {handler} orderBy="encounter_area">Encounter Area</ThSort>
        <ThSort {handler} orderBy="encounter_rate">Encounter Rate</ThSort>
        <ThSort {handler} orderBy="special_note">Special Note</ThSort>
      </tr>
    </thead>
    <tbody>
      {#each $rows as row}
        <tr>
          <td>{row.route}</td>
          <td>{row.encounter_area}</td>
          <td>{row.encounter_rate}</td>
          <td>{row.special_note}</td>
          <td
            class="w-5 rounded-sm hover:cursor-pointer hover:bg-gray-300"
            onclick={() => {
              locationToEdit = { ...row };
              editLocationModalOpen = true;
            }}
          >
            <IconEdit size={18} class="text-gray-500" />
          </td>
          <td
            class="w-5 rounded-sm hover:cursor-pointer hover:bg-gray-300"
            onclick={() => {
              deletePokemonFromLocation(row.route, row.encounter_area, row.id);
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
