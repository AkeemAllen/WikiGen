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

async function generateRoutePages() {
  loading = true;
  await invoke("generate_route_pages_with_handle", {
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
  <Tab bind:group={tabSet} name="routes" value={0}>Routes</Tab>
  <svelte:fragment slot="panel">
    {#if tabSet === 0}
      <Button
        class=" mt-4 w-40"
        title="Generate Pages"
        onClick={generateRoutePages}
        loading={loading}
      />
    {/if}
  </svelte:fragment>
</TabGroup>
