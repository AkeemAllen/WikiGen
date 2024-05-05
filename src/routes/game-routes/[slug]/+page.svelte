<script lang="ts">
import WildEncounters from "$lib/components/game-route-components/WildEncounters.svelte";
import { Tab, TabGroup } from "@skeletonlabs/skeleton";

export let data;
let tabSet: number = 0;
</script>

<strong class="text-lg">{data.title}</strong>
<TabGroup class="mt-4">
  <Tab bind:group={tabSet} name="wild-encounters" value={0} class="text-sm"
    >Wild Encounters</Tab
  >
  <Tab bind:group={tabSet} name="trainer-encounters" value={1} class="text-sm"
    >Trainer Encounters</Tab
  >
  <svelte:fragment slot="panel">
    {#if tabSet === 0}
      <WildEncounters routeName={data.title} />
    {/if}
  </svelte:fragment>
</TabGroup>
