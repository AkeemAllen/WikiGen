<script lang="ts">
import _ from "lodash";
import AutoComplete from "$lib/components/AutoComplete.svelte";
import Button from "$lib/components/Button.svelte";
import { itemsList, type Item, items } from "../../store/items";
import { selectedWiki } from "../../store";
import { BaseDirectory, writeTextFile } from "@tauri-apps/api/fs";
import { getToastStore } from "@skeletonlabs/skeleton";

const toastStore = getToastStore();

let itemName: string = "";
let currentItemName: string = "";
let itemDetails: Item = {} as Item;
let originalItemDetails: Item = {} as Item;

let itemListOptions = $itemsList.map((item) => ({
  label: item,
  value: item,
}));

async function saveItemChanges() {
  $items[currentItemName] = itemDetails;

  await writeTextFile(
    `${$selectedWiki.name}/data/items.json`,
    JSON.stringify($items),
    { dir: BaseDirectory.AppData },
  ).then(() => {
    originalItemDetails = _.cloneDeep(itemDetails);
    // invoke("generate_items_page", {
    //   wikiName: $selectedWiki.name,
    //   dexNumbers: [pokemonId],
    // });
    toastStore.trigger({
      message: "Item changes saved!",
      background: "variant-filled-success",
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
</div>

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