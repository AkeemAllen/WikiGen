<script lang="ts">
  import { IconPlus } from "@tabler/icons-svelte";
  import { wikis, selectedWiki } from "../store";
  import {
    getToastStore,
    popup,
    type PopupSettings,
  } from "@skeletonlabs/skeleton";
  import { loadWikiData } from "$lib/utils/loadWiki";
  import CreateWikiModal from "$lib/components/modals/CreateWikiModal.svelte";

  const addIconToolTip: PopupSettings = {
    event: "hover",
    target: "addIconToolTip",
    placement: "bottom",
  };

  const toastStore = getToastStore();

  let createWikiModalOpen = $state(false);
</script>

<CreateWikiModal bind:open={createWikiModalOpen} />

{#if $selectedWiki.name == ""}
  <div class="flex justify-center h-screen">
    <div class="flex flex-col text-center mt-24 gap-4">
      <p class="text-xl font-bold">Current Wikis</p>
      <div class="grid grid-cols-3 gap-3 w-[60rem]">
        {#each Object.entries($wikis) as [codeName, properties]}
          <button
            class="bg-gray-100 p-3 rounded-md border-2
            hover:bg-red-600 hover:text-white hover:font-semibold
            hover:ring hover:ring-red-500 ease-in-out duration-200"
            onclick={() => {
              $selectedWiki = $wikis[codeName];
              loadWikiData($selectedWiki, toastStore);
            }}>{properties.site_name}</button
          >
        {/each}
      </div>
      <button
        class="bg-gray-100 self-center p-2 rounded-md
        border-2 hover:bg-red-600 hover:text-white hover:ring
        hover:ring-red-500 ease-in-out duration-200"
        use:popup={addIconToolTip}
        onclick={() => (createWikiModalOpen = true)}
      >
        <IconPlus size={25} />
      </button>
      <div data-popup="addIconToolTip">
        <p class="card p-1">Create New Wiki</p>

        <div class="arrow bg-surface-100-800-token"></div>
      </div>
    </div>
  </div>
{/if}
