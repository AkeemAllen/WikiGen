<script lang="ts">
  import { getToastStore, popup } from "@skeletonlabs/skeleton";
  import IconDots from "@tabler/icons-svelte/icons/dots";
  import BaseModal from "$lib/components/BaseModal.svelte";
  import AutoComplete from "$lib/components/AutoComplete.svelte";
  import Button from "$lib/components/Button.svelte";
  import { routes, type Routes } from "../../../store/gameRoutes";
  import { selectedWiki } from "../../../store";
  import { cloneDeep } from "$lib/utils/cloneDeep";
  import { generateRoutePages, updateRoutes } from "$lib/utils/generators";
  import { getToastSettings, ToastType } from "$lib/utils/toasts";

  const toastStore = getToastStore();

  export let index: number;
  export let routeName: string;
  export let encounterArea: string;

  $: console.log({ routeName, encounterArea });

  let copyToRouteModalOpen = false;
  let routeToCopyTo: string = "";

  let routeListOptions = Object.keys($routes.routes)
    .filter((route) => route !== routeName)
    .map((route) => ({
      label: route,
      value: route,
    }));

  async function copyToRoute() {
    let updatedRoutes: Routes = cloneDeep($routes);

    if (
      Object.keys(updatedRoutes.routes[routeToCopyTo].wild_encounters).includes(
        encounterArea,
      )
    ) {
      toastStore.trigger({
        message: "This encounter area already exists in the selected route",
        timeout: 3000,
        background: "variant-filled-error",
      });
      return;
    }

    updatedRoutes.routes[routeToCopyTo].wild_encounters[encounterArea] =
      cloneDeep(updatedRoutes.routes[routeName].wild_encounters[encounterArea]);

    $routes = cloneDeep(updatedRoutes);

    await updateRoutes($routes, $selectedWiki.name)
      .then(() => {
        generateRoutePages([routeToCopyTo], $selectedWiki.name)
          .then((res) => {
            toastStore.trigger(
              getToastSettings(ToastType.SUCCESS, res as string),
            );
          })
          .catch((e) => {
            toastStore.trigger(getToastSettings(ToastType.ERROR, e as string));
          });
      })
      .catch((err) => {
        toastStore.trigger(getToastSettings(ToastType.ERROR, err as string));
      });
    copyToRouteModalOpen = false;
    routeToCopyTo = "";
  }
</script>

<!-- Copy To Route Modal -->
<BaseModal bind:open={copyToRouteModalOpen} class="w-[25rem]">
  <AutoComplete
    bind:value={routeToCopyTo}
    label="Routes"
    popupId="newLocationPopup"
    class="z-10 w-full text-sm"
    options={routeListOptions}
    onSelection={(e) => {
      routeToCopyTo = e.detail.value;
    }}
  />
  <Button
    class="w-32"
    title="Copy To Route"
    disabled={routeToCopyTo === ""}
    onClick={copyToRoute}
  />
</BaseModal>

<button
  class="hover:cursor-pointer"
  use:popup={{
    event: "click",
    target: "wildEncounterAreaMenu" + index,
    placement: "right",
  }}
>
  <IconDots size={20} color="gray" />
</button>
<div
  class="card z-10 w-44 bg-white p-2"
  data-popup="wildEncounterAreaMenu{index}"
>
  <button
    class="w-full rounded-md p-2 text-left text-sm hover:bg-slate-300"
    on:click={() => {
      copyToRouteModalOpen = true;
      console.log(routeName, copyToRouteModalOpen);
    }}>Copy To Route</button
  >
</div>
