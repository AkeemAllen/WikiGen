<script lang="ts">
import TextInput from "$lib/components/TextInput.svelte";
import { BaseDirectory, writeTextFile } from "@tauri-apps/api/fs";
import { routes } from "../../../store/gameRoutes";
import { selectedWiki } from "../../../store";
import { sortRoutesByPosition } from "$lib/utils";

import { popup } from "@skeletonlabs/skeleton";
import { IconDotsVertical } from "@tabler/icons-svelte";

export let positionModalOpen: boolean = false;
export let routeToUpdate: string = "";

let newRouteName: string = "";
let routeBeingEdited: string;

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
</script>

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
        <a href="/game-routes/{routeName}" class="w-full hover:cursor-pointer">
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
