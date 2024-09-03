<script lang="ts">
  import { DataHandler, Pagination } from "@vincjo/datatables";
  import { type ItemLocation } from "../../store/items";
  import TextInput from "$lib/components/TextInput.svelte";
  import Button from "$lib/components/Button.svelte";
  import SelectInput from "$lib/components/SelectInput.svelte";
  import ThSort from "$lib/components/ThSort.svelte";
  import BaseModal from "$lib/components/BaseModal.svelte";
  import AutoComplete from "$lib/components/AutoComplete.svelte";
  import { IconEdit, IconTrash } from "@tabler/icons-svelte";
  import { routes } from "../../store/gameRoutes";
  import { db } from "../../store/db";

  import { getToastStore } from "@skeletonlabs/skeleton";

  const toastStore = getToastStore();

  let searchValue: string = "";
  let addItemLocationModalOpen: boolean = false;
  let editItemLocationModalOpen: boolean = false;
  export let itemName: string = "";
  export let generatePage: Function;
  export let itemLocations: ItemLocation[] = [];

  let newItemLocation: ItemLocation = {
    id: 0,
    item_name: "",
    route: "",
    specific_location: null,
    method: null,
    requirements: null,
  };

  let itemLocationToEdit: ItemLocation = {
    id: 0,
    item_name: "",
    route: "",
    specific_location: null,
    method: null,
    requirements: null,
  };

  let routeListOptions = Object.keys($routes.routes).map((route) => ({
    label: route,
    value: route,
  }));

  const rowsPerPageOptions = [
    { label: "5", value: 5 },
    { label: "10", value: 10 },
    { label: "20", value: 20 },
    { label: "50", value: 50 },
    { label: "100", value: 100 },
  ];

  $: handler = new DataHandler(itemLocations, {
    rowsPerPage: 5,
  });
  $: rows = handler.getRows();
  $: rowsPerPage = handler.getRowsPerPage();

  async function addItemLocation() {
    await $db
      .execute(
        "INSERT INTO item_location (item_name, route, specific_location, method, requirements) VALUES ($1, $2, $3, $4, $5);",
        [
          itemName,
          newItemLocation.route,
          newItemLocation.specific_location,
          newItemLocation.method,
          newItemLocation.requirements,
        ],
      )
      .then((res) => {
        toastStore.trigger({
          message: "New Item Location Added!",
          background: "variant-filled-success",
        });
        addItemLocationModalOpen = false;
        newItemLocation.id = res.lastInsertId;
        newItemLocation.item_name = itemName;
        itemLocations = [...itemLocations, newItemLocation];
        newItemLocation = {} as ItemLocation;
        generatePage();
      })
      .catch((err) => {
        toastStore.trigger({
          message: `Error Adding Item Location: ${err}`,
          background: "variant-filled-error",
        });
      });
  }

  async function editItemLocation() {
    await $db
      .execute(
        "UPDATE item_location SET route = $1, specific_location = $2, method = $3, requirements = $4 WHERE id = $5;",
        [
          itemLocationToEdit.route,
          itemLocationToEdit.specific_location,
          itemLocationToEdit.method,
          itemLocationToEdit.requirements,
          itemLocationToEdit.id,
        ],
      )
      .then(() => {
        toastStore.trigger({
          message: "Item Location Updated!",
          background: "variant-filled-success",
        });
        editItemLocationModalOpen = false;
        itemLocations = itemLocations.map((itemLocation) => {
          if (itemLocation.id === itemLocationToEdit.id) {
            return itemLocationToEdit;
          }
          return itemLocation;
        });
        itemLocationToEdit = {} as ItemLocation;
        generatePage();
      })
      .catch((err) => {
        toastStore.trigger({
          message: `Error Editing Item Location: ${err}`,
          background: "variant-filled-error",
        });
      });
  }

  async function deleteItemLocation(id: number) {
    await $db
      .execute("DELETE FROM item_location WHERE id = $1;", [id])
      .then(() => {
        toastStore.trigger({
          message: "Item Location Deleted!",
          background: "variant-filled-success",
        });
        editItemLocationModalOpen = false;
        itemLocations = itemLocations.filter(
          (itemLocation) => itemLocation.id !== id,
        );
        itemLocationToEdit = {} as ItemLocation;
        generatePage();
      })
      .catch((err) => {
        toastStore.trigger({
          message: `Error Deleting Item Location: ${err}`,
          background: "variant-filled-error",
        });
      });
  }
</script>

