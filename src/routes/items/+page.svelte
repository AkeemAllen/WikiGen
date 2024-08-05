<script lang="ts">
  import _ from "lodash";
  import AutoComplete from "$lib/components/AutoComplete.svelte";
  import Button from "$lib/components/Button.svelte";
  import BaseModal from "$lib/components/BaseModal.svelte";
  import { type Item, itemsList, type SearchItem } from "../../store/items";
  import { selectedWiki } from "../../store";
  import { getToastStore } from "@skeletonlabs/skeleton";
  import { invoke } from "@tauri-apps/api";
  import {
    writeBinaryFile,
    BaseDirectory,
    readBinaryFile,
  } from "@tauri-apps/api/fs";
  import TextInput from "$lib/components/TextInput.svelte";
  import { db } from "../../store/db";
  import { base64ToArray, isNullEmptyOrUndefined } from "$lib/utils";
  import { FALSE, TRUE } from "$lib/utils/CONSTANTS";
  import { cloneDeep } from "$lib/utils/cloneDeep";
  import capitalizeWords from "$lib/utils/capitalizeWords";
  import isEqual from "$lib/utils/isEqual";
  import objectIsEmpty from "$lib/utils/objectIsEmpty";

  const toastStore = getToastStore();

  let itemSearch: [number, string] = [0, ""];

  let item: Item = {} as Item;
  let originalItemDetails: Item = {} as Item;
  let spriteImage: string = "";

  let newItem: Item = {} as Item;
  let newSpriteImage: string = "";
  let newItemModalOpen: boolean = false;

  $: itemListOptions = $itemsList.map(([id, name]) => ({
    label: name,
    value: id,
  }));

  async function generateItemPage() {
    await invoke("generate_item_page", { wikiName: $selectedWiki.name }).then(
      () => {
        toastStore.trigger({
          message: "Item page regenerated!",
          background: "variant-filled-success",
        });
      },
    );
  }

  async function getItem() {
    let retrievedItem = $itemsList.find(([_, name]) => name === itemSearch[1]);

    if (!retrievedItem) {
      toastStore.trigger({
        message: "Item not found!",
        background: "variant-filled-error",
      });
      return;
    }

    await $db
      .select<Item[]>("SELECT * FROM items WHERE id = $1;", [itemSearch[0]])
      .then(async (res) => {
        item = res[0];
        originalItemDetails = cloneDeep(item);

        // Reading in image separately
        spriteImage = await readBinaryFile(
          `${$selectedWiki.name}/dist/docs/img/items/${item.name}.png`,
          { dir: BaseDirectory.AppData },
        )
          .then((res) => {
            const blob = new Blob([res], { type: "image/png" });
            return URL.createObjectURL(blob);
          })
          .catch((err) => {
            console.log(err);
            if (err.includes("No such file or directory")) {
              return "404";
            }
            return "Error loading image";
          });
      });
  }

  async function saveItemChanges() {
    await $db
      .execute(
        "UPDATE items SET effect = $1, is_modified = $2 WHERE id = $3;",
        [item.effect, item.is_modified, itemSearch[0]],
      )
      .then(() => {
        originalItemDetails = cloneDeep(item);
        toastStore.trigger({
          message: "Item changes saved!",
          background: "variant-filled-success",
        });
        generateItemPage();
      })
      .catch(() => {
        toastStore.trigger({
          message: "Error saving item changes!",
          background: "variant-filled-error",
        });
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
        writeBinaryFile(
          `${$selectedWiki.name}/dist/docs/img/items/${newItem.name}.png`,
          imageBytes,
          { dir: BaseDirectory.AppData },
        ).then(() => {
          newSpriteImage = "";
        });

        toastStore.trigger({
          message: "New Item Created!",
          background: "variant-filled-success",
        });
        newItemModalOpen = false;
        newItem = {} as Item;

        // Update the items list
        $db.select("SELECT id, name FROM items").then((items: any) => {
          let itemNames = items.map((item: SearchItem) => [item.id, item.name]);
          itemsList.set(itemNames);
        });
        generateItemPage();
      })
      .catch((err) => {
        toastStore.trigger({
          message: "Error creating new item!",
          background: "variant-filled-error",
        });
      });
  }

  async function deleteItem() {
    await $db
      .execute("DELETE FROM items WHERE id = $1;", [item.id])
      .then(() => {
        toastStore.trigger({
          message: "Item deleted!",
          background: "variant-filled-success",
        });
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
        toastStore.trigger({
          message: "Error deleting item!",
          background: "variant-filled-error",
        });
      });
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
      newSpriteImage = e.target?.result as string;
    };
    reader.readAsDataURL(file);
  }

  function setModified(e: any) {
    item.is_modified = e.target?.checked ? TRUE : FALSE;
  }

  async function convertItemToSqlite() {
    await invoke("convert_items_to_sqlite", {
      wikiName: $selectedWiki.name,
    })
      .then(() => {
        toastStore.trigger({
          message: "Items converted!",
          background: "variant-filled-success",
        });
      })
      .catch((err) => {
        toastStore.trigger({
          message: "Error converting items!",
          background: "variant-filled-error",
        });
      });
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
      on:change={onImageUpload}
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
      />
    </div>
  </div>
  <Button
    title="Create Item"
    class="w-32"
    disabled={isNullEmptyOrUndefined(newItem.name) ||
      isNullEmptyOrUndefined(newItem.effect) ||
      newSpriteImage === ""}
    onClick={createItem}
  />
</BaseModal>

<div class="flex flex-row gap-7">
  <AutoComplete
    bind:value={itemSearch[1]}
    placeholder="Search Items"
    options={itemListOptions}
    popupId="item-search"
    onSelection={(e) => {
      itemSearch = [e.detail.value, e.detail.label];
    }}
    showChevron={false}
  />
  <Button
    title="Search"
    onClick={getItem}
    disabled={itemSearch[0] === 0}
    class="mt-2 w-32"
  />
  <Button
    title="Save Changes"
    onClick={saveItemChanges}
    class="mt-2 w-32"
    disabled={isEqual(item, originalItemDetails)}
  />
  <Button
    title="Add New Item"
    class="ml-auto mr-3 mt-2 w-32"
    onClick={() => (newItemModalOpen = true)}
  />
  <Button
    title="Delete Item"
    class="mr-5 mt-2 w-32"
    disabled={objectIsEmpty(item)}
    onClick={deleteItem}
  />
</div>

{#if !objectIsEmpty(item)}
  <div class="mt-4 flex flex-col gap-4">
    <p class="text-lg">
      {capitalizeWords(item.name.replaceAll("-", " "))}
    </p>
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
      <label
        for="effect"
        class="block text-sm font-medium leading-6 text-gray-900"
        >Effect/Description</label
      >
      <div class="mt-2">
        <textarea
          id="effect"
          bind:value={item.effect}
          class="block h-20 w-[50rem] rounded-md border-0 py-1.5 pl-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 disabled:bg-gray-100 disabled:text-gray-400 sm:text-sm sm:leading-6"
        />
      </div>
    </div>
    {#if !item.is_new}
      <label class="block text-sm font-medium leading-6 text-gray-900">
        <input
          type="checkbox"
          checked={Boolean(item.is_modified)}
          on:change={setModified}
          class="text-sm font-medium leading-6 text-gray-900"
        />
        Mark Item as Modified
      </label>
    {/if}
  </div>
{/if}
<!-- <Button title="Convert Items to SQLite" onClick={convertItemToSqlite} /> -->
