<script lang="ts">
  import { IconPlus } from "@tabler/icons-svelte";
  import { wikis, selectedWiki } from "../store";
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

  const addIconToolTip: PopupSettings = {
    event: "hover",
    target: "addIconToolTip",
    placement: "bottom",
  };

  const toastStore = getToastStore();

  let createModalOpen = false;

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
        createModalOpen = false;
        toastStore.trigger(
          getToastSettings(ToastType.SUCCESS, result as string),
        );
      })
      .catch((error) => {
        loading = false;
        createModalOpen = false;
        toastStore.trigger(getToastSettings(ToastType.ERROR, error as string));
      });
  }
</script>

<BaseModal bind:open={createModalOpen} class="w-[20rem]">
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

{#if $selectedWiki.name == ""}
  <div class="flex justify-center h-screen">
    <div class="flex flex-col text-center mt-24 gap-4">
      <p class="text-xl font-bold">Current Wikis</p>
      <div class="grid grid-cols-3 gap-3 w-[60rem]">
        {#each Object.entries($wikis) as [codeName, properties]}
          <button
            class="bg-gray-100 p-3 rounded-md border-2
            hover:bg-indigo-600 hover:text-white hover:font-semibold
            hover:ring hover:ring-indigo-500 ease-in-out duration-200"
            on:click={() => {
              $selectedWiki = $wikis[codeName];
              loadWikiData($selectedWiki, toastStore);
            }}>{properties.site_name}</button
          >
        {/each}
      </div>
      <button
        class="bg-gray-100 self-center p-2 rounded-md
        border-2 hover:bg-indigo-600 hover:text-white hover:ring
        hover:ring-indigo-500 ease-in-out duration-200"
        use:popup={addIconToolTip}
        on:click={() => (createModalOpen = true)}
      >
        <IconPlus size={25} />
      </button>
      <div data-popup="addIconToolTip">
        <p class="card p-1">Create New Wiki</p>

        <div class="arrow bg-surface-100-800-token" />
      </div>
    </div>
  </div>
{/if}
