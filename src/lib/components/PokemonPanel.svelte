<script lang="ts">
  import { BaseDirectory, readFile } from "@tauri-apps/plugin-fs";
  import { selectedWiki } from "../../store";
  import {
    routes,
    type Routes,
    type WildEncounter,
  } from "../../store/gameRoutes";
  import { pokemonList, type Pokemon } from "../../store/pokemon";
  import { db } from "../../store/db";
  import { cloneDeep } from "$lib/utils/cloneDeep";
  import capitalizeWords from "$lib/utils/capitalizeWords";
  import isEqual from "$lib/utils/isEqual";
  import objectIsEmpty from "$lib/utils/objectIsEmpty";
  import PokemonLocation from "./PokemonLocations.svelte";
  import {
    generatePokemonPages,
    generateRoutePages,
    removePokemonPage,
    updateRoutes,
  } from "$lib/utils/generators";
  import * as Card from "$lib/components/ui/card/index.js";
  import * as Select from "$lib/components/ui/select/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { typeColors } from "$lib/utils.js";
  import { Label } from "./ui/label";
  import SaveIcon from "@lucide/svelte/icons/save";
  import FileTextIcon from "@lucide/svelte/icons/file-text";
  import { types as pokemonTypes } from "../../store/types";
  import { abilitiesList as pokemonAbilities } from "../../store/abilities";
  import { toast } from "svelte-sonner";
  import { Input } from "$lib/components/ui/input/index.js";
  import { itemsList } from "../../store/items";
  import Autocomplete from "./ui/Autocomplete.svelte";
  import PokemonStat from "./ui/PokemonStat.svelte";

  let pokemonSearch: [number, string] = $state([0, ""]);
  let searchingPokemon: string = $state("");
  let pokemonSearchOption: boolean = $state(false);
  let evolutionSearchOption: boolean = $state(false);
  let itemSearchOption: boolean = $state(false);
  let searchingEvolutions: string = $state("");
  let searchingItems: string = $state("");
  let triggerRef = $state<HTMLButtonElement>(null!);
  let triggerRefEvolution = $state<HTMLButtonElement>(null!);
  let triggerRefItems = $state<HTMLButtonElement>(null!);

  let pokemon = $state({} as Pokemon);
  let abilities = $derived.by(() => {
    let result = pokemon.abilities.split(",");
    if (result.length === 1) {
      result.push("");
    }
    return result;
  });
  let types: string[] = $derived.by(() => {
    let result = pokemon.types.split(",");
    if (result.length === 1) {
      result.push("");
    }
    return result;
  });

  let baseTotal = $derived(
    pokemon.hp +
      pokemon.attack +
      pokemon.defense +
      pokemon.sp_attack +
      pokemon.sp_defense +
      pokemon.speed,
  );

  let originalPokemonDetails: Pokemon = $state({} as Pokemon);
  let pokemonLocations: WildEncounter[] = $state([]);
  let pokemonSprite: string = $state("");

  let pokemonListOptions = $derived(
    $pokemonList.map(([id, _, name]) => ({
      label: capitalizeWords(name),
      value: id,
    })),
  );

  let options = $derived(
    pokemonListOptions
      .filter((pokemon) =>
        pokemon.label.toLowerCase().includes(searchingPokemon.toLowerCase()),
      )
      .slice(0, 8),
  );

  let evolutionOptions = $derived(
    pokemonListOptions
      .filter((pokemon) =>
        pokemon.label.toLowerCase().includes(searchingEvolutions.toLowerCase()),
      )
      .slice(0, 8),
  );

  async function generatePage() {
    generatePokemonPages([pokemon.id], $selectedWiki.name)
      .then((res) => {
        toast.success(res as string);
      })
      .catch((err) => {
        toast.error(err as string);
      });
  }

  async function getPokemon() {
    let retrievedPokemon = $pokemonList.find(
      ([__, ___, name]) =>
        name === pokemonSearch[1].toLowerCase().split(" ").join("-"),
    );

    if (!retrievedPokemon) {
      toast.error("Pokemon not found!");
      return;
    }

    await $db
      .select<Pokemon[]>(`SELECT * FROM pokemon WHERE id = $1;`, [
        pokemonSearch[0],
      ])
      .then(async (res) => {
        pokemon = res[0];

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
        toast.error(`Error loading Pokemon!: \n ${err}`);
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
          abilities = "${pokemon.abilities}",
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
          render = "${pokemon.render}"
        WHERE id = ${pokemon.id};`,
        [
          pokemon.evolution_method,
          pokemon.evolution_item,
          pokemon.evolution_level,
          pokemon.evolution_other,
          pokemon.evolves_into,
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
                  toast.success(res as string);
                })
                .catch((err) => {
                  toast.error(err as string);
                });
            })
            .catch((err) => {
              toast.error(err as string);
            });
        }
        originalPokemonDetails = cloneDeep(pokemon);
      })
      .then(() => generatePage())
      .catch((err) => {
        toast.error(err as string);
      });
  }

  function onTypeChange(type: string, type_number: number) {
    if (type_number === 1) {
      if (type === "none") {
        toast.error(
          "First Type cannot be empty. Pokemon need at least one type",
        );
        return;
      }
      types[0] = type;
    } else {
      types[1] = type;
    }

    if (types[0] === types[1] || types[1] === "none") {
      pokemon.types = types[0];
      types[1] = "";
      return;
    }

    pokemon.types = types.join(",");

    // This is meant to drop the trailing comma if it exists
    if (pokemon.types[pokemon.types.length - 1] === ",") {
      pokemon.types = pokemon.types.slice(0, -1);
    }
  }

  function onAbilityChange(ability: string, ability_number: number) {
    if (ability === "None") {
      ability = "";
    }

    if (ability_number === 1) {
      if (ability === "") {
        toast.error(
          "First Ability cannot be empty. Pokemon need at least one ability",
        );
        return;
      }
      abilities[0] = ability;
    } else if (ability_number === 2) {
      abilities[1] = ability;
    } else if (ability_number === 3) {
      abilities[2] = ability;
    }

    if (abilities[0] === abilities[1]) {
      pokemon.abilities = abilities[0];
      return;
    }
    pokemon.abilities = abilities.join(",");
  }
</script>

<Card.Root>
  <Card.Content class="flex flex-row gap-3">
    <Autocomplete
      open={pokemonSearchOption}
      {triggerRef}
      value={pokemonSearch[1]}
      bind:searcher={searchingPokemon}
      {options}
      placeholder="Search Pokemon"
      onSelect={(option) => {
        pokemonSearch = [option.value, option.label];
        getPokemon();
      }}
      class="w-[20rem]"
    />
    <div class="grid grid-cols-2 gap-3">
      <Button
        class="cursor-pointer"
        onclick={savePokemonChanges}
        disabled={isEqual(pokemon, originalPokemonDetails)}
      >
        <SaveIcon />
        Save Changes</Button
      >
      <Button
        variant="outline"
        class="cursor-pointer"
        disabled={objectIsEmpty(pokemon)}
        onclick={generatePage}
      >
        <FileTextIcon />
        Generate Page</Button
      >
    </div>
  </Card.Content>
</Card.Root>

{#if !objectIsEmpty(pokemon)}
  <div class="flex flex-row gap-5 mb-5">
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
        <section class="flex flex-row gap-5 justify-between">
          <div>
            <Label
              for="pokemon-type"
              class="text-sm font-medium text-slate-700 mb-2 block"
              >Type 1</Label
            >
            <Select.Root type="single" bind:value={types[0]}>
              <Select.Trigger id="pokemon-type" class="w-[11rem]">
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
            <Label
              for="pokemon-type-2"
              class="text-sm font-medium text-slate-700 mb-2 block"
              >Type 2</Label
            >
            <Select.Root type="single" bind:value={types[1]}>
              <Select.Trigger id="pokemon-type-2" class="w-[11rem]">
                {#if types[1] === ""}
                  None
                {:else}
                  {capitalizeWords(types[1])}
                {/if}
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
          </div>
        </section>
        <section class="flex flex-col gap-10">
          <section class="flex flex-row justify-between">
            <div>
              <Label
                for="pokemon-ability-1"
                class="text-sm font-medium text-slate-700 mb-2 block"
                >Ability 1</Label
              >
              <Select.Root type="single" bind:value={abilities[0]}>
                <Select.Trigger id="pokemon-ability-1" class="w-[11rem]">
                  {capitalizeWords(abilities[0])}
                </Select.Trigger>
                <Select.Content>
                  {#each $pokemonAbilities as [id, ability]}
                    <Select.Item
                      value={ability}
                      label={ability}
                      onclick={() => {
                        onAbilityChange(ability, 1);
                      }}
                    >
                      {capitalizeWords(ability)}
                    </Select.Item>
                  {/each}
                </Select.Content>
              </Select.Root>
            </div>
            <div>
              <Label
                for="pokemon-ability-2"
                class="text-sm font-medium text-slate-700 mb-2 block"
                >Ability 2</Label
              >
              <Select.Root type="single" bind:value={abilities[1]}>
                <Select.Trigger id="pokemon-ability-2" class="w-[11rem]">
                  {#if abilities[1] === ""}
                    None
                  {:else}
                    {capitalizeWords(abilities[1])}
                  {/if}
                </Select.Trigger>
                <Select.Content>
                  {#each $pokemonAbilities as [id, ability]}
                    <Select.Item
                      value={ability}
                      label={ability}
                      onclick={() => {
                        onAbilityChange(ability, 2);
                      }}
                    >
                      {capitalizeWords(ability)}
                    </Select.Item>
                  {/each}
                </Select.Content>
              </Select.Root>
            </div>
          </section>
          <div>
            <Label
              for="pokemon-ability-hidden"
              class="text-sm font-medium text-slate-700 mb-2 block"
              >Hidden Ability</Label
            >
            <Select.Root type="single" bind:value={abilities[2]}>
              <Select.Trigger id="pokemon-ability-hidden" class="w-full">
                {#if abilities[2] === ""}
                  None
                {:else}
                  {capitalizeWords(abilities[2])}
                {/if}
              </Select.Trigger>
              <Select.Content>
                {#each $pokemonAbilities as [id, ability]}
                  <Select.Item
                    value={ability}
                    label={ability}
                    onclick={() => {
                      onAbilityChange(ability, 3);
                    }}
                  >
                    {capitalizeWords(ability)}
                  </Select.Item>
                {/each}
              </Select.Content>
            </Select.Root>
          </div>
        </section>
      </Card.Content>
    </Card.Root>
    <div class="flex flex-col w-full">
      <Card.Root class="mt-5 h-fit w-full">
        <Card.Header>
          <Card.Title>Base Stats</Card.Title>
          <Card.Description>Base Total: {baseTotal}</Card.Description>
        </Card.Header>
        <Card.Content class="grid grid-cols-2 gap-4">
          <PokemonStat label="HP" bind:value={pokemon.hp} />
          <PokemonStat label="Attack" bind:value={pokemon.attack} />
          <PokemonStat label="Defense" bind:value={pokemon.defense} />
          <PokemonStat label="Special Attack" bind:value={pokemon.sp_attack} />
          <PokemonStat
            label="Special Defense"
            bind:value={pokemon.sp_defense}
          />
          <PokemonStat label="Speed" bind:value={pokemon.speed} />
        </Card.Content>
      </Card.Root>
      <Card.Root class="mt-5 h-fit w-full">
        <Card.Header>
          <Card.Title>Evolution</Card.Title>
        </Card.Header>
        <Card.Content class="flex flex-row gap-4">
          <div>
            <Label
              for="evolution-method"
              class="text-sm font-medium text-slate-700 mb-2 block"
              >Method</Label
            >
            <Select.Root type="single" bind:value={pokemon.evolution_method}>
              <Select.Trigger id="evolution-method" class="w-[11rem]">
                {capitalizeWords(pokemon.evolution_method)}
              </Select.Trigger>
              <Select.Content>
                {#each ["level_up", "item", "other", "no_change"] as method}
                  <Select.Item value={method} label={method}>
                    {capitalizeWords(method)}
                  </Select.Item>
                {/each}
              </Select.Content>
            </Select.Root>
          </div>
          <div>
            {#if pokemon.evolution_method !== "no_change"}
              <Label
                for={pokemon.evolution_method}
                class="text-sm font-medium text-slate-700 mb-2 block"
                >{capitalizeWords(pokemon.evolution_method)}</Label
              >
            {/if}
            {#if pokemon.evolution_method === "item"}
              <Autocomplete
                open={itemSearchOption}
                triggerRef={triggerRefItems}
                value={pokemon.evolution_item}
                bind:searcher={searchingItems}
                options={$itemsList
                  .map((item) => ({
                    value: item[0],
                    label: item[1],
                  }))
                  .filter((item) => item.label.includes(searchingItems))
                  .slice(0, 8)}
                placeholder="Search Items"
                onSelect={(option) => {
                  pokemon.evolution_item = option.label;
                }}
                class="w-fit"
              />
            {/if}
            {#if pokemon.evolution_method === "level_up"}
              <Input
                id={pokemon.evolution_method}
                type="number"
                bind:value={pokemon.evolution_level}
                min={1}
                max={100}
              />
            {/if}
            {#if pokemon.evolution_method === "other"}
              <Input
                id={pokemon.evolution_method}
                bind:value={pokemon.evolution_other}
              />
            {/if}
          </div>
          {#if pokemon.evolution_method !== "no_change"}
            <Autocomplete
              open={evolutionSearchOption}
              triggerRef={triggerRefEvolution}
              value={pokemon.evolves_into}
              label="Evolution Search"
              bind:searcher={searchingEvolutions}
              options={evolutionOptions}
              placeholder="Search Pokemon"
              onSelect={(option) => {
                pokemon.evolves_into = option.label;
              }}
              class="w-fit"
            />
          {/if}
        </Card.Content>
      </Card.Root>
    </div>
  </div>
  <div class="mb-5">
    <PokemonLocation
      {pokemonLocations}
      pokemonId={pokemon.id}
      pokemonDexNumber={pokemon.dex_number}
      pokemonName={pokemon.name}
    />
  </div>
{/if}
