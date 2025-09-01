<script lang="ts">
  import { appDataDir } from "@tauri-apps/api/path";
  import { type } from "@tauri-apps/plugin-os";
  import { selectedWiki, spawnedProcessID } from "../../store";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Command } from "@tauri-apps/plugin-shell";

  async function spawnProcess() {
    const appData = await appDataDir();
    let mkdocsFilePath = `${appData}/${$selectedWiki.name}/dist/mkdocs.yml`;
    const osType = type();
    if (osType === "windows") {
      mkdocsFilePath = mkdocsFilePath.replace(/\//g, "\\");
    }
    const command = Command.sidecar("binaries/mkdocs", [
      "serve",
      "-f",
      mkdocsFilePath,
    ]);
    $spawnedProcessID = await command.spawn();
  }

  async function killProcess() {
    $spawnedProcessID.kill();
    $spawnedProcessID = null;
  }
</script>

<div class="flex items-center gap-4">
  {#if $spawnedProcessID === null}
    <Button title="Start Wiki Server" onclick={spawnProcess}
      >Start Wiki Server</Button
    >
  {:else}
    <Button title="Stop Wiki Server" onclick={killProcess}
      >Stop Wiki Server</Button
    >
  {/if}
  {#if $spawnedProcessID !== null}
    <div class="flex flex-row gap-x-2">
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
        href={`http://localhost:8000/${$selectedWiki.name}`}
        class=" underline italic">http://localhost:8000/{$selectedWiki.name}</a
      >
    </div>
  {/if}
</div>
