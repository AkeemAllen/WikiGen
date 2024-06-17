<script lang="ts">
import { onMount } from "svelte";

import { getToastStore } from "@skeletonlabs/skeleton";
import { type } from "@tauri-apps/api/os";
import { appDataDir } from "@tauri-apps/api/path";
import { invoke } from "@tauri-apps/api/tauri";
import { selectedWiki } from "../../store";
import { processId } from "../../store/mkdocs";
import Button from "$lib/components/Button.svelte";

type Response = {
  process_id: number;
  message: string;
  status: string;
};

const toastStore = getToastStore();
let mkdocsFilePath: string = "";

let isProcessRunning: boolean = false;

onMount(async () => {
  await invoke("check_process_status", {
    processId: $processId,
  }).then((response) => {
    if ((response as Response).status === "Running") {
      isProcessRunning = true;
    }
  });
});
onMount(() => {
  getMkdocsDirectory().then((response) => {
    mkdocsFilePath = response;
  });
});

async function spawnProcess() {
  const appData = await appDataDir();
  await invoke("spawn_mkdocs_process", {
    mkdocsFilePath: `${appData}${$selectedWiki.name}/dist/mkdocs.yml`,
    port: 4000,
  })
    .then((response) => {
      const typedResponse = response as Response;
      $processId = typedResponse.process_id;
      isProcessRunning = true;
    })
    .catch((error) => {
      const typedError = error as Response;
      if (typedError.status === "Occupied") {
        toastStore.trigger({
          message: typedError.message,
          background: "bg-red-500",
        });
      }
    });
}

async function killProcess() {
  await invoke("kill_mkdocs_process", {
    processId: $processId,
  }).then((response) => {
    console.log(response);
    $processId = 0;
    isProcessRunning = false;
  });
}
async function getMkdocsDirectory(): Promise<string> {
  const appData = await appDataDir();
  let mkdocsFilePath = `${appData}${$selectedWiki.name}/dist`;
  const osType = await type();
  if (osType === "Windows_NT") {
    mkdocsFilePath = mkdocsFilePath.replace(/\//g, "\\");
  }
  return mkdocsFilePath;
}
</script>

<div class="ml-2 mt-6">
  <div class="w-44">
    {#if !isProcessRunning}
      <Button title="Start Wiki Server" onClick={spawnProcess} />
    {:else}
      <Button title="Stop Wiki Server" onClick={killProcess} />
    {/if}
  </div>
  {#if isProcessRunning}
    <div class="mt-6 flex flex-row gap-x-2">
      <span class="relative flex h-3 w-3 self-center">
        <span
          class="absolute inline-flex h-full w-full animate-ping rounded-full bg-green-400 opacity-75"
        ></span>
        <span class="relative inline-flex h-3 w-3 rounded-full bg-green-500"
        ></span>
      </span>
      Wiki Running at
      <a
        target="_blank"
        href={`http://localhost:4000/${$selectedWiki.name}`}
        class=" italic underline">http://localhost:4000/{$selectedWiki.name}</a
      >
    </div>
  {/if}
</div>
<!-- <p class=" text-xl font-bold">Under Developement</p>
<p class="text-sm">
  Currently developing the ability to launch and manage the wiki server from the
  app. However, due to complications, this feature is currenctly paused.
  <br />
  Below are instructions for launching the server manually
</p>
<br />

<div class="flex flex-col gap-y-1">
  <p class="text-sm">
    1. Ensure <a href="https://www.python.org" target="_blank" class="underline"
      >Python</a
    > is installed on your system
  </p>
  <p class="text-sm">
    2. Run <code class="bg-gray-200 rounded-md px-1"
      >pip install mkdocs mkdocs-material</code
    > in a terminal to install mkdocs and its material theme
  </p>
  <p class="text-sm">
    3. Navigate to the wiki dist directory <code
      class="bg-gray-200 rounded-md px-1">{mkdocsFilePath}</code
    >
    and run <code class="bg-gray-200 rounded-md px-1">mkdocs serve</code>
  </p>
  <p class="text-sm">
    4. You should now see the server running on
    <a href="http://localhost:8000" target="_blank" class="underline"
      >http://localhost:8000/_wiki_name_</a
    >. From there you can monitor the changes you make to the wiki
  </p>
</div> -->
