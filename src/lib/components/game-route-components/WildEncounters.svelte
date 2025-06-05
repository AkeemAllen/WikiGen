<script lang="ts">
  import { BaseDirectory, readFile } from "@tauri-apps/plugin-fs";
  import { selectedWiki } from "../../../store";
  import { routes, type WildEncounter } from "../../../store/gameRoutes";
  import { pokemonList } from "../../../store/pokemon";
  import { Button } from "$lib/components/ui/button/index.js";
  import { cloneDeep } from "$lib/utils/cloneDeep";
  import capitalizeWords from "$lib/utils/capitalizeWords";
  import isEqual from "$lib/utils/isEqual";
  import { generateRoutePages, updateRoutes } from "$lib/utils/generators";
  import * as Card from "$lib/components/ui/card/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { toast } from "svelte-sonner";
  import Autocomplete from "$lib/components/ui/Autocomplete.svelte";
  import SaveIcon from "@lucide/svelte/icons/save";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import * as Select from "$lib/components/ui/select/index.js";
  import { Label } from "$lib/components/ui/label";
  import EditIcon from "@lucide/svelte/icons/edit";
  import XIcon from "@lucide/svelte/icons/x";
  import WildEncounterAreaMenu from "../modals/WildEncounterAreaMenu.svelte";

  type Props = {
    routeName?: string;
  };

  let { routeName = $bindable("") }: Props = $props();
  let pokemonName: string = $state("");
  let pokemonSearchOpen: boolean = $state(false);
  let triggerRef = $state<HTMLButtonElement>(null!);
  let searchingPokemon = $state("");
  let encounterArea: string = $state("grass");
  let currentWildEncounterIndex: number = $state(0);
  let currentEncounterType: string = $state("");
  let editEncounterModalOpen: boolean = $state(false);
  let encounterRate: number = $state(0);
  let areaLevels = $state(
    cloneDeep($routes.routes[routeName].wild_encounter_area_levels),
  );
  let originalAreaLevels = $state(
    cloneDeep($routes.routes[routeName].wild_encounter_area_levels),
  );
  let routeWildEncounters: { [key: string]: WildEncounter[] } = $state(
    cloneDeep($routes.routes[routeName].wild_encounters),
  );
  let originalRouteWildEncounters = $state(
    cloneDeep($routes.routes[routeName].wild_encounters),
  );
  let pokemonListOptions = $pokemonList.map(([id, _, name]) => ({
    label: capitalizeWords(name),
    value: id,
  }));

  let options = $derived(
    pokemonListOptions
      .filter((pokemon) =>
        pokemon.label.toLowerCase().includes(searchingPokemon.toLowerCase()),
      )
      .slice(0, 8),
  );

  const encounterAreas = $routes.encounter_areas.map((type) => ({
    label: type,
    value: type,
  }));

  async function addEncounter() {
    let searchedPokemon = $pokemonList.find(
      ([_, __, name]) =>
        name === pokemonName.toLowerCase().replaceAll(" ", "-"),
    );

    if (searchedPokemon === undefined) {
      toast.error("Pokemon not found");
      return;
    }

    routeWildEncounters = {
      ...routeWildEncounters,
      [encounterArea]: [
        ...(routeWildEncounters[encounterArea] ?? []),
        {
          id: searchedPokemon[1],
          name: pokemonName.toLowerCase().replaceAll(" ", "-"),
          encounter_rate: encounterRate,
          encounter_area: encounterArea,
          special_note: "",
          route: routeName,
        },
      ],
    };

    routeWildEncounters[encounterArea]
      .sort(
        (encounter1, encounter2) =>
          encounter1.encounter_rate - encounter2.encounter_rate,
      )
      .reverse();
  }

  async function deleteEncounter(pokemonName: string, encounterArea: string) {
    editEncounterModalOpen = false;
    let updatedEncounters = {
      ...routeWildEncounters,
    };
    updatedEncounters[encounterArea] = updatedEncounters[encounterArea].filter(
      (encounter) => encounter.name !== pokemonName,
    );
    if (updatedEncounters[encounterArea].length === 0) {
      delete updatedEncounters[encounterArea];
    }

    if (updatedEncounters[encounterArea] !== undefined) {
      updatedEncounters[encounterArea]
        .sort(
          (encounter1, encounter2) =>
            encounter1.encounter_rate - encounter2.encounter_rate,
        )
        .reverse();
    }

    routeWildEncounters = updatedEncounters;
  }

  async function saveChanges() {
    $routes.routes[routeName].wild_encounters = routeWildEncounters;
    $routes.routes[routeName].wild_encounter_area_levels = areaLevels;

    routeWildEncounters = cloneDeep($routes.routes[routeName].wild_encounters);
    originalRouteWildEncounters = cloneDeep(routeWildEncounters);
    areaLevels = cloneDeep(
      $routes.routes[routeName].wild_encounter_area_levels,
    );
    originalAreaLevels = cloneDeep(areaLevels);

    await updateRoutes($routes, $selectedWiki.name)
      .then(() => {
        generateRoutePages([routeName], $selectedWiki.name)
          .then((res) => {
            toast.success(res as string);
          })
          .catch((e) => {
            toast.error(e as string);
          });
      })
      .catch((e) => {
        toast.error(e as string);
      });
  }

  async function getSpriteImage(pokemonName: string): Promise<string> {
    let sprite = "";
    let spriteName = pokemonName.toLowerCase().replaceAll(" ", "-");
    await readFile(
      `${$selectedWiki.name}/dist/docs/img/pokemon/${spriteName}.png`,
      { baseDir: BaseDirectory.AppData },
    )
      .then((res) => {
        const blob = new Blob([res], { type: "image/png" });
        sprite = URL.createObjectURL(blob);
      })
      .catch((err) => {
        if (err.includes("No such file or directory")) {
          sprite = "Image Not Found";
        }
        sprite = "Error loading image";
      });
    return sprite;
  }
