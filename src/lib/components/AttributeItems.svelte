<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import BaseModal from "$lib/components/BaseModal.svelte";
  import {
    type Item,
    type ItemLocation,
    itemsList,
    type SearchItem,
  } from "../../store/items";
  import { selectedWiki } from "../../store";
  import { invoke } from "@tauri-apps/api/core";
  import { writeFile, BaseDirectory, readFile } from "@tauri-apps/plugin-fs";
  import TextInput from "$lib/components/TextInput.svelte";
  import { db } from "../../store/db";
  import { base64ToArray, isNullEmptyOrUndefined } from "$lib/utils";
  import { FALSE, TRUE } from "$lib/utils/CONSTANTS";
  import { cloneDeep } from "$lib/utils/cloneDeep";
  import isEqual from "$lib/utils/isEqual";
  import objectIsEmpty from "$lib/utils/objectIsEmpty";
  import ItemLocationTable from "$lib/components/ItemLocationTable.svelte";
  import * as Card from "$lib/components/ui/card";
  import Autocomplete from "./ui/Autocomplete.svelte";
  import SaveIcon from "@lucide/svelte/icons/save";
  import { Label } from "./ui/label";
  import { Textarea } from "$lib/components/ui/textarea/index.js";
  import { Checkbox } from "./ui/checkbox";
  import { toast } from "svelte-sonner";
  import TrashIcon from "@lucide/svelte/icons/trash";

  let itemSearch: [number, string] = $state([0, ""]);
  let itemSearchOpen = $state(false);

  let item: Item = $state({} as Item);
  let originalItemDetails: Item = $state({} as Item);
  let itemModified = $state(false);
  let searchingItem: string = $state("");
  let spriteImage: string = $state("");

  let itemLocations: ItemLocation[] = $state([]);

  let newItem: Item = $state({} as Item);
  let newSpriteImage: string = $state("");
  let newItemModalOpen: boolean = $state(false);

  let itemListOptions = $derived(
    $itemsList.map(([id, name]) => ({
      label: name,
      value: id,
    })),
  );

  let options = $derived(
    itemListOptions
      .filter((item) =>
        item.label.toLowerCase().includes(searchingItem.toLowerCase()),
      )
      .slice(0, 8),
  );

  async function generateItemLocationPage() {
    await invoke("generate_item_location_page_with_handle", {
      wikiName: $selectedWiki.name,
    })
      .then((res) => {
        toast.success(res as string);
      })
      .catch((err) => {
        toast.error(err);
      });
  }

  async function generateItemPage() {
    await invoke("generate_item_changes_page_with_handle", {
      wikiName: $selectedWiki.name,
    })
      .then(() => {
        toast.success("Item page regenerated!");
      })
      .then(() => {
        generateItemLocationPage();
      });
  }

  async function getItem() {
    let retrievedItem = $itemsList.find(([_, name]) => name === itemSearch[1]);

    if (!retrievedItem) {
      toast.error("Item not found!");
      return;
    }

    await $db
      .select<Item[]>("SELECT * FROM items WHERE id = $1;", [itemSearch[0]])
      .then(async (res: any) => {
        item = res[0];
        itemModified = item.is_modified === 1;
        originalItemDetails = cloneDeep(item);

        await $db
          .select<ItemLocation[]>(
            "SELECT * FROM item_location WHERE item_name = $1;",
            [item.name],
          )
          .then((res) => {
            itemLocations = res;
          })
          .catch((err) => {
            toast.error(`Error fetching item locations!: \n ${err}`);
          });

        // Reading in image separately
        spriteImage = await readFile(
          `${$selectedWiki.name}/dist/docs/img/items/${item.name}.png`,
          { baseDir: BaseDirectory.AppData },
        )
          .then((res: any) => {
            const blob = new Blob([res], { type: "image/png" });
            return URL.createObjectURL(blob);
          })
          .catch((err: any) => {
            console.log(err);
            if (err.includes("No such file or directory")) {
              return "404";
            }
            return "Error loading image";
          });
      });
  }

  async function saveItemChanges() {
    item.is_modified = itemModified ? 1 : 0;
    await $db
      .execute(
        "UPDATE items SET effect = $1, is_modified = $2 WHERE id = $3;",
        [item.effect, item.is_modified, itemSearch[0]],
      )
      .then(() => {
        originalItemDetails = cloneDeep(item);
        generateItemPage();
      })
      .catch(() => {
        toast.error("Error saving item changes!");
      });
  }

  async function createItem() {
    newItem.is_new = TRUE;
    await $db
      .execute(
        "INSERT INTO items (name, effect, is_new) VALUES ($1, $2, $3);",
        [newItem.name, newItem.effect, newItem.is_new],
      )
      .then(() => {
        // Write image to file
        const imageBytes = base64ToArray(
          newSpriteImage.replace("data:image/png;base64,", ""),
          "image/png",
        );
        const contents = new Uint8Array(imageBytes);
        writeFile(
          `${$selectedWiki.name}/dist/docs/img/items/${newItem.name}.png`,
          contents,
          { baseDir: BaseDirectory.AppData },
        ).then(() => {
          newSpriteImage = "";
        });

        toast.success("New Item Created!");
        newItemModalOpen = false;
        newItem = {} as Item;

        // Update the items list
        $db.select("SELECT id, name FROM items").then((items: any) => {
          let itemNames = items.map((item: SearchItem) => [item.id, item.name]);
          itemsList.set(itemNames);
        });
        generateItemPage();
      })
      .catch((err: any) => {
        toast.error(`Error creating new item!: ${err}`);
      });
  }

  async function deleteItem() {
    await $db
      .execute("DELETE FROM items WHERE id = $1;", [item.id])
      .then(() => {
        toast.success("Item deleted!");
        // Update the items list
        $db.select("SELECT id, name FROM items").then((items: any) => {
          let itemNames = items.map((item: SearchItem) => [item.id, item.name]);
          itemsList.set(itemNames);
        });
        item = {} as Item;
        originalItemDetails = {} as Item;
        generateItemPage();
      })
      .catch((err) => {
        toast.error(`Error deleting item: ${err}`);
      });
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
      newSpriteImage = e.target?.result as string;
    };
    reader.readAsDataURL(file);
  }

  function setModified(e: any) {
    item.is_modified = e.target?.checked ? TRUE : FALSE;
  }
