<script lang="ts">
import {
  Tab,
  TabGroup,
  getToastStore,
  type AutocompleteOption,
} from "@skeletonlabs/skeleton";
import { BaseDirectory, writeTextFile } from "@tauri-apps/api/fs";
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
import { updatePokemonModifications } from "$lib/utils/modificationHelpers";

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

function setPokemonDetails(id: number) {
  pokemonId = id;
  pokemonName = $pokemon.pokemon[pokemonId].name;
  currentPokemonName = pokemonName;
  pokemonDetails = _.cloneDeep($pokemon.pokemon[pokemonId]);
  originalPokemonDetails = _.cloneDeep(pokemonDetails);
  formTabSet = 0;
}

async function savePokemonChanges() {
  if (_.isEqual(originalPokemonDetails, pokemonDetails)) {
    return;
  }
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

  updatePokemonModifications(
    $modifiedPokemon,
    originalPokemonDetails,
    pokemonDetails,
  );

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
    invoke("generate_pokemon_pages_from_list", {
      wikiName: $selectedWiki.name,
      dexNumbers: [pokemonId],
    });
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
  setPokemonDetails(pokemonId + 1);
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
  setPokemonDetails(pokemonId - 1);
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
    showChevron={false}
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
      callback: nextPokemon,
    },
  }}
  use:shortcut={{
    trigger: {
      key: '[',
      modifier:["ctrl", "meta"],
      callback: prevPokemon,
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
      modifier: ["ctrl","meta"],
      callback: () => savePokemonChanges(),
    },
  }}
/>
