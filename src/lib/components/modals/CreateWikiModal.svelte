<script lang="ts">
  import LoaderCircleIcon from "@lucide/svelte/icons/loader-circle";
  import { wikis } from "../../../store";
  import { BaseDirectory, writeTextFile } from "@tauri-apps/plugin-fs";
  import { invoke } from "@tauri-apps/api/core";
  import { Button } from "$lib/components/ui/button/index.js";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import { toast } from "svelte-sonner";

  type Props = {
    open?: boolean;
  };

  let { open = $bindable(false) }: Props = $props();

  let wikiName = $state("");
  let wikiCodeName = $derived(wikiName.toLowerCase().replaceAll(" ", "-"));
  let wikiDescription = $state("");

  let loading: boolean = $state(false);

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
      baseDir: BaseDirectory.AppData,
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
        toast.success(result as string, {
          action: {
            label: "Close",
            onClick: () => {},
          },
        });
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
        toast.error(error as string, {
          action: {
            label: "Close",
            onClick: () => {},
          },
        });
      });
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Content class="sm:max-w-[425px]">
    <Dialog.Header>
      <Dialog.Title>Create New Wiki</Dialog.Title>
    </Dialog.Header>
    <div class="grid gap-4 py-4">
      <div class="grid grid-cols-4 items-center gap-4">
        <Label for="name" class="text-right">Wiki Name</Label>
        <Input id="name" bind:value={wikiName} class="col-span-3" />
      </div>
      <div class="grid grid-cols-4 items-center gap-4">
        <Label for="codename" class="text-right">CodeName</Label>
        <Input
          id="codename"
          bind:value={wikiCodeName}
          class="col-span-3 disabled"
          disabled={true}
        />
      </div>
      <div class="grid grid-cols-4 items-center gap-4">
        <Label for="description" class="text-right">Description</Label>
        <Input
          id="description"
          bind:value={wikiDescription}
          class="col-span-3"
        />
      </div>
    </div>
    <Dialog.Footer>
      <Button
        type="submit"
        onclick={createWiki}
        disabled={loading ||
          wikiName === "" ||
          wikiName.length < 3 ||
          wikiDescription === "" ||
          wikiDescription.length < 3}
      >
        {#if loading}
          <LoaderCircleIcon class="animate-spin" />
        {/if}
        Create Wiki</Button
      >
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
