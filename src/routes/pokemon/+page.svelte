<script lang="ts">
  import Button from "$lib/components/Button.svelte";
  import PokemonPanel from "$lib/components/PokemonPanel.svelte";
  import { getToastStore, Tab, TabGroup } from "@skeletonlabs/skeleton";
  import { selectedWiki } from "../../store";
  import { pokemonList } from "../../store/pokemon";
  import NewPokemonPanel from "$lib/components/NewPokemonPanel.svelte";
  import ModifyMovesetsPanel from "$lib/components/ModifyMovesets.svelte";
  import BaseModal from "$lib/components/BaseModal.svelte";
  import AutoComplete from "$lib/components/AutoComplete.svelte";
  import capitalizeWords from "$lib/utils/capitalizeWords";
  import { ToastType, getToastSettings } from "$lib/utils/toasts";
  import { generatePokemonPages } from "$lib/utils/generators";

  let startingPokemon: [number, number, string] = $state([0, 0, ""]);
  let endingPokemon: [number, number, string] = $state([0, 0, ""]);
  let rangeTotal = $derived(endingPokemon[1] - startingPokemon[1]);

  let tabSet: number = $state(0);
  let loading: boolean = $state(false);
  let pageGenerationWarningModalOpen: boolean = $state(false);

  let pokemonListOptions = $pokemonList.map(([id, dex_number, name]) => ({
    label: `${dex_number} - ${capitalizeWords(name)}`,
    dex_number: dex_number,
    value: id,
  }));

  const toastStore = getToastStore();

  async function generatePokemonPagesInRange(
    startingDexNumber: number,
    endingDexNumber: number,
  ) {
    loading = true;
    let pokemonIds: number[] = [];
    pokemonIds = $pokemonList
      .filter(
        ([_, dex_number, __]) =>
          dex_number >= startingDexNumber && dex_number <= endingDexNumber,
      )
      .map(([id, _, __]) => id);

    generatePokemonPages(pokemonIds, $selectedWiki.name)
      .then((res) => {
        loading = false;
        toastStore.trigger(getToastSettings(ToastType.SUCCESS, res as string));
      })
      .catch((err) => {
        loading = false;
        toastStore.trigger(getToastSettings(ToastType.ERROR, err as string));
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
        generatePokemonPagesInRange(1, 1025);
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
  <Tab bind:group={tabSet} name="modify-movesets" value={3} class="text-sm"
    >Modify Movesets</Tab
  >
  <div slot="panel">
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
        <p class="text-sm italic text-gray-500 mb-3 align-middle">
          Generating {Object.keys($pokemonList).length} Pokemon Pages...This may
          take a while
        </p>
      {/if}
      <p class="text-sm text-gray-500 italic">
        Pages will be generated for all pokemon with pokedex number in this
        range: {startingPokemon[1]}
        - {endingPokemon[1]}
      </p>
      <p class="text-sm text-gray-500 italic mb-4">
        Total Pages: {rangeTotal <= 0 ? 0 : `>=${rangeTotal + 1}`}
      </p>
      <div class="flex gap-16">
        <!-- Only 1025 pokemon exist in the game right now. But setting ranges to 2000 for future proofing -->
        <AutoComplete
          label="Starting Pokemon"
          bind:value={startingPokemon[2]}
          placeholder="Search Pokemon"
          options={pokemonListOptions}
          popupId="starting-pokemon-search"
          onSelection={(e) => {
            startingPokemon = [
              e.detail.value,
              e.detail.dex_number,
              e.detail.label,
            ];
          }}
        />
        <AutoComplete
          label="Ending Pokemon"
          bind:value={endingPokemon[2]}
          placeholder="Search Pokemon"
          options={pokemonListOptions}
          popupId="ending-pokemon-search"
          onSelection={(e) => {
            endingPokemon = [
              e.detail.value,
              e.detail.dex_number,
              e.detail.label,
            ];
          }}
        />
      </div>
      <Button
        class=" mt-4 w-40"
        title="Generate Pages"
        onClick={() =>
          generatePokemonPagesInRange(startingPokemon[1], endingPokemon[1])}
        disabled={loading || startingPokemon[1] === 0 || endingPokemon[1] === 0}
        {loading}
      />
    {/if}
    {#if tabSet === 2}
      <NewPokemonPanel />
    {/if}
    {#if tabSet === 3}
      <ModifyMovesetsPanel />
    {/if}
  </div>
  <!-- {#snippet panel()}
  {/snippet} -->
</TabGroup>
