<script lang="ts">
  import OldButton from "$lib/components/Button.svelte";
  import PokemonPanel from "$lib/components/PokemonPanel.svelte";
  import { getToastStore, Tab, TabGroup } from "@skeletonlabs/skeleton";
  import { selectedWiki } from "../../store";
  import { pokemonList } from "../../store/pokemon";
  import NewPokemonPanel from "$lib/components/NewPokemonPanel.svelte";
  import ModifyMovesetsPanel from "$lib/components/ModifyMovesets.svelte";
  import AutoComplete from "$lib/components/AutoComplete.svelte";
  import capitalizeWords from "$lib/utils/capitalizeWords";
  import { ToastType, getToastSettings } from "$lib/utils/toasts";
  import { generatePokemonPages } from "$lib/utils/generators";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import * as Tabs from "$lib/components/ui/tabs/index.js";
  import { Separator } from "$lib/components/ui/separator";

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

<Tabs.Root value="pokemon" class="w-full">
  <div class="w-full bg-white border-b">
    <Tabs.List class="w-[30rem] rounded-sm ml-5 mt-3 mb-7">
      {#each ["pokemon", "generation", "new-pokemon", "movesets"] as tab}
        <Tabs.Trigger value={tab} class="rounded-sm  cursor-pointer"
          >{capitalizeWords(tab)}</Tabs.Trigger
        >
      {/each}
    </Tabs.List>
  </div>
  <Tabs.Content value="pokemon" class="mx-5">
    <PokemonPanel />
  </Tabs.Content>
  <Tabs.Content value="generation">Change your password here.</Tabs.Content>
  <Tabs.Content value="new-pokemon">Change your password here.</Tabs.Content>
  <Tabs.Content value="movesets">Change your password here.</Tabs.Content>
</Tabs.Root>

<!-- <TabGroup>
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
      <OldButton
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
      <OldButton
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
</TabGroup> -->
