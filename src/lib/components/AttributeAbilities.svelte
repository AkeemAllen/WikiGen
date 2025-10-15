<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import {
    abilitiesList,
    type Ability,
    type SearchAbility,
  } from "../../store/abilities";
  import { selectedWiki } from "../../store";
  import { invoke } from "@tauri-apps/api/core";
  import { db } from "../../store/db";
  import { FALSE, TRUE } from "$lib/utils/CONSTANTS";
  import { isNullEmptyOrUndefined } from "$lib/utils";
  import { cloneDeep } from "$lib/utils/cloneDeep";
  import capitalizeWords from "$lib/utils/capitalizeWords";
  import isEqual from "$lib/utils/isEqual";
  import objectIsEmpty from "$lib/utils/objectIsEmpty";
  import { toast } from "svelte-sonner";
  import * as Card from "$lib/components/ui/card";
  import Autocomplete from "./ui/Autocomplete.svelte";
  import SaveIcon from "@lucide/svelte/icons/save";
  import TrashIcon from "@lucide/svelte/icons/trash";
  import { Textarea } from "$lib/components/ui/textarea/index.js";
  import { Checkbox } from "./ui/checkbox";
  import { Label } from "./ui/label";
  import * as Dialog from "$lib/components/ui/dialog";
  import { Input } from "./ui/input";

  let abilitySearch: [number, string] = $state([0, ""]);

  let ability: Ability = $state({} as Ability);
  let abilitySearchOpen = $state(false);
  let searchingAbility = $state("");
  let abilityModified = $state(false);
  let originalAbilityDetails: Ability = $state({} as Ability);

  let newAbilityModalOpen: boolean = $state(false);
  let newAbility: Ability = $state({} as Ability);

  let abilityListOptions = $derived(
    $abilitiesList.map(([id, name]) => ({
      label: name,
      value: id,
    })),
  );
  let options = $derived(
    abilityListOptions
      .filter((item) =>
        item.label.toLowerCase().includes(searchingAbility.toLowerCase()),
      )
      .slice(0, 8),
  );

  async function generateAbilityPage() {
    await invoke("generate_ability_page_with_handle", {
      wikiName: $selectedWiki.name,
    })
      .then((res) => {
        toast.success(res as string);
      })
      .catch((err) => {
        toast.error(err);
      });
  }

  async function getAbility() {
    let retrievedAbility = $abilitiesList.find(
      ([_, name]) => name === abilitySearch[1],
    );

    if (!retrievedAbility) {
      toast.error("Ability not found!");
      return;
    }

    await $db
      .select<Ability[]>("SELECT * FROM abilities WHERE id = $1;", [
        abilitySearch[0],
      ])
      .then(async (res) => {
        ability = res[0];
        abilityModified = ability.is_modified === 1;
        originalAbilityDetails = cloneDeep(ability);
      })
      .catch((err) => {
        toast.error(`Error retrieving ability ${err}`);
      });
  }

  async function saveAbilityChanges() {
    ability.is_modified = abilityModified ? 1 : 0;
    await $db
      .execute(
        "UPDATE abilities SET effect = $1, is_modified = $2 WHERE id = $3;",
        [ability.effect, ability.is_modified, abilitySearch[0]],
      )
      .then(() => {
        originalAbilityDetails = cloneDeep(ability);
        generateAbilityPage();
      })
      .catch((err) => {
        toast.error(`Error saving ability changes: ${err}`);
      });
  }

  async function createNewAbility() {
    newAbility.is_new = TRUE;

    await $db
      .execute(
        "INSERT INTO abilities (name, effect, is_new) VALUES ($1, $2, $3);",
        [
          newAbility.name.toLowerCase().replaceAll(" ", "-"),
          newAbility.effect,
          newAbility.is_new,
        ],
      )
      .then(() => {
        newAbilityModalOpen = false;
        newAbility = {} as Ability;

        // Update the abilities list
        $db.select("SELECT id, name FROM abilities").then((abilities: any) => {
          abilitiesList.set(
            abilities.map((ability: SearchAbility) => [
              ability.id,
              ability.name,
            ]),
          );
        });
        generateAbilityPage();
      })
      .catch((err) => {
        toast.error(`Error creating new ability: ${err}`);
      });
  }

  async function deleteAbility() {
    await $db
      .execute("DELETE FROM abilities WHERE id = $1;", [ability.id])
      .then(() => {
        toast.success("Ability deleted!");
        // Update the abilities list
        $db.select("SELECT id, name FROM abilities").then((abilities: any) => {
          abilitiesList.set(
            abilities.map((ability: SearchAbility) => [
              ability.id,
              ability.name,
            ]),
          );
        });
        ability = {} as Ability;
        originalAbilityDetails = {} as Ability;
        generateAbilityPage();
      })
      .catch((err) => {
        toast.error(`Error deleting ability: ${err}`);
      });
  }
  function setModified(e: any) {
    ability.is_modified = e.target?.checked ? TRUE : FALSE;
  }
