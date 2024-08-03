<script lang="ts">
import NumberInput from "$lib/components/NumberInput.svelte";
import SelectInput from "$lib/components/SelectInput.svelte";
import Button from "$lib/components/Button.svelte";
import AutoComplete from "$lib/components/AutoComplete.svelte";
import TextInput from "$lib/components/TextInput.svelte";
import { getToastStore } from "@skeletonlabs/skeleton";
import _ from "lodash";
import { selectedWiki } from "../../store";
import { moveList, type Move } from "../../store/moves";
import { PokemonTypes } from "../../store/pokemon";
import { invoke } from "@tauri-apps/api/tauri";
import { db } from "../../store/db";
import { FALSE, TRUE } from "$lib/utils/CONSTANTS";
    import BaseModal from "$lib/components/BaseModal.svelte";

const toastStore = getToastStore();

let moveSearch: [number, string] = [0, ""];
let newMoveModalOpen: boolean = false;

let newMove: Move = {
  damage_class: "status",
  type: "normal"

} as Move;

let move: Move = {
} as Move;
let originalMoveDetails: Move = {} as Move;

let moveListOptions = $moveList.map(([id, name]) => ({
  label: name,
  value: id,
}));
let pokemonWhoCanLearnMove: number[] = [];

async function gatherPokemonWhoCanLearnMove() {
  return await $db
    .select<{pokemon: number}[]>(
      "SELECT DISTINCT pokemon FROM pokemon_movesets WHERE move = $1;",
      [move.id],
    ).then((res) => {
      pokemonWhoCanLearnMove = res.map((r) => r.pokemon);
    })
}

async function getMove() {
  let retrievedMove = $moveList.find(([_, name]) => name === moveSearch[1]);

  if (!retrievedMove) {
    toastStore.trigger({
      message: "Move not found!",
      background: "variant-filled-error",
    });
    return;
  }

  await $db
    .select<Move[]>("SELECT * FROM moves WHERE id = $1;", [moveSearch[0]])
    .then(async (res) => {
      move = res[0];
      originalMoveDetails = _.cloneDeep(move);
    }).then(async () => {
      await gatherPokemonWhoCanLearnMove().then(() => {
        console.log(pokemonWhoCanLearnMove);
      });
    });
}

async function saveMoveChanges() {
  await $db
    .execute(
      "UPDATE moves SET power=$1,accuracy=$2,pp=$3,type=$4,damage_class=$5,machine_name=$6,is_modified=$7 WHERE id = $8;",
      [
        move.power,
        move.accuracy,
        move.pp,
        move.type,
        move.damage_class,
        move.machine_name,
        move.is_modified,
        move.id,
      ],
    )
    .then(() => {
      originalMoveDetails = _.cloneDeep(move);
      invoke("generate_pokemon_pages_from_list", {
              pokemonIds: pokemonWhoCanLearnMove,
              wikiName: $selectedWiki.name,
              });

      toastStore.trigger({
        message: "Move changes saved!",
        background: "variant-filled-success",
      });
    })
    .catch(() => {
      toastStore.trigger({
        message: "Error saving move changes!",
        background: "variant-filled-error",
      });
    });
}


async function createNewMove() {
  if (newMove.power === 0) {
    newMove.power = null;
  }
  if (newMove.accuracy === 0) {
    newMove.accuracy = null;
  }
  await $db
    .execute(
      `INSERT INTO moves (name, power, accuracy, pp, type, damage_class, machine_name)
        VALUES ($1, $2, $3, $4, $5, $6, $7);`,
      [
        newMove.name.toLowerCase(),
        newMove.power,
        newMove.accuracy,
        newMove.pp,
        newMove.type,
        newMove.damage_class,
        newMove.machine_name,
      ],
    )
    .then((res) => {
      toastStore.trigger({
        message: "Move created!",
        background: "variant-filled-success",
      });
      newMoveModalOpen = false;
      $moveList.push([res.lastInsertId, newMove.name]);

      newMove = {
        damage_class: "status",
        type: "normal"
      } as Move;

      moveListOptions = $moveList.map(([id, name]) => ({
        label: name,
        value: id,
      }));
    })
    .catch(() => {
      toastStore.trigger({
        message: "Error creating new move!",
        background: "variant-filled-error",
      });
    });
}

