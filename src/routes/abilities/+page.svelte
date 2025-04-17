<script lang="ts">
  import AutoComplete from "$lib/components/AutoComplete.svelte";
  import Button from "$lib/components/Button.svelte";
  import BaseModal from "$lib/components/BaseModal.svelte";
  import TextInput from "$lib/components/TextInput.svelte";
  import {
    abilitiesList,
    type Ability,
    type SearchAbility,
  } from "../../store/abilities";
  import { selectedWiki } from "../../store";
  import { getToastStore } from "@skeletonlabs/skeleton";
  import { invoke } from "@tauri-apps/api/core";
  import { db } from "../../store/db";
  import { FALSE, TRUE } from "$lib/utils/CONSTANTS";
  import { isNullEmptyOrUndefined } from "$lib/utils";
  import { cloneDeep } from "$lib/utils/cloneDeep";
  import capitalizeWords from "$lib/utils/capitalizeWords";
  import isEqual from "$lib/utils/isEqual";
  import objectIsEmpty from "$lib/utils/objectIsEmpty";
  import { getToastSettings, ToastType } from "$lib/utils/toasts";

  const toastStore = getToastStore();

  let abilitySearch: [number, string] = [0, ""];

  let ability: Ability = {} as Ability;
  let originalAbilityDetails: Ability = {} as Ability;

  let newAbilityModalOpen: boolean = false;
  let newAbility: Ability = {} as Ability;

  $: abilityListOptions = $abilitiesList.map(([id, name]) => ({
    label: name,
    value: id,
  }));

  async function generateAbilityPage() {
    await invoke("generate_ability_page_with_handle", {
      wikiName: $selectedWiki.name,
    })
      .then((res) => {
        toastStore.trigger(getToastSettings(ToastType.SUCCESS, res as string));
      })
      .catch((err) => {
        toastStore.trigger(getToastSettings(ToastType.ERROR, err));
      });
  }

  async function getAbility() {
    let retrievedAbility = $abilitiesList.find(
      ([_, name]) => name === abilitySearch[1],
    );

    if (!retrievedAbility) {
      toastStore.trigger(
        getToastSettings(ToastType.ERROR, "Ability not found!"),
      );
      return;
    }

    await $db
      .select<Ability[]>("SELECT * FROM abilities WHERE id = $1;", [
        abilitySearch[0],
      ])
      .then(async (res) => {
        ability = res[0];
        originalAbilityDetails = cloneDeep(ability);
      })
      .catch((err) => {
        toastStore.trigger(
          getToastSettings(ToastType.ERROR, `Error retrieving ability ${err}`),
        );
      });
  }

  async function saveAbilityChanges() {
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
        toastStore.trigger(
          getToastSettings(
            ToastType.ERROR,
            `Error saving ability changes: ${err}`,
          ),
        );
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
        toastStore.trigger(
          getToastSettings(
            ToastType.ERROR,
            `Error creating new ability: ${err}`,
          ),
        );
      });
  }

  async function deleteAbility() {
    await $db
      .execute("DELETE FROM abilities WHERE id = $1;", [ability.id])
      .then(() => {
        toastStore.trigger({
          message: "Ability deleted!",
          background: "variant-filled-success",
        });
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
        toastStore.trigger(
          getToastSettings(ToastType.ERROR, `Error deleting ability: ${err}`),
        );
      });
  }
  function setModified(e: any) {
    ability.is_modified = e.target?.checked ? TRUE : FALSE;
  }
</script>

<BaseModal class="w-[30rem]" bind:open={newAbilityModalOpen}>
  <h2 class="text-lg font-medium leading-6 text-gray-900">
    Create New Ability
  </h2>
  <TextInput
    label="New Ability Name"
    bind:value={newAbility.name}
    inputHandler={(e) => {
      newAbility.name = e.target.value.toLowerCase().replaceAll(" ", "-");
    }}
  />
  <div>
    <label
      for="effect"
      class="block text-sm font-medium leading-6 text-gray-900">Effect</label
    >
    <div class="mt-2">
      <textarea
        id="effect"
        bind:value={newAbility.effect}
        class="block h-32 w-full rounded-md border-0 py-1.5 pl-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 disabled:bg-gray-100 disabled:text-gray-400 sm:text-sm sm:leading-6"
      />
    </div>
  </div>
  <Button
    title="Create Ability"
    class="w-32"
    disabled={isNullEmptyOrUndefined(newAbility.name) ||
      isNullEmptyOrUndefined(newAbility.effect)}
    onClick={createNewAbility}
  />
</BaseModal>

<div class="flex flex-row gap-7">
  <AutoComplete
    bind:value={abilitySearch[1]}
    placeholder="Search Abilities"
    options={abilityListOptions}
    popupId="abilities-search"
    onSelection={(e) => {
      abilitySearch = [e.detail.value, e.detail.label];
    }}
    showChevron={false}
  />
  <Button
    title="Search"
    onClick={getAbility}
    disabled={abilitySearch[0] === 0}
    class="mt-2 w-32"
  />
  <Button
    title="Save Changes"
    onClick={saveAbilityChanges}
    disabled={isEqual(ability, originalAbilityDetails)}
    class="mt-2 w-32"
  />
  <Button
    title="Add Ability"
    class="ml-auto mr-3 mt-2 w-32"
    onClick={() => (newAbilityModalOpen = true)}
  />
  <Button
    title="Delete Ability"
    class="mr-5 mt-2 w-32"
    disabled={objectIsEmpty(ability)}
    onClick={deleteAbility}
  />
</div>

{#if !objectIsEmpty(ability)}
  <div class="mt-4 flex flex-col gap-4">
    <p class="mt-4 text-lg">
      {capitalizeWords(ability.name.replaceAll("-", " "))}
    </p>
    <div>
      <label
        for="effect"
        class="block text-sm font-medium leading-6 text-gray-900">Effect</label
      >
      <div class="mt-2">
        <textarea
          id="effect"
          bind:value={ability.effect}
          class="block h-32 w-[50rem] rounded-md border-0 py-1.5 pl-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 disabled:bg-gray-100 disabled:text-gray-400 sm:text-sm sm:leading-6"
        />
      </div>
    </div>
    {#if !ability.is_new}
      <label class="block text-sm font-medium leading-6 text-gray-900">
        <input
          type="checkbox"
          checked={Boolean(ability.is_modified)}
          on:change={setModified}
          class="text-sm font-medium leading-6 text-gray-900"
        />
        Mark Item as Modified
      </label>
    {/if}
  </div>
{/if}
