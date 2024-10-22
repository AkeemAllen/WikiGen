<script lang="ts">
  import { IconPlus } from "@tabler/icons-svelte";
  import { wikis, selectedWiki } from "../store";
  import {
    getToastStore,
    popup,
    type PopupSettings,
  } from "@skeletonlabs/skeleton";
  import { loadWikiData } from "$lib/utils/loadWiki";

  const addIconToolTip: PopupSettings = {
    event: "hover",
    target: "addIconToolTip",
    placement: "bottom",
  };

  const toastStore = getToastStore();
</script>

{#if $selectedWiki.name == ""}
  <div class="flex justify-center h-screen">
    <div class="flex flex-col text-center mt-24 gap-4">
      <p class="text-xl font-bold">Current Wikis</p>
      <div class="grid grid-cols-3 gap-x-3 w-auto max-w-[50rem]">
        {#each Object.entries($wikis) as [codeName, properties]}
          <button
            class="bg-gray-100 p-1 rounded-md border-2
            hover:bg-indigo-600 hover:text-white hover:font-semibold
            hover:ring hover:ring-indigo-500 ease-in-out duration-200"
            on:click={() => {
              $selectedWiki = $wikis[codeName];
              loadWikiData($selectedWiki, toastStore);
            }}>{properties.site_name}</button
          >
        {/each}
      </div>
      <button
        class="bg-gray-100 self-center p-2 rounded-md
        border-2 hover:bg-indigo-600 hover:text-white hover:ring
        hover:ring-indigo-500 ease-in-out duration-200"
        use:popup={addIconToolTip}
      >
        <IconPlus size={25} />
      </button>
      <div data-popup="addIconToolTip">
        <p class="card p-1">Create New Wiki</p>

        <div class="arrow bg-surface-100-800-token" />
      </div>
    </div>
  </div>
{/if}

<!-- Improve Page design (Just stick with original design for now) -->
<!-- Add ability to create wiki -->
<!-- Read wiki information from json files -->
