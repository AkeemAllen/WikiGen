<script lang="ts">
  import TextInput from "$lib/components/TextInput.svelte";
  import {
    BaseDirectory,
    copyFile,
    removeFile,
    writeTextFile,
  } from "@tauri-apps/api/fs";
  import { routes } from "../../../store/gameRoutes";
  import { selectedWiki } from "../../../store";
  import { sortRoutesByPosition } from "$lib/utils";

  import { getToastStore, popup } from "@skeletonlabs/skeleton";
  import { IconDotsVertical } from "@tabler/icons-svelte";
  import { cloneDeep } from "$lib/utils/cloneDeep";
  import { invoke } from "@tauri-apps/api/tauri";
  import { generateRoutePages, updateRoutes } from "$lib/utils/generators";
  import { getToastSettings, ToastType } from "$lib/utils/toasts";

  const toastStore = getToastStore();

  interface Props {
    positionModalOpen?: boolean;
    routeToUpdate?: string;
    oldRoutePosition?: number;
  }

  let {
    positionModalOpen = $bindable(false),
    routeToUpdate = $bindable(""),
    oldRoutePosition = $bindable(0),
  }: Props = $props();

  let newRouteName: string = $state("");
  let routeBeingEdited: string = $state("");

  async function renameRoute(originalRouteName: string, newName: string) {
    if (originalRouteName === newName) return;

    let updatedRoutes = { ...$routes };
    for (let [routeName, properties] of Object.entries(updatedRoutes.routes)) {
      if (routeName !== originalRouteName) continue;
      for (let [encounterArea, wildEncounters] of Object.entries(
        properties.wild_encounters,
      )) {
        for (let [index, encounter] of wildEncounters.entries()) {
          updatedRoutes.routes[routeName].wild_encounters[encounterArea][
            index
          ].route = newRouteName;
        }
      }
    }
    updatedRoutes.routes[newName] = updatedRoutes.routes[originalRouteName];
    delete updatedRoutes.routes[originalRouteName];
    $routes = { ...sortRoutesByPosition(updatedRoutes) };
    await updateRoutes($routes, $selectedWiki.name)
      .then(() => {
        // rename image
        copyFile(
          `${$selectedWiki.name}/dist/docs/img/routes/${originalRouteName}.png`,
          `${$selectedWiki.name}/dist/docs/img/routes/${newName}.png`,
          { dir: BaseDirectory.AppData },
        ).then(() => {
          removeFile(
            `${$selectedWiki.name}/dist/docs/img/routes/${originalRouteName}.png`,
            { dir: BaseDirectory.AppData },
          );
        });
      })
      .then(() => {
        invoke("delete_route_page_from_mkdocs", {
          routeName: originalRouteName,
          wikiName: $selectedWiki.name,
        }).then(() => {
          generateRoutePages(Object.keys($routes.routes), $selectedWiki.name)
            .then((res) => {
              toastStore.trigger(
                getToastSettings(ToastType.SUCCESS, res as string),
              );
            })
            .catch((err) => {
              toastStore.trigger(
                getToastSettings(ToastType.ERROR, err as string),
              );
            });
        });
      })
      .catch((err) => {
        toastStore.trigger(getToastSettings(ToastType.ERROR, err as string));
      });
  }

  async function deleteRoute(routeName: string) {
    let updatedRoutes = { ...$routes };
    delete updatedRoutes.routes[routeName];
    $routes = { ...updatedRoutes };

    await updateRoutes($routes, $selectedWiki.name)
      .then(() => {
        invoke("delete_route_page_from_mkdocs", {
          routeName,
          wikiName: $selectedWiki.name,
        })
          .then((res) => {
            toastStore.trigger(
              getToastSettings(ToastType.SUCCESS, res as string),
            );
          })
          .catch((err) => {
            toastStore.trigger(
              getToastSettings(ToastType.ERROR, err as string),
            );
          });
      })
      .catch((err) => {
        toastStore.trigger(getToastSettings(ToastType.ERROR, err as string));
      });
  }

  async function duplicateRoute(routeName: string) {
    $routes.routes[`${routeName} copy`] = cloneDeep($routes.routes[routeName]);
    $routes.routes[`${routeName} copy`].position = Object.keys(
      $routes.routes,
    ).length;
    await writeTextFile(
      `${$selectedWiki.name}/data/routes.json`,
      JSON.stringify(sortRoutesByPosition($routes)),
      { dir: BaseDirectory.AppData },
    ).then(() => {
      copyFile(
        `${$selectedWiki.name}/dist/docs/img/routes/${routeName}.png`,
        `${$selectedWiki.name}/dist/docs/img/routes/${routeName} copy.png`,
        { dir: BaseDirectory.AppData },
      );
    });
  }

  function capitalizeWords(event: any) {
    newRouteName = event.target.value.replace(/\b\w/g, (char: string) =>
      char.toUpperCase(),
    );
  }
</script>

<div class="mt-6 grid grid-cols-5 gap-x-4 gap-y-3">
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
          inputHandler={capitalizeWords}
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
        onclick={() => {
          routeBeingEdited = routeName;
          newRouteName = routeName;
        }}>Rename</button
      >
      <button
        class="w-full rounded-md bg-gray-100 px-3 py-1 text-start text-sm hover:cursor-pointer hover:bg-gray-300"
        onclick={() => duplicateRoute(routeName)}>Duplicate</button
      >
      <button
        class="w-full rounded-md bg-gray-100 px-3 py-1 text-start text-sm hover:cursor-pointer hover:bg-gray-300"
        onclick={() => deleteRoute(routeName)}>Delete</button
      >
      <button
        class="w-full rounded-md bg-gray-100 px-3 py-1 text-start text-sm hover:cursor-pointer hover:bg-gray-300"
        onclick={() => {
          positionModalOpen = true;
          routeToUpdate = routeName;
          oldRoutePosition = $routes.routes[routeName].position;
        }}>Change Position</button
      >
    </div>
  {/each}
</div>
