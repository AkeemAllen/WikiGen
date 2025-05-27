<script lang="ts">
  import { Button } from "$lib/components/ui/button/index.js";
  import { BaseDirectory, remove, writeTextFile } from "@tauri-apps/plugin-fs";
  import { wikis, selectedWiki } from "../../../store";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import * as Select from "$lib/components/ui/select/index.js";
  import { toast } from "svelte-sonner";

  type Props = {
    open?: boolean;
  };

  let { open = $bindable(false) }: Props = $props();
  let wikisToDelete: string[] = $state([]);
  let directoriesRemoved: boolean = false;
  let wikiJsonUpdated: boolean = false;

  let wikiListOptions = $derived(
    Object.keys($wikis).filter((wiki) => wiki !== $selectedWiki.name),
  );

  async function deleteWikis() {
    for (const wiki of wikisToDelete) {
      await remove(wiki, {
        baseDir: BaseDirectory.AppData,
        recursive: true,
      }).then(() => {
        directoriesRemoved = true;
      });
      $wikis = Object.fromEntries(
        Object.entries($wikis).filter(([key, _]) => key !== wiki),
      );
    }
    await writeTextFile("wikis.json", JSON.stringify($wikis), {
      baseDir: BaseDirectory.AppData,
    }).then(() => {
      wikiJsonUpdated = true;
    });
    if (directoriesRemoved && wikiJsonUpdated) {
      toast.success("Wikis Deleted Successfully", {
        action: {
          label: "Close",
          onClick: () => {},
        },
      });
      directoriesRemoved = false;
      wikiJsonUpdated = false;
    }
    open = false;
    wikisToDelete = [];
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>Wikis To Delete</Dialog.Title>
    </Dialog.Header>
    <Select.Root type="multiple" bind:value={wikisToDelete}>
      <Select.Trigger class="">
        {#if wikisToDelete.length <= 0}
          Select Wikis To Delete
        {:else}
          {#each wikisToDelete as wiki}
            <div
              class="px-4 py-1 rounded-full text-sm font-medium bg-indigo-100 text-indigo-800"
            >
              {wiki}
            </div>
          {/each}
        {/if}
      </Select.Trigger>
      <Select.Content>
        {#each wikiListOptions as name}
          <Select.Item value={name}>{name}</Select.Item>
        {/each}
      </Select.Content>
    </Select.Root>
    <Dialog.Footer>
      <Button
        onclick={deleteWikis}
        class="bg-red-400 hover:bg-red-500 cursor-pointer">Delete Wikis</Button
      >
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
