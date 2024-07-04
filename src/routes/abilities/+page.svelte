<script lang="ts">
import _ from "lodash";
import AutoComplete from "$lib/components/AutoComplete.svelte";
import Button from "$lib/components/Button.svelte";
import { abilitiesList, type Ability, abilities } from "../../store/abilities";
import { selectedWiki } from "../../store";
import { BaseDirectory, writeTextFile } from "@tauri-apps/api/fs";
import { getToastStore } from "@skeletonlabs/skeleton";

const toastStore = getToastStore();

let abilityName: string = "";
let currentAbilityName: string = "";
let abilityDetails: Ability = {} as Ability;
let originalAbilityDetails: Ability = {} as Ability;

// let natureListOptions = $naturesList.map((nature) => ({
//   label: nature,
//   value: nature,
// }));

let abilityListOptions = $abilitiesList.map((ability) => ({
  label: ability,
  value: ability,
}));

async function saveAbilityChanges() {
  $abilities[currentAbilityName] = abilityDetails;

  await writeTextFile(
    `${$selectedWiki.name}/data/abilities.json`,
    JSON.stringify($abilities),
    { dir: BaseDirectory.AppData },
  ).then(() => {
    originalAbilityDetails = _.cloneDeep(abilityDetails);
    // invoke("generate_items_page", {
    //   wikiName: $selectedWiki.name,
    //   dexNumbers: [pokemonId],
    // });
    toastStore.trigger({
      message: "Ability changes saved!",
      background: "variant-filled-success",
    });
  });
}
</script>

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
