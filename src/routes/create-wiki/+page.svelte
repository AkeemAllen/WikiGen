<script lang="ts">
  import SelectInput from "$lib/SelectInput.svelte";
  import TextInput from "$lib/TextInput.svelte";
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
    version_group: "red-blue",
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
      wikiName: wikiCodeName,
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

<div class="grid grid-cols-3 p-4 gap-16 self-center">
  <div>
    <h2 class="text-base font-semibold leading-7 text-gray-900">New Wiki</h2>
    <p class="mt-1 text-sm leading-6 text-gray-600">
      Create a New Wiki to document all of the changes made to your Rom Hack.
    </p>
  </div>
  <div
    class="flex flex-col col-span-2 gap-y-5 card border rounded-lg p-5 w-[45rem]"
  >
    <div class="grid grid-cols-2 gap-16">
      <TextInput
        id="wikiName"
        label="Wiki Name"
        placeholder="Enter Wiki Name"
        bind:value={wikiName}
      />
      <TextInput
        id="code-name"
        disabled={true}
        label="Code Name"
        placeholder="Code Name"
        bind:value={wikiCodeName}
      />
    </div>
    <div>
      <label
        for="wiki-description"
        class="block text-sm font-medium leading-6 text-gray-900"
        >Wiki Description</label
      >
      <textarea
        id="wiki-description"
        class="block w-full pl-2 mt-2 rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6 disabled:bg-gray-100 disabled:text-gray-400"
        placeholder="Wiki Description"
        bind:value={wikiDescription}
      />
    </div>
    <div class="grid grid-cols-2 gap-16">
      <TextInput
        id="wiki-author"
        label="Wiki Author"
        placeholder="Recommended: Your Github Username"
        bind:value={wikiAuthor}
      />

      <SelectInput
        id="matchup-generation"
        label="Matchup Generation"
        value={settings.matchup_generation}
        options={[
          { label: "Current", value: "current" },
          { label: "Generation 1", value: "gen-1" },
          { label: "Generation 2", value: "gen-2" },
        ]}
      />
    </div>

    <TextInput
      id="deployment-url"
      label="Deployment URL"
      placeholder="Can be left blank for now"
      bind:value={settings.deployment_url}
    />
    <div class="grid grid-cols-2 gap-16">
      <TextInput
        id="repo_url"
        disabled={true}
        label="Repo Url"
        placeholder="Repo URL"
        bind:value={repoUrl}
      />
      <TextInput
        id="site_url"
        disabled={true}
        label="Site Url"
        placeholder="Site URL"
        bind:value={siteUrl}
      />
    </div>
    <SelectInput
      id="version-group"
      label="Version Group"
      value={settings.version_group}
      options={[
        { label: "Red - Blue", value: "red-blue" },
        { label: "Generation 1", value: "gen-1" },
        { label: "Generation 2", value: "gen-2" },
      ]}
    />

    <button
      disabled={wikiDescription === "" ||
        wikiName === "" ||
        wikiAuthor === "" ||
        settings.version_group === ""}
      class="rounded-md bg-indigo-600 w-32 px-3 py-2 text-sm font-semibold text-white
      shadow-sm hover:bg-indigo-500 focus-visible:outline
      focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600
      disabled:bg-indigo-400"
      on:click={createWiki}>Create Wiki</button
    >
  </div>
</div>
