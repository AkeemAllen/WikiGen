<script lang="ts">
  import { selectedWiki, wikis } from "../store";
</script>

<div class="card p-4 w-40 shadow-xl flex flex-col" data-popup="wikiSelectPopup">
  {#if Object.keys($wikis).length !== 0}
    <p class="text-xs text-slate-400 mb-1">Wikis</p>
  {/if}
  {#each Object.entries($wikis) as [wiki, value]}
    <button
      on:click={() => ($selectedWiki = $wikis[wiki])}
      class="w-30 text-sm text-left p-2 hover:bg-slate-300 rounded-md"
      >{value.name}</button
    >
  {/each}
  <p class="text-xs text-slate-400 mb-1 mt-1">Actions</p>
  <a
    href="/create-wiki"
    class="w-30 text-sm text-left p-2 hover:bg-slate-300 rounded-md"
    >Create Wiki</a
  >
</div>
