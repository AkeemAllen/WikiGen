<script lang="ts">
  import { Button } from "$lib/components/ui/button/index";
  import {
    routes,
    type Routes,
    type WildEncounter,
  } from "../../../store/gameRoutes";
  import { selectedWiki } from "../../../store";
  import { cloneDeep } from "$lib/utils/cloneDeep";
  import { generateRoutePages, updateRoutes } from "$lib/utils/generators";
  import * as Dialog from "$lib/components/ui/dialog/index";
  import * as Select from "$lib/components/ui/select/index";
  import { Label } from "../ui/label";
  import capitalizeWords from "$lib/utils/capitalizeWords";
  import CopyIcon from "@lucide/svelte/icons/copy";
  import { toast } from "svelte-sonner";

  type Props = {
    routeName: string;
    encounterArea: string;
  };

  let { routeName, encounterArea }: Props = $props();

  let copyToRouteModalOpen = $state(false);
  let routeToCopyTo: string = $state("");

  let routeListOptions = Object.keys($routes.routes)
    .filter((route) => route !== routeName)
    .map((route) => ({
      label: route,
      value: route,
    }));

  async function copyToRoute() {
    let updatedRoutes: Routes = cloneDeep($routes);

    for (const [_, encounter] of updatedRoutes.routes[
      routeName
    ].wild_encounters.entries()) {
      if (encounterArea !== encounter.encounter_area) continue;

      let sameEncounterExists = updatedRoutes.routes[
        routeToCopyTo
      ].wild_encounters.find(
        (e) =>
          e.route_variant === encounter.route_variant &&
          e.encounter_area === encounter.encounter_area &&
          e.name === encounter.name,
      );

      if (sameEncounterExists) continue;
      encounter.route = routeToCopyTo;
      updatedRoutes.routes[routeToCopyTo].wild_encounters.push(encounter);
    }

    $routes = cloneDeep(updatedRoutes);

    await updateRoutes($routes, $selectedWiki.name)
      .then(() => {
        generateRoutePages([routeToCopyTo], $selectedWiki.name)
          .then((res) => {
            toast.success(res as string);
          })
          .catch((e) => {
            toast.error(e as string);
          });
      })
      .catch((err) => {
        toast.error(err as string);
      });
    copyToRouteModalOpen = false;
    routeToCopyTo = "";
  }
</script>

<Dialog.Root bind:open={copyToRouteModalOpen}>
  <Dialog.Content class="w-[20rem]">
    <Dialog.Header>
      <Dialog.Title>Copy Encounters</Dialog.Title>
      <Dialog.Description>Copy to another route</Dialog.Description>
    </Dialog.Header>
    <Label for="routes" class="text-sm font-medium text-slate-700 block"
      >Routes</Label
    >
    <Select.Root type="single" bind:value={routeToCopyTo}>
      <Select.Trigger id="routes" class="w-full">
        {#if routeToCopyTo === ""}
          Select Route
        {:else}
          {capitalizeWords(routeToCopyTo)}
        {/if}
      </Select.Trigger>
      <Select.Content>
        {#each routeListOptions as option}
          <Select.Item
            value={option.value}
            label={option.label}
            onselect={() => {
              routeToCopyTo = option.label;
            }}
          >
            {capitalizeWords(option.label)}
          </Select.Item>
        {/each}
      </Select.Content>
    </Select.Root>
    <Dialog.Footer>
      <Button
        type="submit"
        disabled={routeToCopyTo === ""}
        onclick={copyToRoute}>Copy</Button
      >
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>

<button
  class="rounded-md p-1 mr-2 hover:cursor-pointer hover:bg-slate-200"
  onclick={() => {
    copyToRouteModalOpen = true;
  }}
>
  <CopyIcon class="text-slate-400 size-5 self-center" />
</button>
