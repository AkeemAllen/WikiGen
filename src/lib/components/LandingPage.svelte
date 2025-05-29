<script lang="ts">
  import { loadWikiData } from "$lib/utils/loadWiki";
  import { getToastStore } from "@skeletonlabs/skeleton";
  import { selectedWiki, wikis } from "../../store";
  import IconPlus from "@tabler/icons-svelte/icons/plus";
  import CreateWikiModal from "$lib/components/modals/CreateWikiModal.svelte";

  const toastStore = getToastStore();
  let createWikiModalOpen = $state(false);
</script>

<CreateWikiModal bind:open={createWikiModalOpen} />
<main class="container mx-auto px-6 py-12">
  <div class="max-w-4xl mx-auto">
    <section class="text-center mb-12">
      <h2 class="text-3xl font-bold text-slate-900 mb-3">Your Wikis</h2>
      <p class="text-slate-600 text-lg">
        Select a wiki to start editing or create a new one
      </p>
    </section>
    <section class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mb-8">
      {#each Object.entries($wikis) as [codeName, properties], i}
        <button
          class="hover:ring hover:ring-indigo-500 ease-in-out group rounded-lg p-4 cursor-pointer transition-all duration-200 hover:shadow-lg hover:-translate-y-1 border-0 bg-white"
          onclick={() => {
            $selectedWiki = $wikis[codeName];
            loadWikiData($selectedWiki, toastStore);
          }}
        >
          <h3
            class="font-semibold text-slate-900 text-lg group-hover:text-indigo-500"
          >
            {properties.site_name}
          </h3>
        </button>
      {/each}
    </section>
    <section class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3">
      <div></div>
      <button
        class="group cursor-pointer transition-all duration-200 hover:shadow-lg hover:-translate-y-1 border-2 border-dashed border-slate-300 hover:border-indigo-400 bg-slate-50/50 hover:bg-indigo-50/50"
        onclick={() => (createWikiModalOpen = true)}
      >
        <div
          class="p-6 flex flex-col items-center justify-center h-full min-h-[180px]"
        >
          <div
            class="w-12 h-12 bg-slate-200 group-hover:bg-indigo-100 rounded-full flex items-center justify-center mb-4 transition-colors duration-200"
          >
            <IconPlus
              class="w-6 h-6 text-slate-500 group-hover:text-indigo-600 transition-colors duration-200"
            />
          </div>
          <h3
            class="font-medium text-slate-700 group-hover:text-indigo-700 transition-colors duration-200"
          >
            Create New Wiki
          </h3>
          <p class="text-sm text-slate-500 text-center mt-2">
            Start a new wiki project
          </p>
        </div>
      </button>
      <div></div>
    </section>
  </div>
</main>
