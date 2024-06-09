<script lang="ts">
import {
  Autocomplete,
  Tab,
  TabGroup,
  getToastStore,
  popup,
  type AutocompleteOption,
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
import {
  convertToTitle,
  extractPokemonRange,
  getShardToWrite,
} from "$lib/utils";
import { shortcut } from "@svelte-put/shortcut";
import Button from "./Button.svelte";
import AutoComplete from "./AutoComplete.svelte";

const toastStore = getToastStore();

let pokemonName: string = "";
let formName: string = "";
let pokemonId: number = 0;
let pokemonDetails: PokemonDetails = {} as PokemonDetails;
let originalPokemonDetails: PokemonDetails = {} as PokemonDetails;
let pokemonNameInput: HTMLInputElement;
let currentPokemonName: string = "";

let tabSet: number = 0;
let formTabSet: number = 0;

let pokemonListOptions: AutocompleteOption<string | number>[] =
  $pokemonList.map(([name, id]) => ({ label: name, value: id }));

function onPokemonNameSelected(
  event: CustomEvent<AutocompleteOption<string | number>>,
): void {
  pokemonName = event.detail.label;
  pokemonId = event.detail.value as number;
}

function setPokemonDetails(pokemonId: number) {
  pokemonName = $pokemon.pokemon[pokemonId].name;
  currentPokemonName = pokemonName;
  pokemonDetails = _.cloneDeep($pokemon.pokemon[pokemonId]);
  originalPokemonDetails = _.cloneDeep(pokemonDetails);
  formTabSet = 0;
}

async function checkAndWriteMods() {
  if (
    _.isEqual(originalPokemonDetails.types, pokemonDetails.types) &&
    _.isEqual(originalPokemonDetails.evolution, pokemonDetails.evolution)
  ) {
    return;
  }

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

  if (!_.isEqual(originalPokemonDetails.types, pokemonDetails.types)) {
    checkModifiedTypes();
  }
  if (!_.isEqual(originalPokemonDetails.evolution, pokemonDetails.evolution)) {
    checkModifiedEvolutions();
  }

  if (
    $modifiedPokemon[pokemonName].evolution.method === "no_change" &&
    $modifiedPokemon[pokemonName].types.original.length === 0
  ) {
    delete $modifiedPokemon[pokemonName];
  }

  await writeTextFile(
    `${$selectedWiki.name}/data/modifications/modified_pokemon.json`,
    JSON.stringify($modifiedPokemon),
    { dir: BaseDirectory.AppData },
  );
}

function checkModifiedTypes() {
  if ($modifiedPokemon[pokemonName].types.original.length === 0) {
    $modifiedPokemon[pokemonName].types.original = originalPokemonDetails.types;
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
}

function checkModifiedEvolutions() {
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
}

async function generatePokemonPage() {
  await invoke("generate_pokemon_pages_from_list", {
    wikiName: $selectedWiki.name,
    dexNumbers: [pokemonId],
  });
}

async function savePokemonChanges() {
  if (formTabSet !== 0) {
    $pokemon.pokemon[pokemonId].forms[formName] = {
      types: pokemonDetails.types,
      abilities: pokemonDetails.abilities,
      stats: pokemonDetails.stats,
      moves: pokemonDetails.moves,
      sprite: pokemonDetails.sprite,
    };
    formName = "";
  } else {
    $pokemon.pokemon[pokemonId] = pokemonDetails;
  }

  checkAndWriteMods();

  let shard_index = getShardToWrite(pokemonId);

  await writeTextFile(
    `${$selectedWiki.name}/data/pokemon_data/shard_${shard_index}.json`,
    JSON.stringify(extractPokemonRange($pokemon, shard_index)),
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

function nextPokemon() {
  if (pokemonId === 1025) {
    toastStore.trigger({
      message: "No more Pokemon",
      timeout: 3000,
      background: "variant-filled-error",
    });
    return;
  }
  setPokemonDetails(pokemonId++);
}

function prevPokemon() {
  if (pokemonId === 1) {
    toastStore.trigger({
      message: "No more Pokemon",
      timeout: 3000,
      background: "variant-filled-error",
    });
    return;
  }
  setPokemonDetails(pokemonId--);
}
</script>

<div class="flex flex-row gap-7">
  <AutoComplete
    bind:value={pokemonName}
    placeholder="Search Pokemon"
    options={pokemonListOptions}
    popupId="pokemon-search"
    onSelection={onPokemonNameSelected}
    bind:inputNode={pokemonNameInput}
    onKeydown={(e) => {
        if (e.key === "Enter") {
          e.preventDefault();
          let id = Object.entries($pokemon.pokemon).find(([_, value]) => value.name === pokemonName)?.[1].id;
          if (id === undefined) {
            toastStore.trigger({
              message: "Pokemon not found",
              timeout: 3000,
              background: "variant-filled-error",
            });
            return;
          }
          setPokemonDetails(id);
        }
      }}
  />
  <Button
    title="Search"
    onClick={() => {
        setPokemonDetails(pokemonId);
      }}
    disabled={pokemonName === ""}
    class="mt-2 w-32"
  />
  <Button
    title="Save Changes"
    onClick={savePokemonChanges}
    disabled={_.isEqual(pokemonDetails, originalPokemonDetails)}
    class="mt-2 w-32"
  />
</div>

{#if pokemonDetails.id !== undefined}
  {#if Object.keys(pokemonDetails.forms).length !== 0}
    <TabGroup class="mb-4 mt-4">
      <Tab
        bind:group={formTabSet}
        name="pokemon-details"
        value={0}
        class="text-sm"
        on:click={() => {
          pokemonDetails = _.cloneDeep($pokemon.pokemon[pokemonId]);
          originalPokemonDetails = _.cloneDeep(pokemonDetails);
          formName = "";
        }}
        >{convertToTitle(currentPokemonName)}</Tab
      >
      {#each Object.keys(pokemonDetails.forms) as form, index}
        <Tab
          bind:group={formTabSet}
          name={form}
          value={index + 1}
          class="text-sm"
          on:click={() => {
            pokemonDetails = {
              ...pokemonDetails,
              ...pokemonDetails.forms[form],
              name: form,
            };
            originalPokemonDetails = _.cloneDeep(pokemonDetails);
            formName = form;
          }}
          >{convertToTitle(form)}</Tab
        >
      {/each}
    </TabGroup>
  {/if}

  <img src={pokemonDetails.sprite} alt={pokemonDetails.name} width="100" />
  <TabGroup>
    <Tab bind:group={tabSet} name="pokemon-details" value={0} class="text-sm"
      >Details</Tab
    >
    <Tab bind:group={tabSet} name="pokemon-moves" value={1} class="text-sm"
      >Moves</Tab
    >
    <svelte:fragment slot="panel">
      {#if tabSet === 0}
        <PokemonDetailsTab
          bind:pokemonDetails={pokemonDetails}
          bind:formTabSet={formTabSet}
        />
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

<svelte:window
  use:shortcut={{
    trigger: {
      key: ']',
      modifier:["ctrl", "meta"],
      callback: () => nextPokemon(),
    },
  }}
  use:shortcut={{
    trigger: {
      key: '[',
      modifier:["ctrl", "meta"],
      callback: () => prevPokemon(),
    },
  }}
  use:shortcut={{
    trigger: {
      key: 'k',
      modifier:["ctrl", "meta"],
      callback: () => pokemonNameInput.focus(),
    },
  }}
  use:shortcut={{
    trigger: {
      key: 'm',
      modifier:"ctrl",
      callback: () => { tabSet = 1},
    },
  }}
  use:shortcut={{
    trigger: {
      key: 'Enter',
      modifier: "meta",
      callback: () => savePokemonChanges(),
    },
  }}
/>
