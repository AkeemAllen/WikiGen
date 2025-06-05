<script lang="ts">
  import { run } from "svelte/legacy";

  import BaseModal from "$lib/components/BaseModal.svelte";
  import { Button } from "$lib/components/ui/button/index.js";
  import TextInput from "$lib/components/TextInput.svelte";
  import IconTrash from "@tabler/icons-svelte/icons/trash";
  import { selectedWiki } from "../../store";
  import { routes } from "../../store/gameRoutes";
  import { sortRoutesByPosition } from "$lib/utils";
  import NumberInput from "$lib/components/NumberInput.svelte";
  import GameRoutes from "$lib/components/game-route-components/GameRoutes.svelte";
  import { generateRoutePages, updateRoutes } from "$lib/utils/generators";
  import { getToastSettings, ToastType } from "$lib/utils/toasts";
  import * as Card from "$lib/components/ui/card/index.js";
  import { toast } from "svelte-sonner";

  let routeName: string = $state("");
  let routeToUpdate: string = $state("");
  let newRouteModalOpen: boolean = $state(false);
  let encounterTypeModalOpen: boolean = $state(false);
  let positionModalOpen: boolean = $state(false);
  let newEncounterType: string = $state("");
  let oldRoutePosition: number = $state(0);
  let loading = $state(false);

  async function createNewRoute() {
    if (routeName.trim() === "") {
      return;
    }

    if ($routes.routes[routeName.trim()]) {
      toast.error("Route already exists");
      return;
    }

    $routes.routes[routeName.trim()] = {
      render: true,
      position: Object.keys($routes.routes).length + 1,
      trainers: {},
      wild_encounters: {},
      wild_encounter_area_levels: {},
    };

    await updateRoutes($routes, $selectedWiki.name)
      .then(() => {
        routeName = "";
        newRouteModalOpen = false;
      })
      .catch((err) => {
        toast.error(err);
      });
  }

  async function addNewEncounterType() {
    $routes.encounter_areas = [...$routes.encounter_areas, newEncounterType];
    let sortedRoutes = sortRoutesByPosition($routes);
    await updateRoutes(sortedRoutes, $selectedWiki.name).catch((err) => {
      toast.error(err);
    });
  }

  async function deleteEncounterType(encounterType: string) {
    $routes.encounter_areas = $routes.encounter_areas.filter(
      (type) => type !== encounterType,
    );
    await updateRoutes($routes, $selectedWiki.name).catch((err) => {
      toast.error(err);
    });
  }

  async function updatePosition() {
    let newRoutePosition = $routes.routes[routeToUpdate].position;
    if (oldRoutePosition === newRoutePosition) {
      return;
    }

    if (oldRoutePosition < newRoutePosition) {
      Object.keys($routes.routes).forEach((routeName) => {
        if (
          $routes.routes[routeName].position > oldRoutePosition &&
          $routes.routes[routeName].position <= newRoutePosition &&
          routeName !== routeToUpdate
        ) {
          $routes.routes[routeName].position -= 1;
        }
      });
    }

    if (oldRoutePosition > newRoutePosition) {
      Object.keys($routes.routes).forEach((routeName) => {
        if (
          $routes.routes[routeName].position < oldRoutePosition &&
          $routes.routes[routeName].position >= newRoutePosition &&
          routeName !== routeToUpdate
        ) {
          $routes.routes[routeName].position += 1;
        }
      });
    }

    $routes = sortRoutesByPosition($routes);
    await updateRoutes($routes, $selectedWiki.name).catch((err) => {
      toast.error(err);
    });
  }

  async function generatePages() {
    loading = true;
    await generateRoutePages(Object.keys($routes.routes), $selectedWiki.name)
      .then((res) => {
        loading = false;
        toast.success(res as string);
      })
      .catch((err) => {
        loading = false;
        toast.error(err);
      });
  }

  function capitalizeWords(event: any) {
    routeName = event.target.value.replace(/\b\w/g, (char: string) =>
      char.toUpperCase(),
    );
  }
</script>

<!-- New Route Modal -->
<BaseModal bind:open={newRouteModalOpen}>
  <TextInput
    bind:value={routeName}
    label="Route Name"
    inputHandler={capitalizeWords}
  />
  <Button
    onclick={() => {
      createNewRoute();
    }}
    disabled={routeName === ""}
  >
    Save New Route
  </Button>
</BaseModal>

<!-- Position Modal -->
<BaseModal bind:open={positionModalOpen}>
  <NumberInput
    bind:value={$routes.routes[routeToUpdate].position}
    label="Route Position"
  />
  <Button
    onclick={() => {
      updatePosition();
      positionModalOpen = false;
    }}
  >
    Update Route Position
  </Button>
</BaseModal>

<!-- Encounter Area Modal -->
<BaseModal bind:open={encounterTypeModalOpen}>
  <div class="flex flex-row gap-3">
    <Button
      class="mt-2 w-44"
      onclick={addNewEncounterType}
      disabled={newEncounterType === ""}
    >
      Add New Encounter
    </Button>
    <TextInput bind:value={newEncounterType} placeholder="New Encounter Area" />
  </div>
  <div class="grid grid-cols-2 gap-3">
    {#each $routes.encounter_areas as encounterType}
      <div class="card flex flex-row items-center justify-between px-2 py-1">
        {encounterType}
        <button
          class="btn rounded-sm p-2 hover:cursor-pointer hover:bg-gray-300"
          onclick={() => deleteEncounterType(encounterType)}
        >
          <IconTrash size={16} />
        </button>
      </div>
    {/each}
  </div>
</BaseModal>

<Card.Root class="mx-5 my-5">
  <Card.Content class="flex flex-row gap-3">
    <Button class="cursor-pointer" onclick={() => (newRouteModalOpen = true)}>
      Create New Route</Button
    >
    <Button
      variant="outline"
      class="cursor-pointer"
      onclick={() => (encounterTypeModalOpen = true)}
    >
      Modify Encounter Types</Button
    >
    <Button
      variant="outline"
      class="cursor-pointer"
      onclick={() => generatePages()}
    >
      Generate Route Pages</Button
    >
  </Card.Content>
</Card.Root>

<p class="text-sm italic text-gray-600 mx-5">
  <strong>Note: </strong>A route needs to have at least <strong>ONE</strong> wild
  or trainer encounter to be rendered.
</p>
<div class="mx-5">
  <GameRoutes bind:positionModalOpen bind:routeToUpdate bind:oldRoutePosition />
</div>
