<script lang="ts">
import {
  Tab,
  TabGroup,
  getToastStore,
  type AutocompleteOption,
} from "@skeletonlabs/skeleton";
import {
  BaseDirectory,
  readBinaryFile,
  writeBinaryFile,
  writeTextFile,
} from "@tauri-apps/api/fs";
import _ from "lodash";
import { selectedWiki } from "../../store";
import {
  pokemon,
  pokemonList,
  dbPokemonList,
  modifiedPokemon,
  type PokemonDetails,
  type DBPokemon,
} from "../../store/pokemon";
import PokemonDetailsTab from "./PokemonDetailsTab.svelte";
import PokemonMovesTab from "./PokemonMovesTab.svelte";
import { invoke } from "@tauri-apps/api";
import {
  base64ToArray,
  convertToTitle,
  extractPokemonRange,
  getShardToWrite,
} from "$lib/utils";
import { shortcut } from "@svelte-put/shortcut";
import Button from "./Button.svelte";
import AutoComplete from "./AutoComplete.svelte";
import { updatePokemonModifications } from "$lib/utils/modificationHelpers";
import { db } from "../../store/db";

const toastStore = getToastStore();

let pokemonSearch: [number, string] = [0, ""];

let pokemonName: string = "";
let formName: string = "";
let pokemonId: number = 0;
let pokemonDetails: PokemonDetails = {} as PokemonDetails;
let originalPokemonDetails: PokemonDetails = {} as PokemonDetails;
let pokemonNameInput: HTMLInputElement;

let selectedPokemon = {} as DBPokemon;
let originalSelectedPokemon: DBPokemon = {} as DBPokemon;
let pokemonSprite: string = "";

let tabSet: number = 0;

let pokemonListOptions = $dbPokemonList.map(([id, name]) => ({
  label: name,
  value: id,
}));

async function getPokemon() {
  let retrievedPokemon = $dbPokemonList.find(
    ([_, name]) => name === pokemonSearch[1],
  );

  if (!retrievedPokemon) {
    toastStore.trigger({
      message: "Pokemon not found!",
      background: "variant-filled-error",
    });
    return;
  }

  await $db
    .select<DBPokemon[]>("SELECT * FROM pokemon WHERE id = $1;", [
      pokemonSearch[0],
    ])
    .then(async (res) => {
      selectedPokemon = res[0];
      originalSelectedPokemon = _.cloneDeep(selectedPokemon);

      // Reading in image separately
      pokemonSprite = await readBinaryFile(
        `${$selectedWiki.name}/dist/docs/img/pokemon/${selectedPokemon.name}.png`,
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
    })
    .catch((err) => {
      console.log(err);
      toastStore.trigger({
        message: "Error loading Pokemon!",
        background: "variant-filled-error",
      });
    });
}

async function savePokemonChanges() {
  if (_.isEqual(selectedPokemon, originalSelectedPokemon)) {
    return;
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
          speed = $12
        WHERE id = $13;`,
      [
        selectedPokemon.dex_number,
        selectedPokemon.name,
        selectedPokemon.type_1,
        selectedPokemon.type_2,
        selectedPokemon.ability_1,
        selectedPokemon.ability_2,
        selectedPokemon.hp,
        selectedPokemon.attack,
        selectedPokemon.defense,
        selectedPokemon.sp_attack,
        selectedPokemon.sp_defense,
        selectedPokemon.speed,
        selectedPokemon.id,
      ],
    )
    .then(() => {
      originalSelectedPokemon = _.cloneDeep(selectedPokemon);
      toastStore.trigger({
        message: "Pokemon changes saved!",
        background: "variant-filled-success",
      });
    })
    .catch(() => {
      toastStore.trigger({
        message: "Error saving pokemon changes!",
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

async function updateSpriteNames() {
  for (const entry of $pokemonList) {
    let id: string | number = entry[1];

    if (id < 10) {
      id = `00${id}`;
    } else if (id < 100) {
      id = `0${id}`;
    }

    let sprite = await readBinaryFile(
      `${$selectedWiki.name}/dist/docs/img/pokemon/${id}.png`,
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
    if (sprite === "404" || sprite === "Error loading image") {
      continue;
    }

    let reader = new FileReader();
    reader.onloadend = (e) => {
      let imageBytes = base64ToArray(
        (e.target?.result as string).replace("data:image/png;base64,", ""),
        "image/png",
      );
      writeBinaryFile(
        `${$selectedWiki.name}/dist/docs/img/temp_pokemon/${entry[0]}.png`,
        imageBytes,
        { dir: BaseDirectory.AppData },
      );
    };
    reader.readAsDataURL(await fetch(sprite).then((res) => res.blob()));
  }
}
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
    disabled={_.isEqual(selectedPokemon, originalSelectedPokemon)}
    class="mt-2 w-32"
  />
</div>

{#if !_.isEmpty(selectedPokemon)}
  {#if pokemonSprite === "404"}
    <p>No sprite found for {selectedPokemon.name}</p>
  {:else}
    <img src={pokemonSprite} alt={selectedPokemon.name} width="100" />
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
        <PokemonDetailsTab bind:pokemon={selectedPokemon} />
      {/if}
      {#if tabSet === 1}
        <!-- <PokemonMovesTab
          bind:pokemonDetails={pokemonDetails}
          savePokemonChanges={savePokemonChanges}
        /> -->
      {/if}
    </svelte:fragment>
  </TabGroup>
{/if}
<!-- <Button title="Convert Pokemon to SQLite" onClick={convertPokemonToSqlite} />
<Button title="Update Sprite Names" onClick={updateSpriteNames} /> -->

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