</script>

<BaseModal class="w-[30rem]" bind:open={newItemModalOpen}>
  <h2 class="text-lg font-medium leading-6 text-gray-900">Create New Item</h2>
  <TextInput label="New Item Name" bind:value={newItem.name} />
  <div>
    <label
      for="sprite-image"
      class="block text-sm font-medium leading-6 text-gray-900">Sprite</label
    >
    {#if newSpriteImage !== ""}
      <img src={newSpriteImage} alt="Sprite" width="30" height="30" />
    {/if}
    <input
      id="sprite-image"
      type="file"
      accept="image/png"
      class="mt-2"
      onchange={onImageUpload}
    />
  </div>
  <div>
    <label
      for="effect"
      class="block text-sm font-medium leading-6 text-gray-900">Effect</label
    >
    <div class="mt-2">
      <textarea
        id="effect"
        bind:value={newItem.effect}
        class="block h-32 w-full rounded-md border-0 py-1.5 pl-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 disabled:bg-gray-100 disabled:text-gray-400 sm:text-sm sm:leading-6"
      ></textarea>
    </div>
  </div>
  <Button
    title="Create Item"
    class="w-32"
    disabled={isNullEmptyOrUndefined(newItem.name) ||
      isNullEmptyOrUndefined(newItem.effect) ||
      newSpriteImage === ""}
    onclick={createItem}
  />
</BaseModal>

<Card.Root>
  <Card.Content class="flex flex-row gap-3">
    <Autocomplete
      open={itemSearchOpen}
      value={itemSearch[1]}
      bind:searcher={searchingItem}
      {options}
      placeholder="Search Item"
      onSelect={(option) => {
        itemSearch = [option.value, option.label];
        getItem();
      }}
      class="w-[20rem]"
    />
    <div class="flex flex-row justify-between w-full">
      <Button
        class="cursor-pointer"
        onclick={saveItemChanges}
        disabled={isEqual(item, originalItemDetails)}
      >
        <SaveIcon />
        Save Changes</Button
      >
      <Button
        variant="outline"
        class="cursor-pointer"
        onclick={() => (newItemModalOpen = true)}
      >
        Create New Item</Button
      >
    </div>
  </Card.Content>
</Card.Root>

{#if !objectIsEmpty(item)}
  <Card.Root class="my-5">
    <Card.Content class="flex flex-col gap-4">
      {#if spriteImage === "404"}
        <p class="text-sm">No sprite available</p>
      {:else}
        <img
          id="sprite_img"
          alt={item.name}
          src={spriteImage}
          height="50"
          width="50"
        />
      {/if}
      <div>
        <Label
          for="effect"
          class="block text-sm font-medium leading-6 mb-2 text-gray-900"
          >Effect/Description</Label
        >
        <div class="mt-2">
          <Textarea
            id="effect"
            bind:value={item.effect}
            class="block h-20 w-[50rem] rounded-md border-0 py-1.5 pl-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 disabled:bg-gray-100 disabled:text-gray-400 sm:text-sm sm:leading-6"
          />
        </div>
      </div>
      {#if !item.is_new}
        <div class="flex flex-row space-x-2 items-center mt-5">
          <Checkbox
            id="is-modified"
            bind:checked={itemModified}
            onchange={setModified}
            class="text-sm font-medium leading-6 text-gray-900"
          />
          <Label
            for="is-modified"
            class="block text-sm font-medium leading-6 text-gray-900"
          >
            Mark Item as Modified
          </Label>
        </div>
      {/if}
    </Card.Content>
  </Card.Root>
  <ItemLocationTable
    {itemLocations}
    itemName={item.name}
    generatePage={generateItemLocationPage}
  />
  <Button
    class="my-5 bg-red-200 hover:bg-red-400 cursor-pointer"
    onclick={() => {
      deleteItem();
    }}
  >
    <TrashIcon />
    Delete Item</Button
  >
{/if}
