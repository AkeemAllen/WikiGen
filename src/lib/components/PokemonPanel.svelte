<script lang="ts">
  import { Tab, TabGroup, getToastStore } from "@skeletonlabs/skeleton";
  import { BaseDirectory, readBinaryFile } from "@tauri-apps/api/fs";
  import { selectedWiki } from "../../store";
  import { routes, type WildEncounter } from "../../store/gameRoutes";
  import {
    pokemonList,
    type Pokemon,
    type PokemonMove,
  } from "../../store/pokemon";
  import PokemonDetailsTab from "./PokemonDetailsTab.svelte";
  import PokemonMovesTab from "./PokemonMovesTab.svelte";
  import { invoke } from "@tauri-apps/api";
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

  const toastStore = getToastStore();

  let pokemonSearch: [number, string] = [0, ""];

  let pokemonNameInput: HTMLInputElement;

  let pokemon = {} as Pokemon;
  let originalPokemonDetails: Pokemon = {} as Pokemon;
  let pokemonMoveset: PokemonMove[] = [];
  let pokemonSprite: string = "";

  let pokemonLocations: WildEncounter[] = [];

  let tabSet: number = 0;

  let pokemonListOptions = $pokemonList.map(([id, _, name]) => ({
    label: capitalizeWords(name),
    value: id,
  }));

  async function generatePokemonPage() {
    await invoke("generate_pokemon_pages_from_list", {
      wikiName: $selectedWiki.name,
      pokemonIds: [pokemon.id],
    }).then(() => {
      toastStore.trigger({
        message: "Pokemon page regenerated!",
        background: "variant-filled-success",
      });
    });
  }

  async function getPokemon() {
    let retrievedPokemon = $pokemonList.find(
      ([__, ___, name]) =>
        name === pokemonSearch[1].toLowerCase().split(" ").join("-"),
    );

    if (!retrievedPokemon) {
      toastStore.trigger({
        message: "Pokemon not found!",
        background: "variant-filled-error",
      });
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
            toastStore.trigger({
              message: `Error loading Pokemon moveset!: \n ${err}`,
              background: "variant-filled-error",
            });
          });

        // Reading in image separately
        pokemonSprite = await readBinaryFile(
          `${$selectedWiki.name}/dist/docs/img/pokemon/${res[0].name}.png`,
          { dir: BaseDirectory.AppData },
        )
          .then((res) => {
            const blob = new Blob([res], { type: "image/png" });
            return URL.createObjectURL(blob);
          })
          .catch((err) => {
            console.log(err);
            if (err.includes("No such file or directory")) {
              return "404";
            }
            return "Error loading image";
          });
        return res[0];
      })
      .then((res) => {
        originalPokemonDetails = cloneDeep(res);
        return res;
      })
      .then((res) => {
        // Gather location
        for (const [_, properties] of Object.entries($routes.routes)) {
          for (const [_, encounters] of Object.entries(
            properties.wild_encounters,
          )) {
            for (const encounter of encounters) {
              if (encounter.name !== res.name) continue;
              pokemonLocations.push(cloneDeep(encounter));
            }
          }
        }
      })
      .catch((err) => {
        console.log(err);
        toastStore.trigger({
          message: `Error loading Pokemon!: \n ${err}`,
          background: "variant-filled-error",
        });
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
          dex_number = $1,
          name = $2,
          types = $3,
          ability_1 = $4,
          ability_2 = $5,
          hp = $6,
          attack = $7,
          defense = $8,
          sp_attack = $9,
          sp_defense = $10,
          speed = $11,
          evolution_method = $12,
          evolution_item = $13,
          evolution_level = $14,
          evolution_other = $15,
          evolved_pokemon = $16,
          render = $17
        WHERE id = $18;`,
        [
          pokemon.dex_number,
          pokemon.name,
          pokemon.types,
          pokemon.ability_1?.toLowerCase().split(" ").join("-"),
          pokemon.ability_2?.toLowerCase().split(" ").join("-"),
          pokemon.hp,
          pokemon.attack,
          pokemon.defense,
          pokemon.sp_attack,
          pokemon.sp_defense,
          pokemon.speed,
          pokemon.evolution_method,
          pokemon.evolution_item,
          pokemon.evolution_level,
          pokemon.evolution_other,
          pokemon.evolved_pokemon,
          pokemon.render,
          pokemon.id,
        ],
      )
      .then(() => {
        originalPokemonDetails = cloneDeep(pokemon);
        toastStore.trigger({
          message: "Pokemon changes saved!",
          background: "variant-filled-success",
        });
        generatePokemonPage();
      })
      .catch((err) => {
        toastStore.trigger({
          message: `Error saving pokemon changes!: ${err}`,
          background: "variant-filled-error",
        });
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

  async function convertPokemonToSqlite() {
    await invoke("convert_pokemon_to_sqlite", {
      wikiName: $selectedWiki.name,
    })
      .then(() => {
        toastStore.trigger({
          message: "Pokemon converted!",
          background: "variant-filled-success",
        });
      })
      .catch((err) => {
        toastStore.trigger({
          message: "Error converting pokemon!",
          background: "variant-filled-error",
        });
      });
  }

  async function convertMovesetsToSqlite() {
    await invoke("convert_pokemon_movesets_to_sqlite", {
      wikiName: $selectedWiki.name,
    })
      .then(() => {
        toastStore.trigger({
          message: "Moveset converted!",
          background: "variant-filled-success",
        });
      })
      .catch((err) => {
        toastStore.trigger({
          message: "Error converting moveset!",
          background: "variant-filled-error",
        });
      });
  }
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
    onClick={generatePokemonPage}
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
    <svelte:fragment slot="panel">
      {#if tabSet === 0}
        <PokemonDetailsTab bind:pokemon />
      {/if}
      {#if tabSet === 1}
        <PokemonMovesTab
          bind:moveset={pokemonMoveset}
          bind:pokemonId={pokemon.id}
          {generatePokemonPage}
        />
      {/if}
      {#if tabSet === 2}
        <PokemonLocationTab
          {pokemonLocations}
          pokemonId={pokemon.id}
          pokemonName={pokemon.name}
        />
      {/if}
    </svelte:fragment>
  </TabGroup>
{/if}
<!-- <Button title="Convert Pokemon to SQLite" onClick={convertPokemonToSqlite} /> -->
<!-- <Button title="Update Sprite Names" onClick={updateSpriteNames} /> -->
<!-- <Button title="Convert Movesets to SQLite" onClick={convertMovesetsToSqlite} /> -->

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
