<script lang="ts">
import _ from "lodash";
import AutoComplete from "$lib/components/AutoComplete.svelte";
import Button from "$lib/components/Button.svelte";
import BaseModal from "$lib/components/BaseModal.svelte";
import {
  type Item,
  type DBItem,
  dbItemsList,
  type SearchItem,
} from "../../store/items";
import { selectedWiki } from "../../store";
import { getToastStore } from "@skeletonlabs/skeleton";
import { invoke } from "@tauri-apps/api";
import TextInput from "$lib/components/TextInput.svelte";
import { db } from "../../store/db";
import { base64ToArray } from "$lib/utils";

const toastStore = getToastStore();

let itemSearch: [number, string] = [0, ""];
let currentItemName: string = "";

let dbItemDetails: DBItem = {} as DBItem;
let dbOriginalItemDetails: DBItem = {} as DBItem;
let spriteImage: string = "";

let newItemName: string = "";
let newItemDetails: Item = { effect: "", sprite: "" };
let newItemModalOpen: boolean = false;

$: itemListOptions = $dbItemsList.map(([id, name]) => ({
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

async function getItemDetails() {
  let retrievedItem = $dbItemsList.find(([_, name]) => name === itemSearch[1]);

  if (!retrievedItem) {
    toastStore.trigger({
      message: "Item not found!",
      background: "variant-filled-error",
    });
    return;
  }

  await $db
    .select<DBItem[]>("SELECT * FROM items WHERE id = $1;", [itemSearch[0]])
    .then(async (res) => {
      dbItemDetails = res[0];
      dbOriginalItemDetails = _.cloneDeep(dbItemDetails);

      // The image data is stored as a number array in the database
      // However, it gets returned as a string, so we need to convert it back to a number array
      // and then to a Uint8Array to create a blob.
      //
      // I don' know if this is how this works in other languages, but it's a pain in JS
      let array = dbItemDetails.sprite
        .replace("[", "")
        .replace("]", "")
        .split(",");
      let numberArray = array.map((x) => parseInt(x));
      let data = new Uint8Array(numberArray);
      let blob = new Blob([data], { type: "image/png" });
      spriteImage = URL.createObjectURL(blob);
    });
}

async function saveItemDetails() {
  await $db
    .execute("UPDATE items SET effect = $1 WHERE id = $2;", [
      dbItemDetails.effect,
      itemSearch[0],
    ])
    .then(() => {
      dbOriginalItemDetails = _.cloneDeep(dbItemDetails);
      toastStore.trigger({
        message: "Item changes saved!",
        background: "variant-filled-success",
      });
      // generateItemPage();
    })
    .catch((err) => {
      console.error(err);
      toastStore.trigger({
        message: "Error saving item changes!",
        background: "variant-filled-error",
      });
    });
}

async function _createNewItem() {
  // Truth be told, I don't fully how Blobs work in JS, I just know it's been painful :(
  // In order to store a 'blob' in the database, we need to convert it to an number array
  // It's meant to represent the binary data of the image
  const blob = base64ToArray(
    newItemDetails.sprite.replace("data:image/png;base64,", ""),
    "image/png",
  );

  await $db
    .execute("INSERT INTO items (name, effect, sprite) VALUES ($1, $2, $3);", [
      newItemName,
      newItemDetails.effect,
      blob,
    ])
    .then(() => {
      toastStore.trigger({
        message: "New Item Created!",
        background: "variant-filled-success",
      });
      newItemModalOpen = false;
      newItemName = "";
      newItemDetails.sprite = "";
      newItemDetails.effect = "";

      // Update the items list
      $db.select("SELECT id, name FROM items").then((items: any) => {
        let itemNames = items.map((item: SearchItem) => [item.id, item.name]);
        dbItemsList.set(itemNames);
      });
      // generateItemPage();
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
    .execute("DELETE FROM items WHERE id = $1;", [dbItemDetails.id])
    .then(() => {
      toastStore.trigger({
        message: "Item deleted!",
        background: "variant-filled-success",
      });
      // Update the items list
      $db.select("SELECT id, name FROM items").then((items: any) => {
        let itemNames = items.map((item: SearchItem) => [item.id, item.name]);
        dbItemsList.set(itemNames);
      });
      dbItemDetails = {} as DBItem;
      dbOriginalItemDetails = {} as DBItem;
      // generateItemPage();
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
    newItemDetails.sprite = e.target?.result as string;
  };
  reader.readAsDataURL(file);
}
</script>

<BaseModal class="w-[30rem]" bind:open={newItemModalOpen}>
  <h2 class="text-lg font-medium leading-6 text-gray-900">Create New Item</h2>
  <TextInput label="New Item Name" bind:value={newItemName} />
  {#if newItemDetails.sprite !== ""}
    <img src={newItemDetails.sprite} alt="Sprite" width="30" height="30" />
  {/if}
  <div>
    <label
      for="sprite-image"
      class="block text-sm font-medium leading-6 text-gray-900">Sprite</label
    >
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
        bind:value={newItemDetails.effect}
        class="block h-32 w-full rounded-md border-0 py-1.5 pl-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 disabled:bg-gray-100 disabled:text-gray-400 sm:text-sm sm:leading-6"
      />
    </div>
  </div>
  <Button
    title="Create Item"
    class="w-32"
    disabled={newItemName === "" || newItemDetails.effect === "" || newItemDetails.sprite === ""}
    onClick={_createNewItem}
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
    onClick={getItemDetails}
    disabled={itemSearch[0] === 0}
    class="mt-2 w-32"
  />
  <Button
    title="Save Changes"
    onClick={saveItemDetails}
    class="mt-2 w-32"
    disabled={_.isEqual(dbItemDetails, dbOriginalItemDetails)}
  />
  <Button
    title="Add New Item"
    class="ml-auto mr-3 mt-2 w-32"
    onClick={() => newItemModalOpen = true}
  />
  <Button
    title="Delete Item"
    class="mr-5 mt-2 w-32"
    disabled={_.isEmpty(dbItemDetails)}
    onClick={deleteItem}
  />
</div>

{#if !_.isEmpty(dbItemDetails)}
  <p class="mt-4 text-lg">
    {_.capitalize(dbItemDetails.name.replaceAll("-", " "))}
  </p>
  <img id="sprite_img" alt={currentItemName} src={spriteImage} />
  <div>
    <label
      for="effect"
      class="block text-sm font-medium leading-6 text-gray-900">Effect</label
    >
    <div class="mt-2">
      <textarea
        id="effect"
        bind:value={dbItemDetails.effect}
        class="block h-32 w-[50rem] rounded-md border-0 py-1.5 pl-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 disabled:bg-gray-100 disabled:text-gray-400 sm:text-sm sm:leading-6"
      />
    </div>
  </div>
{/if}
