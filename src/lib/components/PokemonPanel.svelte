<script lang="ts">
  import { Tab, TabGroup, getToastStore } from "@skeletonlabs/skeleton";
  import { BaseDirectory, readFile } from "@tauri-apps/plugin-fs";
  import { selectedWiki } from "../../store";
  import {
    routes,
    type Routes,
    type WildEncounter,
  } from "../../store/gameRoutes";
  import {
    pokemonList,
    type Pokemon,
    type PokemonMove,
  } from "../../store/pokemon";
  import PokemonDetailsTab from "./PokemonDetailsTab.svelte";
  import PokemonMovesTab from "./PokemonMovesTab.svelte";
  import { shortcut } from "@svelte-put/shortcut";
  import NumberInput from "./NumberInput.svelte";
  import { db } from "../../store/db";
  import { cloneDeep } from "$lib/utils/cloneDeep";
  import capitalizeWords from "$lib/utils/capitalizeWords";
  import isEqual from "$lib/utils/isEqual";
  import objectIsEmpty from "$lib/utils/objectIsEmpty";
  import PokemonLocationTab from "./PokemonLocationTab.svelte";
  import { getToastSettings, ToastType } from "$lib/utils/toasts";
  import {
    generatePokemonPages,
    generateRoutePages,
    removePokemonPage,
    updateRoutes,
  } from "$lib/utils/generators";
  import * as Card from "$lib/components/ui/card/index.js";
  import CheckIcon from "@lucide/svelte/icons/check";
  import ChevronsUpDownIcon from "@lucide/svelte/icons/chevrons-up-down";
  import { tick } from "svelte";
  import * as Command from "$lib/components/ui/command/index.js";
  import * as Popover from "$lib/components/ui/popover/index.js";
  import * as Select from "$lib/components/ui/select/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { cn, typeColors } from "$lib/utils.js";
  import { Label } from "./ui/label";
  import SaveIcon from "@lucide/svelte/icons/save";
  import FileTextIcon from "@lucide/svelte/icons/file-text";
  import { types as pokemonTypes } from "../../store/types";
  import { abilitiesList as pokemonAbilities } from "../../store/abilities";

  let pokemonSearch: [number, string] = $state([0, ""]);
  let searchingPokemon: string = $state("");
  let pokemonSearchOption: boolean = $state(false);
  let triggerRef = $state<HTMLButtonElement>(null!);

  let pokemon = $state({} as Pokemon);
  let types: string[] = $derived.by(() => {
    let result = pokemon.types.split(",");
    if (result.length === 1) {
      result.push("none");
    }
    return result;
  });

  let originalPokemonDetails: Pokemon = $state({} as Pokemon);
  let pokemonMoveset: PokemonMove[] = $state([]);
  let pokemonLocations: WildEncounter[] = $state([]);
  let pokemonSprite: string = $state("");
  let pokemonNameInput: HTMLInputElement = $state(
    document.createElement("input"),
  );

  // We want to refocus the trigger button when the user selects
  // an item from the list so users can continue navigating the
  // rest of the form with the keyboard.
  function closeAndFocusTrigger() {
    pokemonSearchOption = false;
    tick().then(() => {
      triggerRef.focus();
    });
  }

  let tabSet: number = $state(0);
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

  const toastStore = getToastStore();

  async function generatePage() {
    generatePokemonPages([pokemon.id], $selectedWiki.name)
      .then((res) => {
        toastStore.trigger(getToastSettings(ToastType.SUCCESS, res as string));
      })
      .catch((err) => {
        toastStore.trigger(getToastSettings(ToastType.ERROR, err as string));
      });
  }

  async function getPokemon() {
    let retrievedPokemon = $pokemonList.find(
      ([__, ___, name]) =>
        name === pokemonSearch[1].toLowerCase().split(" ").join("-"),
    );

    if (!retrievedPokemon) {
      toastStore.trigger(
        getToastSettings(ToastType.ERROR, "Pokemon not found!"),
      );
      return;
    }

    await $db
      .select<Pokemon[]>(`SELECT * FROM pokemon WHERE id = $1;`, [
        pokemonSearch[0],
      ])
      .then(async (res) => {
        pokemon = res[0];

        // Gather moveset
        await $db
          .select<PokemonMove[]>(
            `SELECT moves.id as id, moves.name as name, learn_method, level_learned FROM pokemon_movesets
            INNER JOIN moves on moves.id = pokemon_movesets.move
            WHERE pokemon = $1;`,
            [res[0].id],
          )
          .then((res) => {
            pokemonMoveset = res;
          })
          .catch((err) => {
            toastStore.trigger(
              getToastSettings(
                ToastType.ERROR,
                `Error loading Pokemon moveset!: \n ${err}`,
              ),
            );
          });

        // Reading in image separately
        pokemonSprite = await readFile(
          `${$selectedWiki.name}/dist/docs/img/pokemon/${res[0].name}.png`,
          { baseDir: BaseDirectory.AppData },
        )
          .then((res) => {
            const blob = new Blob([res], { type: "image/png" });
            return URL.createObjectURL(blob);
          })
          .catch((err) => {
            if (err.includes("No such file or directory")) {
              return "404";
            }
            return "Error loading image";
          });
        return res[0];
      })
      .then((res) => {
        originalPokemonDetails = cloneDeep(res);
        // Gather location
        pokemonLocations = [];
        for (const [_, properties] of Object.entries($routes.routes)) {
          for (const [_, encounters] of Object.entries(
            properties.wild_encounters,
          )) {
            for (const encounter of encounters) {
              if (encounter.name !== res.name) continue;
              pokemonLocations = [...pokemonLocations, cloneDeep(encounter)];
            }
          }
        }
      })
      .catch((err) => {
        toastStore.trigger(
          getToastSettings(
            ToastType.ERROR,
            `Error loading Pokemon!: \n ${err}`,
          ),
        );
      });
  }

  async function savePokemonChanges() {
    if (isEqual(pokemon, originalPokemonDetails)) {
      return;
    }

    if (pokemon.evolution_method === "no_change") {
      pokemon.evolution_item = null;
      pokemon.evolution_level = null;
      pokemon.evolution_other = null;
    } else if (pokemon.evolution_method === "level_up") {
      pokemon.evolution_item = null;
      pokemon.evolution_other = null;
    } else if (pokemon.evolution_method === "other") {
      pokemon.evolution_item = null;
      pokemon.evolution_level = null;
    } else if (pokemon.evolution_method === "item") {
      pokemon.evolution_level = null;
      pokemon.evolution_other = null;
    }

    await $db
      .execute(
        `UPDATE pokemon SET
          dex_number = ${pokemon.dex_number},
          name = "${pokemon.name}",
          types = "${pokemon.types}",
          hp = ${pokemon.hp},
          attack = ${pokemon.attack},
          defense = ${pokemon.defense},
          sp_attack = ${pokemon.sp_attack},
          sp_defense = ${pokemon.sp_defense},
          speed = ${pokemon.speed},
          evolution_method = $1,
          evolution_item = $2,
          evolution_level = $3,
          evolution_other = $4,
          evolves_into = $5,
          render = "${pokemon.render}",
          ability_1 = $6,
          ability_2 = $7,
          hidden_ability = $8,
        WHERE id = ${pokemon.id};`,
        [
          pokemon.evolution_method,
          pokemon.evolution_item,
          pokemon.evolution_level,
          pokemon.evolution_other,
          pokemon.evolves_into,
          pokemon.ability_1?.toLowerCase().split(" ").join("-"),
          pokemon.ability_2?.toLowerCase().split(" ").join("-"),
          pokemon.hidden_ability?.toLowerCase().split(" ").join("-"),
        ],
      )
      .then(() => {
        if (originalPokemonDetails.dex_number !== pokemon.dex_number) {
          removePokemonPage(
            $selectedWiki.name,
            pokemon.name,
            originalPokemonDetails.dex_number,
          )
            .then(() => {
              let updatedRoutes: Routes = cloneDeep($routes);
              for (const [routeName, properties] of Object.entries(
                $routes.routes,
              )) {
                for (const [encounterArea, wildEncounters] of Object.entries(
                  properties.wild_encounters,
                )) {
                  for (const [index, encounter] of wildEncounters.entries()) {
                    if (encounter.name !== pokemon.name) continue;
                    updatedRoutes.routes[routeName].wild_encounters[
                      encounterArea
                    ][index].id = pokemon.dex_number;
                  }
                }
              }
              $routes = cloneDeep(updatedRoutes);
              updateRoutes($routes, $selectedWiki.name)
                .then(() => {
                  return generateRoutePages(
                    Object.keys($routes.routes),
                    $selectedWiki.name,
                  );
                })
                .then((res) => {
                  toastStore.trigger(
                    getToastSettings(ToastType.SUCCESS, res as string),
                  );
                })
                .catch((e) => {
                  toastStore.trigger(
                    getToastSettings(ToastType.ERROR, e as string),
                  );
                });
            })
            .catch((err) => {
              toastStore.trigger(
                getToastSettings(ToastType.ERROR, err as string),
              );
            });
        }
        originalPokemonDetails = cloneDeep(pokemon);
      })
      .then(() => generatePage())
      .catch((err) => {
        toastStore.trigger(getToastSettings(ToastType.ERROR, err as string));
      });
  }

  function onTypeChange(type: string, type_number: number) {
    if (type_number === 1) {
      types[0] = type;
    } else {
      types[1] = type;
    }

    // This scenario should be unlikely. So default it to normal
    if (types[0] === "none" && types[1] === "none") {
      pokemon.types = "normal";
      return;
    }

    if (types[0] !== "none" && types[1] === "none") {
      pokemon.types = types[0];
      return;
    }

    if (types[0] === "none" && types[1] !== "none") {
      pokemon.types = types[1];
      return;
    }

    if (types[0] === types[1]) {
      pokemon.types = types[0];
      return;
    }

    pokemon.types = `${types[0]},${types[1]}`;
  }
