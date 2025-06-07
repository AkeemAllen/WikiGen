<script lang="ts">
  import { type ItemLocation } from "../../store/items";
  import TextInput from "$lib/components/TextInput.svelte";
  import { Button } from "$lib/components/ui/button";
  import { routes } from "../../store/gameRoutes";
  import { db } from "../../store/db";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import * as Select from "$lib/components/ui/select/index.js";
  import * as Card from "$lib/components/ui/card/index.js";
  import TrashIcon from "@lucide/svelte/icons/trash";
  import { toast } from "svelte-sonner";
  import capitalizeWords from "$lib/utils/capitalizeWords";
  import { Textarea } from "./ui/textarea";

  let searchValue: string = $state("");
  let addItemLocationModalOpen: boolean = $state(false);
  let editItemLocationModalOpen: boolean = $state(false);
  type Props = {
    itemName?: string;
    generatePage: Function;
    itemLocations?: ItemLocation[];
  };

  let {
    itemName = "",
    generatePage,
    itemLocations = $bindable([]),
  }: Props = $props();

  let newItemLocation: ItemLocation = $state({
    id: 0,
    item_name: "",
    route: "",
    specific_location: null,
    method: null,
    requirements: null,
  });

  let itemLocationToEdit: ItemLocation = $state({
    id: 0,
    item_name: "",
    route: "",
    specific_location: null,
    method: null,
    requirements: null,
  });

  let routeListOptions = Object.keys($routes.routes).map((route) => ({
    label: route,
    value: route,
  }));

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
        toast.success("Item Location Added!");
        addItemLocationModalOpen = false;
        newItemLocation.id = res.lastInsertId as number;
        newItemLocation.item_name = itemName;
        itemLocations = [...itemLocations, newItemLocation];
        newItemLocation = {} as ItemLocation;
        generatePage();
      })
      .catch((err) => {
        toast.error(`Error Adding Item Location: ${err}`);
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
        toast.success(`Item Location Updated!`);
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
        toast.error(`Error Editing Item Location: ${err}`);
      });
  }

  async function deleteItemLocation(id: number) {
    await $db
      .execute("DELETE FROM item_location WHERE id = $1;", [id])
      .then(() => {
        toast.success(`Item Location Deleted!`);
        editItemLocationModalOpen = false;
        itemLocations = itemLocations.filter(
          (itemLocation) => itemLocation.id !== id,
        );
        itemLocationToEdit = {} as ItemLocation;
        generatePage();
      })
      .catch((err) => {
        toast.error(`Error Deleting Item Location: ${err}`);
      });
  }
</script>

