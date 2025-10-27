<script lang="ts">
  import { appDataDir } from "@tauri-apps/api/path";
  import LoaderCircleIcon from "@lucide/svelte/icons/loader-circle";
  import { type } from "@tauri-apps/plugin-os";
  import { selectedWiki, spawnedProcessID } from "../../store";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Command, type ChildProcess } from "@tauri-apps/plugin-shell";
  import * as Dialog from "$lib/components/ui/dialog";
  import { toast } from "svelte-sonner";
  import { exists, BaseDirectory } from "@tauri-apps/plugin-fs";

  let openPythonInstallationModal = false;
  let installingPython = false;

  async function checkForPython(): Promise<boolean> {
    let command: ChildProcess<string>;

    if (type() === "windows") {
      command = await Command.create("python-win", ["--version"]).execute();
    } else {
      command = await Command.create("python-unix", ["--version"]).execute();
    }
    if (!command.stdout.includes("Python")) {
      openPythonInstallationModal = true;
      return false;
    }

    return true;
  }

  async function installPython() {
    installingPython = true;
    const installPython = await Command.sidecar("binaries/uv", [
      "python",
      "install",
    ]).execute();

    installingPython = false;

    if (installPython.code === 0) {
      openPythonInstallationModal = false;
      toast.success("Python installed successfully!");
    } else {
      toast.error(`Failed to install Python: ${installPython.stderr}`);
    }
  }

  async function spawnProcess() {
    const appData = await appDataDir();
    let mkdocsFilePath = `${appData}/${$selectedWiki.name}/dist/mkdocs.yml`;
    let pythonCommand = "python-unix";

    if (type() === "windows") {
      mkdocsFilePath = mkdocsFilePath.replace(/\//g, "\\");
      pythonCommand = "python-win";
    }

    const isPythonInstalled = await checkForPython();
    if (!isPythonInstalled) {
      return;
    }

    const checkMkdocs = await Command.create(pythonCommand, [
      "-m",
      "pip",
      "show",
      "mkdocs",
      mkdocsFilePath,
    ]).execute();

    if (checkMkdocs.code !== 0) {
      toast.info(`Mkdocs not installed. Installing. Please wait a few seconds`);
      const installMkdocs = await Command.create(pythonCommand, [
        "-m",
        "pip",
        "install",
        "mkdocs",
        "mkdocs-material",
      ]).execute();
      if (installMkdocs.code !== 0) {
        toast.error(
          `Failed to install Mkdocs: ${installMkdocs.stderr}\nUnable to launch server`,
        );
        return;
      }
    }

    $spawnedProcessID = await Command.create(pythonCommand, [
      "-m",
      "mkdocs",
      "serve",
      "-f",
      mkdocsFilePath,
    ]).spawn();

    toast.success("Wiki server started");
  }

  async function killProcess() {
    await $spawnedProcessID.kill();

    $spawnedProcessID = null;
    toast.success("Wiki server stopped");
  }
</script>

<Dialog.Root bind:open={openPythonInstallationModal}>
  <Dialog.Content class="min-w-[40rem]">
    <Dialog.Header>Install Python</Dialog.Header>
    <div class="grid gap-3">
      <p>
        Python is not installed. Python is required to start the server from
        WikiGen.
      </p>
      <p>Do you want WikiGen to install Python for you?</p>
    </div>
    <Button onclick={installPython} disabled={installingPython}>
      {#if installingPython}
        <LoaderCircleIcon class="animate-spin" />
      {/if}
      Install Python</Button
    >
  </Dialog.Content>
</Dialog.Root>

<div class="flex items-center gap-4">
  {#if $spawnedProcessID === null}
    <Button onclick={spawnProcess}>Start Wiki Server</Button>
  {:else}
    <Button onclick={killProcess}>Stop Wiki Server</Button>
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