</script>

<Card.Root>
  <Card.Content class="flex flex-row gap-3">
    <Popover.Root bind:open={pokemonSearchOption}>
      <Popover.Trigger bind:ref={triggerRef}>
        {#snippet child({ props })}
          <Button
            id="pokemon-search"
            variant="outline"
            {...props}
            class="w-[20rem]"
            role="combobox"
            aria-expanded={pokemonSearchOption}
          >
            {pokemonSearch[1] || "Select Pokemon"}
            <ChevronsUpDownIcon class="opacity-50" />
          </Button>
        {/snippet}
      </Popover.Trigger>
      <Popover.Content class="w-full p-0">
        <Command.Root shouldFilter={false}>
          <Command.Input
            placeholder="Search Pokemon"
            bind:value={searchingPokemon}
          />
          <Command.List>
            <Command.Empty>No Pokemon found.</Command.Empty>
            {#each options as pokemon}
              <Command.Item
                value={pokemon.label}
                onSelect={() => {
                  pokemonSearch = [pokemon.value, pokemon.label];
                  getPokemon();
                  closeAndFocusTrigger();
                }}
              >
                <!-- <CheckIcon
                      class={cn(pokemon.value !== framework.value && "text-transparent")}
                    /> -->
                {pokemon.label}
              </Command.Item>
            {/each}
          </Command.List>
        </Command.Root>
      </Popover.Content>
    </Popover.Root>
    <div class="grid grid-cols-2 gap-3">
      <Button class="cursor-pointer">
        <SaveIcon />
        Save Changes</Button
      >
      <Button variant="outline" class="cursor-pointer">
        <FileTextIcon />
        Generate Page</Button
      >
    </div>
  </Card.Content>
</Card.Root>

{#if !objectIsEmpty(pokemon)}
  <div class="grid grid-cols-2">
    <Card.Root class="mt-5">
      <Card.Content class="flex flex-col gap-10">
        <section class="flex flex-row gap-5">
          {#if pokemonSprite === "404"}
            <p>No sprite found for {pokemon.name}</p>
          {:else}
            <div
              class={`size-24 rounded-full ${typeColors[pokemon.types.split(",")[0]].background} grid align-center place-content-center shadow-xl`}
            >
              <img
                src={pokemonSprite}
                alt={pokemon.name}
                class="size-20 object-contain"
              />
            </div>
          {/if}
          <div class="flex flex-col justify-between">
            <div class="text-[25px] font-bold">
              {capitalizeWords(pokemon.name)}
            </div>
            <div class="text-slate-400">#{pokemon.dex_number}</div>
            <div class="flex flex-row gap-2">
              {#each pokemon.types.split(",") as type}
                <div
                  class={`text-sm px-2 rounded-lg font-medium ${typeColors[type].background} ${typeColors[type].text}`}
                >
                  {capitalizeWords(type)}
                </div>
              {/each}
            </div>
          </div>
        </section>
        <section class="flex flex-row justify-between">
          <div>
            <Label
              for="pokemon-type"
              class="text-sm font-medium text-slate-700 mb-2 block"
              >Type 1</Label
            >
            <Select.Root type="single" bind:value={types[0]}>
              <Select.Trigger id="pokemon-type" class="w-[13rem]">
                {capitalizeWords(types[0])}
              </Select.Trigger>
              <Select.Content>
                {#each $pokemonTypes as type}
                  <Select.Item
                    value={type}
                    onclick={() => onTypeChange(type, 1)}
                    label={type}
                  >
                    {capitalizeWords(type)}
                  </Select.Item>
                {/each}
              </Select.Content>
            </Select.Root>
          </div>
          <div>
            {#if types[1]}
              <Label
                for="pokemon-type-2"
                class="text-sm font-medium text-slate-700 mb-2 block"
                >Type 2</Label
              >
              <Select.Root type="single" bind:value={types[1]}>
                <Select.Trigger id="pokemon-type-2" class="w-[13rem]">
                  {capitalizeWords(types[1])}
                </Select.Trigger>
                <Select.Content>
                  {#each $pokemonTypes as type}
                    <Select.Item
                      value={type}
                      onclick={() => onTypeChange(type, 2)}
                      label={type}
                    >
                      {capitalizeWords(type)}
                    </Select.Item>
                  {/each}
                </Select.Content>
              </Select.Root>
            {/if}
          </div>
        </section>
        <section class="flex flex-row justify-between">
          <div>
            <Label
              for="pokemon-ability"
              class="text-sm font-medium text-slate-700 mb-2 block"
              >Ability 1</Label
            >
            <Select.Root type="single" bind:value={pokemon.ability_1 as string}>
              <Select.Trigger id="pokemon-ability" class="w-[13rem]">
                {capitalizeWords(types[0])}
              </Select.Trigger>
              <Select.Content>
                {#each $pokemonAbilities as ability}
                  <Select.Item
                    value={pokemon.ability_1 as string}
                    label={ability[1]}
                  >
                    {capitalizeWords(pokemon.ability_1 as string)}
                  </Select.Item>
                {/each}
              </Select.Content>
            </Select.Root>
          </div>
          <div>
            {#if types[1]}
              <Label
                for="pokemon-type-2"
                class="text-sm font-medium text-slate-700 mb-2 block"
                >Type 2</Label
              >
              <Select.Root type="single" bind:value={types[1]}>
                <Select.Trigger id="pokemon-type-2" class="w-[13rem]">
                  {capitalizeWords(types[1])}
                </Select.Trigger>
                <Select.Content>
                  {#each $pokemonTypes as type}
                    <Select.Item
                      value={type}
                      onclick={() => onTypeChange(type, 2)}
                      label={type}
                    >
                      {capitalizeWords(type)}
                    </Select.Item>
                  {/each}
                </Select.Content>
              </Select.Root>
            {/if}
          </div>
        </section>
      </Card.Content>
    </Card.Root>
  </div>
{/if}

{#if false}
  {#if pokemonSprite === "404"}
    <p>No sprite found for {pokemon.name}</p>
  {:else}
    <NumberInput
      label="Dex Number"
      bind:value={pokemon.dex_number}
      class="w-40 mb-5"
    />
  {/if}
  <TabGroup>
    <Tab bind:group={tabSet} name="pokemon-details" value={0} class="text-sm"
      >Details</Tab
    >
    <Tab bind:group={tabSet} name="pokemon-moves" value={1} class="text-sm"
      >Moves</Tab
    >
    <Tab bind:group={tabSet} name="location" value={2} class="text-sm"
      >Location</Tab
    >
    <div slot="panel">
      {#if tabSet === 0}
        <PokemonDetailsTab bind:pokemon />
      {/if}
      {#if tabSet === 1}
        <PokemonMovesTab
          bind:moveset={pokemonMoveset}
          bind:pokemonId={pokemon.id}
          generatePokemonPage={() => generatePage()}
        />
      {/if}
      {#if tabSet === 2}
        <PokemonLocationTab
          {pokemonLocations}
          pokemonId={pokemon.id}
          pokemonDexNumber={pokemon.dex_number}
          pokemonName={pokemon.name}
        />
      {/if}
    </div>
  </TabGroup>
{/if}

<svelte:window
  use:shortcut={{
    trigger: {
      key: "k",
      modifier: ["ctrl", "meta"],
      callback: () => pokemonNameInput.focus(),
    },
  }}
  use:shortcut={{
    trigger: {
      key: "m",
      modifier: "ctrl",
      callback: () => {
        tabSet = 1;
      },
    },
  }}
  use:shortcut={{
    trigger: {
      key: "Enter",
      modifier: ["ctrl", "meta"],
      callback: () => savePokemonChanges(),
    },
  }}
/>
