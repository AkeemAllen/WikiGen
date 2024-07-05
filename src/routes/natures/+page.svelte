<script lang="ts">
import _ from "lodash";
import AutoComplete from "$lib/components/AutoComplete.svelte";
import Button from "$lib/components/Button.svelte";
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

const toastStore = getToastStore();

let natureName: string = "";
let currentNatureName: string = "";
let natureDetails: Nature = {} as Nature;
let originalNatureDetails: Nature = {} as Nature;

let natureListOptions = $naturesList.map((nature) => ({
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
  if (!$modifiedNatures[currentNatureName]) {
    $modifiedNatures[currentNatureName] = {
      original: null,
      modified: null,
    };
  }

  if ($modifiedNatures[currentNatureName].original === null) {
    $modifiedNatures[currentNatureName].original = {
      increased_stat: $natures[currentNatureName].increased_stat,
      decreased_stat: $natures[currentNatureName].decreased_stat,
    };
  }

  $modifiedNatures[currentNatureName].modified = {
    increased_stat: natureDetails.increased_stat,
    decreased_stat: natureDetails.decreased_stat,
  };

  if (
    _.isEqual(
      $modifiedNatures[currentNatureName].original,
      $modifiedNatures[currentNatureName].modified,
    )
  ) {
    delete $modifiedNatures[currentNatureName];
  }

  $natures[currentNatureName] = natureDetails;

  await writeTextFile(
    `${$selectedWiki.name}/data/modifications/modified_items_natures_abilities.json`,
    JSON.stringify({
      items: $modifiedItems,
      natures: $modifiedNatures,
      abilities: $modifiedAbilities,
    }),
    { dir: BaseDirectory.AppData },
  );

  await writeTextFile(
    `${$selectedWiki.name}/data/natures.json`,
    JSON.stringify($natures),
    { dir: BaseDirectory.AppData },
  ).then(() => {
    originalNatureDetails = _.cloneDeep(natureDetails);
    // invoke("generate_items_page", {
    //   wikiName: $selectedWiki.name,
    //   dexNumbers: [pokemonId],
    // });
    toastStore.trigger({
      message: "Nature changes saved!",
      background: "variant-filled-success",
    });
  });
}
</script>

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
