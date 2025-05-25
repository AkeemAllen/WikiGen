<script lang="ts">
  import { run } from "svelte/legacy";
  // import { getToastStore } from "@skeletonlabs/skeleton";
  import { appDataDir } from "@tauri-apps/api/path";
  import { type } from "@tauri-apps/plugin-os";
  // import { invoke } from "@tauri-apps/api/tauri";
  import { selectedWiki } from "../../store";

  // type Response = {
  //   process_id: number;
  //   message: string;
  //   status: string;
  // };

  // const toastStore = getToastStore();
  let mkdocsFilePath: string = $state("");

  // let isProcessRunning: boolean = false;

  // // onMount(async () => {
  // //   await invoke("check_process_status", {
  // //     processId: $processId,
  // //   }).then((response) => {
  // //     if ((response as Response).status === "Running") {
  // //       isProcessRunning = true;
  // //     }
  // //   });
  // // });

  // async function spawnProcess() {
  //   const appData = await appDataDir();
  //   await invoke("spawn_mkdocs_process", {
  //     mkdocsFilePath: `${appData}${$selectedWiki.name}/dist/mkdocs.yml`,
  //     port: 4000,
  //   })
  //     .then((response) => {
  //       const typedResponse = response as Response;
  //       $processId = typedResponse.process_id;
  //       isProcessRunning = true;
  //     })
  //     .catch((error) => {
  //       const typedError = error as Response;
  //       if (typedError.status === "Occupied") {
  //         toastStore.trigger({
  //           message: typedError.message,
  //           background: "bg-red-500",
  //         });
  //       }
  //     });
  // }

  // async function killProcess() {
  //   await invoke("kill_mkdocs_process", {
  //     processId: $processId,
  //   }).then((response) => {
  //     console.log(response);
  //     $processId = 0;
  //     isProcessRunning = false;
  //   });
  // }
  async function getMkdocsDirectory(wikiName: string): Promise<string> {
    const appData = await appDataDir();
    let mkdocsFilePath = `${appData}/${wikiName}/dist`;
    const osType = type();
    if (osType === "windows") {
      mkdocsFilePath = mkdocsFilePath.replace(/\//g, "\\");
    }
    return mkdocsFilePath;
  }
  run(() => {
    getMkdocsDirectory($selectedWiki.name).then((response) => {
      mkdocsFilePath = response;
    });
  });
</script>

<!-- <div class="mt-6 ml-2">
  <div class="w-44">
    {#if !isProcessRunning}
      <Button title="Start Wiki Server" onClick={spawnProcess} />
    {:else}
      <Button title="Stop Wiki Server" onClick={killProcess} />
    {/if}
  </div>
  {#if isProcessRunning}
    <div class="flex flex-row mt-6 gap-x-2">
      <span class="relative flex self-center h-3 w-3">
        <span
          class="animate-ping absolute inline-flex h-full w-full rounded-full bg-green-400 opacity-75"
        ></span>
        <span class="relative inline-flex rounded-full h-3 w-3 bg-green-500"
        ></span>
      </span>
      Wiki Running at
      <a
        target="_blank"
        href={`http://localhost:4000/${$selectedWiki.name}`}
        class=" underline italic">http://localhost:4000/{$selectedWiki.name}</a
      >
    </div>
  {/if}
</div> -->
<p class=" text-xl font-bold">Under Developement</p>
<p class="text-sm">
  Currently developing the ability to launch and manage the wiki server from the
  app. However, due to complications, this feature is currently paused.
  <br />
  Below are instructions for launching the server manually
</p>
<br />

<div class="flex flex-col gap-y-1">
  <p class="text-sm">
    1. Ensure <a
      href="https://www.python.org/downloads/"
      target="_blank"
      class="underline">Python</a
    > is installed on your system.
  </p>
  <p class="text-sm">
    2. Run <code class="rounded-md bg-gray-200 px-1"
      >pip install mkdocs mkdocs-material</code
    > in a terminal to install mkdocs and its material theme
  </p>
  <p class="text-sm">
    3. In your terminal, navigate to the wiki's distribution directory with the
    following command: <code class="rounded-md bg-gray-200 px-1"
      >cd {mkdocsFilePath}</code
    >
    and then run
    <code class="rounded-md bg-gray-200 px-1">mkdocs serve</code>
  </p>
  <p>
    Note: If you run into Issue running <code
      class="rounded-md bg-gray-200 px-1">mkdocs serve</code
    >, try
    <code class="rounded-md bg-gray-200 px-1">python -m mkdocs serve</code>
  </p>
  <p class="text-sm">
    4. You should now see the server running on
    <a href="http://localhost:8000" target="_blank" class="underline"
      >http://localhost:8000/{$selectedWiki.name}</a
    >. From there you can monitor the changes you make to the wiki
  </p>
</div>
