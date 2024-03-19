<script lang="ts">
  import { getToastStore, type ToastSettings } from "@skeletonlabs/skeleton";
  import { BaseDirectory, writeTextFile } from "@tauri-apps/api/fs";
  import { appDataDir, resourceDir } from "@tauri-apps/api/path";
  import { invoke } from "@tauri-apps/api/tauri";
  import { wikis, type WikiSettings } from "../../store";

  const toastStore = getToastStore();

  let wikiName = "";
  let wikiCodeName = "";
  let wikiDescription = "";
  let wikiAuthor = "";
  let settings: WikiSettings = {
    version_group: "",
    matchup_generation: "current",
    deployment_url: "",
  };

  $: wikiCodeName = wikiName.toLowerCase().replaceAll(" ", "-");
  $: siteUrl = `https://${wikiAuthor}.github.io/${wikiCodeName}`;
  $: repoUrl = `https://github.com/${wikiAuthor}/${wikiCodeName}`;
  $: siteName = wikiName;

  const wikiCreatedToast: ToastSettings = {
    message: "Wiki Created",
    timeout: 5000,
    hoverable: true,
    background: "variant-filled-success",
  };

  async function createWiki() {
    const directory = await appDataDir();
    const resourceDirectory = await resourceDir();
    $wikis[wikiCodeName] = {
      name: wikiCodeName,
      description: wikiDescription,
      author: wikiAuthor,
      site_name: siteName,
      repo_url: repoUrl,
      site_url: siteUrl,
      settings: settings,
    };
    await writeTextFile("wikis.json", JSON.stringify($wikis), {
      dir: BaseDirectory.AppData,
    });
    await invoke("create_wiki", {
      wikiName,
      wikiDescription,
      wikiAuthor,
      siteName,
      dir: directory,
      resourceDir: resourceDirectory,
    }).then(() => {
      toastStore.trigger(wikiCreatedToast);
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
    disabled
    class="p-1 border rounded-md disabled: bg-gray-100 text-gray-400"
    placeholder="Code Name"
    bind:value={wikiCodeName}
  />
  <textarea
    class="p-1 border rounded-md"
    placeholder="Wiki Description"
    bind:value={wikiDescription}
  />
  <input
    class="p-1 border rounded-md"
    placeholder="Recommended: Your Github Username"
    bind:value={wikiAuthor}
  />
  <input
    class="p-1 border rounded-md"
    placeholder="Can be left blank for now"
    bind:value={settings.deployment_url}
  />
  <select
    class="p-1 border rounded-md bg-white"
    bind:value={settings.matchup_generation}
  >
    <option value="current">Current</option>
    <option value="gen-1">Gen 1</option>
    <option value="gen-2">Gen 2</option>
  </select>
  <input
    disabled
    class="p-1 border rounded-md disabled: bg-gray-100 text-gray-400"
    placeholder="Repo URL"
    bind:value={repoUrl}
  />
  <input
    disabled
    class="p-1 border rounded-md disabled: bg-gray-100 text-gray-400"
    placeholder="Site URL"
    bind:value={siteUrl}
  />
  <input
    class="p-1 border rounded-md"
    placeholder="Version Group"
    bind:value={settings.version_group}
  />
  <button
    disabled={wikiDescription === "" ||
      wikiName === "" ||
      wikiAuthor === "" ||
      settings.version_group === ""}
    class="btn btn-md w-30 bg-blue-600"
    on:click={createWiki}>Create Wiki</button
  >
</div>
