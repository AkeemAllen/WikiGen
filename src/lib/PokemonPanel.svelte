<script lang="ts">
  import {
    Autocomplete,
    popup,
    type AutocompleteOption,
    type PopupSettings,
  } from "@skeletonlabs/skeleton";
  import { pokemon, pokemonList, type PokemonData } from "../store/pokemon";

  let pokemonName: string = "";
  let pokemonId: number = 0;
  let pokemonData: PokemonData = {} as PokemonData;

  const pokemonListOptions: AutocompleteOption<string | number>[] =
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

  function getPokemonData(): void {
    pokemonData = $pokemon.pokemon[pokemonId];
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
    on:click={getPokemonData}
    class="mt-2 rounded-md bg-indigo-600 w-32 text-sm font-semibold text-white
      shadow-sm hover:bg-indigo-500 focus-visible:outline
      focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600
      disabled:bg-indigo-400"
  >
    Search
  </button>
  <button
    disabled={true}
    class="mt-2 rounded-md bg-indigo-600 w-32 text-sm font-semibold text-white
      shadow-sm hover:bg-indigo-500 focus-visible:outline
      focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600
      disabled:bg-indigo-400"
  >
    Save Changes
  </button>
</div>

{#if pokemonData.id !== undefined}
  <img src={pokemonData.sprite} alt={pokemonData.name} />
{/if}
