<script lang="ts">
import BaseModal from "$lib/components/BaseModal.svelte";
import Button from "$lib/components/Button.svelte";
import TextInput from "$lib/components/TextInput.svelte";
import { Tab, TabGroup, getToastStore, popup } from "@skeletonlabs/skeleton";
import { IconDotsVertical, IconTrash } from "@tabler/icons-svelte";
import { invoke } from "@tauri-apps/api";
import { selectedWiki } from "../../store";
import { routes, type Routes } from "../../store/gameRoutes";
import { BaseDirectory, writeTextFile } from "@tauri-apps/api/fs";
import { goto } from "$app/navigation";
import _ from "lodash";
import { sortRoutesByPosition } from "$lib/utils";
import NumberInput from "$lib/components/NumberInput.svelte";

const toastStore = getToastStore();

let routeName: string = "";
let routeToUpdate: string = "";
let newRouteName: string = "";
let newRouteModalOpen: boolean = false;
let encounterTypeModalOpen: boolean = false;
let positionModalOpen: boolean = false;
let newEncounterType: string = "";
let routeBeingEdited: string = "";
let loading = false;
let tabSet = 0;

async function createNewRoute() {
  $routes.routes[routeName.trim()] = {
    position: Object.keys($routes.routes).length,
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

async function duplicateRoute(routeName: string) {
  $routes.routes[`${routeName} copy`] = _.cloneDeep($routes.routes[routeName]);
  $routes.routes[`${routeName} copy`].position = Object.keys(
    $routes.routes,
  ).length;
  await writeTextFile(
    `${$selectedWiki.name}/data/routes.json`,
    JSON.stringify(sortRoutesByPosition($routes)),
    { dir: BaseDirectory.AppData },
  );
}

async function deleteRoute(routeName: string) {
  let updateRoutes = { ...$routes };
  delete updateRoutes.routes[routeName];
  $routes = { ...updateRoutes };

  await writeTextFile(
    `${$selectedWiki.name}/data/routes.json`,
    JSON.stringify(sortRoutesByPosition($routes)),
    { dir: BaseDirectory.AppData },
  );
}

async function renameRoute(originalRouteName: string, newName: string) {
  let updatedRoutes = { ...$routes };
  updatedRoutes.routes[newName] = updatedRoutes.routes[originalRouteName];
  delete updatedRoutes.routes[originalRouteName];
  $routes = { ...sortRoutesByPosition(updatedRoutes) };
  await writeTextFile(
    `${$selectedWiki.name}/data/routes.json`,
    JSON.stringify($routes),
    { dir: BaseDirectory.AppData },
  );
}

async function updatePosition() {
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

<TabGroup>
  <Tab bind:group={tabSet} name="routes" value={0}>Routes</Tab>
  <Tab bind:group={tabSet} name="routes" value={1}>Generation</Tab>
  <svelte:fragment slot="panel">
    {#if tabSet === 0}
      <div class="flex flex-row gap-3">
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
      </div>

      <div class="mt-6 grid grid-cols-5 gap-x-4 gap-y-1">
        {#each Object.keys($routes.routes) as routeName, index}
          <div
            class="card flex flex-row items-center justify-between !bg-transparent p-3 shadow-sm"
          >
            {#if routeName === routeBeingEdited}
              <TextInput
                bind:value={newRouteName}
                onKeyDownHandler={(e) => {
            if (e.key === "Enter") {
              renameRoute(routeBeingEdited, newRouteName);
              routeBeingEdited = "";
            }
          }}
              />
            {:else}
              <a
                href="/game-routes/{routeName}"
                class="w-full hover:cursor-pointer"
              >
                {routeName}
              </a>
            {/if}
            <button
              class="rounded-md p-1 hover:cursor-pointer hover:bg-gray-300"
              use:popup={{
          event: "click",
          target: "routeMenu-" + index,
          placement: "bottom",
        }}
            >
              <IconDotsVertical size={16} />
            </button>
          </div>
          <div class="card w-44 grid-cols-1 p-4" data-popup="routeMenu-{index}">
            <button
              class="w-full rounded-md bg-gray-100 px-3 py-1 text-start text-sm hover:cursor-pointer hover:bg-gray-300"
              on:click={() => {routeBeingEdited = routeName; newRouteName = routeName;}}
              >Rename</button
            >
            <button
              class="w-full rounded-md bg-gray-100 px-3 py-1 text-start text-sm hover:cursor-pointer hover:bg-gray-300"
              on:click={() => duplicateRoute(routeName)}>Duplicate</button
            >
            <button
              class="w-full rounded-md bg-gray-100 px-3 py-1 text-start text-sm hover:cursor-pointer hover:bg-gray-300"
              on:click={() => deleteRoute(routeName)}>Delete</button
            >
            <button
              class="w-full rounded-md bg-gray-100 px-3 py-1 text-start text-sm hover:cursor-pointer hover:bg-gray-300"
              on:click={() => {positionModalOpen = true;routeToUpdate=routeName}}
              >Change Position</button
            >
          </div>
        {/each}
      </div>
    {/if}
    {#if tabSet === 1}
      <Button
        class="w-40"
        title="Generate Pages"
        onClick={generateRoutePages}
        loading={loading}
      />
    {/if}
  </svelte:fragment>
</TabGroup>