<Dialog.Root bind:open={addItemLocationModalOpen}>
  <Dialog.Content class="w-[25rem]">
    <Dialog.Header>
      <Dialog.Title>New Item Location</Dialog.Title>
      <Dialog.Description>Add a new item location</Dialog.Description>
    </Dialog.Header>
    <form onsubmit={addItemLocation} class="flex flex-col gap-4">
      <div>
        <Label
          for="route-name"
          class="text-sm font-medium text-slate-700 mb-2 block">Route</Label
        >
        <Select.Root type="single" bind:value={newItemLocation.route}>
          <Select.Trigger id="route-name" class="w-full">
            {capitalizeWords(newItemLocation.route) || "Select a Route"}
          </Select.Trigger>
          <Select.Content>
            {#each routeListOptions as route}
              <Select.Item value={route.value} label={route.label}>
                {capitalizeWords(route.value)}
              </Select.Item>
            {/each}
          </Select.Content>
        </Select.Root>
      </div>
      <div>
        <Label
          for="specific-location"
          class="block text-sm font-medium leading-6 text-gray-900"
          >Specific Location</Label
        >
        <div class="mt-2">
          <Textarea
            id="specific-location"
            bind:value={newItemLocation.specific_location}
            placeholder="Specific Location"
          />
        </div>
      </div>
      <div>
        <Label
          for="method"
          class="block text-sm font-medium leading-6 text-gray-900"
          >Method</Label
        >
        <Input
          id="method"
          bind:value={newItemLocation.method}
          placeholder="Method"
        />
      </div>
      <div>
        <Label
          for="requirements"
          class="block text-sm font-medium leading-6 text-gray-900"
          >Requirements</Label
        >
        <div class="mt-2">
          <Textarea
            id="requirements"
            bind:value={newItemLocation.requirements}
            placeholder="Requirements"
          />
        </div>
      </div>
      <Dialog.Footer>
        <Button type="submit" disabled={newItemLocation.route === ""}>
          Add Location
        </Button>
      </Dialog.Footer>
    </form>
  </Dialog.Content>
</Dialog.Root>

<Dialog.Root bind:open={editItemLocationModalOpen}>
  <Dialog.Content class="w-[20rem]">
    <Dialog.Header>
      <Dialog.Title>Edit Item Location</Dialog.Title>
      <Dialog.Description
        >Edit the details of the item location.</Dialog.Description
      >
    </Dialog.Header>
    <form onsubmit={editItemLocation} class="flex flex-col gap-4">
      <div>
        <Label
          for="route-name"
          class="text-sm font-medium text-slate-700 mb-2 block">Route</Label
        >
        <Select.Root type="single" bind:value={itemLocationToEdit.route}>
          <Select.Trigger id="route-name" class="w-full">
            {capitalizeWords(itemLocationToEdit.route) || "Select a Route"}
          </Select.Trigger>
          <Select.Content>
            {#each routeListOptions as route}
              <Select.Item value={route.value} label={route.label}>
                {capitalizeWords(route.value)}
              </Select.Item>
            {/each}
          </Select.Content>
        </Select.Root>
      </div>
      <div>
        <Label
          for="specific-location"
          class="block text-sm font-medium leading-6 text-gray-900"
          >Specific Location</Label
        >
        <div class="mt-2">
          <Textarea
            id="specific-location"
            bind:value={itemLocationToEdit.specific_location}
            placeholder="Specific Location"
          />
        </div>
      </div>
      <TextInput
        bind:value={itemLocationToEdit.method}
        label="Method"
        placeholder="Method"
      />
      <div>
        <Label
          for="requirements"
          class="block text-sm font-medium leading-6 text-gray-900"
          >Requirements</Label
        >
        <div class="mt-2">
          <Textarea
            id="requirements"
            bind:value={itemLocationToEdit.requirements}
            placeholder="Requirements"
          />
        </div>
      </div>
      <Dialog.Footer>
        <Button type="submit">Edit Location</Button>
      </Dialog.Footer>
    </form>
  </Dialog.Content>
</Dialog.Root>

<Card.Root>
  <Card.Header>
    <Card.Title>Locations</Card.Title>
  </Card.Header>
  <Card.Content>
    <div class="flex flex-row gap-2">
      <Input
        type="text"
        placeholder="Filter Routes"
        class="w-[20rem]"
        bind:value={searchValue}
      />
      <Button
        onclick={() => {
          addItemLocationModalOpen = true;
        }}
        class="cursor-pointer">Add Location</Button
      >
    </div>
    <div class="grid grid-cols-3 gap-2 mt-5">
      {#each itemLocations.filter((location) => location.route
          .toLowerCase()
          .includes(searchValue.toLowerCase())) as location}
        <button
          class="hover:border-indigo-500 ease-in-out group rounded-lg p-4 cursor-pointer transition-all duration-200 hover:shadow-lg hover:-translate-y-1 bg-white border group relative"
          onclick={() => {
            itemLocationToEdit = location;
            editItemLocationModalOpen = true;
          }}
        >
          <div class="grid gap-2">
            <div class="text-left">
              {location.route}
              <p class="text-sm text-gray-500">
                {location.specific_location}
              </p>
              <p class="text-sm text-gray-500">
                {capitalizeWords(location.method || "")}
              </p>
              <p class="text-sm text-gray-500">
                {location.requirements}
              </p>
            </div>
          </div>
          <Button
            class="invisible absolute -right-2 -top-5 z-10 rounded-md bg-red-200 hover:scale-110 group-hover:visible cursor-pointer hover:bg-red-400 p-0"
            onclick={(e) => {
              e.stopPropagation();
              deleteItemLocation(location.id);
            }}
          >
            <TrashIcon />
          </Button>
        </button>
      {/each}
    </div>
  </Card.Content>
</Card.Root>
