<script lang="ts">
  import NumberInput from "$lib/NumberInput.svelte";
  import { Tab, TabGroup } from "@skeletonlabs/skeleton";
  import { appDataDir } from "@tauri-apps/api/path";
  import { invoke } from "@tauri-apps/api/tauri";
  import { selectedWiki } from "../../store";

  let tabSet: number = 1;

  let pokemonName: string = "";
  let message: string = "";
  let rangeStart: number = 0;
  let rangeEnd: number = 0;

  async function downloadPokemonData() {
    const directory = await appDataDir();
    await invoke("download_pokemon_data", {
      wikiName: $selectedWiki.name,
      rangeStart,
      rangeEnd,
      dir: directory,
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
      <div>
        <input
          id="greet-input"
          type="text"
          placeholder="Pokemon Name"
          bind:value={pokemonName}
          class="ml-2"
        />
        <p>{message}</p>
      </div>
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
        class="rounded-md bg-indigo-600 w-32 px-3 py-2 mt-5 text-sm font-semibold text-white
      shadow-sm hover:bg-indigo-500 focus-visible:outline
      focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600
      disabled:bg-indigo-400"
        on:click={downloadPokemonData}>Prepare Data</button
      >
    {/if}
  </svelte:fragment>
</TabGroup>
