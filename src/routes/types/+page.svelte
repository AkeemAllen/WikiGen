<script lang="ts">
  import { IconTrash } from "@tabler/icons-svelte";
  import { types } from "../../store/types";
  import Button from "$lib/components/Button.svelte";
  import TextInput from "$lib/components/TextInput.svelte";
  import { BaseDirectory, writeTextFile } from "@tauri-apps/api/fs";
  import { selectedWiki } from "../../store";
  import { getToastStore } from "@skeletonlabs/skeleton";

  const toastStore = getToastStore();
  let newType = "";

  async function addType() {
    if (newType === "") return;
    if ($types.includes(newType.toLowerCase().replaceAll(" ", "-"))) {
      toastStore.trigger({
        message: "Type already exists",
        background: "variant-filled-error",
      });
      return;
    }
    types.update((t) => [...t, newType.toLowerCase().replaceAll(" ", "-")]);
    newType = "";
    await writeTextFile(
      `${$selectedWiki.name}/data/types.json`,
      JSON.stringify({ types: $types }),
      { dir: BaseDirectory.AppData },
    );
  }
  async function deleteType(selectedType: string) {
    $types = $types.filter(
      (type) => type !== selectedType.toLowerCase().replaceAll(" ", "-"),
    );
    await writeTextFile(
      `${$selectedWiki.name}/data/types.json`,
      JSON.stringify({ types: $types }),
      { dir: BaseDirectory.AppData },
    );
  }
</script>

<div class="flex flex-row gap-3">
  <TextInput
    bind:value={newType}
    placeholder="New Type"
    inputHandler={(e) => {
      newType = e.target.value.toLowerCase().replaceAll(" ", "-");
    }}
  />
  <Button
    class="mt-2 mb-4 w-44"
    title="Add New Type"
    onClick={addType}
    disabled={newType === ""}
  />
</div>
<div class="grid grid-cols-3 gap-3">
  {#each $types as type}
    <div class="card flex flex-row items-center justify-between px-2 py-1">
      {type}
      <button
        class="btn rounded-sm p-2 hover:cursor-pointer hover:bg-gray-300"
        on:click={() => deleteType(type)}
      >
        <IconTrash size={16} />
      </button>
    </div>
  {/each}
</div>
