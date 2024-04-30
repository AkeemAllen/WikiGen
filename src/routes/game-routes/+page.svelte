<script lang="ts">
  import BaseModal from "$lib/components/BaseModal.svelte";
  import Button from "$lib/components/Button.svelte";
  import TextInput from "$lib/components/TextInput.svelte";
  import { getToastStore } from "@skeletonlabs/skeleton";
  import { IconDotsVertical } from "@tabler/icons-svelte";
  import { invoke } from "@tauri-apps/api";
  import { selectedWiki } from "../../store";
  import { routes, type Routes } from "../../store/gameRoutes";

  const toastStore = getToastStore();

  let routeName: string = "";
  let newRouteModalOpen: boolean = false;
  let loading = false;

  async function createNewRoute() {
    loading = true;
    await invoke("create_new_route", {
      wikiName: $selectedWiki.name,
      routeName,
    }).then((response) => {
      loading = false;
      toastStore.trigger({
        message: "New Route Added",
      });
      routes.set(response as Routes);
      console.log($routes);
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
  />
</BaseModal>

<div class="w-40">
  <Button title="Create New Route" onClick={() => (newRouteModalOpen = true)} />
</div>

<div class="flex flex-row gap-x-4 gap-y-1 mt-6">
  {#each Object.keys($routes.routes) as routeName}
    <a href={`/game-routes/${routeName}`}>
      <div
        class="flex flex-row items-center justify-between w-40 card pl-6 pr-2 py-2 shadow-sm hover:cursor-pointer"
      >
        {routeName}
        <div class="hover:cursor-pointer hover:bg-gray-300 rounded-md p-1">
          <IconDotsVertical size={16} />
        </div>
      </div>
    </a>
  {/each}
</div>
