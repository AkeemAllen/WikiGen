<script lang="ts">
  import { NavButton, WikiSelectMenu } from "$lib";
  import {
    arrow,
    autoUpdate,
    computePosition,
    flip,
    offset,
    shift,
  } from "@floating-ui/dom";
  import type { PopupSettings } from "@skeletonlabs/skeleton";

  import {
    AppBar,
    AppShell,
    Toast,
    initializeStores,
    popup,
    storePopup,
  } from "@skeletonlabs/skeleton";
  import "../app.pcss";
  import { selectedWiki } from "../store";

  initializeStores();

  storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

  const wikiSelectPopup: PopupSettings = {
    event: "click",
    target: "wikiSelectPopup",
    placement: "top",
  };
</script>

<Toast />
<AppShell class="h-screen bg-white">
  <svelte:fragment slot="header">
    <AppBar>
      <a href="/">WikiGen</a>
    </AppBar>
  </svelte:fragment>
  <svelte:fragment slot="sidebarLeft">
    <div class="p-4 flex flex-col h-full gap-4 bg-white">
      {#if $selectedWiki.name !== ""}
        <div class="flex flex-col gap-3 grow">
          <NavButton name="Pokemon" route="/pokemon" />
        </div>
      {/if}
      <a
        id="sidebar-left"
        use:popup={wikiSelectPopup}
        class="btn border-0 hover:cursor-pointer rounded-md p-2 bg-blue-200 w-40"
      >
        {$selectedWiki.name ? $selectedWiki.name : "Select Wiki"}
      </a>
    </div>
    <WikiSelectMenu />
  </svelte:fragment>
  <slot />
</AppShell>