<BaseModal bind:open={addItemLocationModalOpen} class="w-[20rem]">
  <h2 class="text-lg font-medium leading-6 text-gray-900">New Item Location</h2>
  <AutoComplete
    bind:value={newItemLocation.route}
    label="Routes"
    popupId="newLocationPopup"
    class="z-10 w-full text-sm"
    options={routeListOptions}
    placeholder="Routes"
    onSelection={(e) => {
      newItemLocation.route = e.detail.value;
    }}
  />
  <div>
    <label
      for="specific-location"
      class="block text-sm font-medium leading-6 text-gray-900"
      >Specific Location</label
    >
    <div class="mt-2">
      <textarea
        id="specific-location"
        bind:value={newItemLocation.specific_location}
        placeholder="Specific Location"
        class="block h-15 w-full rounded-md border-0 py-1.5 pl-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 disabled:bg-gray-100 disabled:text-gray-400 sm:text-sm sm:leading-6"
      />
    </div>
  </div>
  <TextInput
    bind:value={newItemLocation.method}
    label="Method"
    placeholder="Method"
  />
  <div>
    <label
      for="requirements"
      class="block text-sm font-medium leading-6 text-gray-900"
      >Requirements</label
    >
    <div class="mt-2">
      <textarea
        id="requirements"
        bind:value={newItemLocation.requirements}
        placeholder="Requirements"
        class="block h-15 w-full rounded-md border-0 py-1.5 pl-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 disabled:bg-gray-100 disabled:text-gray-400 sm:text-sm sm:leading-6"
      />
    </div>
  </div>
  <Button
    title="Add Location"
    disabled={newItemLocation.route === ""}
    onClick={addItemLocation}
  />
</BaseModal>

<BaseModal bind:open={editItemLocationModalOpen} class="w-[20rem]">
  <h2 class="text-lg font-medium leading-6 text-gray-900">
    Edit Item Location
  </h2>
  <AutoComplete
    bind:value={itemLocationToEdit.route}
    label="Routes"
    popupId="newLocationPopup"
    class="z-10 w-full text-sm"
    options={routeListOptions}
    placeholder="Routes"
    disabled
    onSelection={(e) => {
      itemLocationToEdit.route = e.detail.value;
    }}
  />
  <div>
    <label
      for="specific-location"
      class="block text-sm font-medium leading-6 text-gray-900"
      >Specific Location</label
    >
    <div class="mt-2">
      <textarea
        id="specific-location"
        bind:value={itemLocationToEdit.specific_location}
        placeholder="Specific Location"
        class="block h-15 w-full rounded-md border-0 py-1.5 pl-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 disabled:bg-gray-100 disabled:text-gray-400 sm:text-sm sm:leading-6"
      />
    </div>
  </div>
  <TextInput
    bind:value={itemLocationToEdit.method}
    label="Method"
    placeholder="Method"
  />
  <div>
    <label
      for="requirements"
      class="block text-sm font-medium leading-6 text-gray-900"
      >Requirements</label
    >
    <div class="mt-2">
      <textarea
        id="requirements"
        bind:value={itemLocationToEdit.requirements}
        placeholder="Requirements"
        class="block h-15 w-full rounded-md border-0 py-1.5 pl-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 disabled:bg-gray-100 disabled:text-gray-400 sm:text-sm sm:leading-6"
      />
    </div>
  </div>
  <Button title="Edit Location" onClick={editItemLocation} />
</BaseModal>

<div class="space-y-4 overflow-x-auto pr-4">
  <header class="flex items-center justify-between gap-4">
    <div class="flex gap-x-3">
      <TextInput
        id="route-name"
        bind:value={searchValue}
        inputHandler={() => handler.search(searchValue)}
        placeholder="Search route name..."
      />
      <Button
        title="Add Item Location"
        class="mt-2"
        onClick={() => {
          addItemLocationModalOpen = true;
        }}
      />
    </div>
    <aside class="flex items-center gap-x-3">
      <p class="mt-2">Show</p>
      <SelectInput bind:value={$rowsPerPage} options={rowsPerPageOptions} />
    </aside>
  </header>
  <table class="table table-hover table-compact w-full table-auto bg-white">
    <thead>
      <tr class="bg-white">
        <ThSort {handler} orderBy="route">Route</ThSort>
        <ThSort {handler} orderBy="specific_location">Specific Location</ThSort>
        <ThSort {handler} orderBy="method">Method</ThSort>
        <ThSort {handler} orderBy="requirements">Requirements</ThSort>
      </tr>
    </thead>
    <tbody>
      {#each $rows as row}
        <tr>
          <td>{row.route}</td>
          <td>{row.specific_location ? row.specific_location : ""}</td>
          <td>{row.method ? row.method : ""}</td>
          <td>{row.requirements ? row.requirements : ""}</td>
          <td
            class="w-5 rounded-sm hover:cursor-pointer hover:bg-gray-300"
            on:click={() => {
              itemLocationToEdit = { ...row };
              editItemLocationModalOpen = true;
            }}
          >
            <IconEdit size={18} class="text-gray-500" />
          </td>
          <td
            class="w-5 rounded-sm hover:cursor-pointer hover:bg-gray-300"
            on:click={() => {
              deleteItemLocation(row.id);
            }}
          >
            <IconTrash size={18} class="text-gray-500" />
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
  <footer class="flex">
    <Pagination {handler} />
  </footer>
</div>