</script>

<Dialog.Root bind:open={newAbilityModalOpen}>
  <Dialog.Content class="w-[30rem]">
    <Dialog.Header>
      <Dialog.Title>Create New Ability</Dialog.Title>
      <Dialog.Description>
        Create a new ability with name and effect
      </Dialog.Description>
    </Dialog.Header>
    <form onsubmit={createNewAbility} class="flex flex-col gap-4">
      <div>
        <Label
          for="abilityName"
          class="block text-sm mb-2 font-medium leading-6 text-gray-900"
          >Ability Name</Label
        >
        <Input
          id="abilityName"
          bind:value={newAbility.name}
          oninput={(e: any) => {
            newAbility.name = e.target.value.toLowerCase().replaceAll(" ", "-");
          }}
        />
      </div>
      <div>
        <Label
          for="effect"
          class="block mb-2 text-sm font-medium leading-6 text-gray-900"
          >Effect</Label
        >
        <Textarea id="effect" bind:value={newAbility.effect} />
      </div>
      <Dialog.Footer>
        <Button
          class="w-32"
          disabled={isNullEmptyOrUndefined(newAbility.name) ||
            isNullEmptyOrUndefined(newAbility.effect)}
          onclick={createNewAbility}
        >
          Create Ability
        </Button>
      </Dialog.Footer>
    </form>
  </Dialog.Content>
</Dialog.Root>

<Card.Root>
  <Card.Content class="flex flex-row gap-3">
    <Autocomplete
      open={abilitySearchOpen}
      value={abilitySearch[1]}
      bind:searcher={searchingAbility}
      {options}
      placeholder="Search Ability"
      onSelect={(option) => {
        abilitySearch = [option.value, option.label];
        getAbility();
      }}
      class="w-[20rem]"
    />
    <div class="flex flex-row justify-between w-full">
      <Button
        class="cursor-pointer"
        onclick={saveAbilityChanges}
        disabled={isEqual(ability, originalAbilityDetails)}
      >
        <SaveIcon />
        Save Changes</Button
      >
      <Button
        variant="outline"
        class="cursor-pointer"
        onclick={() => (newAbilityModalOpen = true)}
      >
        Create New Ability</Button
      >
    </div>
  </Card.Content>
</Card.Root>

{#if !objectIsEmpty(ability)}
  <Card.Root class="mt-5">
    <Card.Content class="flex flex-col gap-4">
      <div>
        <Label
          for="effect"
          class="block text-sm font-medium leading-6 text-gray-900"
          >Effect</Label
        >
        <div class="mt-2">
          <Textarea id="effect" bind:value={ability.effect} />
        </div>
      </div>
      {#if !ability.is_new}
        <div class="flex flex-row space-x-2 items-center mt-5">
          <Checkbox
            id="is-modified"
            bind:checked={abilityModified}
            onchange={setModified}
            class="text-sm font-medium leading-6 text-gray-900"
          />
          <Label
            for="is-modified"
            class="block text-sm font-medium leading-6 text-gray-900"
          >
            Mark Ability as Modified
          </Label>
        </div>
      {/if}
    </Card.Content>
  </Card.Root>
  <Button
    class="my-5 bg-red-200 hover:bg-red-400 cursor-pointer"
    onclick={() => {
      deleteAbility();
    }}
  >
    <TrashIcon />
    Delete Ability</Button
  >
{/if}
