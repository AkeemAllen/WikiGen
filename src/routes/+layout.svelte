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
  import { IconDotsVertical } from "@tabler/icons-svelte";
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
    <AppBar class="bg-white">
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
      <div class="flex flex-row w-40 justify-between items-center">
        <p>
          {$selectedWiki.name ? $selectedWiki.site_name : "Select Wiki"}
        </p>
        <span use:popup={wikiSelectPopup} class="hover:cursor-pointer">
          <IconDotsVertical size={"20"} />
        </span>
      </div>
      <WikiSelectMenu />
    </div>
  </svelte:fragment>
  <slot />
</AppShell>
