<script lang="ts">
  import {
    naturesList,
    type Nature,
    type SearchNature,
  } from "../../store/natures";
  import { selectedWiki } from "../../store";
  import { invoke } from "@tauri-apps/api/core";
  import { db } from "../../store/db";
  import { FALSE, TRUE } from "$lib/utils/CONSTANTS";
  import { cloneDeep } from "$lib/utils/cloneDeep";
  import capitalizeWords from "$lib/utils/capitalizeWords";
  import isEqual from "$lib/utils/isEqual";
  import objectIsEmpty from "$lib/utils/objectIsEmpty";
  import Autocomplete from "./ui/Autocomplete.svelte";
  import SaveIcon from "@lucide/svelte/icons/save";
  import TrashIcon from "@lucide/svelte/icons/trash";
  import { Checkbox } from "./ui/checkbox";
  import { Label } from "./ui/label";
  import * as Dialog from "$lib/components/ui/dialog";
  import { Input } from "./ui/input";
  import * as Card from "$lib/components/ui/card/index.js";
  import * as Select from "$lib/components/ui/select/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { toast } from "svelte-sonner";
  let natureSearch: [number, string] = $state([0, ""]);

  let nature: Nature = $state({} as Nature);
  let natureSearchOpen = $state(false);
  let searchingNature = $state("");
  let natureModified = $state(false);
  let originalNatureDetails: Nature = $state({} as Nature);

  let newNatureModalOpen: boolean = $state(false);
  let newNature: Nature = $state({
    id: 0,
    name: "",
    increased_stat: undefined,
    decreased_stat: undefined,
    is_new: 1,
    is_modified: 0,
  });

  let natureListOptions = $derived(
    $naturesList.map(([id, name]) => ({
      label: name,
      value: id,
    })),
  );

  const natureOptions = [
    "",
    "hp",
    "attack",
    "defense",
    "special-attack",
    "special-defense",
    "speed",
  ].map((nature) => {
    if (nature === "") {
      return { label: "None", value: "" };
    }
    return { label: nature, value: nature };
  });

  let options = $derived(
    natureListOptions
      .filter((nature) =>
        nature.label.toLowerCase().includes(searchingNature.toLowerCase()),
      )
      .slice(0, 8),
  );

  async function generateNaturePage() {
    await invoke("generate_nature_page_with_handle", {
      wikiName: $selectedWiki.name,
    })
      .then((res) => {
        toast.success(res as string);
      })
      .catch((err) => {
        toast.error(err as string);
      });
  }

  async function getNature() {
    let retrievedNature = $naturesList.find(
      ([_, name]) => name === natureSearch[1],
    );

    if (!retrievedNature) {
      toast.error("Nature not found");
      return;
    }

    await $db
      .select<Nature[]>("SELECT * FROM natures WHERE name = $1", [
        natureSearch[1],
      ])
      .then((result) => {
        nature = result[0];
        natureModified = nature.is_modified === 1;
        originalNatureDetails = cloneDeep(nature);
      })
      .catch(() => {
        toast.error("Error fetching nature");
      });
  }

  async function saveNatureChanges() {
    nature.is_modified = natureModified ? 1 : 0;
    await $db
      .execute(
        "UPDATE natures SET increased_stat = $1, decreased_stat = $2, is_modified = $3 WHERE id = $4;",
        [
          nature.increased_stat,
          nature.decreased_stat,
          nature.is_modified,
          natureSearch[0],
        ],
      )
      .then(() => {
        originalNatureDetails = cloneDeep(nature);
        generateNaturePage();
      })
      .catch((err) => {
        toast.error(`Error saving nature changes: ${err}`);
      });
  }

  async function createNewNature() {
    newNature.is_new = TRUE;

    await $db
      .execute(
        "INSERT INTO natures (name, increased_stat, decreased_stat, is_new) VALUES ($1, $2, $3, $4);",
        [
          newNature.name,
          newNature.increased_stat,
          newNature.decreased_stat,
          newNature.is_new,
        ],
      )
      .then(() => {
        toast.success("Nature created!");
        newNatureModalOpen = false;
        newNature = {
          id: 0,
          name: "",
          increased_stat: undefined,
          decreased_stat: undefined,
          is_new: 1,
          is_modified: 0,
        };

        // Update the natures list
        $db.select("SELECT id, name FROM natures").then((natures: any) => {
          naturesList.set(
            natures.map((nature: SearchNature) => [nature.id, nature.name]),
          );
        });
        generateNaturePage();
      })
      .catch((err) => {
        toast.error(`Error creating new nature: ${err}`);
      });
  }

  async function deleteNature() {
    await $db
      .execute("DELETE FROM natures WHERE id = $1;", [nature.id])
      .then(() => {
        /// Update the natures list
        $db.select("SELECT id, name FROM natures").then((natures: any) => {
          naturesList.set(
            natures.map((nature: SearchNature) => [nature.id, nature.name]),
          );
        });
        nature = {} as Nature;
        originalNatureDetails = {} as Nature;
        generateNaturePage();
      })
      .catch((err) => {
        toast.error(`Error deleting nature: ${err}`);
      });
  }

  function setModified(e: any) {
    nature.is_modified = e.target?.checked ? TRUE : FALSE;
  }
</script>

