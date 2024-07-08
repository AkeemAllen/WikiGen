<script lang="ts">
import _ from "lodash";
import AutoComplete from "$lib/components/AutoComplete.svelte";
import Button from "$lib/components/Button.svelte";
import BaseModal from "$lib/components/BaseModal.svelte";
import { itemsList, modifiedItems, type Item, items } from "../../store/items";
import { modifiedNatures } from "../../store/natures";
import { modifiedAbilities } from "../../store/abilities";
import { selectedWiki } from "../../store";
import { BaseDirectory, writeTextFile } from "@tauri-apps/api/fs";
import { getToastStore } from "@skeletonlabs/skeleton";
import { invoke } from "@tauri-apps/api";
import TextInput from "$lib/components/TextInput.svelte";
import { updateItemModifications } from "$lib/utils/modificationHelpers";

const toastStore = getToastStore();

let itemName: string = "";
let currentItemName: string = "";
let itemDetails: Item = {} as Item;
let originalItemDetails: Item = {} as Item;

let newItemName: string = "";
let newItemDetails: Item = { effect: "", sprite: "" };
let newItemModalOpen: boolean = false;

$: itemListOptions = $itemsList.map((item) => ({
  label: item,
  value: item,
}));

async function saveItemChanges() {
  updateItemModifications($modifiedItems, currentItemName, itemDetails);

  $items[currentItemName] = itemDetails;

  await writeTextFile(
    `${$selectedWiki.name}/data/items.json`,
    JSON.stringify($items),
    { dir: BaseDirectory.AppData },
  ).then(() => {
    originalItemDetails = _.cloneDeep(itemDetails);
    toastStore.trigger({
      message: "Item changes saved!",
      background: "variant-filled-success",
    });
    invoke("generate_item_page", { wikiName: $selectedWiki.name }).then(() => {
      toastStore.trigger({
        message: "Item page regenerated!",
        background: "variant-filled-success",
      });
    });
  });
}

async function createNewItem() {
  if ($items[newItemName]) {
    toastStore.trigger({
      message: "Item already exists!",
      background: "variant-filled-error",
    });
    return;
  }

  $modifiedItems[newItemName] = {
    original: {
      effect: newItemDetails.effect,
      sprite: newItemDetails.sprite,
    },
    modified: {
      effect: "",
      sprite: "",
    },
    is_new_item: true,
  };

  await writeTextFile(
    `${$selectedWiki.name}/data/modifications/modified_items_natures_abilities.json`,
    JSON.stringify({
      items: $modifiedItems,
      natures: $modifiedNatures,
      abilities: $modifiedAbilities,
    }),
    { dir: BaseDirectory.AppData },
  );

  $items[newItemName] = newItemDetails;
  itemsList.update((list) => {
    list.push(newItemName);
    return list;
  });

  await writeTextFile(
    `${$selectedWiki.name}/data/items.json`,
    JSON.stringify($items),
    { dir: BaseDirectory.AppData },
  ).then(() => {
    originalItemDetails = _.cloneDeep(itemDetails);
    toastStore.trigger({
      message: "New Item Created!",
      background: "variant-filled-success",
    });
    newItemModalOpen = false;
    invoke("generate_item_page", { wikiName: $selectedWiki.name }).then(() => {
      toastStore.trigger({
        message: "Item page regenerated!",
        background: "variant-filled-success",
      });
    });
  });
}
</script>

<div class="flex flex-row gap-7">
  <AutoComplete
    bind:value={itemName}
    placeholder="Search Items"
    options={itemListOptions}
    popupId="item-search"
    onSelection={(e) => {
      itemName = e.detail.value;
    }}
    showChevron={false}
  />
  <Button
    title="Search"
    onClick={() => {
      currentItemName = itemName;
        itemDetails = _.cloneDeep($items[itemName]);
        originalItemDetails = _.cloneDeep(itemDetails);
      }}
    disabled={itemName === ""}
    class="mt-2 w-32"
  />
  <Button
    title="Save Changes"
    onClick={saveItemChanges}
    disabled={_.isEqual(itemDetails, originalItemDetails)}
    class="mt-2 w-32"
  />
  <Button
    title="Add New Item"
    class="ml-auto mr-5 mt-2 w-32"
    onClick={() => newItemModalOpen = true}
  />
</div>

<BaseModal
  class="w-[30rem]"
  bind:open={newItemModalOpen}
  close={() => {
  newItemModalOpen = false;
  newItemName = "";
  newItemDetails.sprite = "";
  newItemDetails.effect = "";
}}
>
  <h2 class="text-lg font-medium leading-6 text-gray-900">Create New Item</h2>
  <TextInput label="New Item Name" bind:value={newItemName} />
  <TextInput label="Sprite" bind:value={newItemDetails.sprite} />
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
    onClick={createNewItem}
  />
</BaseModal>

{#if !_.isEmpty(itemDetails)}
  <p class="mt-4 text-lg">
    {_.capitalize(currentItemName.replaceAll("-", " "))}
  </p>
  <img src={itemDetails.sprite} alt={currentItemName} />
  <div>
    <label
      for="effect"
      class="block text-sm font-medium leading-6 text-gray-900">Effect</label
    >
    <div class="mt-2">
      <textarea
        id="effect"
        bind:value={itemDetails.effect}
        class="block h-32 w-[50rem] rounded-md border-0 py-1.5 pl-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 disabled:bg-gray-100 disabled:text-gray-400 sm:text-sm sm:leading-6"
      />
    </div>
  </div>
{/if}
