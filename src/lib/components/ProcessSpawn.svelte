<script lang="ts">
  import { appDataDir } from "@tauri-apps/api/path";
  import { type } from "@tauri-apps/plugin-os";
  import { selectedWiki, spawnedProcessID } from "../../store";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Command } from "@tauri-apps/plugin-shell";
  import * as Dialog from "$lib/components/ui/dialog";
  import { toast } from "svelte-sonner";
  import { exists, BaseDirectory } from "@tauri-apps/plugin-fs";

  let openPythonInstallationModal = false;

  async function checkForPython(): Promise<boolean> {
    const checkForPython = await Command.create("python-check", ["--version"])
      .execute()
      .then((results) => {
        return results;
      })
      .catch((error) => {
        toast.error(`Failed Python check: ${error}`);
        return { stdout: "" };
      });

    if (!checkForPython.stdout.includes("Python")) {
      openPythonInstallationModal = true;
      return false;
    }

    return true;
  }

  async function installPython() {
    const installPython = await Command.sidecar("binaries/uv", [
      "python",
      "install",
    ]).execute();

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
    let venvDir = `${appData}/.venv`;

    const osType = type();
    if (osType === "windows") {
      mkdocsFilePath = mkdocsFilePath.replace(/\//g, "\\");
      venvDir = venvDir.replace(/\//g, "\\");
    }

    const isPythonInstalled = await checkForPython();
    if (!isPythonInstalled) {
      return;
    }

    // Check for virtual environment
    const isVirtualEnvInstalled = await exists(".venv", {
      baseDir: BaseDirectory.AppData,
    });

    if (!isVirtualEnvInstalled) {
      const virtualEnv = await Command.sidecar("binaries/uv", [
        "venv",
        venvDir,
      ]).execute();

      if (virtualEnv.code !== 0) {
        toast.error(
          `Failed to create virtual environment: ${virtualEnv.stderr}`,
        );
        return;
      }
    }

    const command = Command.sidecar("binaries/uv", [
      "run",
      "--python",
      `${venvDir}`,
      "mkdocs",
      "serve",
      "-f",
      mkdocsFilePath,
    ]);

    $spawnedProcessID = await command.spawn();
  }

  async function killProcess() {
    await $spawnedProcessID.kill();

    const osType = type();
    if (osType === "windows") {
      await Command.create("kill-mkdocs-win", [
        "/F",
        "/FI",
        "COMMANDLINE eq *mkdocs*",
      ]).execute();
    } else {
      await Command.create("kill-mkdocs", ["-f", "mkdocs"]).execute();
    }

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
    <Button onclick={installPython}>Install Python</Button>
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
