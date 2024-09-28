<script lang="ts">
  import { page } from "$app/stores";
  import NavButton from "$lib/components/NavButton.svelte";
  import WikiSelectMenu from "$lib/components/WikiSelectMenu.svelte";
  import BaseModal from "$lib/components/BaseModal.svelte";
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
    AppShell,
    Modal,
    Toast,
    getToastStore,
    initializeStores,
    popup,
    storePopup,
  } from "@skeletonlabs/skeleton";
  import {
    IconAdjustmentsUp,
    IconBallBasketball,
    IconBottleFilled,
    IconDisc,
    IconDotsVertical,
    IconDownload,
    IconFlame,
    IconHome,
    IconRoute2,
    IconTestPipe,
    IconTree,
  } from "@tabler/icons-svelte";
  import "../app.pcss";
  import { selectedWiki } from "../store";
  import {
    checkUpdate,
    installUpdate,
    onUpdaterEvent,
    type UpdateManifest,
  } from "@tauri-apps/api/updater";
  import { onMount } from "svelte";
  import { relaunch } from "@tauri-apps/api/process";
  import { BaseDirectory, copyFile, createDir } from "@tauri-apps/api/fs";
  import { invoke } from "@tauri-apps/api";
  import { getToastSettings, ToastType } from "$lib/utils/toasts";

  initializeStores();

  storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

  const wikiSelectPopup: PopupSettings = {
    event: "click",
    target: "wikiSelectPopup",
    placement: "top",
  };

  const modalRegistry: Record<string, ModalComponent> = {};
  let updaterModalOpen = false;
  let displayUpdateButton = false;
  let _manifest: UpdateManifest | undefined;
  onMount(() => {
    async function checkForUpdate() {
      const { shouldUpdate, manifest } = await checkUpdate();
      if (shouldUpdate) {
        displayUpdateButton = true;
        _manifest = manifest;
      }
    }
    const unlisten = async () => {
      return await onUpdaterEvent(({ error, status }) => {});
    };

    checkForUpdate();

    return () => unlisten();
  });

  async function updateApp() {
    // Keep file migrations in app and let the user run them.
    // await invoke("run_migrations", {
    //   manifest: _manifest,
    // })
    //   .then(() => {
    installUpdate()
      .then(() => {
        relaunch();
      })
      .catch((err) => {
        getToastStore().trigger(
          getToastSettings(ToastType.ERROR, `Error installing update: ${err}`),
        );
      });
    // })
    // .catch((err) => {
    //   getToastStore().trigger(
    //     getToastSettings(ToastType.ERROR, `Error running migrations: ${err}`),
    //   );
    // });

    // Read them form the run migrations function in rust side and execute them.
    // Push this app version to test.
    // Create a new test release with migrations to test with.
  }
</script>

<BaseModal bind:open={updaterModalOpen} class="w-[40rem]"></BaseModal>

<Toast position="br" rounded="rounded-none" padding="px-4 py-2" max={10} />
<Modal components={modalRegistry} />
<AppShell class="h-screen bg-white">
  <svelte:fragment slot="sidebarLeft">
    <div class="flex h-full flex-col gap-4 bg-white p-4 pt-6">
      {#if $selectedWiki.name !== ""}
        <div class="flex grow flex-col gap-y-3">
          <NavButton
            name="Pokemon"
            route="/pokemon"
            active={$page.url.pathname.includes("pokemon")}
          >
            <IconBallBasketball slot="icon" size={16} color="indigo" />
          </NavButton>
          <NavButton
            name="Game Routes"
            route="/game-routes"
            active={$page.url.pathname.includes("game-routes")}
          >
            <IconRoute2 slot="icon" size={16} color="indigo" />
          </NavButton>
          <NavButton
            name="Moves"
            route="/moves"
            active={$page.url.pathname.includes("moves")}
          >
            <IconDisc slot="icon" size={16} color="indigo" />
          </NavButton>
          <NavButton
            name="Types"
            route="/types"
            active={$page.url.pathname.includes("types")}
          >
            <IconFlame slot="icon" size={16} color="indigo" />
          </NavButton>
          <NavButton
            name="Items"
            route="/items"
            active={$page.url.pathname.includes("items")}
          >
            <IconBottleFilled slot="icon" size={16} color="indigo" />
          </NavButton>
          <NavButton
            name="Abiities"
            route="/abilities"
            active={$page.url.pathname.includes("abilities")}
          >
            <IconAdjustmentsUp slot="icon" size={16} color="indigo" />
          </NavButton>
          <NavButton
            name="Natures"
            route="/natures"
            active={$page.url.pathname.includes("natures")}
          >
            <IconTree slot="icon" size={16} color="indigo" />
          </NavButton>
          <NavButton
            name="Wiki Testing"
            route="/wiki-testing"
            active={$page.url.pathname.includes("wiki-testing")}
          >
            <IconTestPipe slot="icon" size={16} color="indigo" />
          </NavButton>
          <NavButton
            name="Home Page"
            route="/home"
            active={$page.url.pathname.includes("home")}
          >
            <IconHome slot="icon" size={16} color="indigo" />
          </NavButton>
        </div>
      {/if}
      <div class="flex flex-row items-center justify-between min-w-40">
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
  <svelte:fragment slot="pageFooter">
    <div id="page-footer" class={`${displayUpdateButton ? "" : "hidden"}`}>
      <div class="flex w-full p-2 justify-end">
        <button
          class="flex items-center self-end justify-self-end gap-1 text-sm hover:ring-offset-1 hover:ring hover:ring-green-500 rounded-md p-2"
          on:click={() => updateApp()}
        >
          <IconDownload size={18} color="green" />
          Update Available!
        </button>
      </div>
    </div>
  </svelte:fragment>
  <div class="ml-2 mt-6">
    <slot />
  </div>
</AppShell>
