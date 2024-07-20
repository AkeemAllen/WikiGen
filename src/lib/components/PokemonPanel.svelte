<script lang="ts">
import { Tab, TabGroup, getToastStore } from "@skeletonlabs/skeleton";
import {
  BaseDirectory,
  readBinaryFile,
  writeBinaryFile,
} from "@tauri-apps/api/fs";
import _ from "lodash";
import { selectedWiki } from "../../store";
import {
  pokemonList,
  type Pokemon,
  type PokemonMove,
  type EvolutionChange,
} from "../../store/pokemon";
import PokemonDetailsTab from "./PokemonDetailsTab.svelte";
import PokemonMovesTab from "./PokemonMovesTab.svelte";
import { invoke } from "@tauri-apps/api";
import { base64ToArray } from "$lib/utils";
import { shortcut } from "@svelte-put/shortcut";
import Button from "./Button.svelte";
import AutoComplete from "./AutoComplete.svelte";
import { db } from "../../store/db";

const toastStore = getToastStore();

let pokemonSearch: [number, string] = [0, ""];

let pokemonNameInput: HTMLInputElement;

let pokemon = {} as Pokemon;
let originalPokemonDetails: Pokemon = {} as Pokemon;
let pokemonMoveset: PokemonMove[] = [];
let pokemonSprite: string = "";

let tabSet: number = 0;

let pokemonListOptions = $pokemonList.map(([id, name]) => ({
  label: _.capitalize(name),
  value: id,
}));

