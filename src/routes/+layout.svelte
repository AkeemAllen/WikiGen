<script lang="ts">
  import { run } from "svelte/legacy";

  import { page } from "$app/stores";
  import NavButton from "$lib/components/NavButton.svelte";
  import BaseModal from "$lib/components/BaseModal.svelte";
  import {
    arrow,
    autoUpdate,
    computePosition,
    flip,
    offset,
    shift,
  } from "@floating-ui/dom";
  import type { ModalComponent } from "@skeletonlabs/skeleton";
  import {
    AppShell,
    Modal,
    Toast,
    getToastStore,
    initializeStores,
    popup,
    storePopup,
  } from "@skeletonlabs/skeleton";
  import IconTreadmill from "@tabler/icons-svelte/icons/treadmill";
  import IconBottle from "@tabler/icons-svelte/icons/bottle";
  import IconChevronDown from "@tabler/icons-svelte/icons/chevron-down";
  import IconDeviceFloppy from "@tabler/icons-svelte/icons/device-floppy";
  import IconDisc from "@tabler/icons-svelte/icons/disc";
  import IconDownload from "@tabler/icons-svelte/icons/download";
  import IconFlame from "@tabler/icons-svelte/icons/flame";
  import IconHome from "@tabler/icons-svelte/icons/home";
  import IconMapRoute from "@tabler/icons-svelte/icons/map-route";
  import IconPlus from "@tabler/icons-svelte/icons/plus";
  import IconPokeball from "@tabler/icons-svelte/icons/pokeball";
  import IconSeedling from "@tabler/icons-svelte/icons/seedling";
  import IconTrash from "@tabler/icons-svelte/icons/trash";

  import "../app.pcss";
  import { selectedWiki, wikis, user, type User, type Wiki } from "../store";
  import { check } from "@tauri-apps/plugin-updater";
  import { onMount } from "svelte";
  import { relaunch } from "@tauri-apps/plugin-process";
  import { invoke } from "@tauri-apps/api/core";
  import { getToastSettings, ToastType } from "$lib/utils/toasts";
  import { loadWikiData } from "$lib/utils/loadWiki";
  import CreateWikiModal from "$lib/components/modals/CreateWikiModal.svelte";
  import DeleteWikiModal from "$lib/components/modals/DeleteWikiModal.svelte";
  import SelectInput from "$lib/components/SelectInput.svelte";
  import { goto } from "$app/navigation";
  import logo from "$lib/assets/icon.png";
  import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
  import {
    BaseDirectory,
    readTextFile,
    writeTextFile,
  } from "@tauri-apps/plugin-fs";
  import { appDataDir } from "@tauri-apps/api/path";
  import { type } from "@tauri-apps/plugin-os";
  import LoadingModal from "$lib/components/modals/LoadingModal.svelte";
  import { load } from "@tauri-apps/plugin-store";
  interface Props {
    children?: import("svelte").Snippet;
  }

  let { children }: Props = $props();

  initializeStores();

  storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

  const modalRegistry: Record<string, ModalComponent> = {};
  let updaterModalOpen = $state(false);
  let displayUpdateButton = $state(false);
  let updateStatus = $state("Updating");
  let runningMigrations = $state(false);
  let createWikiModalOpen = $state(false);
  let deleteWikiModalOpen = $state(false);
  let deployingWiki = $state(false);
  let deployWikiFinalStepsModal = $state(false);
  let signingIntoGithub = $state(false);
  let osType = $state("");

  let mkdocsFilePath: string = $state("");

  async function getMkdocsDirectory(wikiName: string): Promise<string> {
    const appData = await appDataDir();
    let mkdocsFilePath = `${appData}${wikiName}/dist`;
    osType = type();
    if (osType === "windows") {
      mkdocsFilePath = mkdocsFilePath.replace(/\//g, "\\");
    } else if (osType === "macos") {
      mkdocsFilePath = mkdocsFilePath.replace(/\s/g, "\\ ");
    }
    return mkdocsFilePath;
  }

  async function checkForUpdate() {
    const update = await check();
    if (update?.available) {
      displayUpdateButton = true;
    }
  }

  onMount(async () => {
    setInterval(checkForUpdate, 60000);

    const new_migrations_present = await readTextFile(
      `resources/migrations/new_migrations_present.txt`,
      {
        baseDir: BaseDirectory.Resource,
      },
    ).catch((error) => {
      console.error("Error reading new_migrations_present.txt:", error);
      return "false";
    });

    if (new_migrations_present.trim() === "true") {
      await checkAndRunMigrations()
        .then(() => {
          toastStore.trigger(
            getToastSettings(
              ToastType.SUCCESS,
              "Migrations completed successfully",
            ),
          );
          writeTextFile(
            `resources/migrations/new_migrations_present.txt`,
            "false",
            {
              baseDir: BaseDirectory.Resource,
            },
          );
        })
        .catch((error) => {
          console.error("Error running migrations:", error);
        });
    }
  });

  const toastStore = getToastStore();

  async function updateApp() {
    updaterModalOpen = true;
    const update = await check();
    if (!update) return;

    await update
      .downloadAndInstall((progress) => {
        switch (progress.event) {
          case "Started":
            updateStatus = "Update Started";
            // contentLength = progress.data.contentLength as number;
            break;
          case "Progress":
            updateStatus = `Update In Progress`;
            // downloadProgress += progress.data.chunkLength as number;
            break;
          case "Finished":
            updateStatus = "Update Completed";
            break;
        }
      })
      .catch((err) => {
        toastStore.trigger(
          getToastSettings(ToastType.ERROR, `Error installing update: ${err}`),
        );
      });
    await relaunch().catch((err) => {
      toastStore.trigger(
        getToastSettings(ToastType.ERROR, `Error relaunching app: ${err}`),
      );
    });
  }

  async function checkAndRunMigrations() {
    await invoke("check_and_run_migrations").catch((err) => {
      toastStore.trigger(
        getToastSettings(ToastType.ERROR, `Error running migrations: ${err}`),
      );
    });
  }

  function loadSelectedWiki(e: any) {
    $selectedWiki = $wikis[e.target.value];
    loadWikiData($selectedWiki, toastStore);
    goto("/");
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
    signingIntoGithub = true;
    const url = new URL("https://github.com/login/oauth/authorize");
    const params = new URLSearchParams();
    params.append("client_id", "Ov23li9oWejO62cA6Kee");
    params.append("scope", "read:user public_repo");

    url.search = params.toString();

    const webview = new WebviewWindow("github-access-request", {
      url: url.toString(),
      title: "Github Access Request",
      alwaysOnTop: true,
    });

    const unlisten = await webview.listen(
      "token-loaded",
      async (event: any) => {
        const store = await load("store.json");
        const token = await store.get<string>("token");

        if (token === undefined || token === "") {
          webview.close();
          toastStore.trigger(
            getToastSettings(
              ToastType.ERROR,
              `Failed to load token: token is ${token}`,
            ),
          );
        }

        let data = parseJwt(token as string);

        $user = data;
        signingIntoGithub = false;

        webview.close();
      },
    );

    webview.onCloseRequested(() => {
      unlisten();
    });
  }

  async function signOut() {
    const store = await load("store.json");
    await store.delete("token");
    await store.save();
    $user = {
      userName: "",
      avatarUrl: "",
      isConnected: false,
    };
  }

  function parseJwt(jsonWebToken: string): User {
    var base64Url = jsonWebToken.split(".")[1];
    var base64 = base64Url.replace(/-/g, "+").replace(/_/g, "/");
    var jsonPayload = decodeURIComponent(
      window
        .atob(base64)
        .split("")
        .map(function (c) {
          return "%" + ("00" + c.charCodeAt(0).toString(16)).slice(-2);
        })
        .join(""),
    );

    let parsedJson = JSON.parse(jsonPayload);

    let loggedInUser: User = {
      userName: parsedJson.user_name as string,
      avatarUrl: parsedJson.avatar as string,
      isConnected: true,
    };

    return loggedInUser;
  }

  async function deployWiki() {
    if ($selectedWiki.name === "") {
      toastStore.trigger(
        getToastSettings(
          ToastType.ERROR,
          "Wiki needs to be selected before deploying",
        ),
      );
      return;
    }
    deployingWiki = true;
    const store = await load("store.json");
    const token = await store.get<string>("token");

    await fetch("https://wikigen-auth.fly.dev/create-repo", {
      method: "POST",
      body: JSON.stringify({
        token: token,
        wikiName: $selectedWiki.name,
      }),
      headers: {
        "Content-Type": "application/json",
      },
    })
      .then((res) => res.json())
      .then((res) => {
        if (res.status === 401) {
          toastStore.trigger(
            getToastSettings(
              ToastType.ERROR,
              "Token has expired. Relogin to deploy wiki",
            ),
          );
          signOut();
        }
        console.log(res);
        return res;
      })
      .then(async (res) => {
        if ($selectedWiki.settings.deployment_url === "") {
          $wikis[$selectedWiki.name].settings.deployment_url = res.ssh_url;
          await writeTextFile("wikis.json", JSON.stringify($wikis), {
            baseDir: BaseDirectory.AppData,
          });
        }
        deployingWiki = true;
        return res;
      })
      .then(async (deploy_result) => {
        invoke("deploy_wiki", {
          wikiName: $selectedWiki.name,
          sshUrl: $selectedWiki.settings.deployment_url,
        }).then(() => {
          toastStore.trigger(
            getToastSettings(ToastType.SUCCESS, `Wiki Preparation Complete!`),
          );
          deployingWiki = false;
          deployWikiFinalStepsModal = true;
        });
      })
      .catch((err) => {
        toastStore.trigger(
          getToastSettings(
            ToastType.ERROR,
            `Error while preparing wiki for deployment!: ${err}`,
          ),
        );
        deployingWiki = false;
      });
  }

  function navigateToSelectWikisPage() {
    $selectedWiki = { name: "" } as Wiki;
    goto("/");
  }
  run(() => {
    getMkdocsDirectory($selectedWiki.name).then((response) => {
      mkdocsFilePath = response;
    });
  });
</script>

<LoadingModal
  message="Current Upating Database...This might take a while"
  bind:loading={runningMigrations}
/>
<LoadingModal
  message="Preparing Wiki For Deployment"
  bind:loading={deployingWiki}
/>
<LoadingModal message={updateStatus} bind:loading={updaterModalOpen} />
<LoadingModal
  message="Signing Into Github. Webview may take a moment to load."
  bind:loading={signingIntoGithub}
/>
<BaseModal
  bind:open={deployWikiFinalStepsModal}
  close={() => (deployWikiFinalStepsModal = false)}
  class="min-w-[30rem]"
>
  <h2 class="text-lg font-bold leading-6 text-gray-900">Final Deploy Steps</h2>
  <div class="grid gap-3">
    <p>
      1.
      {#if osType === "Windows_NT"}
        Press Windows Start Key, type Terminal, and open it
      {:else if osType === "Darwin"}
        Open spotlight search, type Terminal, and open it
      {/if}
    </p>
    <h2 class="text-md font-semibold leading-3 text-gray-900">
      Copy and paste the below commands
    </h2>
    <p>
      2. Navigate to the Wiki
      <br />
      <span class="code font-semibold"> cd {mkdocsFilePath}</span>
    </p>
    <p>
      3. Update the main branch
      <br />
      <span class="code font-semibold"> git push -u origin main</span>
    </p>
    <p>
      4. Deploy the Wiki
      <br />
      <span class="code font-semibold"> mkdocs gh-deploy</span>
    </p>
    <p>
      4. Once done, your docs should be available at the below URL. Note that it
      may take a few minutes to load.
      <br />
      <a
        href={`https://${$user.userName.toLowerCase()}.github.io/${$selectedWiki.name}/`}
        target="_blank"
      >
        <span class="code font-semibold"
          >https://{$user.userName.toLowerCase()}.github.io/{$selectedWiki.name}/</span
        >
      </a>
    </p>
  </div>
</BaseModal>

<CreateWikiModal bind:open={createWikiModalOpen} />
<DeleteWikiModal bind:open={deleteWikiModalOpen} />

<Toast position="br" rounded="rounded-none" padding="px-4 py-2" max={10} />
<Modal components={modalRegistry} />
<AppShell class="h-screen bg-indigo-100">
  {#snippet header()}
    <div
      class="bg-white h-[60px] px-4 flex border-b border-indigo-100 items-center justify-between"
    >
      <button onclick={navigateToSelectWikisPage}>
        <div class="flex flex-row items-center">
          <img src={logo} alt="WikiGen Logo" width="40rem" />
          <h1>WikiGen</h1>
        </div>
      </button>
      <div class="flex flex-row items-center gap-1">
        {#if displayUpdateButton}
          <button
            class="flex items-center gap-1 border-0
                    text-sm text-gray-400 ring-gray-300 hover:bg-indigo-600
                    hover:text-white ease-in-out duration-200 rounded-md p-2"
            onclick={() => updateApp()}
          >
            <IconDownload size={16} />
            Update Available!
          </button>
        {/if}
        {#if !$user.isConnected}
          <button
            class="p-2 rounded-md text-sm text-gray-400 hover:bg-gray-100"
            onclick={signInToGithub}>Sign in to github</button
          >
        {:else}
          <div
            class="flex flex-row items-center gap-2 hover:cursor-pointer hover:bg-gray-200 rounded-2xl py-2 px-4"
            use:popup={{
              event: "click",
              target: "profileMenu",
            }}
          >
            <img
              src={$user.avatarUrl}
              alt="Avatar"
              class="rounded-full ring-1 ring-inset ring-gray-300 border-0 h-7"
            />
            <IconChevronDown size={16} color="gray" />
          </div>
          <ul
            class="card z-10 w-36 grid-cols-1 p-2 shadow-xl"
            data-popup="profileMenu"
          >
            <button
              onclick={deployWiki}
              class="w-full rounded-md p-2 text-left text-sm hover:bg-slate-300"
              >Deploy Wiki</button
            >
            <button
              onclick={signOut}
              class="w-full rounded-md p-2 text-left text-sm hover:bg-slate-300"
              >Sign Out</button
            >
          </ul>
        {/if}
      </div>
    </div>
  {/snippet}
  {#snippet sidebarLeft()}
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
            {#snippet icon()}
              <IconHome
                size={20}
                class={`${$page.url.pathname.includes("home") && "text-indigo-500"}`}
              />
            {/snippet}
          </NavButton>
          <NavButton
            name="Pokemon"
            route="/pokemon"
            active={$page.url.pathname.includes("pokemon")}
          >
            {#snippet icon()}
              <IconPokeball
                size={20}
                class={`${$page.url.pathname.includes("pokemon") && "text-indigo-500"}`}
              />
            {/snippet}
          </NavButton>
          <NavButton
            name="Routes"
            route="/game-routes"
            active={$page.url.pathname.includes("game-routes")}
          >
            {#snippet icon()}
              <IconMapRoute
                size={20}
                class={`${$page.url.pathname.includes("game-routes") && "text-indigo-500"}`}
              />
            {/snippet}
          </NavButton>
          <NavButton
            name="Moves"
            route="/moves"
            active={$page.url.pathname.includes("moves")}
          >
            {#snippet icon()}
              <IconDisc
                size={20}
                class={`${$page.url.pathname.includes("moves") && "text-indigo-500"}`}
              />
            {/snippet}
          </NavButton>
          <NavButton
            name="Types"
            route="/types"
            active={$page.url.pathname.includes("types")}
          >
            {#snippet icon()}
              <IconFlame
                size={20}
                class={`${$page.url.pathname.includes("types") && "text-indigo-500"}`}
              />
            {/snippet}
          </NavButton>
          <p class="mb-2 mt-4 text-sm text-slate-400 font-semibold">
            Attributes
          </p>
          <NavButton
            name="Items"
            route="/items"
            active={$page.url.pathname.includes("items")}
          >
            {#snippet icon()}
              <IconBottle
                size={20}
                class={`${$page.url.pathname.includes("items") && "text-indigo-500"}`}
              />
            {/snippet}
          </NavButton>
          <NavButton
            name="Abiities"
            route="/abilities"
            active={$page.url.pathname.includes("abilities")}
          >
            {#snippet icon()}
              <IconTreadmill
                size={20}
                class={`${$page.url.pathname.includes("abilities") && "text-indigo-500"}`}
              />
            {/snippet}
          </NavButton>
          <NavButton
            name="Natures"
            route="/natures"
            active={$page.url.pathname.includes("natures")}
          >
            {#snippet icon()}
              <IconSeedling
                size={20}
                class={`${$page.url.pathname.includes("natures") && "text-indigo-500"}`}
              />
            {/snippet}
          </NavButton>
          <!-- <NavButton
                name="Wiki Testing"
                route="/wiki-testing"
                active={$page.url.pathname.includes("wiki-testing")}
              >
                <IconTestPipe slot="icon" size={16} color="indigo" />
              </NavButton> -->
        </div>
      </div>
    {/if}
  {/snippet}
  {#snippet pageHeader()}
    <div class="flex flex-row justify-end mr-10 gap-x-3 items-center"></div>
  {/snippet}
  <div class="my-3 mr-5 ml-5 p-2 bg-white rounded-md">
    {@render children?.()}
  </div>
  {#snippet footer()}
    {#if $selectedWiki.name !== ""}
      <div
        class="flex flex-row w-full p-2 justify-end pr-5 gap-x-3 bg-white items-center border-t border-indigo-100"
      >
        <button
          class="self-center p-2 rounded-md
            shadow-sm ring-1 ring-inset ring-gray-300
            text-gray-500
              border-0 hover:bg-indigo-500 hover:ring-0 hover:text-white ease-in-out duration-200"
          onclick={() => (createWikiModalOpen = true)}
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
                border-0 hover:bg-indigo-100 hover:ring-0 hover:text-white ease-in-out duration-200"
          onclick={backupWiki}
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
          onclick={() => (deleteWikiModalOpen = true)}
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
      </div>
    {/if}
    <!-- </div> -->
  {/snippet}
</AppShell>
