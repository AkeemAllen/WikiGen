<script lang="ts">
  import { Tab, TabGroup } from "@skeletonlabs/skeleton";

  let tabSet: number = 0;

  let pokemonName = "";
  let message = "";
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
  </svelte:fragment>
</TabGroup>
