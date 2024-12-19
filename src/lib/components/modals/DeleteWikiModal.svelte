<script lang="ts">
  import BaseModal from "$lib/components/BaseModal.svelte";
  import Button from "$lib/components/Button.svelte";
  import { BaseDirectory, removeDir, writeTextFile } from "@tauri-apps/api/fs";
  import { wikis, selectedWiki } from "../../../store";
  import { getToastStore } from "@skeletonlabs/skeleton";
  import MultiSelect from "svelte-multiselect";

  const toastStore = getToastStore();

  interface Props {
    open?: boolean;
  }

  let { open = $bindable(false) }: Props = $props();
  let wikisToDelete: string[] = $state([]);
  let directoriesRemoved: boolean = false;
  let wikiJsonUpdated: boolean = false;

  let wikiListOptions = $derived(Object.keys($wikis).filter(
    (wiki) => wiki !== $selectedWiki.name,
  ));

  async function deleteWikis() {
    for (const wiki of wikisToDelete) {
      await removeDir(wiki, {
        dir: BaseDirectory.AppData,
        recursive: true,
      }).then(() => {
        directoriesRemoved = true;
      });
      $wikis = Object.fromEntries(
        Object.entries($wikis).filter(([key, _]) => key !== wiki),
      );
    }
    await writeTextFile("wikis.json", JSON.stringify($wikis), {
      dir: BaseDirectory.AppData,
    }).then(() => {
      wikiJsonUpdated = true;
    });
    if (directoriesRemoved && wikiJsonUpdated) {
      toastStore.trigger({
        message: "Wikis Deleted Successfully",
        background: "variant-filled-success",
      });
      directoriesRemoved = false;
      wikiJsonUpdated = false;
    }
    open = false;
    wikisToDelete = [];
  }
</script>

<BaseModal bind:open>
  <div>
    <label for="wikis" class="block text-sm font-medium leading-6 text-gray-900"
      >Wikis To Delete</label
    >
    <MultiSelect
      id="wikis"
      bind:selected={wikisToDelete}
      options={wikiListOptions}
      style="height: 36px; border-color: rgb(209 213 219); border-radius: 0.375rem; box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05); font-size: 0.875rem;"
    />
  </div>
  <Button onClick={() => deleteWikis()} title="Delete Selected Wikis" />
</BaseModal>
