<script lang="ts">
import NumberInput from "$lib/components/NumberInput.svelte";
import SelectInput from "$lib/components/SelectInput.svelte";
import {
  Autocomplete,
  getToastStore,
  popup,
  type AutocompleteOption,
  type PopupSettings,
  type ToastSettings,
} from "@skeletonlabs/skeleton";
import { BaseDirectory, writeTextFile } from "@tauri-apps/api/fs";
import _ from "lodash";
import { selectedWiki } from "../../store";
import { moveList, moves, type MoveDetails } from "../../store/moves";
import { PokemonTypes, pokemon } from "../../store/pokemon";
import { invoke } from "@tauri-apps/api/tauri";

const toastStore = getToastStore();

let moveName: string = "";
let currentMoveName: string = "";
let moveDetails: MoveDetails = {} as MoveDetails;
let originalMoveDetails: MoveDetails = {} as MoveDetails;

const moveListOptions: AutocompleteOption<string>[] = $moveList.map((name) => ({
  label: name,
  value: name,
}));

const autoCompletePopup: PopupSettings = {
  event: "focus-click",
  target: "popupAutoComplete",
  placement: "bottom",
};

const moveDataSavedToast: ToastSettings = {
  message: "Data saved",
  timeout: 3000,
  background: "variant-filled-success",
};

function onMoveNameSelected(
  event: CustomEvent<AutocompleteOption<string>>,
): void {
  moveName = event.detail.value;
}

function getMoveDetails() {
  moveDetails = _.cloneDeep($moves.moves[moveName]);
  originalMoveDetails = _.cloneDeep(moveDetails);
  currentMoveName = moveName;
}

async function updatePagesForPokemonWithMove() {
  let pokemonToUpdate = Object.values($pokemon.pokemon)
    .filter((p) => Object.keys(p.moves).includes(moveName))
    .map((p) => p.id);

  await invoke("generate_pokemon_pages_from_list", {
    dexNumbers: pokemonToUpdate,
    wikiName: $selectedWiki.name,
  });
}

async function saveMoveChanges() {
  $moves.moves[moveName] = moveDetails;
  await writeTextFile(
    `${$selectedWiki.name}/data/moves.json`,
    JSON.stringify($moves),
    { dir: BaseDirectory.AppData },
  ).then(() => {
    originalMoveDetails = _.cloneDeep(moveDetails);
    toastStore.trigger(moveDataSavedToast);
    updatePagesForPokemonWithMove();
  });
}
</script>

<div class="flex flex-row gap-7">
  <div class="mt-2 w-60">
    <input
      id="pokemon-name"
      type="text"
      placeholder="Move Name"
      class="block w-full rounded-md border-0 py-1.5 pl-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 disabled:bg-gray-100 disabled:text-gray-400 sm:text-sm sm:leading-6"
      bind:value={moveName}
      use:popup={autoCompletePopup}
    />
    <div
      data-popup="popupAutoComplete"
      class="card mt-2 w-60 overflow-y-auto rounded-sm bg-white"
      tabindex="-1"
    >
      <Autocomplete
        bind:input={moveName}
        options={moveListOptions}
        limit={5}
        on:selection={onMoveNameSelected}
        class="w-full rounded-md border bg-white p-2 text-sm"
      />
    </div>
  </div>
  <button
    disabled={moveName === ""}
    on:click={getMoveDetails}
    class="mt-2 w-32 rounded-md bg-indigo-600 text-sm font-semibold text-white
      shadow-sm hover:bg-indigo-500 focus-visible:outline
      focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600
      disabled:bg-indigo-400"
  >
    Search
  </button>
  <button
    disabled={_.isEqual(moveDetails, originalMoveDetails)}
    on:click={saveMoveChanges}
    class="mt-2 w-32 rounded-md bg-indigo-600 text-sm font-semibold text-white
      shadow-sm hover:bg-indigo-500 focus-visible:outline
      focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600
      disabled:bg-indigo-400"
  >
    Save Changes
  </button>
</div>

{#if !_.isEmpty(moveDetails)}
  <p class="ml-2 mt-4 text-lg">{_.capitalize(currentMoveName)}</p>
  <div class="ml-2 mt-4">
    <div class="grid grid-cols-2 gap-x-10 gap-y-5 pr-4">
      <NumberInput label="Power" id="power" bind:value={moveDetails.power} />
      <SelectInput
        label="Type"
        id="type"
        bind:value={moveDetails.type}
        options={PokemonTypes.map((type) => ({
          label: _.capitalize(type),
          value: type,
        }))}
      />
      <NumberInput
        label="Accuracy"
        id="accuracy"
        bind:value={moveDetails.accuracy}
      />
      <NumberInput label="PP" id="pp" bind:value={moveDetails.pp} />
      <SelectInput
        label="Damage Class"
        id="damage-class"
        bind:value={moveDetails.damage_class}
        options={[
          { label: "status", value: "status" },
          { label: "physical", value: "physical" },
          { label: "special", value: "special" },
        ]}
      />
    </div>
  </div>
{/if}
