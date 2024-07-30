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
import _ from "lodash";
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
  $routes.routes[routeName.trim()] = {
    position: Object.keys($routes.routes).length + 1,
    trainers: {},
    wild_encounters: {},
    wild_encounter_area_levels: {},
  };
  await writeTextFile(
    `${$selectedWiki.name}/data/routes.json`,
    JSON.stringify($routes),
    { dir: BaseDirectory.AppData },
  );
}

async function addNewEncounterType() {
  $routes.encounter_types = [...$routes.encounter_types, newEncounterType];
  await writeTextFile(
    `${$selectedWiki.name}/data/routes.json`,
    JSON.stringify(sortRoutesByPosition($routes)),
    { dir: BaseDirectory.AppData },
  );
}

async function deleteEncounterType(encounterType: string) {
  $routes.encounter_types = $routes.encounter_types.filter(
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

  if (oldRoutePosition > newRoutePosition) {
    for (const route in $routes.routes) {
      if (
        $routes.routes[route].position >= newRoutePosition &&
        route !== routeToUpdate
      ) {
        $routes.routes[route].position += 1;
      }
    }
  } else {
    for (const route in $routes.routes) {
      if (
        $routes.routes[route].position <= newRoutePosition &&
        route !== routeToUpdate
      ) {
        $routes.routes[route].position -= 1;
      }
    }
  }

  $routes = sortRoutesByPosition($routes);
  await writeTextFile(
    `${$selectedWiki.name}/data/routes.json`,
    JSON.stringify($routes),
    { dir: BaseDirectory.AppData },
  );
}

async function generateRoutePages() {
  loading = true;
  await invoke("generate_route_pages_with_handle", {
    wikiName: $selectedWiki.name,
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
</script>

<!-- New Route Modal -->
<BaseModal bind:open={newRouteModalOpen}>
  <TextInput bind:value={routeName} label="Route Name" />
  <Button
    title="Save New Route"
    onClick={() => {
      createNewRoute();
      newRouteModalOpen = false;
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
    onClick={() => {updatePosition(); positionModalOpen = false;}}
  />
</BaseModal>

<!-- Encounter Type Modal -->
<BaseModal bind:open={encounterTypeModalOpen}>
  <div class="flex flex-row gap-3">
    <Button
      class="mt-2 w-44"
      title="Add New Encounter"
      onClick={addNewEncounterType}
      disabled={newEncounterType === ""}
    />
    <TextInput bind:value={newEncounterType} placeholder="New Encounter Type" />
  </div>
  <div class="grid grid-cols-2 gap-3">
    {#each $routes.encounter_types as encounterType}
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
    title="Modify Encounter Types"
    onClick={() => (encounterTypeModalOpen = true)}
  />
  <Button
    class="w-42"
    title="Generate Route Pages"
    onClick={generateRoutePages}
    loading={loading}
  />
</div>
<GameRoutes
  bind:positionModalOpen={positionModalOpen}
  bind:routeToUpdate={routeToUpdate}
  bind:oldRoutePosition={oldRoutePosition}
/>
