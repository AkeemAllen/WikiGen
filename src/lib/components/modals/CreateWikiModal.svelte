<script lang="ts">
  import { IconPlus } from "@tabler/icons-svelte";
  import { wikis, selectedWiki } from "../../../store";
  import {
    getToastStore,
    popup,
    type PopupSettings,
  } from "@skeletonlabs/skeleton";
  import { loadWikiData } from "$lib/utils/loadWiki";
  import BaseModal from "$lib/components/BaseModal.svelte";
  import TextInput from "$lib/components/TextInput.svelte";
  import Button from "$lib/components/Button.svelte";
  import { BaseDirectory, writeTextFile } from "@tauri-apps/api/fs";
  import { invoke } from "@tauri-apps/api";
  import { getToastSettings, ToastType } from "$lib/utils/toasts";

  const toastStore = getToastStore();

  export let open = false;

  let wikiName = "";
  let wikiCodeName = "";
  let wikiDescription = "";

  let loading: boolean = false;

  $: wikiCodeName = wikiName.toLowerCase().replaceAll(" ", "-");

  async function createWiki() {
    loading = true;

    $wikis[wikiCodeName] = {
      name: wikiCodeName,
      description: wikiDescription,
      author: "",
      site_name: wikiName,
      repo_url: "",
      site_url: "",
      settings: { deployment_url: "" },
    };

    await writeTextFile("wikis.json", JSON.stringify($wikis), {
      dir: BaseDirectory.AppData,
    });
    await invoke("create_wiki", {
      wikiName: wikiCodeName,
      wikiDescription,
      wikiAuthor: "",
      siteName: wikiName,
    })
      .then((result) => {
        loading = false;
        open = false;
        toastStore.trigger(
          getToastSettings(ToastType.SUCCESS, result as string),
        );
        wikiName = "";
        wikiCodeName = "";
        wikiDescription = "";
      })
      .catch((error) => {
        loading = false;
        open = false;
        wikiName = "";
        wikiCodeName = "";
        wikiDescription = "";
        toastStore.trigger(getToastSettings(ToastType.ERROR, error as string));
      });
  }
</script>

<BaseModal bind:open class="w-[20rem]">
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
  <Button
    class="w-32"
    disabled={wikiDescription === "" || wikiName === "" || loading === true}
    title="Create Wiki"
    onClick={createWiki}
    {loading}
  />
</BaseModal>
