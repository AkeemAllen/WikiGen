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

const toastStore = getToastStore();

let tabSet: number = 0;
let rangeStart: number = 0;
let rangeEnd: number = 0;
let loading: boolean = false;

async function generatePokemonPagesInRange() {
  loading = true;
  let dexNumbers: number[] = [];
  for (let i = rangeStart; i <= rangeEnd; i++) {
    dexNumbers.push(i);
  }
  await invoke("generate_pokemon_pages_from_list", {
    dexNumbers: dexNumbers,
    wikiName: $selectedWiki.name,
  }).then(() => {
    loading = false;
    toastStore.trigger({
      message: "Pokemon Pages generated",
      timeout: 5000,
      hoverable: true,
      background: "variant-filled-success",
    });
  });
}
</script>

<TabGroup>
  <Tab bind:group={tabSet} name="pokemon" value={0}>Pokemon</Tab>
  <Tab bind:group={tabSet} name="prepare-pokemon-data" value={1}>Generation</Tab
  >
  <svelte:fragment slot="panel">
    {#if tabSet === 0}
      <PokemonPanel />
    {/if}
    {#if tabSet === 1}
      <div class="flex gap-16">
        <!-- Only 1025 pokemon exist in the game right now. But setting ranges to 2000 for future proofing -->
        <NumberInput
          id="range-start"
          label="Range Start"
          bind:value={rangeStart}
          max={2000}
        />
        <NumberInput
          id="range-end"
          label="Range End"
          bind:value={rangeEnd}
          max={2000}
        />
      </div>
      <Button
        class=" mt-4 w-40"
        disabled={rangeStart === 0 ||
                    rangeEnd === 0 ||
                    rangeStart > rangeEnd ||
                    loading === true}
        title="Generate Pages"
        onClick={generatePokemonPagesInRange}
        loading={loading}
      />
    {/if}
  </svelte:fragment>
</TabGroup>
