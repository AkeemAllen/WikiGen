<script lang="ts">
  import { Button } from "$lib/components/ui/button/index.js";
  import { type WildEncounter, routes } from "../../store/gameRoutes";
  import { BaseDirectory, writeTextFile } from "@tauri-apps/plugin-fs";
  import { invoke } from "@tauri-apps/api/core";
  import { selectedWiki } from "../../store";
  import { cloneDeep } from "$lib/utils/cloneDeep";
  import { generatePokemonPages } from "$lib/utils/generators";
  import { getToastSettings, ToastType } from "$lib/utils/toasts";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import * as Select from "$lib/components/ui/select/index.js";
  import * as Card from "$lib/components/ui/card/index.js";
  import TrashIcon from "@lucide/svelte/icons/trash";
  import { toast } from "svelte-sonner";
  import capitalizeWords from "$lib/utils/capitalizeWords";

  let searchValue: string = $state("");
  let addLocationModalOpen: boolean = $state(false);
  let editLocationModalOpen: boolean = $state(false);
  type Props = {
    pokemonId: number;
    pokemonDexNumber: number;
    pokemonName: string;
    pokemonLocations?: WildEncounter[];
  };

  let {
    pokemonId,
    pokemonDexNumber,
    pokemonName,
    pokemonLocations = $bindable([]),
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
      toast.error("Route is required");
      return;
    }

    if (
      $routes.routes[location.route].wild_encounters[
        location.encounter_area
      ]?.find((encounter) => encounter.name === location.name)
    ) {
      toast.error("This Pokemon is already in this location");
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
      { baseDir: BaseDirectory.AppData },
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
            toast.success("Pokemon page regenerated");
          })
          .catch((e) => {
            toast.error(e);
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
      { baseDir: BaseDirectory.AppData },
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
            toast.success("Pokemon page regenerated");
          })
          .catch((e) => {
            toast.error(e);
          });
      });
  }

  async function deletePokemonFromLocation(
    routeName: string,
    encounterType: string,
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
      { baseDir: BaseDirectory.AppData },
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

<Dialog.Root bind:open={addLocationModalOpen}>
  <Dialog.Content class="sm:max-w-[425px]">
    <Dialog.Header>
      <Dialog.Title>New Location</Dialog.Title>
    </Dialog.Header>
    <div class="grid gap-4 py-4">
      <div>
        <Label
          for="route-name"
          class="text-sm font-medium text-slate-700 mb-2 block">Route</Label
        >
        <Select.Root type="single" bind:value={newLocation.route}>
          <Select.Trigger id="route-name" class="w-full">
            {capitalizeWords(newLocation.route) || "Select a Route"}
          </Select.Trigger>
          <Select.Content>
            {#each routeListOptions as route}
              <Select.Item value={route.value} label={route.label}>
                {capitalizeWords(route.value)}
              </Select.Item>
            {/each}
          </Select.Content>
        </Select.Root>
      </div>
      <div>
        <Label
          for="encounter-area"
          class="text-sm font-medium text-slate-700 mb-2 block"
          >Encounter Area</Label
        >
        <Select.Root type="single" bind:value={newLocation.encounter_area}>
          <Select.Trigger id="encounter-area" class="w-full">
            {capitalizeWords(newLocation.encounter_area)}
          </Select.Trigger>
          <Select.Content>
            {#each encounterTypeOptions as encounterTypes}
              <Select.Item
                value={encounterTypes.value}
                label={encounterTypes.label}
              >
                {capitalizeWords(encounterTypes.value)}
              </Select.Item>
            {/each}
          </Select.Content>
        </Select.Root>
      </div>
      <div>
        <Label
          for="encounter-rate"
          class="text-sm font-medium text-slate-700 mb-2 block"
          >Encounter Rate</Label
        >
        <Input
          id="encounter-rate"
          type="number"
          bind:value={newLocation.encounter_rate}
          max={100}
          min={0}
        />
      </div>
      <div>
        <Label
          for="special-note"
          class="text-sm font-medium text-slate-700 mb-2 block"
          >Special Note</Label
        >
        <Input
          type="text"
          id="special-note"
          bind:value={newLocation.special_note}
        />
      </div>
    </div>
    <Dialog.Footer>
      <Button
        disabled={newLocation.route === "" ||
          newLocation.encounter_rate <= 0 ||
          newLocation.encounter_area === ""}
        onclick={addPokemonToLocation}
        class="cursor-pointer">Add Location</Button
      >
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>

<Dialog.Root bind:open={editLocationModalOpen}>
  <Dialog.Content class="sm:max-w-[425px]">
    <Dialog.Header>
      <Dialog.Title>Edit Location</Dialog.Title>
    </Dialog.Header>
    <div class="grid gap-4 py-4">
      <div>
        <Label
          for="route-name"
          class="text-sm font-medium text-slate-700 mb-2 block">Route</Label
        >
        <Select.Root
          type="single"
          disabled={true}
          bind:value={locationToEdit.route}
        >
          <Select.Trigger id="route-name" class="w-full">
            {capitalizeWords(locationToEdit.route)}
          </Select.Trigger>
          <Select.Content>
            {#each routeListOptions as route}
              <Select.Item value={route.value} label={route.label}>
                {capitalizeWords(route.value)}
              </Select.Item>
            {/each}
          </Select.Content>
        </Select.Root>
      </div>
      <div>
        <Label
          for="encounter-area"
          class="text-sm font-medium text-slate-700 mb-2 block"
          >Encounter Area</Label
        >
        <Select.Root
          type="single"
          disabled={true}
          bind:value={locationToEdit.encounter_area}
        >
          <Select.Trigger id="encounter-area" class="w-full">
            {capitalizeWords(locationToEdit.encounter_area)}
          </Select.Trigger>
          <Select.Content>
            {#each encounterTypeOptions as encounterTypes}
              <Select.Item
                value={encounterTypes.value}
                label={encounterTypes.label}
              >
                {capitalizeWords(encounterTypes.value)}
              </Select.Item>
            {/each}
          </Select.Content>
        </Select.Root>
      </div>
      <div>
        <Label
          for="encounter-rate"
          class="text-sm font-medium text-slate-700 mb-2 block"
          >Encounter Rate</Label
        >
        <Input
          id="encounter-rate"
          type="number"
          bind:value={locationToEdit.encounter_rate}
          max={100}
          min={0}
        />
      </div>
      <div>
        <Label
          for="special-note"
          class="text-sm font-medium text-slate-700 mb-2 block"
          >Special Note</Label
        >
        <Input
          type="text"
          id="special-note"
          bind:value={locationToEdit.special_note}
        />
      </div>
    </div>
    <Dialog.Footer>
      <Button onclick={editPokemonLocation} class="cursor-pointer"
        >Edit Location</Button
      >
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>

<Card.Root>
  <Card.Header>
    <Card.Title>Locations</Card.Title>
  </Card.Header>
  <Card.Content>
    <div class="flex flex-row gap-2">
      <Input
        type="text"
        placeholder="Filter Routes"
        class="w-[20rem]"
        bind:value={searchValue}
      />
      <Button
        onclick={() => {
          addLocationModalOpen = true;
        }}
        class="cursor-pointer">Add Route</Button
      >
    </div>
    <div class="grid grid-cols-4 gap-2 mt-5">
      {#each pokemonLocations.filter((location) => location.route
          .toLowerCase()
          .includes(searchValue.toLowerCase())) as location}
        <button
          class="hover:border-indigo-500 ease-in-out group rounded-lg p-4 cursor-pointer transition-all duration-200 hover:shadow-lg hover:-translate-y-1 bg-white border group relative"
          onclick={() => {
            locationToEdit = location;
            editLocationModalOpen = true;
          }}
        >
          <div class="grid grid-cols-2 gap-2">
            <div class="text-left">
              {location.route}
              <p class="text-sm text-gray-500">
                {location.special_note}
              </p>
            </div>
            <div class="flex justify-end text-sm text-gray-500">
              {capitalizeWords(location.encounter_area)}: {location.encounter_rate}%
            </div>
          </div>
          <Button
            class="invisible absolute -right-2 -top-5 z-10 rounded-md bg-red-200 hover:scale-110 group-hover:visible cursor-pointer hover:bg-red-400 p-0"
            onclick={(e) => {
              e.stopPropagation();
              deletePokemonFromLocation(
                location.route,
                location.encounter_area,
              );
            }}
          >
            <TrashIcon />
          </Button>
        </button>
      {/each}
    </div>
  </Card.Content>
</Card.Root>