function setModified(e: any) {
  move.is_modified = e.target?.checked ? TRUE : FALSE;
}

async function convertMovesToSqlite() {
  await invoke("convert_moves_to_sqlite", {
    wikiName: $selectedWiki.name,
  })
    .then(() => {
      toastStore.trigger({
        message: "Moves converted!",
        background: "variant-filled-success",
      });
    })
    .catch((err) => {
      toastStore.trigger({
        message: "Error converting moves!",
        background: "variant-filled-error",
      });
    });
}
</script>

<BaseModal class="w-[30rem]" bind:open={newMoveModalOpen}>
    <h2 class="text-lg font-medium leading-6 text-gray-900">Create New Move</h2>
      <TextInput label="New Move Name" bind:value={newMove.name}
          inputHandler={(e) => {
              newMove.name = e.target.value.toLowerCase().replaceAll(" ", "-");
          }}
            />
      <NumberInput label="Power" bind:value={newMove.power}/>
      <NumberInput label="Accuracy" bind:value={newMove.accuracy}/>
      <NumberInput label="PP" bind:value={newMove.pp}/>
      <SelectInput
              label="Type"
              id="type"
              bind:value={newMove.type}
              options={PokemonTypes.map((type) => {
                if (type === null) {
                  return {
                    label: "None",
                    value: null,
                  };
                }
                return {
                label: _.capitalize(type),
                value: type,
              }})}
            />
      <SelectInput
              label="Damage Class"
              id="damage-class"
              bind:value={newMove.damage_class}
              options={[
                { label: "status", value: "status" },
                { label: "physical", value: "physical" },
                { label: "special", value: "special" },
              ]}
            />
      <TextInput label="Machine Name" bind:value={newMove.machine_name}/>
      <Button
          title="Create Move"
          class="w-32"
          disabled={newMove.name === ""}
          onClick={createNewMove}
        />
    </BaseModal>

<div class="flex flex-row gap-7">
  <AutoComplete
    bind:value={moveSearch[1]}
    placeholder="Search Moves"
    options={moveListOptions}
    popupId="abilities-search"
    onSelection={(e) => {
          moveSearch = [e.detail.value, e.detail.label];
        }}
    showChevron={false}
  />
  <Button
    title="Search"
    onClick={getMove}
    disabled={moveSearch[0] === 0}
    class="mt-2 w-32"
  />
  <Button
    title="Save Changes"
    onClick={saveMoveChanges}
    disabled={_.isEqual(move, originalMoveDetails)}
    class="mt-2 w-32"
  />
  <Button
      title="Create New Move"
      class="ml-auto mr-3 mt-2 w-36"
      onClick={() => newMoveModalOpen = true}
    />
</div>

{#if !_.isEmpty(move)}
  <p class="ml-2 mt-4 text-lg">{_.capitalize(move.name)}</p>
  <div class="ml-2 mt-4">
    <div class="grid grid-cols-2 gap-x-10 gap-y-5 pr-4">
      <NumberInput label="Power" id="power" bind:value={move.power} max={255} />
      <SelectInput
        label="Type"
        id="type"
        bind:value={move.type}
        options={PokemonTypes.map((type) => {
          if (type === null) {
            return {
              label: "None",
              value: null,
            };
          }
          return {
          label: _.capitalize(type),
          value: type,
        }})}
      />
      <NumberInput
        label="Accuracy"
        id="accuracy"
        bind:value={move.accuracy}
        max={100}
      />
      <!-- Highest PP move is 40, but setting it to 100 for future proofing -->
      <NumberInput label="PP" id="pp" bind:value={move.pp} max={100} />
      <SelectInput
        label="Damage Class"
        id="damage-class"
        bind:value={move.damage_class}
        options={[
          { label: "status", value: "status" },
          { label: "physical", value: "physical" },
          { label: "special", value: "special" },
        ]}
      />
    </div>
  </div>
  {#if !move.is_new}
    <label class="block text-sm font-medium leading-6 text-gray-900">
      <input
        type="checkbox"
        checked={Boolean(move.is_modified)}
        on:change={setModified}
        class="text-sm font-medium leading-6 text-gray-900"
      />
      Mark Move as Modified
    </label>
  {/if}
{/if}
<!-- <Button title="Convert Moves to SQLite" onClick={convertMovesToSqlite} /> -->
