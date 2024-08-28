<script lang="ts">
  import TrainerEncounters from "$lib/components/game-route-components/TrainerEncounters.svelte";
  import WildEncounters from "$lib/components/game-route-components/WildEncounters.svelte";
  import SelectInput from "$lib/components/SelectInput.svelte";
  import { getToastStore, Tab, TabGroup } from "@skeletonlabs/skeleton";
  import { IconArrowLeft } from "@tabler/icons-svelte";
  import { routes } from "../../../store/gameRoutes";
  import { selectedWiki } from "../../../store";
  import { BaseDirectory, writeTextFile } from "@tauri-apps/api/fs";
  import { invoke } from "@tauri-apps/api";

  const toastStore = getToastStore();
  export let data;
  let tabSet: number = 0;

  async function saveRenderChange() {
    await writeTextFile(
      `${$selectedWiki.name}/data/routes.json`,
      JSON.stringify($routes),
      { dir: BaseDirectory.AppData },
    ).then(() => {
      invoke("generate_route_pages_with_handle", {
        wikiName: $selectedWiki.name,
        routeNames: [data.title],
      })
        .then(() => {
          toastStore.trigger({
            message: "Render Value Updated",
            timeout: 3000,
            background: "variant-filled-success",
          });
        })
        .catch((e) => {
          console.error(e);
        });
    });
  }
</script>

<strong class="text-l flex flex-row items-center gap-5">
  <a href="/game-routes" class="hover:cursor-pointer">
    <IconArrowLeft size={20} />
  </a>
  {data.title}</strong
>
<TabGroup class="mt-4">
  <Tab bind:group={tabSet} name="wild-encounters" value={0} class="text-sm"
    >Wild Encounters</Tab
  >
  <Tab bind:group={tabSet} name="trainer-encounters" value={1} class="text-sm"
    >Trainer Encounters</Tab
  >
  <Tab bind:group={tabSet} name="properties" value={2} class="text-sm"
    >Properties</Tab
  >
  <svelte:fragment slot="panel">
    {#if tabSet === 0}
      <WildEncounters routeName={data.title} />
    {/if}
    {#if tabSet === 1}
      <TrainerEncounters routeName={data.title} />
    {/if}
    {#if tabSet === 2}
      <div class="w-36">
        <SelectInput
          label="Render Route Page"
          bind:value={$routes.routes[data.title].render}
          options={[
            {
              value: true,
              label: "True",
            },
            {
              value: false,
              label: "False",
            },
          ]}
          onChange={saveRenderChange}
        />
      </div>
    {/if}
  </svelte:fragment>
</TabGroup>
