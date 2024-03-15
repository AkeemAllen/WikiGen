<script lang="ts">
  import { getToastStore, type ToastSettings } from "@skeletonlabs/skeleton";
  import { BaseDirectory, createDir, writeTextFile } from "@tauri-apps/api/fs";
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
    if (wikiName === "" || wikiDescription === "") {
      return;
    }
    if ($wikis[wikiName] !== undefined) {
      return;
    }
    wikis.update((w) => {
      w[wikiName] = {
        name: wikiName,
        description: wikiDescription,
      };
      return w;
    });
    await createDir(`data/${wikiName}`, {
      dir: BaseDirectory.AppData,
      recursive: true,
    })
      .then(() => {
        toastStore.trigger(dirCreatedToast);
      })
      .catch((e) => {
        console.error(e);
      });
    await writeTextFile(`data/wikis.json`, JSON.stringify($wikis), {
      dir: BaseDirectory.AppData,
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
