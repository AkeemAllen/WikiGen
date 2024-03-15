<script lang="ts">
  import {
    arrow,
    autoUpdate,
    computePosition,
    flip,
    offset,
    shift,
  } from "@floating-ui/dom";
  import type { PopupSettings } from "@skeletonlabs/skeleton";
  import { AppBar, AppShell, popup, storePopup } from "@skeletonlabs/skeleton";
  import "../app.pcss";
  import { selectedWiki, wikis } from "../store";

  storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

  const wikiSelectPopup: PopupSettings = {
    event: "click",
    target: "wikiSelectPopup",
    placement: "bottom-end",
  };
</script>

<AppShell class="h-screen bg-white">
  <svelte:fragment slot="header">
    <AppBar>
      <a href="/">WikiGen</a>
    </AppBar>
  </svelte:fragment>
  <svelte:fragment slot="sidebarLeft">
    <div class="p-4 flex flex-col h-full gap-4 bg-white">
      {#if $selectedWiki.name !== ""}
        <div class="grow">
          <a
            href="/pokemon"
            id="sidebar-left"
            class="btn border-0 hover:cursor-pointer rounded-md p-2 bg-blue-200 w-40"
          >
            Pokemon
          </a>
        </div>
      {/if}
      <a
        id="sidebar-left"
        use:popup={wikiSelectPopup}
        class="btn border-0 hover:cursor-pointer rounded-md p-2 bg-blue-200 w-40 self-end"
      >
        {$selectedWiki.name ? $selectedWiki.name : "Select Wiki"}
      </a>
    </div>
    <div
      class="card p-4 w-40 shadow-xl flex flex-col"
      data-popup="wikiSelectPopup"
    >
      <p class="text-xs text-slate-400 mb-1">Wikis</p>
      {#each Object.entries($wikis) as [wiki, value]}
        <button
          on:click={() => ($selectedWiki = $wikis[wiki])}
          class="w-30 text-sm text-left p-2 hover:bg-slate-300 rounded-md"
          >{value.name}</button
        >
      {/each}
    </div>
  </svelte:fragment>
  <slot />
</AppShell>
