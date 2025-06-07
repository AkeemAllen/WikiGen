<script lang="ts">
  import capitalizeWords from "$lib/utils/capitalizeWords";
  import * as Tabs from "$lib/components/ui/tabs/index.js";
  import AttributeItems from "$lib/components/AttributeItems.svelte";
</script>

<Tabs.Root value="items" class="w-full">
  <div class="w-full bg-white border-b">
    <Tabs.List class="w-[30rem] rounded-sm ml-5 mt-3 mb-7">
      {#each ["items", "abilities", "natures"] as tab}
        <Tabs.Trigger value={tab} class="rounded-sm  cursor-pointer"
          >{capitalizeWords(tab)}</Tabs.Trigger
        >
      {/each}
    </Tabs.List>
  </div>
  <Tabs.Content value="items" class="mx-5">
    <AttributeItems />
  </Tabs.Content>
  <Tabs.Content value="abilities" class="mx-5"></Tabs.Content>
  <Tabs.Content value="natures" class="mx-5"></Tabs.Content>
</Tabs.Root>
