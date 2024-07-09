<script lang="ts">
import _ from "lodash";
import AutoComplete from "$lib/components/AutoComplete.svelte";
import Button from "$lib/components/Button.svelte";
import BaseModal from "$lib/components/BaseModal.svelte";
import TextInput from "$lib/components/TextInput.svelte";
import {
  naturesList,
  modifiedNatures,
  type Nature,
  natures,
} from "../../store/natures";
import { selectedWiki } from "../../store";
import { modifiedItems } from "../../store/items";
import { modifiedAbilities } from "../../store/abilities";
import { BaseDirectory, writeTextFile } from "@tauri-apps/api/fs";
import { getToastStore } from "@skeletonlabs/skeleton";
import SelectInput from "$lib/components/SelectInput.svelte";
import { invoke } from "@tauri-apps/api";
import { updateNatureModifications } from "$lib/utils/modificationHelpers";

const toastStore = getToastStore();

let natureName: string = "";
let currentNatureName: string = "";
let natureDetails: Nature = {} as Nature;
let originalNatureDetails: Nature = {} as Nature;

let newNatureModalOpen: boolean = false;
let newNatureName: string = "";
let newNatureDetails: Nature = { increased_stat: "", decreased_stat: "" };

$: natureListOptions = $naturesList.map((nature) => ({
  label: nature,
  value: nature,
}));

const natureOptions = [
  "",
  "hp",
  "attack",
  "defense",
  "special-attack",
  "special-defense",
  "speed",
].map((nature) => {
  return { label: nature, value: nature };
});

async function saveNatureChanges() {
  updateNatureModifications(currentNatureName, natureDetails);
  $natures[currentNatureName] = natureDetails;

  await writeTextFile(
    `${$selectedWiki.name}/data/natures.json`,
    JSON.stringify($natures),
    { dir: BaseDirectory.AppData },
  ).then(() => {
    originalNatureDetails = _.cloneDeep(natureDetails);
    invoke("generate_nature_page", { wikiName: $selectedWiki.name })
      .then(() => {
        toastStore.trigger({
          message: "Nature page regenerated!",
          background: "variant-filled-success",
        });
      })
      .catch((err) => {
        console.error("Failed to regenerate nature page: ", err);
      });
    toastStore.trigger({
      message: "Nature changes saved!",
      background: "variant-filled-success",
    });
  });
}

async function createNewNature() {
  if ($natures[newNatureName]) {
    toastStore.trigger({
      message: "Nature already exists!",
      background: "variant-filled-error",
    });
    return;
  }

  $modifiedNatures[newNatureName] = {
    original: {
      increased_stat: newNatureDetails.increased_stat,
      decreased_stat: newNatureDetails.decreased_stat,
    },
    modified: {
      increased_stat: "",
      decreased_stat: "",
    },
    is_new_nature: true,
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

  $natures[newNatureName] = _.clone(newNatureDetails);
  console.log($natures[newNatureName]);
  naturesList.update((list) => {
    list.push(newNatureName);
    return list;
  });

  await writeTextFile(
    `${$selectedWiki.name}/data/natures.json`,
    JSON.stringify($natures),
    { dir: BaseDirectory.AppData },
  ).then(() => {
    originalNatureDetails = _.cloneDeep(natureDetails);
    toastStore.trigger({
      message: "New Nature Created!",
      background: "variant-filled-success",
    });
    newNatureModalOpen = false;
    newNatureName = "";
    newNatureDetails = { increased_stat: "", decreased_stat: "" };
    invoke("generate_nature_page", { wikiName: $selectedWiki.name }).then(
      () => {
        toastStore.trigger({
          message: "Nature page regenerated!",
          background: "variant-filled-success",
        });
      },
    );
  });
}
async function deleteNature() {
  if ($modifiedNatures[currentNatureName]) {
    delete $modifiedNatures[currentNatureName];

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

  $natures = Object.entries($natures)
    .filter(([name, _]) => name !== currentNatureName)
    .map(([name, nature]) => ({ [name]: nature }))
    .reduce((acc, nature) => ({ ...acc, ...nature }), {});

  naturesList.update((list) => {
    return list.filter((nature) => nature !== currentNatureName);
  });

  await writeTextFile(
    `${$selectedWiki.name}/data/items.json`,
    JSON.stringify($natures),
    { dir: BaseDirectory.AppData },
  ).then(() => {
    currentNatureName = "";
    natureName = "";
    natureDetails = {} as Nature;
    originalNatureDetails = _.cloneDeep(natureDetails);
    toastStore.trigger({
      message: "Nature Deleted!",
      background: "variant-filled-success",
    });
    invoke("generate_nature_page", { wikiName: $selectedWiki.name }).then(
      () => {
        toastStore.trigger({
          message: "Nature page regenerated!",
          background: "variant-filled-success",
        });
      },
    );
  });
}
</script>

<BaseModal class="w-[30rem]" bind:open={newNatureModalOpen}>
  <h2 class="text-lg font-medium leading-6 text-gray-900">Create New Nature</h2>
  <TextInput label="New Nature Name" bind:value={newNatureName} />

  <SelectInput
    label="Increased Stat"
    bind:value={newNatureDetails.increased_stat}
    options={natureOptions}
  />
  <SelectInput
    label="Decreased Stat"
    bind:value={newNatureDetails.decreased_stat}
    options={natureOptions}
  />
  <Button
    title="Create Nature"
    class="w-32"
    disabled={newNatureName === ""}
    onClick={createNewNature}
  />
</BaseModal>

<div class="flex flex-row gap-7">
  <AutoComplete
    bind:value={natureName}
    placeholder="Search Natures"
    options={natureListOptions}
    popupId="natures-search"
    onSelection={(e) => {
      natureName = e.detail.value;
    }}
    showChevron={false}
  />
  <Button
    title="Search"
    onClick={() => {
      currentNatureName = natureName;
      natureDetails = _.cloneDeep($natures[natureName]);
        originalNatureDetails = _.cloneDeep(natureDetails);
      }}
    disabled={natureName === ""}
    class="mt-2 w-32"
  />
  <Button
    title="Save Changes"
    onClick={saveNatureChanges}
    disabled={_.isEqual(natureDetails, originalNatureDetails)}
    class="mt-2 w-32"
  />
  <Button
    title="Add Nature"
    class="ml-auto mr-3 mt-2 w-32"
    onClick={() => newNatureModalOpen = true}
  />
  <Button
    title="Delete Nature"
    class="mr-5 mt-2 w-32"
    disabled={currentNatureName === ""}
    onClick={deleteNature}
  />
</div>

{#if !_.isEmpty(natureDetails)}
  <p class="mt-4 text-lg">
    {_.capitalize(currentNatureName.replaceAll("-", " "))}
  </p>
  <div class="grid grid-cols-2 gap-5">
    <SelectInput
      label="Increased Stat"
      bind:value={natureDetails.increased_stat}
      options={natureOptions}
    />
    <SelectInput
      label="Decreased Stat"
      bind:value={natureDetails.decreased_stat}
      options={natureOptions}
    />
  </div>
{/if}
