<script lang="ts">
  import PokemonPanel from "$lib/components/PokemonPanel.svelte";
  import PokemonGeneration from "$lib/components/PokemonGeneration.svelte";
  import capitalizeWords from "$lib/utils/capitalizeWords";
  import * as Tabs from "$lib/components/ui/tabs/index.js";
  import NewPokemonPanel from "$lib/components/NewPokemonPanel.svelte";
  // import ModifyMovesets from "$lib/components/ModifyMovesets.svelte";
</script>

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
  <Tabs.Content value="generation" class="mx-5">
    <PokemonGeneration />
  </Tabs.Content>
  <Tabs.Content value="new-pokemon" class="mx-5">
    <NewPokemonPanel />
  </Tabs.Content>
  <Tabs.Content value="movesets" class="mx-5">
    <!-- <ModifyMovesets /> -->
  </Tabs.Content>
</Tabs.Root>
