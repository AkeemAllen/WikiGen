<script lang="ts">
import {
  Autocomplete,
  Tab,
  TabGroup,
  getToastStore,
  popup,
  type AutocompleteOption,
  type PopupSettings,
  type ToastSettings,
} from "@skeletonlabs/skeleton";
import { BaseDirectory, readTextFile, writeTextFile } from "@tauri-apps/api/fs";
import _ from "lodash";
import { selectedWiki } from "../../store";
import {
  pokemon,
  pokemonList,
  modifiedPokemon,
  type PokemonDetails,
} from "../../store/pokemon";
import PokemonDetailsTab from "./PokemonDetailsTab.svelte";
import PokemonMovesTab from "./PokemonMovesTab.svelte";
import { invoke } from "@tauri-apps/api";

const toastStore = getToastStore();

let pokemonName: string = "";
let pokemonId: number = 0;
let pokemonDetails: PokemonDetails = {} as PokemonDetails;
let originalPokemonDetails: PokemonDetails = {} as PokemonDetails;

let tabSet: number = 0;

let pokemonListOptions: AutocompleteOption<string | number>[] =
  $pokemonList.map(([name, id]) => ({ label: name, value: id }));

const autoCompletePopup: PopupSettings = {
  event: "focus-click",
  target: "popupAutoComplete",
  placement: "bottom",
};

function onPokemonNameSelected(
  event: CustomEvent<AutocompleteOption<string | number>>,
): void {
  pokemonName = event.detail.label;
  pokemonId = event.detail.value as number;
}

function getPokemonDetails(): void {
  pokemonDetails = _.cloneDeep($pokemon.pokemon[pokemonId]);
  originalPokemonDetails = _.cloneDeep(pokemonDetails);
}

async function generatePokemonPage() {
  await invoke("generate_pokemon_pages_from_list", {
    wikiName: $selectedWiki.name,
    dexNumbers: [pokemonId],
  });
}

async function checkAndWriteModifiedTypes() {
  if (!_.isEqual(originalPokemonDetails.types, pokemonDetails.types)) {
    if (!$modifiedPokemon[pokemonName]) {
      $modifiedPokemon[pokemonName] = {
        id: pokemonDetails.id,
        evolution: {
          method: "no_change",
          level: 0,
          item: "",
          other: "",
          evolves_to: { id: 0, pokemon_name: "" },
        },
        types: {
          original: [],
          modified: [],
        },
      };
    }
    if ($modifiedPokemon[pokemonName].types.original.length === 0) {
      $modifiedPokemon[pokemonName].types.original =
        originalPokemonDetails.types;
    }
    $modifiedPokemon[pokemonName].types.modified = pokemonDetails.types;

    if (
      _.isEqual(
        $modifiedPokemon[pokemonName].types.original.sort(),
        $modifiedPokemon[pokemonName].types.modified.sort(),
      )
    ) {
      $modifiedPokemon[pokemonName].types = {
        original: [],
        modified: [],
      };
    }

    await writeTextFile(
      `${$selectedWiki.name}/data/modifications/modified_pokemon.json`,
      JSON.stringify($modifiedPokemon),
      { dir: BaseDirectory.AppData },
    );
  }
}

async function checkAndWriteModifiedEvolutions() {
  if (!_.isEqual(originalPokemonDetails.evolution, pokemonDetails.evolution)) {
    if (!$modifiedPokemon[pokemonName]) {
      $modifiedPokemon[pokemonName] = {
        id: pokemonDetails.id,
        evolution: {
          method: "no_change",
          level: 0,
          item: "",
          other: "",
          evolves_to: { id: 0, pokemon_name: "" },
        },
        types: {
          original: [],
          modified: [],
        },
      };
    }
    if (pokemonDetails.evolution.method !== "no_change") {
      $modifiedPokemon[pokemonName].evolution = pokemonDetails.evolution;
    } else {
      $modifiedPokemon[pokemonName].evolution = {
        method: "no_change",
        level: 0,
        item: "",
        other: "",
        evolves_to: { id: 0, pokemon_name: "" },
      };
    }

    await writeTextFile(
      `${$selectedWiki.name}/data/modifications/modified_pokemon.json`,
      JSON.stringify($modifiedPokemon),
      { dir: BaseDirectory.AppData },
    );
  }
}

async function savePokemonChanges() {
  pokemon.update((p) => {
    p.pokemon[pokemonId] = pokemonDetails;
    return p;
  });
  checkAndWriteModifiedTypes();
  checkAndWriteModifiedEvolutions();

  if (
    $modifiedPokemon[pokemonName].evolution.method === "no_change" &&
    $modifiedPokemon[pokemonName].types.original.length === 0
  ) {
    delete $modifiedPokemon[pokemonName];
  }

  await writeTextFile(
    `${$selectedWiki.name}/data/pokemon.json`,
    JSON.stringify($pokemon),
    { dir: BaseDirectory.AppData },
  ).then(() => {
    originalPokemonDetails = _.cloneDeep(pokemonDetails);
    toastStore.trigger({
      message: "Pokemon Changes Saved",
      timeout: 3000,
      background: "variant-filled-success",
    });
    generatePokemonPage();
  });
}
</script>

<div class="flex flex-row gap-7">
  <div class="mt-2 w-60">
    <input
      id="pokemon-name"
      type="text"
      placeholder="Pokemon Name"
      class="block w-full rounded-md border-0 py-1.5 pl-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 disabled:bg-gray-100 disabled:text-gray-400 sm:text-sm sm:leading-6"
      bind:value={pokemonName}
      use:popup={autoCompletePopup}
    />
    <div
      data-popup="popupAutoComplete"
      class="card mt-2 w-60 overflow-y-auto rounded-sm bg-white"
      tabindex="-1"
    >
      <Autocomplete
        bind:input={pokemonName}
        options={pokemonListOptions}
        limit={5}
        on:selection={onPokemonNameSelected}
        class="w-full rounded-md border bg-white p-2 text-sm"
      />
    </div>
  </div>
  <button
    disabled={pokemonName === ""}
    on:click={getPokemonDetails}
    class="mt-2 w-32 rounded-md bg-indigo-600 text-sm font-semibold text-white
      shadow-sm hover:bg-indigo-500 focus-visible:outline
      focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600
      disabled:bg-indigo-400"
  >
    Search
  </button>
  <button
    disabled={_.isEqual(pokemonDetails, originalPokemonDetails)}
    on:click={savePokemonChanges}
    class="mt-2 w-32 rounded-md bg-indigo-600 text-sm font-semibold text-white
      shadow-sm hover:bg-indigo-500 focus-visible:outline
      focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600
      disabled:bg-indigo-400"
  >
    Save Changes
  </button>
</div>

{#if pokemonDetails.id !== undefined}
  <img src={pokemonDetails.sprite} alt={pokemonDetails.name} width="100" />
  <TabGroup class="px-4">
    <Tab bind:group={tabSet} name="pokemon-details" value={0} class="text-sm"
      >Pokemon Details</Tab
    >
    <Tab bind:group={tabSet} name="pokemon-moves" value={1} class="text-sm"
      >Pokemon Moves</Tab
    >
    <svelte:fragment slot="panel">
      {#if tabSet === 0}
        <PokemonDetailsTab bind:pokemonDetails={pokemonDetails} />
      {/if}
      {#if tabSet === 1}
        <PokemonMovesTab
          bind:pokemonDetails={pokemonDetails}
          savePokemonChanges={savePokemonChanges}
        />
      {/if}
    </svelte:fragment>
  </TabGroup>
{/if}
