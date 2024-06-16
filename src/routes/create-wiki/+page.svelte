<script lang="ts">
import Button from "$lib/components/Button.svelte";
import TextInput from "$lib/components/TextInput.svelte";
import { getToastStore, type ToastSettings } from "@skeletonlabs/skeleton";
import { BaseDirectory, writeTextFile } from "@tauri-apps/api/fs";
import { invoke } from "@tauri-apps/api/tauri";
import { wikis, type WikiSettings } from "../../store";

const toastStore = getToastStore();

let wikiName = "";
let wikiCodeName = "";
let wikiDescription = "";
let wikiAuthor = "";
let settings: WikiSettings = {
  deployment_url: "",
};

let loading: boolean = false;

$: wikiCodeName = wikiName.toLowerCase().replaceAll(" ", "-");
$: siteUrl = `https://${wikiAuthor}.github.io/${wikiCodeName}`;
$: repoUrl = `https://github.com/${wikiAuthor}/${wikiCodeName}`;
$: siteName = wikiName;

async function createWiki() {
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
  })
    .then((result) => {
      loading = false;
      toastStore.trigger({
        message: result as string,
        timeout: 5000,
        hoverable: true,
        background: "variant-filled-success",
      });
    })
    .catch((error) => {
      loading = false;
      toastStore.trigger({
        message: error as string,
        timeout: 5000,
        hoverable: true,
        background: "variant-filled-error",
      });
    });
}
</script>

<div class="grid grid-cols-3 gap-16 self-center p-4">
  <div>
    <h2 class="text-base font-semibold leading-7 text-gray-900">New Wiki</h2>
    <p class="mt-1 text-sm leading-6 text-gray-600">
      Create a New Wiki to document all of the changes made to your Rom Hack.
    </p>
  </div>
  <div
    class="card col-span-2 flex w-[45rem] flex-col gap-y-5 rounded-lg border p-5"
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
        class="mt-2 block w-full rounded-md border-0 py-1.5 pl-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 disabled:bg-gray-100 disabled:text-gray-400 sm:text-sm sm:leading-6"
        placeholder="Wiki Description"
        bind:value={wikiDescription}
      />
    </div>
    <TextInput
      id="wiki-author"
      label="Wiki Author"
      placeholder="Recommended: Your Github Username"
      bind:value={wikiAuthor}
    />

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
    <Button
      class="w-32"
      disabled={wikiDescription === "" ||
          wikiName === "" ||
          wikiAuthor === "" ||
          loading === true}
      onClick={createWiki}
      title="Create Wiki"
      loading={loading}
    />
  </div>
</div>
