<script lang="ts">
import _ from "lodash";
import AutoComplete from "$lib/components/AutoComplete.svelte";
import Button from "$lib/components/Button.svelte";
import BaseModal from "$lib/components/BaseModal.svelte";
import TextInput from "$lib/components/TextInput.svelte";
import {
  abilitiesList,
  type Ability,
  abilities,
  modifiedAbilities,
} from "../../store/abilities";
import { pokemon } from "../../store/pokemon";
import { modifiedItems } from "../../store/items";
import { modifiedNatures } from "../../store/natures";
import { selectedWiki } from "../../store";
import { BaseDirectory, writeTextFile } from "@tauri-apps/api/fs";
import { getToastStore } from "@skeletonlabs/skeleton";
import { invoke } from "@tauri-apps/api";
import { updateAbilityModifications } from "$lib/utils/modificationHelpers";

const toastStore = getToastStore();

let abilityName: string = "";
let currentAbilityName: string = "";
let abilityDetails: Ability = {} as Ability;
let originalAbilityDetails: Ability = {} as Ability;

let newAbilityModalOpen: boolean = false;
let newAbilityName: string = "";
let newAbilityDetails: Ability = { effect: "" };

$: abilityListOptions = $abilitiesList.map((ability) => ({
  label: ability,
  value: ability,
}));

$: console.log(abilityDetails);

async function saveAbilityChanges() {
  updateAbilityModifications(currentAbilityName, abilityDetails);
  $abilities[currentAbilityName] = abilityDetails;

  let pokemonWithAbility = Object.values($pokemon.pokemon)
    .filter((pokemon) => pokemon.abilities.includes(currentAbilityName))
    .map((pokemon) => pokemon.id);

  await writeTextFile(
    `${$selectedWiki.name}/data/abilities.json`,
    JSON.stringify($abilities),
    { dir: BaseDirectory.AppData },
  ).then(() => {
    originalAbilityDetails = _.cloneDeep(abilityDetails);
    toastStore.trigger({
      message: "Ability changes saved!",
      background: "variant-filled-success",
    });
    // regenerate pages for pokemon with ability
    invoke("generate_pokemon_pages_from_list", {
      wikiName: $selectedWiki.name,
      dexNumbers: pokemonWithAbility,
    });
    invoke("generate_ability_page", { wikiName: $selectedWiki.name }).then(
      () => {
        toastStore.trigger({
          message: "Ability page regenerated!",
          background: "variant-filled-success",
        });
      },
    );
  });
}