async function getPokemon() {
  let retrievedPokemon = $pokemonList.find(
    ([__, name]) => name === pokemonSearch[1].toLowerCase(),
  );

  if (!retrievedPokemon) {
    toastStore.trigger({
      message: "Pokemon not found!",
      background: "variant-filled-error",
    });
    return;
  }

  await $db
    .select<Pokemon[]>(
      `
      SELECT *
      FROM pokemon
      WHERE pokemon.id = $1;`,
      [pokemonSearch[0]],
    )
    .then(async (res) => {
      // Using temp_pokemon to gather pokemon data before moveing it to pokemon variable so that
      // the PokemonDetailsTab component doesn't load too early.
      let temp_pokemon = res[0];

      // Gather moveset
      await $db
        .select<PokemonMove[]>(
          `SELECT moves.id as id, moves.name as name, learn_method, level_learned FROM pokemon_movesets
            INNER JOIN moves on moves.id = pokemon_movesets.move
            WHERE pokemon = $1;`,
          [temp_pokemon.id],
        )
        .then((res) => {
          pokemonMoveset = res;
        })
        .catch((err) => {
          toastStore.trigger({
            message: "Error loading Pokemon moveset!",
            background: "variant-filled-error",
          });
        });

      // Gather abilities
      await $db
        .select<{ name: string }[]>(
          `SELECT name FROM abilities
            WHERE id = $1 OR id = $2;`,
          [temp_pokemon.ability_1, temp_pokemon.ability_2],
        )
        .then((res) => {
          if (res.length > 1) {
            temp_pokemon.ability_2_name = _.capitalize(res[0].name);
            temp_pokemon.ability_1_name = _.capitalize(res[1].name);
            return;
          }
          temp_pokemon.ability_1_name = _.capitalize(res[0].name);
          temp_pokemon.ability_2_name = "";
        })
        .catch((err) => {
          toastStore.trigger({
            message: "Error loading Pokemon abilities!",
            background: "variant-filled-error",
          });
        });

      // Gather evolution changes
      await $db
        .select<EvolutionChange[]>(
          `SELECT
            *,
            pokemon.name as evolved_pokemon_name,
            pokemon.id as evolved_pokemon_id
          FROM evolution_changes
          INNER JOIN pokemon ON pokemon.id = evolution_changes.evolved_pokemon
          WHERE evolution_changes.id = $1;`,
          [temp_pokemon.evolution_change],
        )
        .then((res) => {
          console.log(res);
          if (res.length === 0) {
            temp_pokemon.evolution_change_object = {
              id: 0,
              method: "no_change",
              evolved_pokemon: {
                id: 0,
                name: "",
              },
              item: null,
              level: null,
              other: null,
            };
          } else {
            temp_pokemon.evolution_change_object = {
              ...res[0],
              evolved_pokemon: {
                // @ts-ignore
                id: res[0].evolved_pokemon_id,
                // @ts-ignore
                name: _.capitalize(res[0].evolved_pokemon_name),
              },
            };
          }
        })
        .catch((err) => {
          console.log(err);
          console.log(temp_pokemon);
          toastStore.trigger({
            message: "Error loading Evolution change!",
            background: "variant-filled-error",
          });
        });

      // Reading in image separately
      pokemonSprite = await readBinaryFile(
        `${$selectedWiki.name}/dist/docs/img/pokemon/${temp_pokemon.name}.png`,
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
      return temp_pokemon;
    })
    .then((res) => {
      pokemon = res;
      originalPokemonDetails = _.cloneDeep(pokemon);
    })
    .catch((err) => {
      toastStore.trigger({
        message: "Error loading Pokemon!",
        background: "variant-filled-error",
      });
      return {} as Pokemon;
    });
}

async function savePokemonChanges() {
  if (_.isEqual(pokemon, originalPokemonDetails)) {
    return;
  }

  let evolutionChangeId = null;

  if (
    !_.isEqual(
      pokemon.evolution_change_object,
      originalPokemonDetails.evolution_change_object,
    )
  ) {
    evolutionChangeId = await updateEvolutionChange();
  }

  await $db
    .execute(
      `UPDATE pokemon SET
          dex_number = $1,
          name = $2,
          type_1 = $3,
          type_2 = $4,
          ability_1 = $5,
          ability_2 = $6,
          hp = $7,
          attack = $8,
          defense = $9,
          sp_attack = $10,
          sp_defense = $11,
          speed = $12,
          evolution_change = $13
        WHERE id = $14;`,
      [
        pokemon.dex_number,
        pokemon.name,
        pokemon.type_1,
        pokemon.type_2,
        pokemon.ability_1,
        pokemon.ability_2,
        pokemon.hp,
        pokemon.attack,
        pokemon.defense,
        pokemon.sp_attack,
        pokemon.sp_defense,
        pokemon.speed,
        evolutionChangeId,
        pokemon.id,
      ],
    )
    .then(() => {
      originalPokemonDetails = _.cloneDeep(pokemon);
      toastStore.trigger({
        message: "Pokemon changes saved!",
        background: "variant-filled-success",
      });
    })
    .catch((err) => {
      console.log(err);
      toastStore.trigger({
        message: "Error saving pokemon changes!",
        background: "variant-filled-error",
      });
    });
}

async function updateEvolutionChange(): Promise<number | null> {
  if (pokemon.evolution_change_object.method === "no_change") {
    await $db
      .execute(
        `UPDATE pokemon SET
          evolution_change = NULL
        WHERE id = $1;`,
        [pokemon.id],
      )
      .then(() => {
        originalPokemonDetails.evolution_change_object = _.cloneDeep(
          pokemon.evolution_change_object,
        );
        $db
          .execute(`DELETE FROM evolution_changes WHERE id=$1`, [
            pokemon.evolution_change,
          ])
          .then(() => {
            pokemon.evolution_change = null;
            pokemon.evolution_change_object = {
              id: 0,
              method: "no_change",
              evolved_pokemon: {
                id: 0,
                name: "",
              },
              item: null,
              level: null,
              other: null,
            };
          })
          .catch(() => {
            console.log("Error deleting evolution change from table");
          });
        toastStore.trigger({
          message: "Evolution change removed!",
          background: "variant-filled-success",
        });
      })
      .catch(() => {
        toastStore.trigger({
          message: "Error removing evolution change!",
          background: "variant-filled-error",
        });
      });
    return null;
  }

  if (pokemon.evolution_change === null) {
    let evolved_pokemon_id = null;
    if (pokemon.evolution_change_object.evolved_pokemon.id !== 0) {
      evolved_pokemon_id = pokemon.evolution_change_object.evolved_pokemon.id;
    }
    let evolution_change_id = null;
    await $db
      .execute(
        `INSERT INTO evolution_changes (
            method,
            level,
            item,
            other,
            evolved_pokemon
          ) VALUES ($1, $2, $3, $4, $5)`,
        [
          pokemon.evolution_change_object.method,
          pokemon.evolution_change_object.level,
          pokemon.evolution_change_object.item,
          pokemon.evolution_change_object.other,
          evolved_pokemon_id,
        ],
      )
      .then((res) => {
        pokemon.evolution_change = res.lastInsertId;
        pokemon.evolution_change_object.id = res.lastInsertId;
        evolution_change_id = res.lastInsertId;
        originalPokemonDetails = _.cloneDeep(pokemon);
        toastStore.trigger({
          message: "Evolution Change Added",
          background: "variant-filled-success",
        });
        return res.lastInsertId;
      })
      .catch((err) => {
        console.log(err);
        console.log("Evolution Change", pokemon.evolution_change);
        console.log("Evolution Change Object", pokemon.evolution_change_object);
      });
    return evolution_change_id;
  }

  if (pokemon.evolution_change_object.method === "item") {
    pokemon.evolution_change_object.level = null;
    pokemon.evolution_change_object.other = null;
  } else if (pokemon.evolution_change_object.method === "level_up") {
    pokemon.evolution_change_object.item = null;
    pokemon.evolution_change_object.other = null;
  } else if (pokemon.evolution_change_object.method === "other") {
    pokemon.evolution_change_object.item = null;
    pokemon.evolution_change_object.level = null;
  }

  await $db
    .execute(
      `UPDATE evolution_changes SET
        method=$1,
        level=$2,
        item=$3,
        other=$4,
        evolved_pokemon=$5
      WHERE id=$6`,
      [
        pokemon.evolution_change_object.method,
        pokemon.evolution_change_object.level,
        pokemon.evolution_change_object.item,
        pokemon.evolution_change_object.other,
        pokemon.evolution_change_object.evolved_pokemon.id,
        pokemon.evolution_change,
      ],
    )
    .then(() => {
      toastStore.trigger({
        message: "Evolution Change Updated",
        background: "variant-filled-success",
      });
      originalPokemonDetails = _.cloneDeep(pokemon);
    });
  return pokemon.evolution_change;
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

// async function updateSpriteNames() {
//   for (const entry of $pokemonList) {
//     let id: string | number = entry[1];

//     if (id < 10) {
//       id = `00${id}`;
//     } else if (id < 100) {
//       id = `0${id}`;
//     }

//     let sprite = await readBinaryFile(
//       `${$selectedWiki.name}/dist/docs/img/pokemon/${id}.png`,
//       { dir: BaseDirectory.AppData },
//     )
//       .then((res) => {
//         const blob = new Blob([res], { type: "image/png" });
//         return URL.createObjectURL(blob);
//       })
//       .catch((err) => {
//         console.log(err);
//         if (err.includes("No such file or directory")) {
//           return "404";
//         }
//         return "Error loading image";
//       });
//     if (sprite === "404" || sprite === "Error loading image") {
//       continue;
//     }

//     let reader = new FileReader();
//     reader.onloadend = (e) => {
//       let imageBytes = base64ToArray(
//         (e.target?.result as string).replace("data:image/png;base64,", ""),
//         "image/png",
//       );
//       writeBinaryFile(
//         `${$selectedWiki.name}/dist/docs/img/temp_pokemon/${entry[0]}.png`,
//         imageBytes,
//         { dir: BaseDirectory.AppData },
//       );
//     };
//     reader.readAsDataURL(await fetch(sprite).then((res) => res.blob()));
//   }
// }
</script>

<div class="flex flex-row gap-7">
  <AutoComplete
    bind:value={pokemonSearch[1]}
    placeholder="Search Pokemon"
    options={pokemonListOptions}
    popupId="pokemon-search"
    onSelection={(e) => {
      pokemonSearch = [e.detail.value, e.detail.label]
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
    disabled={_.isEqual(pokemon, originalPokemonDetails)}
    class="mt-2 w-32"
  />
</div>

{#if !_.isEmpty(pokemon)}
  {#if pokemonSprite === "404"}
    <p>No sprite found for {pokemon.name}</p>
  {:else}
    <img src={pokemonSprite} alt={pokemon.name} width="100" />
  {/if}
  <TabGroup>
    <Tab bind:group={tabSet} name="pokemon-details" value={0} class="text-sm"
      >Details</Tab
    >
    <Tab bind:group={tabSet} name="pokemon-moves" value={1} class="text-sm"
      >Moves</Tab
    >
    <svelte:fragment slot="panel">
      {#if tabSet === 0}
        <PokemonDetailsTab bind:pokemon={pokemon} />
      {/if}
      {#if tabSet === 1}
        <PokemonMovesTab
          bind:moveset={pokemonMoveset}
          bind:pokemonId={pokemon.id}
        />
      {/if}
    </svelte:fragment>
  </TabGroup>
{/if}
<!-- <Button title="Convert Pokemon to SQLite" onClick={convertPokemonToSqlite} />
<Button title="Update Sprite Names" onClick={updateSpriteNames} /> -->
<!-- <Button title="Convert Movesets to SQLite" onClick={convertMovesetsToSqlite} /> -->

<svelte:window
  use:shortcut={{
    trigger: {
      key: 'k',
      modifier:["ctrl", "meta"],
      callback: () => pokemonNameInput.focus(),
    },
  }}
  use:shortcut={{
    trigger: {
      key: 'm',
      modifier:"ctrl",
      callback: () => { tabSet = 1},
    },
  }}
  use:shortcut={{
    trigger: {
      key: 'Enter',
      modifier: ["ctrl","meta"],
      callback: () => savePokemonChanges(),
    },
  }}
/>
