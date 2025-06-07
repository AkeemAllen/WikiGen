<script lang="ts">
  import { types } from "../../store/types";
  import { Button } from "$lib/components/ui/button";
  import {
    BaseDirectory,
    readFile,
    writeFile,
    writeTextFile,
  } from "@tauri-apps/plugin-fs";
  import { selectedWiki } from "../../store";
  import { toast } from "svelte-sonner";
  import { base64ToArray } from "$lib/utils";
  import * as Card from "$lib/components/ui/card/index.js";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import capitalizeWords from "$lib/utils/capitalizeWords";
  import IconTrash from "@lucide/svelte/icons/trash";

  let newType = $state("");
  let newTypeImage = $state("");

  async function addType() {
    if (newType === "") return;
    if ($types.includes(newType.toLowerCase().replaceAll(" ", "-"))) {
      toast.error("Type already exists");
      return;
    }
    types.update((t) => [...t, newType.toLowerCase().replaceAll(" ", "-")]);
    await writeTextFile(
      `${$selectedWiki.name}/data/types.json`,
      JSON.stringify({ types: $types }),
      { baseDir: BaseDirectory.AppData },
    ).then(() => {
      const imageBytes = base64ToArray(
        newTypeImage.replace("data:image/png;base64,", ""),
        "image/png",
      );
      writeFile(
        `${$selectedWiki.name}/dist/docs/img/types/${newType}.png`,
        new Uint8Array(imageBytes),
        { baseDir: BaseDirectory.AppData },
      ).then(() => {
        newType = "";
        newTypeImage = "";
        toast.success("Type added successfully");
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
      { baseDir: BaseDirectory.AppData },
    ).then(() => {
      toast.success("Type deleted successfully");
    });
  }
  async function getTypeImage(type: string): Promise<string> {
    let sprite = "";
    await readFile(`${$selectedWiki.name}/dist/docs/img/types/${type}.png`, {
      baseDir: BaseDirectory.AppData,
    })
      .then((res) => {
        const blob = new Blob([res], { type: "image/png" });
        sprite = URL.createObjectURL(blob);
      })
      .catch((err) => {
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
        toast.error("Invalid image format!");
        return;
      }
      newTypeImage = e.target?.result as string;
    };
    reader.readAsDataURL(file);
  }
</script>

<Card.Root class="mx-5 mt-5">
  <Card.Content class="flex flex-col gap-3">
    <Input
      type="text"
      bind:value={newType}
      placeholder="Enter New Type Name"
      oninput={(e: any) => {
        newType = e.target.value.toLowerCase().replaceAll(" ", "-");
      }}
      class="w-[15rem]"
    />
    <div>
      <Label
        for="sprite-image"
        class="block text-sm font-medium leading-6 text-gray-900"
        >Type Image</Label
      >
      {#if newTypeImage !== ""}
        <img src={newTypeImage} alt="Type" width="96" height="32" />
      {/if}
      <p class="text-sm italic text-gray-600 mt-2">
        <strong>Note: </strong>Recommended to use 96 × 32 png image for
        consistency.
      </p>
      <Input
        id="sprite-image"
        type="file"
        accept="image/png"
        class="w-[15rem] mt-2"
        onchange={onImageUpload}
      />
    </div>
    <Button
      class="mt-2 mb-4 w-44"
      title="Add New Type"
      onclick={addType}
      disabled={newType === "" || newTypeImage === ""}
    >
      Add Type
    </Button>
    <div class="grid grid-cols-3 gap-3">
      {#each $types as type}
        <div
          class="flex flex-row bg-slate-200 rounded-sm items-center justify-between px-2 py-1"
        >
          {capitalizeWords(type)}
          <div class="flex flex-row gap-4 items-center">
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
                onclick={() => deleteType(type)}
              >
                <IconTrash class="size-4 text-slate-500" />
              </button>
            {/if}
          </div>
        </div>
      {/each}
    </div>
    <p class="text-sm italic text-gray-600 mt-2">
      <strong>Note: </strong>Tab will need to be reloaded to see the changes.
    </p>
  </Card.Content>
</Card.Root>
