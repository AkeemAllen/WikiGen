<script lang="ts">
  import BaseModal from "$lib/components/BaseModal.svelte";
  import Button from "$lib/components/Button.svelte";
  import TextInput from "$lib/components/TextInput.svelte";
  import { getToastStore } from "@skeletonlabs/skeleton";
  import { IconDotsVertical, IconTrash } from "@tabler/icons-svelte";
  import { invoke } from "@tauri-apps/api";
  import { selectedWiki } from "../../store";
  import { routes, type Routes } from "../../store/gameRoutes";
    import { BaseDirectory, writeTextFile } from "@tauri-apps/api/fs";

  const toastStore = getToastStore();

  let routeName: string = "";
  let newRouteModalOpen: boolean = false;
  let encounterTypeModalOpen: boolean = false;
  let newEncounterType: string = "";
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
    });
  }

  async function addNewEncounterType() {
    $routes.encounter_types = [...$routes.encounter_types, newEncounterType];
    await writeTextFile(
      `${$selectedWiki.name}/data/routes.json`,
      JSON.stringify($routes),
      { dir: BaseDirectory.AppData },
    );
  }

  async function deleteEncounterType(encounterType: string) {
    $routes.encounter_types = $routes.encounter_types.filter((type) => type !== encounterType);
    console.log($routes);
    await writeTextFile(
      `${$selectedWiki.name}/data/routes.json`,
      JSON.stringify($routes),
      { dir: BaseDirectory.AppData },
    );
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
<BaseModal bind:open={encounterTypeModalOpen}>
    <div class="flex flex-row gap-3">
        <div class="w-44 mt-2">
            <Button title="Add New Encounter" onClick={addNewEncounterType} disabled={
                newEncounterType === ""
                } />
        </div>
        <TextInput bind:value={newEncounterType} placeholder="New Encounter Type"/>
    </div>
    <div class="grid grid-cols-2 gap-3">
    {#each $routes.encounter_types as encounterType}
        <div class="flex flex-row justify-between items-center card px-2 py-1">
          {encounterType}
          <div class="btn hover:cursor-pointer hover:bg-gray-300 p-2 rounded-sm" on:click={() => deleteEncounterType(encounterType)} >
            <IconTrash size={16} />
          </div>
        </div>
    {/each}
    </div>
</BaseModal>

<div class="flex flex-row gap-3">
<div class="w-40">
  <Button title="Create New Route" onClick={() => (newRouteModalOpen = true)} />
</div>
<div class="w-48">
  <Button title="Modify Encounter Types" onClick={() => (encounterTypeModalOpen = true)} />
</div>
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
