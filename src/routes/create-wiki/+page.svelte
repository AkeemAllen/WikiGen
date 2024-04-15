<script lang="ts">
  import SelectInput from "$lib/components/SelectInput.svelte";
  import TextInput from "$lib/components/TextInput.svelte";
  import { getToastStore, type ToastSettings } from "@skeletonlabs/skeleton";
  import { BaseDirectory, writeTextFile } from "@tauri-apps/api/fs";
  import { type } from "@tauri-apps/api/os";
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

  let loading: boolean = false;

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
    let completeResourceDirectory = `${resourceDirectory}/resources/`;

    const osType = await type();

    // Check if platform is windows and change the directory path
    if (osType === "Windows_NT") {
      completeResourceDirectory = `${resourceDirectory}\\resources\\`;
    }

    loading = true;

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
      resourceDir: completeResourceDirectory,
    }).then(() => {
      loading = false;
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
        bind:value={settings.matchup_generation}
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
      bind:value={settings.version_group}
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
        settings.version_group === "" ||
        loading === true}
      class="{loading &&
        'flex justify-between'} rounded-md bg-indigo-600 w-32 px-3 py-2 text-sm font-semibold text-white
      shadow-sm hover:bg-indigo-500 focus-visible:outline
      focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600
      disabled:bg-indigo-400"
      on:click={createWiki}
    >
      {#if loading}
        <svg
          aria-hidden="true"
          class="w-5 h-5 text-gray-200 animate-spin fill-indigo-600"
          viewBox="0 0 100 101"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
          ><path
            d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z"
            fill="currentColor"
          /><path
            d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z"
            fill="currentFill"
          /></svg
        >
      {/if}
      Create Wiki</button
    >
  </div>
</div>
