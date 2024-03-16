<script lang="ts">
  import { getToastStore, type ToastSettings } from "@skeletonlabs/skeleton";
  import { BaseDirectory, writeTextFile } from "@tauri-apps/api/fs";
  import { appDataDir, resourceDir } from "@tauri-apps/api/path";
  import { invoke } from "@tauri-apps/api/tauri";
  import { wikis } from "../../store";

  const toastStore = getToastStore();

  let wikiName = "";
  let wikiDescription = "";

  const dirCreatedToast: ToastSettings = {
    message: "Wiki Created",
    timeout: 5000,
    hoverable: true,
    background: "variant-filled-success",
  };

  async function createWiki() {
    const directory = await appDataDir();
    const resourceDirectory = await resourceDir();
    $wikis[wikiName] = {
      name: wikiName,
      description: wikiDescription,
      author: "author",
      site_name: "site_name",
      repo_url: "repo_url",
      site_url: "site_url",
    };
    await writeTextFile("wikis.json", JSON.stringify($wikis), {
      dir: BaseDirectory.AppData,
    });
    await invoke("create_wiki", {
      wikiName,
      wikiDescription,
      wikiAuthor: "author",
      siteName: "site_name",
      dir: directory,
      resourceDir: resourceDirectory,
    }).then(() => {
      toastStore.trigger(dirCreatedToast);
    });
  }
</script>

<div class="flex flex-col gap-4 w-1/2">
  <p class="mt-4 font-bold text-xl w-1/2">New Wiki</p>
  <input
    class="p-1 border rounded-md"
    placeholder="Wiki Name"
    bind:value={wikiName}
  />
  <input
    class="p-1 border rounded-md"
    placeholder="Wiki Description"
    bind:value={wikiDescription}
  />
  <button
    disabled={wikiDescription === "" || wikiName === ""}
    class="btn btn-md w-30 bg-blue-600"
    on:click={createWiki}>Create Wiki</button
  >
</div>