async function createNewAbility() {
  if ($abilities[newAbilityName]) {
    toastStore.trigger({
      message: "Ability already exists!",
      background: "variant-filled-error",
    });
    return;
  }

  $modifiedAbilities[newAbilityName] = {
    original: {
      effect: newAbilityDetails.effect,
    },
    modified: {
      effect: "",
    },
    is_new_ability: true,
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

  $abilities[newAbilityName] = _.clone(newAbilityDetails);
  console.log($abilities[newAbilityName]);
  abilitiesList.update((list) => {
    list.push(newAbilityName);
    return list;
  });

  await writeTextFile(
    `${$selectedWiki.name}/data/abilities.json`,
    JSON.stringify($abilities),
    { dir: BaseDirectory.AppData },
  ).then(() => {
    originalAbilityDetails = _.cloneDeep(abilityDetails);
    toastStore.trigger({
      message: "New Ability Created!",
      background: "variant-filled-success",
    });
    newAbilityModalOpen = false;
    newAbilityName = "";
    newAbilityDetails.effect = "";
    invoke("generate_ability_page", { wikiName: $selectedWiki.name }).then(
      () => {
        toastStore.trigger({
          message: "Ability page regenerated!",
          background: "variant-filled-success",
        });
      },
    );
  });
}

async function deleteAbility() {
  if ($modifiedAbilities[currentAbilityName]) {
    delete $modifiedAbilities[currentAbilityName];

    await writeTextFile(
      `${$selectedWiki.name}/data/modifications/modified_items_natures_abilities.json`,
      JSON.stringify({
        items: $modifiedItems,
        natures: $modifiedNatures,
        abilities: $modifiedAbilities,
      }),
      { dir: BaseDirectory.AppData },
    );
  }

  $abilities = Object.entries($abilities)
    .filter(([name, _]) => name !== currentAbilityName)
    .map(([name, item]) => ({ [name]: item }))
    .reduce((acc, item) => ({ ...acc, ...item }), {});

  abilitiesList.update((list) => {
    return list.filter((ability) => ability !== currentAbilityName);
  });

  await writeTextFile(
    `${$selectedWiki.name}/data/items.json`,
    JSON.stringify($abilities),
    { dir: BaseDirectory.AppData },
  ).then(() => {
    currentAbilityName = "";
    abilityName = "";
    abilityDetails = {} as Ability;
    originalAbilityDetails = _.cloneDeep(abilityDetails);
    toastStore.trigger({
      message: "Ability Deleted!",
      background: "variant-filled-success",
    });
    invoke("generate_ability_page", { wikiName: $selectedWiki.name }).then(
      () => {
        toastStore.trigger({
          message: "Ability page regenerated!",
          background: "variant-filled-success",
        });
      },
    );
  });
}
</script>

<BaseModal class="w-[30rem]" bind:open={newAbilityModalOpen}>
  <h2 class="text-lg font-medium leading-6 text-gray-900">
    Create New Ability
  </h2>
  <TextInput label="New Ability Name" bind:value={newAbilityName} />
  <div>
    <label
      for="effect"
      class="block text-sm font-medium leading-6 text-gray-900">Effect</label
    >
    <div class="mt-2">
      <textarea
        id="effect"
        bind:value={newAbilityDetails.effect}
        class="block h-32 w-full rounded-md border-0 py-1.5 pl-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 disabled:bg-gray-100 disabled:text-gray-400 sm:text-sm sm:leading-6"
      />
    </div>
  </div>
  <Button
    title="Create Ability"
    class="w-32"
    disabled={newAbilityName === "" || newAbilityDetails.effect === ""}
    onClick={createNewAbility}
  />
</BaseModal>

<div class="flex flex-row gap-7">
  <AutoComplete
    bind:value={abilityName}
    placeholder="Search Abilities"
    options={abilityListOptions}
    popupId="abilities-search"
    onSelection={(e) => {
      abilityName = e.detail.value;
    }}
    showChevron={false}
  />
  <Button
    title="Search"
    onClick={() => {
      currentAbilityName = abilityName;
      abilityDetails = _.cloneDeep($abilities[abilityName]);
        originalAbilityDetails = _.cloneDeep(abilityDetails);
      }}
    disabled={abilityName === ""}
    class="mt-2 w-32"
  />
  <Button
    title="Save Changes"
    onClick={saveAbilityChanges}
    disabled={_.isEqual(abilityDetails, originalAbilityDetails)}
    class="mt-2 w-32"
  />
  <Button
    title="Add Ability"
    class="ml-auto mr-3 mt-2 w-32"
    onClick={() => newAbilityModalOpen = true}
  />
  <Button
    title="Delete Ability"
    class="mr-5 mt-2 w-32"
    disabled={currentAbilityName === ""}
    onClick={deleteAbility}
  />
</div>

{#if !_.isEmpty(abilityDetails)}
  <p class="mt-4 text-lg">
    {_.capitalize(currentAbilityName.replaceAll("-", " "))}
  </p>
  <div>
    <label
      for="effect"
      class="block text-sm font-medium leading-6 text-gray-900">Effect</label
    >
    <div class="mt-2">
      <textarea
        id="effect"
        bind:value={abilityDetails.effect}
        class="block h-32 w-[50rem] rounded-md border-0 py-1.5 pl-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 disabled:bg-gray-100 disabled:text-gray-400 sm:text-sm sm:leading-6"
      />
    </div>
  </div>
{/if}
