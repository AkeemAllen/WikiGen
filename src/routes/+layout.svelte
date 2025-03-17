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
    IconBottle,
    IconBrandGithub,
    IconDeviceFloppy,
    IconDisc,
    IconDownload,
    IconFlame,
    IconHome,
    IconMapRoute,
    IconPlus,
    IconPokeball,
    IconSeeding,
    IconTrash,
    IconTreadmill,
  } from "@tabler/icons-svelte";
  import "../app.pcss";
  import { selectedWiki, wikis } from "../store";
  import {
    checkUpdate,
    installUpdate,
    onUpdaterEvent,
  } from "@tauri-apps/api/updater";
  import { onMount } from "svelte";
  import { relaunch } from "@tauri-apps/api/process";
  import { invoke } from "@tauri-apps/api";
  import { getToastSettings, ToastType } from "$lib/utils/toasts";
  import { loadWikiData } from "$lib/utils/loadWiki";
  import CreateWikiModal from "$lib/components/modals/CreateWikiModal.svelte";
  import DeleteWikiModal from "$lib/components/modals/DeleteWikiModal.svelte";
  import SelectInput from "$lib/components/SelectInput.svelte";
  import { goto, invalidateAll } from "$app/navigation";
  import logo from "$lib/assets/icon.png";
  import { PUBLIC_CLIENT_ID, PUBLIC_REDIRECT_URI } from "$env/static/public";
  import { WebviewWindow } from "@tauri-apps/api/window";

  initializeStores();

  storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

  const modalRegistry: Record<string, ModalComponent> = {};
  let updaterModalOpen = false;
  let displayUpdateButton = false;
  let updateStatus = "";
  let runningMigrations = false;
  let createWikiModalOpen = false;
  let deleteWikiModalOpen = false;
  let connectedToGithub = false;
  let backingUpWiki = false;

  onMount(() => {
    async function runMigrations() {
      invoke("run_migrations")
        .then((res) => {
          runningMigrations = false;
          if (res !== "skipping") {
            toastStore.trigger(
              getToastSettings(
                ToastType.SUCCESS,
                "Migrations ran successfully",
              ),
            );
          }
        })
        .catch((err) => {
          toastStore.trigger(
            getToastSettings(
              ToastType.ERROR,
              `Error running migrations: ${err}`,
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

  function loadSelectedWiki(e: any) {
    $selectedWiki = $wikis[e.target.value];
    loadWikiData($selectedWiki, toastStore);
    goto("/");
  }

  function isActivePage(pageName: string) {
    if ($page.url.pathname.includes(pageName)) return true;

    return false;
  }

  async function backupWiki() {
    toastStore.trigger(getToastSettings(ToastType.INFO, "Backing Up Wiki..."));
    await invoke("backup_wiki", {
      wikiName: $selectedWiki.name,
    })
      .then(() => {
        toastStore.trigger(
          getToastSettings(ToastType.INFO, "Wiki Backed Up Successfully"),
        );
      })
      .catch((err) => {
        toastStore.trigger(
          getToastSettings(ToastType.ERROR, `Error Backing Up Wiki: ${err}`),
        );
      });
  }

  async function signInToGithub() {
    // Have application create custom protocol
    // Open Webview to the signin endpoint
    // Have endpoint redirect to github.com/login/oatuh/authorize
    // get code and redirect to /authorize endpoint, get access code, create cookie
    // Open desktop app using custom protocol and pass cookie as a parameter.
    // const url = new URL("https://github.com/login/oauth/authorize");
    // const params = new URLSearchParams();
    // params.append("client_id", PUBLIC_CLIENT_ID);
    // // params.append(
    // //   "redirect_uri",
    // //   `http://127.0.0.1:${window.location.port}auth`,
    // // );
    // params.append("scope", "read:user public_repo");
    // url.search = params.toString();
    // const webview = new WebviewWindow("GithubAccessRequest", {
    //   url: url.toString(),
    //   title: "Github Access Request",
    // });
    // const unlisten = await webview.listen("event", (event) => {
    //   console.log(event);
    // });
    // webview.once("event", () => {
    //   console.log("Event");
    // });
    // webview.onCloseRequested(() => {
    //   unlisten();
    //   // Ping server again for new token and add that to state.
    // });
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

<CreateWikiModal bind:open={createWikiModalOpen} />
<DeleteWikiModal bind:open={deleteWikiModalOpen} />

<Toast position="br" rounded="rounded-none" padding="px-4 py-2" max={10} />
<Modal components={modalRegistry} />
<AppShell class="h-screen bg-indigo-100">
  <svelte:fragment slot="header">
    <div
      class="bg-white h-[60px] px-4 flex border-b border-indigo-100 items-center justify-between"
    >
      <div class="flex flex-row items-center">
        <img src={logo} alt="WikiGen Logo" width="40rem" />
        <h1>WikiGen</h1>
      </div>
      <div class="flex flex-row items-center gap-1">
        {#if !connectedToGithub}
          <button
            class="p-2 rounded-md text-sm text-gray-400 hover:bg-gray-100"
            on:click={signInToGithub}>Sign in to github</button
          >
        {:else}
          <button
            class="self-center p-2 rounded-full
              shadow-sm ring-1 ring-inset ring-gray-300
              text-gray-500
                border-0 hover:bg-indigo-600 hover:text-white ease-in-out duration-200"
            use:popup={{
              event: "hover",
              target: "connectToGithubToolTip",
              placement: "bottom",
            }}
          >
            <IconBrandGithub size={20} />
          </button>
          <div data-popup="connectToGithubToolTip">
            <p class="card p-1 text-sm">Connect To Github</p>

            <div class="arrow bg-surface-100-800-token"></div>
          </div>
        {/if}
      </div>
    </div>
  </svelte:fragment>
  <svelte:fragment slot="sidebarLeft">
    {#if $selectedWiki.name !== ""}
      <div
        class="flex h-full flex-col bg-white gap-4 bg-touch-indigo p-4 pt-2 w-[12rem] border-r border-indigo-100"
      >
        <div class="flex grow flex-col">
          <NavButton
            name="Home"
            route="/home"
            active={$page.url.pathname.includes("home")}
          >
            <IconHome
              slot="icon"
              size={20}
              class={`${$page.url.pathname.includes("home") && "text-indigo-500"}`}
            />
          </NavButton>
          <NavButton
            name="Pokemon"
            route="/pokemon"
            active={$page.url.pathname.includes("pokemon")}
          >
            <IconPokeball
              slot="icon"
              size={20}
              class={`${$page.url.pathname.includes("pokemon") && "text-indigo-500"}`}
            />
          </NavButton>
          <NavButton
            name="Routes"
            route="/game-routes"
            active={$page.url.pathname.includes("game-routes")}
          >
            <IconMapRoute
              slot="icon"
              size={20}
              class={`${$page.url.pathname.includes("game-routes") && "text-indigo-500"}`}
            />
          </NavButton>
          <NavButton
            name="Moves"
            route="/moves"
            active={$page.url.pathname.includes("moves")}
          >
            <IconDisc
              slot="icon"
              size={20}
              class={`${$page.url.pathname.includes("moves") && "text-indigo-500"}`}
            />
          </NavButton>
          <NavButton
            name="Types"
            route="/types"
            active={$page.url.pathname.includes("types")}
          >
            <IconFlame
              slot="icon"
              size={20}
              class={`${$page.url.pathname.includes("types") && "text-indigo-500"}`}
            />
          </NavButton>
          <p class="mb-2 mt-4 text-sm text-slate-400 font-semibold">
            Attributes
          </p>
          <NavButton
            name="Items"
            route="/items"
            active={$page.url.pathname.includes("items")}
          >
            <IconBottle
              slot="icon"
              size={20}
              class={`${$page.url.pathname.includes("items") && "text-indigo-500"}`}
            />
          </NavButton>
          <NavButton
            name="Abiities"
            route="/abilities"
            active={$page.url.pathname.includes("abilities")}
          >
            <IconTreadmill
              slot="icon"
              size={20}
              class={`${$page.url.pathname.includes("abilities") && "text-indigo-500"}`}
            />
          </NavButton>
          <NavButton
            name="Natures"
            route="/natures"
            active={$page.url.pathname.includes("natures")}
          >
            <IconSeeding
              slot="icon"
              size={20}
              class={`${$page.url.pathname.includes("natures") && "text-indigo-500"}`}
            />
          </NavButton>
          <!-- <NavButton
              name="Wiki Testing"
              route="/wiki-testing"
              active={$page.url.pathname.includes("wiki-testing")}
            >
              <IconTestPipe slot="icon" size={16} color="indigo" />
            </NavButton> -->
        </div>
        <!-- <div class="flex flex-row items-center justify-between w-[12rem]">
            <p>
              {$selectedWiki.name ? $selectedWiki.site_name : "Select Wiki"}
            </p>
            <span use:popup={wikiSelectPopup} class="hover:cursor-pointer">
              <IconDotsVertical size={"20"} />
            </span>
          </div>
          <WikiSelectMenu /> -->
      </div>
    {/if}
  </svelte:fragment>
  <svelte:fragment slot="pageHeader">
    <div class="flex flex-row justify-end mr-10 gap-x-3 items-center"></div>
  </svelte:fragment>
  <div class="my-3 mr-5 ml-5 p-2 bg-white rounded-md">
    <slot />
  </div>
  <svelte:fragment slot="footer">
    {#if $selectedWiki.name !== ""}
      <div
        class="flex flex-row w-full p-2 justify-end pr-5 gap-x-3 bg-white items-center border-t border-indigo-100"
      >
        <button
          class="self-center p-2 rounded-md
          shadow-sm ring-1 ring-inset ring-gray-300
          text-gray-500
            border-0 hover:bg-indigo-500 hover:ring-0 hover:text-white ease-in-out duration-200"
          on:click={() => (createWikiModalOpen = true)}
        >
          <IconPlus size={20} />
        </button>
        <!-- <div data-popup="addIconToolTip">
          <p class="card p-1 text-sm">Create New Wiki</p>
          <div class="arrow bg-surface-100-800-token"></div>
        </div> -->
        <!-- <button
          class="self-center p-2 rounded-md
          shadow-sm ring-1 ring-inset ring-gray-300
          text-gray-500
            border-0 hover:bg-indigo-600 hover:text-white hover:ring-0 ease-in-out duration-200"
          use:popup={{
            event: "hover",
            target: "previewWikiToolTip",
            placement: "bottom",
          }}
        >
          <IconTestPipe size={20} />
        </button>
        <div data-popup="previewWikiToolTip">
          <p class="card p-1 text-sm">Preview Wiki</p>

          <div class="arrow bg-surface-100-800-token" />
        </div> -->
        <!-- <div class="flex flex-row w-full p-2 justify-end mr-10 gap-x-3"> -->
        <button
          class="self-center p-2 rounded-md
            shadow-sm ring-1 ring-inset ring-gray-300
            text-gray-500
              border-0 hover:bg-indigo-500 hover:ring-0 hover:text-white ease-in-out duration-200"
          on:click={backupWiki}
        >
          <IconDeviceFloppy size={20} />
        </button>
        <!-- <div data-popup="backupWikiToolTip">
          <p class="card p-1 text-sm">Backup Wiki</p>
          <div class="arrow bg-surface-100-800-token"></div>
        </div> -->
        <button
          class="self-center p-2 rounded-md
            shadow-sm ring-1 ring-inset ring-gray-300
            text-gray-500
              border-0 hover:bg-red-400 hover:ring-0 hover:text-white ease-in-out duration-200"
          on:click={() => (deleteWikiModalOpen = true)}
        >
          <IconTrash size={20} />
        </button>
        <SelectInput
          options={Object.entries($wikis).map(([name, props]) => ({
            label: props.site_name,
            value: name,
          }))}
          value={$selectedWiki.name}
          onChange={loadSelectedWiki}
          class="w-[17rem] mt-0"
        />
        {#if displayUpdateButton}
          <button
            class="flex items-center self-end justify-self-end gap-1 shadow-sm border-0
          text-sm text-gray-500 ring-inset ring-gray-300 hover:bg-indigo-600
          hover:text-white ease-in-out duration-200 rounded-md p-2"
            on:click={() => updateApp()}
          >
            <IconDownload size={18} />
            Update Available!
          </button>
        {/if}
      </div>
    {/if}
    <!-- </div> -->
  </svelte:fragment>
</AppShell>
