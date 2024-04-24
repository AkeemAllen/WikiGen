<script lang="ts">
  import Button from "$lib/components/Button.svelte";

  import NumberInput from "$lib/components/NumberInput.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { selectedWiki } from "../../store";

  let rangeStart: number = 1;
  let rangeEnd: number = 3;
  let loading: boolean = false;

  async function generatePokemonPagesInRange() {
    console.log("Generating Pokemon from range");
    await invoke("generate_pokemon_pages_in_range", {
      rangeStart,
      rangeEnd,
      wikiName: $selectedWiki.name,
    }).then(() => {
      console.log("Pokemon generated");
    });
  }
</script>

<div class="mt-6 ml-2">
  <p>Wiki Generation</p>
  <div class="flex gap-16">
    <NumberInput id="range-start" label="Range Start" bind:value={rangeStart} />
    <NumberInput id="range-end" label="Range End" bind:value={rangeEnd} />
  </div>
  <div class=" w-52 mt-4">
    <Button
      disabled={rangeStart === 0 ||
        rangeEnd === 0 ||
        rangeStart > rangeEnd ||
        rangeStart === rangeEnd ||
        loading === true}
      title="Generate Pokemon between range"
      onClick={generatePokemonPagesInRange}
      {loading}
    />
  </div>
</div>
