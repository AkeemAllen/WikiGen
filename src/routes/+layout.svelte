<script lang="ts">
  import { page } from "$app/stores";
  import NavButton from "$lib/components/NavButton.svelte";
  import WikiSelectMenu from "$lib/components/WikiSelectMenu.svelte";
  import {
    arrow,
    autoUpdate,
    computePosition,
    flip,
    offset,
    shift,
  } from "@floating-ui/dom";
  import type { ModalComponent, PopupSettings } from "@skeletonlabs/skeleton";

  import {
    AppBar,
    AppShell,
    Modal,
    Toast,
    initializeStores,
    popup,
    storePopup,
  } from "@skeletonlabs/skeleton";
  import {
    IconBallBasketball,
    IconBinaryTree,
    IconDisc,
    IconDotsVertical,
    IconRoute2,
    IconTestPipe,
  } from "@tabler/icons-svelte";
  import "../app.pcss";
  import { selectedWiki } from "../store";

  initializeStores();

  storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

  const wikiSelectPopup: PopupSettings = {
    event: "click",
    target: "wikiSelectPopup",
    placement: "top",
  };

  const modalRegistry: Record<string, ModalComponent> = {};
</script>

<Toast />
<Modal components={modalRegistry} />
<AppShell class="h-screen bg-white">
  <svelte:fragment slot="header">
    <AppBar class="bg-white">
      <a href="/">WikiGen</a>
    </AppBar>
  </svelte:fragment>
  <svelte:fragment slot="sidebarLeft">
    <div class="p-4 flex flex-col h-full gap-4 bg-white">
      {#if $selectedWiki.name !== ""}
        <div class="flex flex-col grow gap-y-3">
          <NavButton
            name="Pokemon"
            route="/pokemon"
            active={$page.url.pathname.includes("pokemon")}
          >
            <IconBallBasketball slot="icon" size={16} color="indigo" />
          </NavButton>
          <NavButton
            name="Moves"
            route="/moves"
            active={$page.url.pathname.includes("moves")}
          >
            <IconDisc slot="icon" size={16} color="indigo" />
          </NavButton>
          <NavButton
            name="Game Routes"
            route="/game-routes"
            active={$page.url.pathname.includes("game-routes")}
          >
            <IconRoute2 slot="icon" size={16} color="indigo" />
          </NavButton>
          <NavButton
            name="Wiki Generation"
            route="/wiki-generation"
            active={$page.url.pathname.includes("wiki-generation")}
          >
            <IconBinaryTree slot="icon" size={16} color="indigo" />
          </NavButton>
          <NavButton
            name="Wiki Testing"
            route="/wiki-testing"
            active={$page.url.pathname.includes("wiki-testing")}
          >
            <IconTestPipe slot="icon" size={16} color="indigo" />
          </NavButton>
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
  <div class="mt-6 ml-2">
    <slot />
  </div>
</AppShell>
