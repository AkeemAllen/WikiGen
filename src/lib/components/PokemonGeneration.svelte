<script lang="ts">
  import Autocomplete from "$lib/components/ui/Autocomplete.svelte";
  import LoaderCircleIcon from "@lucide/svelte/icons/loader-circle";
  import { toast } from "svelte-sonner";
  import { pokemonList } from "../../store/pokemon";
  import capitalizeWords from "$lib/utils/capitalizeWords";
  import { generatePokemonPages } from "$lib/utils/generators";
  import { selectedWiki } from "../../store";
  import { Button } from "$lib/components/ui/button/index.js";
  import * as Dialog from "$lib/components/ui/dialog/index.js";

  let startingPokemon: [number, number, string] = $state([0, 0, ""]);
  let endingPokemon: [number, number, string] = $state([0, 0, ""]);
  let rangeTotal = $derived(endingPokemon[1] - startingPokemon[1]);

  let startingPokemonSearchOpen = $state(false);
  let startingTriggerRef = $state<HTMLButtonElement>(null!);
  let searchingStartingPokemon = $state("");
  let endingPokemonSearchOpen = $state(false);
  let endingTriggerRef = $state<HTMLButtonElement>(null!);
  let searchingEndingPokemon = $state("");
  let loading: boolean = $state(false);
  let pageGenerationWarningModalOpen: boolean = $state(false);

  let pokemonListOptions = $pokemonList.map(([id, dex_number, name]) => ({
    label: `${dex_number} - ${capitalizeWords(name)}`,
    dex_number: dex_number,
    value: id,
  }));

  let startingOptions = $derived(
    pokemonListOptions
      .filter((pokemon) =>
        pokemon.label
          .toLowerCase()
          .includes(searchingStartingPokemon.toLowerCase()),
      )
      .slice(0, 8),
  );

  let endingOptions = $derived(
    pokemonListOptions
      .filter((pokemon) =>
        pokemon.label
          .toLowerCase()
          .includes(searchingEndingPokemon.toLowerCase()),
      )
      .slice(0, 8),
  );

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
        toast.success(res as string);
      })
      .catch((err) => {
        loading = false;
        toast.error(err as string);
      });
  }
</script>

<Dialog.Root bind:open={pageGenerationWarningModalOpen}>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>Pokemon Generation Warning</Dialog.Title>
    </Dialog.Header>
    This action will generate pages for all {Object.keys($pokemonList).length}
    pokemon. Do you wish to continue?
    <Dialog.Footer>
      <Button
        onclick={() => (pageGenerationWarningModalOpen = false)}
        class="cursor-pointer">Cancel</Button
      >
      <Button
        onclick={() => {
          generatePokemonPagesInRange(1, 1025);
          pageGenerationWarningModalOpen = false;
        }}
        class="cursor-pointer"
        >{`Generate ALL ${Object.entries($pokemonList).length} Pokemon Pages`}</Button
      >
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>

<Button
  onclick={() => (pageGenerationWarningModalOpen = true)}
  disabled={loading === true}
  class="mb-3"
>
  Generate All Pokemon Pages
</Button>
{#if loading}
  <p class="text-sm italic text-gray-500 mb-3 align-middle">
    Generating {Object.keys($pokemonList).length} Pokemon Pages...This may take a
    while
  </p>
{/if}
<p class="text-sm text-gray-500 italic">
  Pages will be generated for all pokemon with pokedex number in this range: {startingPokemon[1]}
  - {endingPokemon[1]}
</p>
<p class="text-sm text-gray-500 italic mb-4">
  Total Pages: {rangeTotal <= 0 ? 0 : `${rangeTotal + 1}`}
</p>
<div class="flex gap-16">
  <Autocomplete
    open={startingPokemonSearchOpen}
    triggerRef={startingTriggerRef}
    value={startingPokemon[2]}
    bind:searcher={searchingStartingPokemon}
    options={startingOptions}
    placeholder="Search Starting Pokemon"
    onSelect={(option) => {
      let dex_number = pokemonListOptions.find(
        (pokemon) =>
          pokemon.label === option.label && pokemon.value === option.value,
      )?.dex_number as number;
      startingPokemon = [option.value, dex_number, option.label];
    }}
    label="Search Starting Pokemon"
  />
  <Autocomplete
    open={endingPokemonSearchOpen}
    triggerRef={endingTriggerRef}
    value={endingPokemon[2]}
    bind:searcher={searchingEndingPokemon}
    options={endingOptions}
    placeholder="Search Ending Pokemon"
    onSelect={(option) => {
      let dex_number = pokemonListOptions.find(
        (pokemon) =>
          pokemon.label === option.label && pokemon.value === option.value,
      )?.dex_number as number;
      endingPokemon = [option.value, dex_number, option.label];
    }}
    label="Search Ending Pokemon"
  />
</div>
<Button
  onclick={() =>
    generatePokemonPagesInRange(startingPokemon[1], endingPokemon[1])}
  disabled={loading || startingPokemon[1] === 0 || endingPokemon[1] === 0}
  class="mt-5"
  >{#if loading}
    <LoaderCircleIcon class="animate-spin" />
  {/if}
  Generate Pages</Button
>
