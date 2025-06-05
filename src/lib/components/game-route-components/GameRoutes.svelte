<script lang="ts">
  import {
    BaseDirectory,
    copyFile,
    remove,
    writeTextFile,
  } from "@tauri-apps/plugin-fs";
  import { routes } from "../../../store/gameRoutes";
  import { selectedWiki } from "../../../store";
  import { sortRoutesByPosition } from "$lib/utils";
  import { cloneDeep } from "$lib/utils/cloneDeep";
  import { invoke } from "@tauri-apps/api/core";
  import { generateRoutePages, updateRoutes } from "$lib/utils/generators";
  import GripIcon from "@lucide/svelte/icons/grip";
  import EllipsisIcon from "@lucide/svelte/icons/ellipsis-vertical";
  import * as Popover from "$lib/components/ui/popover/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { toast } from "svelte-sonner";

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

    for (let [routeName, _] of Object.entries($routes.routes)) {
      if (routeName === newName) {
        toast.error("Route name already exists");
        return;
      }
    }

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
          {
            fromPathBaseDir: BaseDirectory.AppData,
            toPathBaseDir: BaseDirectory.AppData,
          },
        ).then(() => {
          remove(
            `${$selectedWiki.name}/dist/docs/img/routes/${originalRouteName}.png`,
            { baseDir: BaseDirectory.AppData },
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
              toast.success(res as string);
            })
            .catch((err) => {
              toast.error(err as string);
            });
        });
      })
      .catch((err) => {
        toast.error(err as string);
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
            toast.success(res as string);
          })
          .catch((err) => {
            toast.error(err as string);
          });
      })
      .catch((err) => {
        toast.error(err as string);
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
      { baseDir: BaseDirectory.AppData },
    ).then(() => {
      copyFile(
        `${$selectedWiki.name}/dist/docs/img/routes/${routeName}.png`,
        `${$selectedWiki.name}/dist/docs/img/routes/${routeName} copy.png`,
        {
          fromPathBaseDir: BaseDirectory.AppData,
          toPathBaseDir: BaseDirectory.AppData,
        },
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
      class="flex flex-row justify-between align-middle hover:ring-2 hover:ring-slate-400 ease-in-out group rounded-lg cursor-pointer transition-all duration-200 shadow-sm hover:shadow-lg hover:-translate-y-0.5 border-0 bg-white relative"
    >
      {#if routeBeingEdited === routeName}
        <Input
          bind:value={newRouteName}
          onkeydown={(e) => {
            if (e.key === "Enter") {
              renameRoute(routeBeingEdited, newRouteName);
              routeBeingEdited = "";
            }
          }}
          oninput={capitalizeWords}
          class="p-4 w-full"
        />
      {:else}
        <a href="/game-routes/{routeName}" class="w-full p-4">
          {routeName}
        </a>
      {/if}
      <Popover.Root>
        <Popover.Trigger>
          <button
            class="rounded-md p-1 mr-2 hover:cursor-pointer hover:bg-slate-300"
          >
            <EllipsisIcon
              class="text-slate-400 size-4"
              onclick={() => console.log("test")}
            />
          </button>
        </Popover.Trigger>
        <Popover.Content class="w-[10rem] flex flex-col gap-2">
          <Button
            variant="outline"
            class="w-full"
            onclick={() => {
              routeBeingEdited = routeName;
              newRouteName = routeName;
            }}
          >
            Rename
          </Button>
          <Button
            variant="outline"
            class="w-full"
            onclick={() => duplicateRoute(routeName)}
          >
            Duplicate
          </Button>
          <Button
            variant="outline"
            class="w-full"
            onclick={() => {
              positionModalOpen = true;
              routeToUpdate = routeName;
              oldRoutePosition = $routes.routes[routeName].position;
            }}
          >
            Reorder
          </Button>
          <Button
            class="w-full bg-red-400 hover:bg-red-500"
            onclick={() => deleteRoute(routeName)}
          >
            Delete
          </Button>
        </Popover.Content>
      </Popover.Root>
    </div>
    <!-- <div class="card w-44 grid-cols-1 p-4" data-popup="routeMenu-{index}">
      <button
        class="w-full rounded-md bg-gray-100 px-3 py-1 text-start text-sm hover:cursor-pointer hover:bg-gray-300"
        onclick={() => {
          routeBeingEdited = routeName;
          newRouteName = routeName;
        }}>Rename</button
      >
      <button
        class="w-full rounded-md bg-gray-100 px-3 py-1 text-start text-sm hover:cursor-pointer hover:bg-gray-300"
        onclick={() => {
          positionModalOpen = true;
          routeToUpdate = routeName;
          oldRoutePosition = $routes.routes[routeName].position;
        }}>Change Position</button
      >
    </div> -->
  {/each}
</div>
