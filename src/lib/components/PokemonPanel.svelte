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
  import Button from "./Button.svelte";
  import AutoComplete from "./AutoComplete.svelte";
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

  let pokemonSearch: [number, string] = $state([0, ""]);
  let pokemon = $state({} as Pokemon);
  let originalPokemonDetails: Pokemon = $state({} as Pokemon);
  let pokemonMoveset: PokemonMove[] = $state([]);
  let pokemonLocations: WildEncounter[] = $state([]);
  let pokemonSprite: string = $state("");
  let pokemonNameInput: HTMLInputElement = $state(
    document.createElement("input"),
  );

  let tabSet: number = $state(0);
  let pokemonListOptions = $pokemonList.map(([id, _, name]) => ({
    label: capitalizeWords(name),
    value: id,
  }));

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
      pokemon.evolved_pokemon = null;
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
          evolved_pokemon = $5,
          render = "${pokemon.render}",
          ability_1 = $6,
          ability_2 = $7,
          hidden_ability = $8
        WHERE id = ${pokemon.id};`,
        [
          pokemon.evolution_method,
          pokemon.evolution_item,
          pokemon.evolution_level,
          pokemon.evolution_other,
          pokemon.evolved_pokemon,
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

  // function nextPokemon() {
  //   if (pokemonId === 1025) {
  //     toastStore.trigger({
  //       message: "No more Pokemon",
  //       timeout: 3000,
  //       background: "variant-filled-error",
  //     });
  //     return;
  //   }
  //   setPokemonDetails(pokemonId + 1);
  // }

  // function prevPokemon() {
  //   if (pokemonId === 1) {
  //     toastStore.trigger({
  //       message: "No more Pokemon",
  //       timeout: 3000,
  //       background: "variant-filled-error",
  //     });
  //     return;
  //   }
  //   setPokemonDetails(pokemonId - 1);
  // }
</script>

<div class="flex flex-row gap-7">
  <AutoComplete
    bind:value={pokemonSearch[1]}
    placeholder="Search Pokemon"
    options={pokemonListOptions}
    popupId="pokemon-search"
    onSelection={(e) => {
      pokemonSearch = [e.detail.value, e.detail.label];
    }}
    bind:inputNode={pokemonNameInput}
    showChevron={false}
  />
  <Button
    title="Search"
    onClick={getPokemon}
    disabled={pokemonSearch[0] === 0}
    class="mt-2 w-32"
  />
  <Button
    title="Save Changes"
    onClick={savePokemonChanges}
    disabled={isEqual(pokemon, originalPokemonDetails)}
    class="mt-2 w-32"
  />
  <Button
    title="Generate Page"
    onClick={() => generatePage()}
    disabled={objectIsEmpty(pokemon)}
    class="mt-2 w-32"
  />
</div>

{#if !objectIsEmpty(pokemon)}
  {#if pokemonSprite === "404"}
    <p>No sprite found for {pokemon.name}</p>
  {:else}
    <img src={pokemonSprite} alt={pokemon.name} width="100" />
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
    <!-- {#snippet panel()}
    {/snippet} -->
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
