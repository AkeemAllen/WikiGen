<script lang="ts">
  import { IconTrash } from "@tabler/icons-svelte";
  import { types } from "../../store/types";
  import Button from "$lib/components/Button.svelte";
  import TextInput from "$lib/components/TextInput.svelte";
  import {
    BaseDirectory,
    readBinaryFile,
    writeBinaryFile,
    writeTextFile,
  } from "@tauri-apps/api/fs";
  import { selectedWiki } from "../../store";
  import { getToastStore } from "@skeletonlabs/skeleton";
  import { base64ToArray } from "$lib/utils";

  const toastStore = getToastStore();
  let newType = "";
  let newTypeImage = "";

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
    await writeTextFile(
      `${$selectedWiki.name}/data/types.json`,
      JSON.stringify({ types: $types }),
      { dir: BaseDirectory.AppData },
    ).then(() => {
      const imageBytes = base64ToArray(
        newTypeImage.replace("data:image/png;base64,", ""),
        "image/png",
      );
      writeBinaryFile(
        `${$selectedWiki.name}/dist/docs/img/types/${newType}.png`,
        imageBytes,
        { dir: BaseDirectory.AppData },
      ).then(() => {
        newType = "";
        newTypeImage = "";
        toastStore.trigger({
          message: "Type added successfully",
          background: "variant-filled-success",
        });
      });
    });
  }
  async function deleteType(selectedType: string) {
    $types = $types.filter(
      (type) => type !== selectedType.toLowerCase().replaceAll(" ", "-"),
    );
    await writeTextFile(
      `${$selectedWiki.name}/data/types.json`,
      JSON.stringify({ types: $types }),
      { dir: BaseDirectory.AppData },
    ).then(() => {
      toastStore.trigger({
        message: "Type deleted successfully",
        background: "variant-filled-success",
      });
    });
  }
  async function getTypeImage(type: string): Promise<string> {
    let sprite = "";
    await readBinaryFile(
      `${$selectedWiki.name}/dist/docs/img/types/${type}.png`,
      { dir: BaseDirectory.AppData },
    )
      .then((res) => {
        const blob = new Blob([res], { type: "image/png" });
        sprite = URL.createObjectURL(blob);
      })
      .catch((err) => {
        console.log(err);
        if (err.includes("No such file or directory")) {
          sprite = "Image Not Found";
        }
        sprite = "Error loading image";
      });
    return sprite;
  }
  function onImageUpload(e: any) {
    let file = e.target.files[0];
    let reader = new FileReader();
    reader.onloadend = (e) => {
      let base64 = e.target?.result as string;
      if (!base64.includes("data:image/png;base64,")) {
        toastStore.trigger({
          message: "Invalid image format!",
          background: "variant-filled-error",
        });
        return;
      }
      newTypeImage = e.target?.result as string;
    };
    reader.readAsDataURL(file);
  }
</script>

<div class="flex flex-col gap-3">
  <TextInput
    bind:value={newType}
    placeholder="New Type"
    inputHandler={(e) => {
      newType = e.target.value.toLowerCase().replaceAll(" ", "-");
    }}
    class="w-44"
  />
  <div>
    <label
      for="sprite-image"
      class="block text-sm font-medium leading-6 text-gray-900"
      >Type Image</label
    >
    {#if newTypeImage !== ""}
      <img src={newTypeImage} alt="Type" width="96" height="32" />
    {/if}
    <p class="text-sm italic text-gray-600 mt-2">
      <strong>Note: </strong>Recommended to use 96 × 32 png image for
      consistency.
    </p>
    <input
      id="sprite-image"
      type="file"
      accept="image/png"
      class="mt-2"
      on:change={onImageUpload}
      placeholder=""
    />
  </div>
  <Button
    class="mt-2 mb-4 w-44"
    title="Add New Type"
    onClick={addType}
    disabled={newType === "" || newTypeImage === ""}
  />
</div>
<div class="grid grid-cols-3 gap-3">
  {#each $types as type}
    <div class="card flex flex-row items-center justify-between px-2 py-1">
      {type}
      <div class="grid grid-cols-2 gap-4 items-center">
        {#if type !== "none"}
          {#await getTypeImage(type) then spriteUrl}
            <img
              src={spriteUrl}
              alt={type}
              class="m-0 justify-self-center"
              width="80"
            />
          {/await}
          <button
            class="btn rounded-sm p-2 hover:cursor-pointer hover:bg-gray-300"
            on:click={() => deleteType(type)}
          >
            <IconTrash size={16} />
          </button>
        {/if}
      </div>
    </div>
  {/each}
</div>
<p class="text-sm italic text-gray-600 mt-2">
  <strong>Note: </strong>Tab will need to be reloaded to see the changes.
</p>
