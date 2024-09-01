<script lang="ts">
  import BaseModal from "$lib/components/BaseModal.svelte";
  import Button from "$lib/components/Button.svelte";
  import TextInput from "$lib/components/TextInput.svelte";
  import { getToastStore } from "@skeletonlabs/skeleton";
  import { IconTrash } from "@tabler/icons-svelte";
  import { invoke } from "@tauri-apps/api";
  import { selectedWiki } from "../../store";
  import { routes } from "../../store/gameRoutes";
  import { BaseDirectory, writeTextFile } from "@tauri-apps/api/fs";
  import { sortRoutesByPosition } from "$lib/utils";
  import NumberInput from "$lib/components/NumberInput.svelte";
  import GameRoutes from "$lib/components/game-route-components/GameRoutes.svelte";

  const toastStore = getToastStore();

  let routeName: string = "";
  let routeToUpdate: string = "";
  let newRouteModalOpen: boolean = false;
  let encounterTypeModalOpen: boolean = false;
  let positionModalOpen: boolean = false;
  let newEncounterType: string = "";
  let oldRoutePosition: number = 0;
  let loading = false;

  async function createNewRoute() {
    if (routeName.trim() === "") {
      return;
    }

    if ($routes.routes[routeName.trim()]) {
      toastStore.trigger({
        message: "Route already exists",
        timeout: 5000,
        hoverable: true,
        background: "variant-filled-error",
      });
      return;
    }

    $routes.routes[routeName.trim()] = {
      render: true,
      position: Object.keys($routes.routes).length + 1,
      trainers: {},
      wild_encounters: {},
      wild_encounter_area_levels: {},
    };
    await writeTextFile(
      `${$selectedWiki.name}/data/routes.json`,
      JSON.stringify($routes),
      { dir: BaseDirectory.AppData },
    ).then(() => {
      routeName = "";
      newRouteModalOpen = false;
    });
  }

  async function addNewEncounterType() {
    $routes.encounter_areas = [...$routes.encounter_areas, newEncounterType];
    await writeTextFile(
      `${$selectedWiki.name}/data/routes.json`,
      JSON.stringify(sortRoutesByPosition($routes)),
      { dir: BaseDirectory.AppData },
    );
  }

  async function deleteEncounterType(encounterType: string) {
    $routes.encounter_areas = $routes.encounter_areas.filter(
      (type) => type !== encounterType,
    );
    await writeTextFile(
      `${$selectedWiki.name}/data/routes.json`,
      JSON.stringify($routes),
      { dir: BaseDirectory.AppData },
    );
  }

  async function updatePosition() {
    let newRoutePosition = $routes.routes[routeToUpdate].position;
    if (oldRoutePosition === newRoutePosition) {
      return;
    }

    let routeWithNewPosition = Object.keys($routes.routes).find(
      (route) =>
        $routes.routes[route].position === newRoutePosition &&
        route !== routeToUpdate,
    );

    if (!routeWithNewPosition) {
      return;
    }

    $routes.routes[routeWithNewPosition].position = oldRoutePosition;

    $routes = sortRoutesByPosition($routes);
    await writeTextFile(
      `${$selectedWiki.name}/data/routes.json`,
      JSON.stringify($routes),
      { dir: BaseDirectory.AppData },
    );
  }

  async function generateRoutePages() {
    loading = true;
    let routeNames = Object.keys($routes.routes);
    await invoke("generate_route_pages_with_handle", {
      wikiName: $selectedWiki.name,
      routeNames,
    }).then((response: any) => {
      loading = false;
      toastStore.trigger({
        message: response || "Route Pages generated",
        timeout: 5000,
        hoverable: true,
        background: "variant-filled-success",
      });
    });
  }
  function capitalizeWords(event: any) {
    routeName = event.target.value.replace(/\b\w/g, (char: string) =>
      char.toUpperCase(),
    );
  }
</script>

<!-- New Route Modal -->
<BaseModal bind:open={newRouteModalOpen}>
  <TextInput
    bind:value={routeName}
    label="Route Name"
    inputHandler={capitalizeWords}
  />
  <Button
    title="Save New Route"
    onClick={() => {
      createNewRoute();
    }}
    disabled={routeName === ""}
  />
</BaseModal>

<!-- Position Modal -->
<BaseModal bind:open={positionModalOpen}>
  <NumberInput
    bind:value={$routes.routes[routeToUpdate].position}
    label="Route Position"
  />
  <Button
    title="Update Route Position"
    onClick={() => {
      updatePosition();
      positionModalOpen = false;
    }}
  />
</BaseModal>

<!-- Encounter Area Modal -->
<BaseModal bind:open={encounterTypeModalOpen}>
  <div class="flex flex-row gap-3">
    <Button
      class="mt-2 w-44"
      title="Add New Encounter"
      onClick={addNewEncounterType}
      disabled={newEncounterType === ""}
    />
    <TextInput bind:value={newEncounterType} placeholder="New Encounter Area" />
  </div>
  <div class="grid grid-cols-2 gap-3">
    {#each $routes.encounter_areas as encounterType}
      <div class="card flex flex-row items-center justify-between px-2 py-1">
        {encounterType}
        <button
          class="btn rounded-sm p-2 hover:cursor-pointer hover:bg-gray-300"
          on:click={() => deleteEncounterType(encounterType)}
        >
          <IconTrash size={16} />
        </button>
      </div>
    {/each}
  </div>
</BaseModal>

<div class="mt-2 flex flex-row gap-3">
  <Button
    class="w-40"
    title="Create New Route"
    onClick={() => (newRouteModalOpen = true)}
  />
  <Button
    class="w-48"
    title="Modify Encounter Areas"
    onClick={() => (encounterTypeModalOpen = true)}
  />
  <Button
    class="w-42"
    title="Generate Route Pages"
    onClick={generateRoutePages}
    {loading}
  />
</div>
<p class="text-sm italic text-gray-600 mt-2">
  <strong>Note: </strong>A route needs to have at least <strong>ONE</strong> wild
  or trainer encounter to be rendered.
</p>
<GameRoutes bind:positionModalOpen bind:routeToUpdate bind:oldRoutePosition />
