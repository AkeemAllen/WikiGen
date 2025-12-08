<script lang="ts">
  import { page } from "$app/state";
  import NavButton from "$lib/components/NavButton.svelte";
  import IconChevronDown from "@tabler/icons-svelte/icons/chevron-down";
  import IconDeviceFloppy from "@tabler/icons-svelte/icons/device-floppy";
  import IconDisc from "@tabler/icons-svelte/icons/disc";
  import IconDownload from "@tabler/icons-svelte/icons/download";
  import IconFlame from "@tabler/icons-svelte/icons/flame";
  import IconHome from "@tabler/icons-svelte/icons/home";
  import IconMapRoute from "@tabler/icons-svelte/icons/map-route";
  import IconPlus from "@tabler/icons-svelte/icons/plus";
  import IconPokeball from "@tabler/icons-svelte/icons/pokeball";
  import IconTrash from "@tabler/icons-svelte/icons/trash";
  import IconStackMiddle from "@tabler/icons-svelte/icons/stack-middle";

  import "../app.css";
  import {
    selectedWiki,
    wikis,
    user,
    type User,
    type Wiki,
    spawnedProcessID,
  } from "../store";
  import { check } from "@tauri-apps/plugin-updater";
  import { onMount } from "svelte";
  import { relaunch } from "@tauri-apps/plugin-process";
  import { invoke } from "@tauri-apps/api/core";
  import { loadWikiData } from "$lib/utils/loadWiki";
  import CreateWikiModal from "$lib/components/modals/CreateWikiModal.svelte";
  import DeleteWikiModal from "$lib/components/modals/DeleteWikiModal.svelte";
  import { goto } from "$app/navigation";
  import logo from "$lib/assets/icon.png";
  import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
  import {
    BaseDirectory,
    readTextFile,
    writeTextFile,
  } from "@tauri-apps/plugin-fs";

  import LoadingModal from "$lib/components/modals/LoadingModal.svelte";
  import { load } from "@tauri-apps/plugin-store";
  import IconTestPipe from "@tabler/icons-svelte/icons/test-pipe";
  import LandingPage from "$lib/components/LandingPage.svelte";
  import { Toaster } from "$lib/components/ui/sonner/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import Github from "@lucide/svelte/icons/github";
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
  import { toast } from "svelte-sonner";
  import * as Select from "$lib/components/ui/select";
  import capitalizeWords from "$lib/utils/capitalizeWords";
  import ProcessSpawn from "$lib/components/ProcessSpawn.svelte";
  import { IconBrandGithub } from "@tabler/icons-svelte";

  type Props = {
    children?: import("svelte").Snippet;
  };
  let { children }: Props = $props();

  let updaterModalOpen = $state(false);
  let displayUpdateButton = $state(false);
  let updateStatus = $state("Updating");
  let runningMigrations = $state(false);
  let createWikiModalOpen = $state(false);
  let deleteWikiModalOpen = $state(false);

  let signingIntoGithub = $state(false);
  let loadedWiki = $derived($selectedWiki.name);

  $effect(() => {
    if ($selectedWiki.name) {
      loadWikiData($selectedWiki, toast);
      goto("/");
    }
  });

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
          toast.success("Migrations completed successfully");
          writeTextFile(
            `resources/migrations/new_migrations_present.txt`,
            "false",
            {
              baseDir: BaseDirectory.Resource,
            },
          );
        })
        .catch((error) => {
          toast.error(`Error running migrations: ${error}`);
        });
    }
  });

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
        toast.error(`Error installing update: ${err}`);
      });
    await relaunch().catch((err) => {
      toast.error(`Error relaunching app: ${err}`);
    });
  }

  async function checkAndRunMigrations() {
    await invoke("check_and_run_migrations").catch((err) => {
      toast.error(`Error running migrations: ${err}`);
    });
  }

  async function backupWiki() {
    toast.info("Backing Up Wiki...");
    await invoke("backup_wiki", {
      wikiName: $selectedWiki.name,
    })
      .then(() => {
        toast.info("Wiki Backed Up Successfully");
      })
      .catch((err) => {
        toast.error(`Error Backing Up Wiki: ${err}`);
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
          toast.error(`Failed to load token: token is ${token}`);
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

  function navigateToSelectWikisPage() {
    $selectedWiki = { name: "" } as Wiki;
    goto("/");
    // Kill the spawned process if it is running
    if ($spawnedProcessID !== null) {
      $spawnedProcessID.kill();
      $spawnedProcessID = null;
    }
  }
</script>

<LoadingModal
  message="Upating Database...This might take a while"
  bind:loading={runningMigrations}
/>

<LoadingModal message={updateStatus} bind:loading={updaterModalOpen} />
<LoadingModal
  message="Signing Into Github. Webview may take a moment to load."
  bind:loading={signingIntoGithub}
/>
<CreateWikiModal bind:open={createWikiModalOpen} />
<DeleteWikiModal bind:open={deleteWikiModalOpen} />

<Toaster richColors />

<div
  class="grid h-screen bg-gradient-to-br from-slate-50 to-slate-100 grid-rows-[auto_1fr_auto]"
>
  <header
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
        <Button
          variant="outline"
          size="sm"
          class="gap-2 cursor-pointer"
          onclick={signInToGithub}
        >
          <Github class="w-4 h-4" />
          Sign in to GitHub
        </Button>
      {:else}
        <DropdownMenu.Root>
          <DropdownMenu.Trigger>
            <div
              class="flex flex-row items-center gap-2 hover:cursor-pointer hover:bg-slate-200 rounded-2xl py-2 px-4"
            >
              <img
                src={$user.avatarUrl}
                alt="Avatar"
                class="rounded-full ring-1 ring-inset ring-gray-300 border-0 h-7"
              />
              <IconChevronDown size={16} color="gray" />
            </div>
          </DropdownMenu.Trigger>
          <DropdownMenu.Content class="flex flex-col gap-2">
            <DropdownMenu.Item onclick={signOut}>Sign Out</DropdownMenu.Item>
          </DropdownMenu.Content>
        </DropdownMenu.Root>
      {/if}
    </div>
  </header>
  {#if $selectedWiki.name !== ""}
    <div class="grid grid-cols-1 md:grid-cols-[auto_1fr] overflow-hidden">
      <aside
        class="flex flex-col bg-white gap-4 bg-touch-indigo p-4 pt-2 w-[12rem] border-r border-indigo-100 overflow-auto"
      >
        <div class="flex grow flex-col">
          <NavButton
            name="Home"
            route="/home"
            active={page.url.pathname.includes("home")}
          >
            {#snippet icon()}
              <IconHome
                size={20}
                class={`${page.url.pathname.includes("home") && "text-indigo-500"}`}
              />
            {/snippet}
          </NavButton>
          <NavButton
            name="Pokemon"
            route="/pokemon"
            active={page.url.pathname.includes("pokemon")}
          >
            {#snippet icon()}
              <IconPokeball
                size={20}
                class={`${page.url.pathname.includes("pokemon") && "text-indigo-500"}`}
              />
            {/snippet}
          </NavButton>
          <NavButton
            name="Routes"
            route="/game-routes"
            active={page.url.pathname.includes("game-routes")}
          >
            {#snippet icon()}
              <IconMapRoute
                size={20}
                class={`${page.url.pathname.includes("game-routes") && "text-indigo-500"}`}
              />
            {/snippet}
          </NavButton>
          <NavButton
            name="Moves"
            route="/moves"
            active={page.url.pathname.includes("moves")}
          >
            {#snippet icon()}
              <IconDisc
                size={20}
                class={`${page.url.pathname.includes("moves") && "text-indigo-500"}`}
              />
            {/snippet}
          </NavButton>
          <NavButton
            name="Types"
            route="/types"
            active={page.url.pathname.includes("types")}
          >
            {#snippet icon()}
              <IconFlame
                size={20}
                class={`${page.url.pathname.includes("types") && "text-indigo-500"}`}
              />
            {/snippet}
          </NavButton>
          <NavButton
            name="Attributes"
            route="/attributes"
            active={page.url.pathname.includes("attributes")}
          >
            {#snippet icon()}
              <IconStackMiddle
                size={20}
                class={`${page.url.pathname.includes("attributes") && "text-indigo-500"}`}
              />
            {/snippet}
          </NavButton>
          <NavButton
            name="Deployment"
            route="/deployment"
            active={page.url.pathname.includes("deployment")}
          >
            {#snippet icon()}
              <IconBrandGithub
                size={20}
                class={`${page.url.pathname.includes("deployment") && "text-indigo-500"}`}
              />
            {/snippet}
          </NavButton>
        </div>
      </aside>
      <main class="overflow-auto">
        {@render children?.()}
      </main>
    </div>
    <footer
      class="flex flex-row w-full justify-between p-2 pr-5 gap-x-3 bg-white items-center border-t border-indigo-100"
    >
      <ProcessSpawn />
      <div class="flex flex-row w-auto gap-x-3 bg-white items-center">
        <button
          class="self-center p-2 rounded-md
                  shadow-sm ring-1 ring-inset ring-gray-300
                  text-gray-500
                    border-0 hover:bg-indigo-500 hover:ring-0 hover:text-white ease-in-out duration-200"
          onclick={() => (createWikiModalOpen = true)}
        >
          <IconPlus size={20} />
        </button>
        <button
          class="self-center p-2 rounded-md
                    shadow-sm ring-1 ring-inset ring-gray-300
                    text-gray-500
                      border-0 hover:bg-indigo-100 hover:ring-0 hover:text-white ease-in-out duration-200"
          onclick={backupWiki}
        >
          <IconDeviceFloppy size={20} />
        </button>
        <button
          class="self-center p-2 rounded-md
                    shadow-sm ring-1 ring-inset ring-gray-300
                    text-gray-500
                      border-0 hover:bg-red-400 hover:ring-0 hover:text-white ease-in-out duration-200"
          onclick={() => (deleteWikiModalOpen = true)}
        >
          <IconTrash size={20} />
        </button>
        <Select.Root
          type="single"
          bind:value={loadedWiki}
          onValueChange={() => {
            $selectedWiki = $wikis[loadedWiki];
            loadWikiData($selectedWiki, toast);
            // Kill the spawned process if it is running
            if ($spawnedProcessID !== null) {
              $spawnedProcessID.kill();
              $spawnedProcessID = null;
            }
          }}
        >
          <Select.Trigger id="pokemon-type" class="w-[17rem]">
            {capitalizeWords(loadedWiki)}
          </Select.Trigger>
          <Select.Content>
            {#each Object.entries($wikis).map( ([name, props]) => ({ label: props.site_name, value: name }), ) as wiki}
              <Select.Item value={wiki.value} label={wiki.label}>
                {capitalizeWords(wiki.label)}
              </Select.Item>
            {/each}
          </Select.Content>
        </Select.Root>
      </div>
    </footer>
  {:else}
    <LandingPage />
  {/if}
</div>
