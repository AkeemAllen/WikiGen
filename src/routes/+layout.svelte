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
  } from "@tauri-apps/api/updater";
  import { onMount } from "svelte";
  import { relaunch } from "@tauri-apps/api/process";
  import {
    BaseDirectory,
    readTextFile,
    writeTextFile,
  } from "@tauri-apps/api/fs";
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
  let updateStatus = "";
  let runningMigrations = false;

  onMount(() => {
    async function runMigrations() {
      await readTextFile("resources/migrations/migration.json", {
        dir: BaseDirectory.Resource,
      })
        .then((res) => {
          let migration: { migrations_present: boolean; ran: boolean } =
            JSON.parse(res);
          if (!migration.migrations_present) {
            return;
          }
          if (migration.ran) {
            return;
          }
          runningMigrations = true;
          invoke("run_migrations")
            .then(() => {
              runningMigrations = false;
              toastStore.trigger(
                getToastSettings(
                  ToastType.SUCCESS,
                  "Migrations ran successfully",
                ),
              );
              writeTextFile(
                "resources/migrations/migration.json",
                JSON.stringify({ ran: true, migrations_present: false }),
                { dir: BaseDirectory.Resource },
              ).catch((err) => {
                toastStore.trigger(
                  getToastSettings(
                    ToastType.ERROR,
                    `Error writing migration file: ${err}`,
                  ),
                );
              });
            })
            .catch((err) => {
              toastStore.trigger(
                getToastSettings(
                  ToastType.ERROR,
                  `Error running migrations: ${err}`,
                ),
              );
            });
        })
        .catch((err) => {
          toastStore.trigger(
            getToastSettings(
              ToastType.ERROR,
              `Error reading migration file: ${err}`,
            ),
          );
        });
    }
    async function checkForUpdate() {
      const { shouldUpdate } = await checkUpdate();
      if (shouldUpdate) {
        displayUpdateButton = true;
      }
    }

    checkForUpdate();
    runMigrations();
  });

  const toastStore = getToastStore();

  async function updateApp() {
    updaterModalOpen = true;
    const unlisten = await onUpdaterEvent(({ error, status }) => {
      if (status === "PENDING") {
        updateStatus = "Update is pending...";
        toastStore.trigger(
          getToastSettings(ToastType.INFO, "Update is pending..."),
        );
      }

      if (status === "DONE") {
        updateStatus = "Update applied successfully";
        toastStore.trigger(
          getToastSettings(ToastType.INFO, "Update applied successfully"),
        );
      }

      if (status === "ERROR") {
        updateStatus = `Error applying update: ${error}`;
        toastStore.trigger(
          getToastSettings(ToastType.ERROR, `Error applying update: ${error}`),
        );
      }
    });
    await installUpdate()
      .then(() => {
        relaunch().catch((err) => {
          toastStore.trigger(
            getToastSettings(ToastType.ERROR, `Error relaunching app: ${err}`),
          );
        });
        unlisten();
      })
      .catch((err) => {
        toastStore.trigger(
          getToastSettings(ToastType.ERROR, `Error installing update: ${err}`),
        );
      });
  }
</script>

<BaseModal bind:open={runningMigrations} close={() => {}} class="w-[30rem]">
  <div class="flex gap-2 items-center">
    <svg
      aria-hidden="true"
      class="h-5 w-5 animate-spin fill-indigo-600 text-gray-200"
      viewBox="0 0 100 101"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      ><path
        d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z"
        fill="currentColor"
      /><path
        d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z"
        fill="currentFill"
      /></svg
    >
    <p>Current Upating Database...This might take a while</p>
  </div>
</BaseModal>
<BaseModal bind:open={updaterModalOpen} close={() => {}} class="w-[30rem]">
  <div class="flex gap-2 items-center">
    <svg
      aria-hidden="true"
      class="h-5 w-5 animate-spin fill-indigo-600 text-gray-200"
      viewBox="0 0 100 101"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      ><path
        d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z"
        fill="currentColor"
      /><path
        d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z"
        fill="currentFill"
      /></svg
    >
    <p>{updateStatus}</p>
  </div>
</BaseModal>

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
      <div class="flex flex-row items-center justify-between w-[12rem]">
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
