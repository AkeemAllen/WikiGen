<script lang="ts">
  import NumberInput from "$lib/NumberInput.svelte";
  import PokemonPanel from "$lib/PokemonPanel.svelte";
  import {
    Tab,
    TabGroup,
    getToastStore,
    type ToastSettings,
  } from "@skeletonlabs/skeleton";
  import { readTextFile } from "@tauri-apps/api/fs";
  import { appDataDir } from "@tauri-apps/api/path";
  import { invoke } from "@tauri-apps/api/tauri";
  import { selectedWiki } from "../../store";
  import { pokemon, pokemonList } from "../../store/pokemon";

  const toastStore = getToastStore();

  let tabSet: number = 0;

  let rangeStart: number = 0;
  let rangeEnd: number = 0;

  const dataPreparedToast: ToastSettings = {
    message: "Data Prepared",
    timeout: 5000,
    hoverable: true,
    background: "variant-filled-success",
  };

  async function downloadAndPrepPokemonData() {
    const directory = await appDataDir();
    await invoke("download_and_prep_pokemon_data", {
      wikiName: $selectedWiki.name,
      rangeStart,
      rangeEnd,
      dir: directory,
    }).then(async () => {
      const pokemonFromFile = await readTextFile(
        `${directory}${$selectedWiki.name}/data/pokemon.json`,
      );
      pokemon.set(JSON.parse(pokemonFromFile));
      pokemonList.set(
        Object.entries($pokemon.pokemon).map(([key, value]) => [
          value.name,
          parseInt(key),
        ]),
      );
      toastStore.trigger(dataPreparedToast);
    });
  }
</script>

<TabGroup class="mt-4">
  <Tab bind:group={tabSet} name="pokemon" value={0}>Pokemon</Tab>
  <Tab bind:group={tabSet} name="prepare-pokemon-data" value={1}
    >Prepare Data</Tab
  >
  <svelte:fragment slot="panel">
    {#if tabSet === 0}
      <PokemonPanel />
    {/if}
    {#if tabSet === 1}
      <div class="flex gap-16">
        <NumberInput
          id="range-start"
          label="Range Start"
          bind:value={rangeStart}
        />
        <NumberInput id="range-end" label="Range End" bind:value={rangeEnd} />
      </div>
      <button
        disabled={rangeStart === 0 ||
          rangeEnd === 0 ||
          rangeStart > rangeEnd ||
          rangeStart === rangeEnd}
        class=" rounded-md bg-indigo-600 w-32 px-3 py-2 mt-5 text-sm font-semibold text-white
      shadow-sm hover:bg-indigo-500 focus-visible:outline
      focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600
      disabled:bg-indigo-400"
        on:click|preventDefault={downloadAndPrepPokemonData}
      >
        Prepare Data</button
      >
    {/if}
  </svelte:fragment>
</TabGroup>
