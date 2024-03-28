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
  import { BaseDirectory, writeTextFile } from "@tauri-apps/api/fs";
  import _ from "lodash";
  import { selectedWiki } from "../store";
  import {
    PokemonTypes,
    pokemon,
    pokemonList,
    type PokemonDetails,
  } from "../store/pokemon";
  import NumberInput from "./NumberInput.svelte";
  import SelectInput from "./SelectInput.svelte";
  import TextInput from "./TextInput.svelte";
  import { capitalize } from "./utils";

  const toastStore = getToastStore();

  let pokemonName: string = "";
  let pokemonId: number = 0;
  let pokemonDetails: PokemonDetails = {} as PokemonDetails;
  let originalPokemonDetails: PokemonDetails = {} as PokemonDetails;

  let tabSet: number = 0;

  const pokemonListOptions: AutocompleteOption<string | number>[] =
    $pokemonList.map(([name, id]) => ({ label: name, value: id }));

  const autoCompletePopup: PopupSettings = {
    event: "focus-click",
    target: "popupAutoComplete",
    placement: "bottom",
  };

  const pokemonDetailsSavedToast: ToastSettings = {
    message: "Data saved",
    timeout: 3000,
    background: "variant-filled-success",
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

  async function savePokemonChanges() {
    pokemon.update((p) => {
      p.pokemon[pokemonId] = pokemonDetails;
      return p;
    });
    console.log($pokemon);
    await writeTextFile(
      `${$selectedWiki.name}/data/pokemon.json`,
      JSON.stringify($pokemon),
      { dir: BaseDirectory.AppData },
    ).then(() => {
      originalPokemonDetails = _.cloneDeep(pokemonDetails);
      toastStore.trigger(pokemonDetailsSavedToast);
    });
  }
</script>

<div class="flex flex-row gap-7">
  <div class="w-60 mt-2">
    <input
      id="pokemon-name"
      type="text"
      placeholder="Pokemon Name"
      class="block w-full pl-2 rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6 disabled:bg-gray-100 disabled:text-gray-400"
      bind:value={pokemonName}
      use:popup={autoCompletePopup}
    />
    <div
      data-popup="popupAutoComplete"
      class="card w-60 mt-2 overflow-y-auto bg-white rounded-sm"
      tabindex="-1"
    >
      <Autocomplete
        bind:input={pokemonName}
        options={pokemonListOptions}
        limit={5}
        on:selection={onPokemonNameSelected}
        class="bg-white w-full text-sm border rounded-md p-2"
      />
    </div>
  </div>
  <button
    disabled={pokemonName === ""}
    on:click={getPokemonDetails}
    class="mt-2 rounded-md bg-indigo-600 w-32 text-sm font-semibold text-white
      shadow-sm hover:bg-indigo-500 focus-visible:outline
      focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600
      disabled:bg-indigo-400"
  >
    Search
  </button>
  <button
    disabled={_.isEqual(pokemonDetails, originalPokemonDetails)}
    on:click={savePokemonChanges}
    class="mt-2 rounded-md bg-indigo-600 w-32 text-sm font-semibold text-white
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
      <div class="scroll-smooth">
        {#if tabSet === 0}
          <div class="grid grid-cols-2 gap-x-10 gap-y-5 mt-10">
            <SelectInput
              id="pokemon-type-1"
              bind:value={pokemonDetails.types[0]}
              label="Type 1"
              options={PokemonTypes.map((type) => ({
                label: capitalize(type),
                value: type,
              }))}
            />
            <SelectInput
              id="pokemon-type-2"
              bind:value={pokemonDetails.types[1]}
              label="Type 2"
              options={PokemonTypes.map((type) => ({
                label: capitalize(type),
                value: type,
              }))}
            />
            <SelectInput
              id="pokemon-ability-1"
              bind:value={pokemonDetails.abilities[0]}
              label="Ability 1"
              options={pokemonDetails.abilities.map((ability) => ({
                label: capitalize(ability),
                value: ability,
              }))}
            />
            <SelectInput
              id="pokemon-ability-2"
              bind:value={pokemonDetails.abilities[1]}
              label="Ability 2"
              options={pokemonDetails.abilities.map((ability) => ({
                label: capitalize(ability),
                value: ability,
              }))}
            />
          </div>
          <div class="flex flex-row gap-x-10 mt-5">
            <div class="w-44">
              <SelectInput
                id="evolution-method"
                label="Evolution Method"
                bind:value={pokemonDetails.evolution.method}
                options={[
                  { label: "No Change", value: "no_change" },
                  { label: "Level Up", value: "level_up" },
                  { label: "Other", value: "other" },
                  { label: "Use Item", value: "item" },
                ]}
              />
            </div>
            {#if pokemonDetails.evolution.method === "item"}
              <TextInput
                id="evolution-item"
                value={pokemonDetails.evolution?.item ?? ""}
                label="Evolution Item"
              />
            {/if}
            {#if pokemonDetails.evolution.method === "level_up"}
              <NumberInput
                id="evolution-level"
                bind:value={pokemonDetails.evolution.level}
                label="Evolution Level"
              />
            {/if}
            {#if pokemonDetails.evolution.method === "other"}
              <TextInput
                id="evolution-other"
                bind:value={pokemonDetails.evolution.other}
                label="Evolution Other"
              />
            {/if}
            {#if pokemonDetails.evolution.method !== "no_change"}
              <div class="w-44">
                <SelectInput
                  id="evolution-to"
                  label="Evoles To"
                  bind:value={pokemonDetails.evolution.evolves_to}
                  options={pokemonListOptions}
                />
              </div>
            {/if}
          </div>
          <p class="text-lg mt-10">Stats</p>
          <div class="grid grid-cols-2 gap-x-10 gap-y-5 mt-2 mb-2">
            <NumberInput
              id="pokemon-hp"
              bind:value={pokemonDetails.stats.hp}
              label="HP"
            />
            <NumberInput
              id="pokemon-attack"
              bind:value={pokemonDetails.stats.attack}
              label="Attack"
            />
            <NumberInput
              id="pokemon-defense"
              bind:value={pokemonDetails.stats.defense}
              label="Defense"
            />
            <NumberInput
              id="pokemon-special-attack"
              bind:value={pokemonDetails.stats.sp_attack}
              label="Special Attack"
            />
            <NumberInput
              id="pokemon-special-defense"
              bind:value={pokemonDetails.stats.sp_defense}
              label="Special Defense"
            />
            <NumberInput
              id="pokemon-speed"
              bind:value={pokemonDetails.stats.speed}
              label="Speed"
            />
          </div>
        {/if}
      </div>
      {#if tabSet === 1}
        <p>Moves</p>
      {/if}
    </svelte:fragment>
  </TabGroup>
{/if}
