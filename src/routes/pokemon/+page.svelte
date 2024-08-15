<script lang="ts">
  import Button from "$lib/components/Button.svelte";
  import NumberInput from "$lib/components/NumberInput.svelte";
  import PokemonPanel from "$lib/components/PokemonPanel.svelte";
  import { Tab, TabGroup, getToastStore } from "@skeletonlabs/skeleton";
  import { invoke } from "@tauri-apps/api/tauri";
  import { selectedWiki } from "../../store";
  import { pokemonList } from "../../store/pokemon";
  import NewPokemonPanel from "$lib/components/NewPokemonPanel.svelte";
  import BaseModal from "$lib/components/BaseModal.svelte";

  const toastStore = getToastStore();

  let tabSet: number = 0;
  let rangeStart: number = 0;
  let rangeEnd: number = 0;
  let loading: boolean = false;

  let pageGenerationWarningModalOpen: boolean = false;

  async function generatePokemonPagesInRange() {
    loading = true;
    let pokemonIds: number[] = [];
    for (let i = rangeStart; i <= rangeEnd; i++) {
      pokemonIds.push(i);
    }
    await invoke("generate_pokemon_pages_from_list", {
      pokemonIds: pokemonIds,
      wikiName: $selectedWiki.name,
    })
      .then(() => {
        loading = false;
        toastStore.trigger({
          message: "Pokemon Pages generated",
          timeout: 5000,
          hoverable: true,
          background: "variant-filled-success",
        });
      })
      .catch((err) => {
        loading = false;
        toastStore.trigger({
          message: "Failed to generate Pokemon Pages: " + err,
          autohide: false,
          hoverable: true,
          background: "variant-filled-error",
        });
      });
  }
</script>

<BaseModal bind:open={pageGenerationWarningModalOpen}>
  <p class="italic">
    NOTE: {Object.keys($pokemonList).length} pokemon pages will be rendered. Do you
    wish to continue?
  </p>
  <div class="flex flex-row gap-2">
    <Button
      title="Cancel"
      onClick={() => (pageGenerationWarningModalOpen = false)}
      class="bg-gray-300"
    />
    <Button
      title={`Generate ALL ${Object.entries($pokemonList).length} Pokemon Pages`}
      onClick={() => {
        rangeStart = 1;
        rangeEnd = Object.entries($pokemonList).length;
        generatePokemonPagesInRange();
        pageGenerationWarningModalOpen = false;
      }}
    />
  </div>
</BaseModal>

<TabGroup>
  <Tab bind:group={tabSet} name="pokemon" value={0} class="text-sm">Pokemon</Tab
  >
  <Tab bind:group={tabSet} name="prepare-pokemon-data" value={1} class="text-sm"
    >Generation</Tab
  >
  <Tab bind:group={tabSet} name="new-pokemon" value={2} class="text-sm"
    >Create New Pokemon</Tab
  >
  <svelte:fragment slot="panel">
    {#if tabSet === 0}
      <PokemonPanel />
    {/if}
    {#if tabSet === 1}
      <Button
        title="Generate All Pokemon Pages"
        onClick={() => (pageGenerationWarningModalOpen = true)}
        disabled={loading === true}
        {loading}
        class="mb-3"
      />
      {#if loading}
        <p class="text-sm italic text-gray-500 mb-3">
          Generating {Object.keys($pokemonList).length} Pokemon Pages...This may
          take a while
        </p>
      {/if}
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
        {loading}
      />
    {/if}
    {#if tabSet === 2}
      <NewPokemonPanel />
    {/if}
  </svelte:fragment>
</TabGroup>
