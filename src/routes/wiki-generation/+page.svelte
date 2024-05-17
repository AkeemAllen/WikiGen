<script lang="ts">
import Button from "$lib/components/Button.svelte";

import NumberInput from "$lib/components/NumberInput.svelte";
import { Tab, TabGroup, getToastStore } from "@skeletonlabs/skeleton";
import { invoke } from "@tauri-apps/api/tauri";
import { selectedWiki } from "../../store";

const toastStore = getToastStore();

let rangeStart: number = 1;
let rangeEnd: number = 1;
let loading: boolean = false;
let tabSet: number = 0;

async function generatePokemonPagesInRange() {
  loading = true;
  await invoke("generate_pokemon_page_in_range_with_handle", {
    rangeStart,
    rangeEnd,
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

async function generateRoutePages() {
  loading = true;
  await invoke("generate_route_page_with_handle", {
    wikiName: $selectedWiki.name,
  }).then((response: any) => {
    loading = false;
    toastStore.trigger({
      message: response || "Route Pages generated",
      timeout: 5000,
      hoverable: true,
      background: "variant-filled-success",
    });
  });
}
</script>

<TabGroup>
  <Tab bind:group={tabSet} name="pokemon" value={0}>Pokemon</Tab>
  <Tab bind:group={tabSet} name="routes" value={1}>Routes</Tab>
  <svelte:fragment slot="panel">
    {#if tabSet === 0}
      <div class="flex gap-16">
        <NumberInput
          id="range-start"
          label="Range Start"
          bind:value={rangeStart}
        />
        <NumberInput id="range-end" label="Range End" bind:value={rangeEnd} />
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
    {#if tabSet === 1}
      <Button
        class=" mt-4 w-40"
        title="Generate Pages"
        onClick={generateRoutePages}
        loading={loading}
      />
    {/if}
  </svelte:fragment>
</TabGroup>
