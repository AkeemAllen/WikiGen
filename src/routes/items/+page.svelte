<script lang="ts">
  import AutoComplete from "$lib/components/AutoComplete.svelte";
  import Button from "$lib/components/Button.svelte";
  import BaseModal from "$lib/components/BaseModal.svelte";
  import {
    type Item,
    type ItemLocation,
    itemsList,
    type SearchItem,
  } from "../../store/items";
  import { selectedWiki } from "../../store";
  import { getToastStore, Tab, TabGroup } from "@skeletonlabs/skeleton";
  import { invoke } from "@tauri-apps/api/core";
  import { writeFile, BaseDirectory, readFile } from "@tauri-apps/plugin-fs";
  import TextInput from "$lib/components/TextInput.svelte";
  import { db } from "../../store/db";
  import { base64ToArray, isNullEmptyOrUndefined } from "$lib/utils";
  import { FALSE, TRUE } from "$lib/utils/CONSTANTS";
  import { cloneDeep } from "$lib/utils/cloneDeep";
  import capitalizeWords from "$lib/utils/capitalizeWords";
  import isEqual from "$lib/utils/isEqual";
  import objectIsEmpty from "$lib/utils/objectIsEmpty";
  import ItemLocationTable from "$lib/components/ItemLocationTable.svelte";
  import { getToastSettings, ToastType } from "$lib/utils/toasts";

  const toastStore = getToastStore();

  let itemSearch: [number, string] = [0, ""];

  let item: Item = {} as Item;
  let originalItemDetails: Item = {} as Item;
  let spriteImage: string = "";

  let tabSet: number = 0;

  let itemLocations: ItemLocation[] = [];

  let newItem: Item = {} as Item;
  let newSpriteImage: string = "";
  let newItemModalOpen: boolean = false;

  $: itemListOptions = $itemsList.map(([id, name]) => ({
    label: name,
    value: id,
  }));

  async function generateItemLocationPage() {
    await invoke("generate_item_location_page_with_handle", {
      wikiName: $selectedWiki.name,
    })
      .then((res) => {
        toastStore.trigger(getToastSettings(ToastType.SUCCESS, res as string));
      })
      .catch((err) => {
        toastStore.trigger(getToastSettings(ToastType.ERROR, err));
      });
  }

  async function generateItemPage() {
    await invoke("generate_item_changes_page_with_handle", {
      wikiName: $selectedWiki.name,
    })
      .then(() => {
        toastStore.trigger(
          getToastSettings(ToastType.SUCCESS, "Item page regenerated!"),
        );
      })
      .then(() => {
        generateItemLocationPage();
      });
  }

  async function getItem() {
    let retrievedItem = $itemsList.find(([_, name]) => name === itemSearch[1]);

    if (!retrievedItem) {
      toastStore.trigger(getToastSettings(ToastType.ERROR, "Item not found!"));
      return;
    }

    await $db
      .select<Item[]>("SELECT * FROM items WHERE id = $1;", [itemSearch[0]])
      .then(async (res: any) => {
        item = res[0];
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
            toastStore.trigger(
              getToastSettings(
                ToastType.ERROR,
                `Error fetching item locations!: \n ${err}`,
              ),
            );
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
        toastStore.trigger(
          getToastSettings(ToastType.ERROR, "Error saving item changes!"),
        );
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

        toastStore.trigger(
          getToastSettings(ToastType.SUCCESS, "New Item Created!"),
        );
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
        toastStore.trigger(
          getToastSettings(ToastType.ERROR, "Error creating new item!"),
        );
      });
  }

  async function deleteItem() {
    await $db
      .execute("DELETE FROM items WHERE id = $1;", [item.id])
      .then(() => {
        toastStore.trigger(
          getToastSettings(ToastType.SUCCESS, "Item deleted!"),
        );
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
        toastStore.trigger(
          getToastSettings(ToastType.ERROR, `Error deleting item: ${err}`),
        );
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
    <TabGroup>
      <Tab bind:group={tabSet} name="item-locations" value={0} class="text-sm"
        >Item Locations</Tab
      >
      <svelte:fragment slot="panel">
        {#if tabSet === 0}
          <ItemLocationTable
            {itemLocations}
            itemName={item.name}
            generatePage={generateItemLocationPage}
          />
        {/if}
      </svelte:fragment>
    </TabGroup>
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