<Dialog.Root bind:open={newNatureModalOpen}>
  <Dialog.Content class="w-[30rem]">
    <Dialog.Header>
      <Dialog.Title>Create New Nature</Dialog.Title>
      <Dialog.Description>
        Create a new nature with increased and decreased stats
      </Dialog.Description>
    </Dialog.Header>
    <form onsubmit={createNewNature} class="flex flex-col gap-4">
      <div>
        <Label for="name" class="text-sm font-medium text-slate-700 mb-2 block"
          >Name</Label
        >
        <Input id="name" type="text" bind:value={newNature.name} />
      </div>
      <div>
        <Label
          for="increased_stat"
          class="text-sm font-medium text-slate-700 mb-2 block"
          >Increased Stat</Label
        >
        <Select.Root type="single" bind:value={newNature.increased_stat}>
          <Select.Trigger id="increased_stat" class="w-[11rem]">
            {capitalizeWords(newNature.increased_stat || "Select Stat")}
          </Select.Trigger>
          <Select.Content>
            {#each natureOptions as option}
              <Select.Item
                label={option.label}
                value={option.value}
                onselect={() => (newNature.increased_stat = option.value)}
              >
                {capitalizeWords(option.label)}
              </Select.Item>
            {/each}
          </Select.Content>
        </Select.Root>
      </div>
      <div>
        <Label
          for="decreased_stat"
          class="text-sm font-medium text-slate-700 mb-2 block"
          >Decreased Stat</Label
        >
        <Select.Root type="single" bind:value={newNature.decreased_stat}>
          <Select.Trigger id="decreased_stat" class="w-[11rem]">
            {capitalizeWords(
              newNature.decreased_stat?.toString() || "Select Stat",
            )}
          </Select.Trigger>
          <Select.Content>
            {#each natureOptions as option}
              <Select.Item
                value={option.value}
                label={option.label}
                onselect={() => (newNature.decreased_stat = option.value)}
              >
                {capitalizeWords(option.label)}
              </Select.Item>
            {/each}
          </Select.Content>
        </Select.Root>
      </div>
      <Dialog.Footer>
        <Button
          type="submit"
          class="w-32"
          disabled={newNature.name === "" ||
            newNature.decreased_stat === "" ||
            newNature.increased_stat === ""}
        >
          Create Nature
        </Button>
      </Dialog.Footer>
    </form>
  </Dialog.Content>
</Dialog.Root>

<Card.Root>
  <Card.Content class="flex flex-row gap-3">
    <Autocomplete
      open={natureSearchOpen}
      value={natureSearch[1]}
      bind:searcher={searchingNature}
      {options}
      placeholder="Search Nature"
      onSelect={(option) => {
        natureSearch = [option.value, option.label];
        getNature();
      }}
      class="w-[20rem]"
    />
    <div class="flex flex-row justify-between w-full">
      <Button
        class="cursor-pointer"
        onclick={saveNatureChanges}
        disabled={isEqual(nature, originalNatureDetails)}
      >
        <SaveIcon />
        Save Changes</Button
      >
      <Button
        variant="outline"
        class="cursor-pointer"
        onclick={() => (newNatureModalOpen = true)}
      >
        Create New Nature</Button
      >
    </div>
  </Card.Content>
</Card.Root>

{#if !objectIsEmpty(nature)}
  <Card.Root class="mt-5">
    <Card.Content class="flex flex-col gap-4">
      <div>
        <Label
          for="increased_stat"
          class="text-sm font-medium text-slate-700 mb-2 block"
          >Increased Stat</Label
        >
        <Select.Root type="single" bind:value={nature.increased_stat}>
          <Select.Trigger id="increased_stat" class="w-[11rem]">
            {capitalizeWords(nature.increased_stat || "Select Stat")}
          </Select.Trigger>
          <Select.Content>
            {#each natureOptions as option}
              <Select.Item
                label={option.label}
                value={option.value}
                onselect={() => (nature.increased_stat = option.value)}
              >
                {capitalizeWords(option.label)}
              </Select.Item>
            {/each}
          </Select.Content>
        </Select.Root>
      </div>
      <div>
        <Label
          for="decreased_stat"
          class="text-sm font-medium text-slate-700 mb-2 block"
          >Decreased Stat</Label
        >
        <Select.Root type="single" bind:value={nature.decreased_stat}>
          <Select.Trigger id="decreased_stat" class="w-[11rem]">
            {capitalizeWords(
              nature.decreased_stat?.toString() || "Select Stat",
            )}
          </Select.Trigger>
          <Select.Content>
            {#each natureOptions as option}
              <Select.Item
                value={option.value}
                label={option.label}
                onselect={() => (nature.decreased_stat = option.value)}
              >
                {capitalizeWords(option.label)}
              </Select.Item>
            {/each}
          </Select.Content>
        </Select.Root>
      </div>
      {#if !nature.is_new}
        <div class="flex flex-row space-x-2 items-center mt-5">
          <Checkbox
            id="is-modified"
            bind:checked={natureModified}
            onchange={setModified}
            class="text-sm font-medium leading-6 text-gray-900"
          />
          <Label
            for="is-modified"
            class="block text-sm font-medium leading-6 text-gray-900"
          >
            Mark Nature as Modified
          </Label>
        </div>
      {/if}
    </Card.Content>
  </Card.Root>
  <Button
    class="my-5 bg-red-200 hover:bg-red-400 cursor-pointer"
    onclick={() => {
      deleteNature();
    }}
  >
    <TrashIcon />
    Delete Nature</Button
  >
{/if}
