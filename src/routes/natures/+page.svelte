<script lang="ts">
import _ from "lodash";
import AutoComplete from "$lib/components/AutoComplete.svelte";
import Button from "$lib/components/Button.svelte";
import BaseModal from "$lib/components/BaseModal.svelte";
import TextInput from "$lib/components/TextInput.svelte";
import {
  naturesList,
  type Nature,
  type SearchNature,
} from "../../store/natures";
import { selectedWiki } from "../../store";
import { getToastStore } from "@skeletonlabs/skeleton";
import SelectInput from "$lib/components/SelectInput.svelte";
import { invoke } from "@tauri-apps/api";
import { db } from "../../store/db";
import { FALSE, TRUE } from "$lib/utils/CONSTANTS";

const toastStore = getToastStore();

let natureSearch: [number, string] = [0, ""];

let nature: Nature = {} as Nature;
let originalNatureDetails: Nature = {} as Nature;

let newNatureModalOpen: boolean = false;
let newNature: Nature = {} as Nature;

$: natureListOptions = $naturesList.map(([id, name]) => ({
  label: name,
  value: id,
}));

const natureOptions = [
  null,
  "hp",
  "attack",
  "defense",
  "special-attack",
  "special-defense",
  "speed",
].map((nature) => {
  if (nature === null) {
    return { label: "None", value: null };
  }
  return { label: nature, value: nature };
});

async function generateNaturePage() {
  await invoke("generate_nature_page", { wikiName: $selectedWiki.name }).then(
    () => {
      toastStore.trigger({
        message: "Nature page regenerated!",
        background: "variant-filled-success",
      });
    },
  );
}

async function getNature() {
  let retrievedNature = $naturesList.find(
    ([_, name]) => name === natureSearch[1],
  );

  if (!retrievedNature) {
    toastStore.trigger({
      message: "Nature not found!",
      background: "variant-filled-error",
    });
    return;
  }

  await $db
    .select<
      Nature[]
    >("SELECT * FROM natures WHERE name = $1", [natureSearch[1]])
    .then((result) => {
      nature = result[0];
      originalNatureDetails = _.cloneDeep(nature);
    });
}

async function saveNatureChanges() {
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
      originalNatureDetails = _.cloneDeep(nature);
      toastStore.trigger({
        message: "Nature changes saved!",
        background: "variant-filled-success",
      });
      generateNaturePage();
    })
    .catch(() => {
      toastStore.trigger({
        message: "Error saving nature changes!",
        background: "variant-filled-error",
      });
    });
}

async function createNewNature() {
  newNature.is_new = TRUE;

  await $db
    .execute(
      "INSERT INTO abilities (name, increased_stat, decreased_stat, is_new) VALUES ($1, $2, $3, $4);",
      [
        newNature.name,
        newNature.increased_stat,
        newNature.decreased_stat,
        newNature.is_new,
      ],
    )
    .then(() => {
      toastStore.trigger({
        message: "Nature created!",
        background: "variant-filled-success",
      });
      newNatureModalOpen = false;
      newNature = {} as Nature;

      // Update the natures list
      $db.select("SELECT id, name FROM natures").then((abilities: any) => {
        naturesList.set(
          abilities.map((nature: SearchNature) => [nature.id, nature.name]),
        );
      });
      generateNaturePage();
    })
    .catch(() => {
      toastStore.trigger({
        message: "Error creating new nature!",
        background: "variant-filled-error",
      });
    });
}

async function deleteNature() {
  await $db
    .execute("DELETE FROM natures WHERE id = $1;", [nature.id])
    .then(() => {
      toastStore.trigger({
        message: "Nature deleted!",
        background: "variant-filled-success",
      });
      /// Update the abilities list
      $db.select("SELECT id, name FROM abilities").then((natures: any) => {
        naturesList.set(
          natures.map((nature: SearchNature) => [nature.id, nature.name]),
        );
      });
      nature = {} as Nature;
      originalNatureDetails = {} as Nature;
      generateNaturePage();
    })
    .catch(() => {
      toastStore.trigger({
        message: "Error deleting nature!",
        background: "variant-filled-error",
      });
    });
}

function setModified(e: any) {
  nature.is_modified = e.target?.checked ? TRUE : FALSE;
}
async function convertNatureToSqlite() {
  await invoke("convert_natures_to_sqlite", {
    wikiName: $selectedWiki.name,
  })
    .then(() => {
      toastStore.trigger({
        message: "Natures converted!",
        background: "variant-filled-success",
      });
    })
    .catch((err) => {
      toastStore.trigger({
        message: "Error converting natures!",
        background: "variant-filled-error",
      });
    });
}
</script>

<BaseModal class="w-[30rem]" bind:open={newNatureModalOpen}>
  <h2 class="text-lg font-medium leading-6 text-gray-900">Create New Nature</h2>
  <TextInput label="New Nature Name" bind:value={newNature.name} />
  <SelectInput
    label="Increased Stat"
    bind:value={newNature.increased_stat}
    options={natureOptions}
  />
  <SelectInput
    label="Decreased Stat"
    bind:value={newNature.decreased_stat}
    options={natureOptions}
  />
  <Button
    title="Create Nature"
    class="w-32"
    disabled={newNature.name === ""}
    onClick={createNewNature}
  />
</BaseModal>

<div class="flex flex-row gap-7">
  <AutoComplete
    bind:value={natureSearch[1]}
    placeholder="Search Natures"
    options={natureListOptions}
    popupId="natures-search"
    onSelection={(e) => {
      natureSearch = [e.detail.value, e.detail.label];
    }}
    showChevron={false}
  />
  <Button
    title="Search"
    onClick={getNature}
    disabled={natureSearch[0] === 0}
    class="mt-2 w-32"
  />
  <Button
    title="Save Changes"
    onClick={saveNatureChanges}
    disabled={_.isEqual(nature, originalNatureDetails)}
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
    disabled={nature.name === ""}
    onClick={deleteNature}
  />
</div>

{#if !_.isEmpty(nature)}
  <p class="mt-4 text-lg">
    {_.capitalize(nature.name.replaceAll("-", " "))}
  </p>
  <div class="grid grid-cols-2 gap-5">
    <SelectInput
      label="Increased Stat"
      bind:value={nature.increased_stat}
      options={natureOptions}
    />
    <SelectInput
      label="Decreased Stat"
      bind:value={nature.decreased_stat}
      options={natureOptions}
    />
  </div>
  {#if !nature.is_new}
    <label class="block text-sm font-medium leading-6 text-gray-900">
      <input
        type="checkbox"
        checked={Boolean(nature.is_modified)}
        on:change={setModified}
        class="text-sm font-medium leading-6 text-gray-900"
      />
      Mark Nature as Modified
    </label>
  {/if}
{/if}
<!-- <Button title="Convert Natures to SQLite" onClick={convertNatureToSqlite} /> -->