</script>

<Dialog.Root bind:open={editEncounterModalOpen}>
  <Dialog.Content class="w-[15rem]">
    <Label
      for="encounter-rate"
      class="text-sm font-medium text-slate-700 block"
    >
      Encounter Rate
    </Label>
    <Input
      type="number"
      id="encounter-rate"
      bind:value={
        routeWildEncounters[currentEncounterType][currentWildEncounterIndex]
          .encounter_rate
      }
      class="w-full"
      max={100}
    />
    <Dialog.Footer>
      <Button
        type="submit"
        onclick={() => {
          routeWildEncounters[currentEncounterType]
            .sort(
              (encounter1, encounter2) =>
                encounter1.encounter_rate - encounter2.encounter_rate,
            )
            .reverse();
          saveChanges();
          editEncounterModalOpen = false;
        }}
        disabled={isEqual(routeWildEncounters, originalRouteWildEncounters) &&
          isEqual(areaLevels, originalAreaLevels)}>Save Changes</Button
      >
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>

<Card.Root>
  <Card.Content class="flex flex-row gap-3">
    <section class="flex flex-row gap-5 justify-between">
      <div>
        <Label
          for="encounter-area"
          class="text-sm font-medium text-slate-700 mb-2 block"
          >Encounter Area</Label
        >
        <Select.Root type="single" bind:value={encounterArea}>
          <Select.Trigger id="encounter-area" class="w-[11rem]">
            {capitalizeWords(encounterArea)}
          </Select.Trigger>
          <Select.Content>
            {#each encounterAreas as area}
              <Select.Item value={area.value} label={area.label}>
                {capitalizeWords(area.label)}
              </Select.Item>
            {/each}
          </Select.Content>
        </Select.Root>
      </div>
      <Autocomplete
        open={pokemonSearchOpen}
        {triggerRef}
        value={pokemonName}
        label="Wild Pokemon"
        bind:searcher={searchingPokemon}
        {options}
        placeholder="Search Pokemon"
        onSelect={(option) => {
          pokemonName = option.label;
        }}
        class="w-[10rem]"
      />
      <div class="flex flex-col">
        <Label
          for="encounter-rate"
          class="text-sm font-medium text-slate-700 mb-2 block"
          >Encounter Rate</Label
        >

        <Input
          type="number"
          id="encounter-rate"
          bind:value={encounterRate}
          max={100}
          class="w-40"
        />
      </div>
      <div class="grid grid-cols-2 gap-3 mt-7">
        <Button class="cursor-pointer" onclick={addEncounter}>
          Add Encounter</Button
        >
        <Button
          variant="outline"
          class="cursor-pointer"
          disabled={isEqual(routeWildEncounters, originalRouteWildEncounters) &&
            isEqual(areaLevels, originalAreaLevels)}
          onclick={saveChanges}
        >
          <SaveIcon />
          Save Changes</Button
        >
      </div>
    </section></Card.Content
  >
</Card.Root>

<main class=" mt-5 grid grid-cols-3 gap-5">
  {#each Object.entries(routeWildEncounters) as [_encounterType, encounters], index}
    <Card.Root>
      <Card.Header>
        <div class="flex flex-row justify-between">
          <Card.Title class="text-slate-900 text-lg"
            >{capitalizeWords(_encounterType)} Encounters</Card.Title
          >
          <WildEncounterAreaMenu encounterArea={_encounterType} {routeName} />
        </div>
        <Card.Description>
          <Input
            type="text"
            bind:value={areaLevels[_encounterType]}
            placeholder="Lv."
            class="w-20"
          />
        </Card.Description>
      </Card.Header>
      <Card.Content>
        <div class="space-y-2 max-h-80 overflow-y-auto">
          {#each encounters as encounter, index}
            <div
              class="flex items-center gap-3 p-2 bg-slate-100 dark:bg-slate-800/60 rounded-lg group hover:bg-slate-200 dark:hover:bg-slate-700/80 transition-colors"
            >
              {#await getSpriteImage(encounter.name) then spriteUrl}
                <img
                  src={spriteUrl}
                  alt={encounter.name}
                  class="size-13 object-contain"
                />
              {/await}
              <div class="flex-1 min-w-0">
                <div
                  class="font-medium text-slate-900 dark:text-slate-100 truncate"
                >
                  {capitalizeWords(encounter.name)}
                </div>
                <div
                  class="flex items-center gap-2 text-xs text-slate-600 dark:text-slate-400"
                >
                  <span class="flex items-center gap-1">
                    {encounter.encounter_rate}%
                  </span>
                </div>
              </div>
              <div
                class="flex gap-1 opacity-0 group-hover:opacity-100 transition-opacity"
              >
                <Button
                  size="sm"
                  variant="ghost"
                  class="h-6 w-6 p-0 cursor-pointer"
                  onclick={() => {
                    editEncounterModalOpen = true;
                    currentEncounterType = _encounterType;
                    currentWildEncounterIndex = index;
                  }}
                >
                  <EditIcon class="w-3 h-3" />
                </Button>
                <Button
                  size="sm"
                  variant="ghost"
                  class="h-6 w-6 p-0 text-red-600 hover:text-red-700 cursor-pointer"
                  onclick={() => {
                    deleteEncounter(encounter.name, _encounterType);
                  }}
                >
                  <XIcon class="w-3 h-3" />
                </Button>
              </div>
            </div>
          {/each}
        </div>
      </Card.Content>
    </Card.Root>
  {/each}
</main>
