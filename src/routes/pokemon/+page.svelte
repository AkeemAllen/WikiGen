<script lang="ts">
  import Button from "$lib/components/Button.svelte";
  import NumberInput from "$lib/components/NumberInput.svelte";
  import PokemonPanel from "$lib/components/PokemonPanel.svelte";
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
  let loading: boolean = false;

  const dataPreparedToast: ToastSettings = {
    message: "Data Prepared",
    timeout: 5000,
    hoverable: true,
    background: "variant-filled-success",
  };

  async function downloadAndPrepPokemonData() {
    const directory = await appDataDir();
    loading = true;
    await invoke("download_and_prep_pokemon_data", {
      wikiName: $selectedWiki.name,
      rangeStart,
      rangeEnd,
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
      loading = false;
      toastStore.trigger(dataPreparedToast);
    });

    await invoke("download_pokemon_sprites", {
      wikiName: $selectedWiki.name,
      rangeStart,
      rangeEnd,
    }).then((response) => {
      console.log(response);
    });
  }
</script>

<TabGroup class="mt-4 ml-2">
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
      <div class="w-40 mt-4">
        <Button
          disabled={rangeStart === 0 ||
            rangeEnd === 0 ||
            rangeStart > rangeEnd ||
            rangeStart === rangeEnd ||
            loading === true}
          title="Prepare Data"
          onClick={downloadAndPrepPokemonData}
          {loading}
        />
      </div>
    {/if}
  </svelte:fragment>
</TabGroup>
