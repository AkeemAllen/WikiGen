<script lang="ts">
  import NumberInput from "$lib/components/NumberInput.svelte";
  import SelectInput from "$lib/components/SelectInput.svelte";
  import Button from "$lib/components/Button.svelte";
  import AutoComplete from "$lib/components/AutoComplete.svelte";
  import TextInput from "$lib/components/TextInput.svelte";
  import Pagination from "$lib/components/Pagination.svelte";
  import ThSort from "$lib/components/ThSort.svelte";
  import { getToastStore, Tab, TabGroup } from "@skeletonlabs/skeleton";
  import _ from "lodash";
  import { selectedWiki } from "../../store";
  import { moveList, type Move } from "../../store/moves";
  import { PokemonTypes, pokemonList } from "../../store/pokemon";
  import { invoke } from "@tauri-apps/api/tauri";
  import { db } from "../../store/db";
  import { FALSE, TRUE } from "$lib/utils/CONSTANTS";
  import BaseModal from "$lib/components/BaseModal.svelte";
  import { DataHandler } from "@vincjo/datatables";
  import { IconEdit, IconTrash } from "@tabler/icons-svelte";
  import { BaseDirectory, readBinaryFile } from "@tauri-apps/api/fs";
  import MultiSelect from "svelte-multiselect";

  const toastStore = getToastStore();
  let tabSet: number = 0;

  let moveSearch: [number, string] = [0, ""];
  let pokemonSearch: string = "";

  let newMoveModalOpen: boolean = false;
  let newMove: Move = {
    damage_class: "status",
    type: "normal",
  } as Move;
  let move: Move = {} as Move;
  let originalMoveDetails: Move = {} as Move;

  let addMoveModalOpen: boolean = false;
  let editMoveModalOpen: boolean = false;
  let newMoveLearner: MoveLearner = {
    pokemonId: 0,
    name: "",
    learn_method: "",
    level_learned: 0,
  };

  let moveListOptions = $moveList.map(([id, name]) => ({
    label: name,
    value: id,
  }));

  let pokemonListOptions = $pokemonList.map(([id, name]) => ({
    label: _.capitalize(name),
    value: id,
  }));

  type MoveLearner = {
    pokemonId: number;
    name: string;
    learn_method: string;
    level_learned: number;
  };
  let pokemonWhoCanLearnMove: MoveLearner[] = [];
  let newLearnMethods: string[] = [];

  const rowsPerPageOptions = [
    { label: "5", value: 5 },
    { label: "10", value: 10 },
    { label: "20", value: 20 },
    { label: "50", value: 50 },
    { label: "100", value: 100 },
  ];

  $: handler = new DataHandler(pokemonWhoCanLearnMove, {
    rowsPerPage: 5,
  });
  $: rows = handler.getRows();
  $: rowsPerPage = handler.getRowsPerPage();

  async function generatePokemonPage(pokemonId: number) {
    await invoke("generate_pokemon_pages_from_list", {
      wikiName: $selectedWiki.name,
      pokemonIds: [pokemonId],
    }).then(() => {
      toastStore.trigger({
        message: "Pokemon page regenerated!",
        background: "variant-filled-success",
      });
    });
  }
  async function gatherPokemonWhoCanLearnMove() {
    return await $db
      .select<MoveLearner[]>(
        `SELECT DISTINCT pokemon as pokemonId, p.name, learn_method, level_learned
      FROM pokemon_movesets
      INNER JOIN pokemon p on p.id = pokemon
      WHERE move=$1;`,
        [move.id],
      )
      .then((res) => {
        pokemonWhoCanLearnMove = res;
      });
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
      })
      .then(async () => {
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
          pokemonIds: pokemonWhoCanLearnMove.map((p) => p.pokemonId),
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
          type: "normal",
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

  async function deleteMoveFromPokemon(pokemonId: number) {
    await $db
      .execute(
        `DELETE FROM pokemon_movesets WHERE pokemon = $1 AND move = $2`,
        [pokemonId, move.id],
      )
      .then(() => {
        const updatedMoveLearners = pokemonWhoCanLearnMove.filter(
          (p) => p.pokemonId !== pokemonId,
        );
        pokemonWhoCanLearnMove = updatedMoveLearners;
        toastStore.trigger({
          message: "Move deleted successfully",
          background: "variant-filled-success",
        });
      })
      .then(() => generatePokemonPage(pokemonId));
  }

  async function addMoveToPokemon() {
    await $db
      .execute(
        `INSERT INTO pokemon_movesets (pokemon, move, learn_method, level_learned)
    VALUES ($1, $2, $3, $4)`,
        [
          newMoveLearner.pokemonId,
          move.id,
          newMoveLearner.learn_method,
          newMoveLearner.level_learned,
        ],
      )
      .then(() => {
        pokemonWhoCanLearnMove = [newMoveLearner, ...pokemonWhoCanLearnMove];
        toastStore.trigger({
          message: "Move added successfully",
          background: "variant-filled-success",
        });
      })
      .then(() => generatePokemonPage(newMoveLearner.pokemonId))
      .then(() => {
        newMoveLearner = {
          pokemonId: 0,
          name: "",
          learn_method: "",
          level_learned: 0,
        };
      });
  }

  async function editMove() {
    await $db
      .execute(
        `UPDATE pokemon_movesets
    SET learn_method = $1, level_learned = $2
    WHERE pokemon = $3 AND move = $4`,
        [
          newMoveLearner.learn_method,
          newMoveLearner.level_learned,
          newMoveLearner.pokemonId,
          move.id,
        ],
      )
      .then(() => {
        pokemonWhoCanLearnMove = pokemonWhoCanLearnMove.map((pokemon) => {
          if (pokemon.pokemonId === newMoveLearner.pokemonId) {
            return {
              ...pokemon,
              learn_method: newMoveLearner.learn_method,
              level_learned: newMoveLearner.level_learned,
            };
          }
          return pokemon;
        });
        editMoveModalOpen = false;
        toastStore.trigger({
          message: "Move updated successfully",
          background: "variant-filled-success",
        });
      })
      .then(() => generatePokemonPage(newMoveLearner.pokemonId))
      .then(() => {
        newMoveLearner = {
          pokemonId: 0,
          name: "",
          learn_method: "",
          level_learned: 0,
        };
        newLearnMethods = [];
      });
  }

  async function getSpriteImage(pokemonName: string): Promise<string> {
    let sprite = "";
    await readBinaryFile(
      `${$selectedWiki.name}/dist/docs/img/pokemon/${pokemonName}.png`,
      { dir: BaseDirectory.AppData },
    )
      .then((res) => {
        const blob = new Blob([res], { type: "image/png" });
        sprite = URL.createObjectURL(blob);
      })
      .catch((err) => {
        console.log(err);
        if (err.includes("No such file or directory")) {
          sprite = "Image Not Found";
        }
        sprite = "Error loading image";
      });
    return sprite;
  }
</script>

<BaseModal class="w-[30rem]" bind:open={newMoveModalOpen}>
  <h2 class="text-lg font-medium leading-6 text-gray-900">Create New Move</h2>
  <TextInput
    label="New Move Name"
    bind:value={newMove.name}
    inputHandler={(e) => {
      newMove.name = e.target.value.toLowerCase().replaceAll(" ", "-");
    }}
  />
  <NumberInput label="Power" bind:value={newMove.power} />
  <NumberInput label="Accuracy" bind:value={newMove.accuracy} />
  <NumberInput label="PP" bind:value={newMove.pp} />
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
      };
    })}
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
  <TextInput label="Machine Name" bind:value={newMove.machine_name} />
  <Button
    title="Create Move"
    class="w-32"
    disabled={newMove.name === ""}
    onClick={createNewMove}
  />
</BaseModal>

<BaseModal class="w-[20rem]" bind:open={addMoveModalOpen}>
  <h2 class="text-lg font-medium leading-6 text-gray-900">Move Learner</h2>
  <AutoComplete
    bind:value={newMoveLearner.name}
    label="Pokemon Name"
    popupId="newMovePopup"
    class="z-10 w-full text-sm"
    options={pokemonListOptions.filter((p) => {
      return !pokemonWhoCanLearnMove.some((_p) => _p.pokemonId === p.value);
    })}
    onSelection={(event) => {
      newMoveLearner.pokemonId = event.detail.value;
      newMoveLearner.name = event.detail.label;
    }}
  />
  <div>
    <label
      for="methods"
      class="mb-2 block text-sm font-medium leading-6 text-gray-900"
      >Learn Method(s)</label
    >
    <MultiSelect
      id="methods"
      bind:selected={newLearnMethods}
      options={["level-up", "machine"]}
      on:change={(e) => {
        newMoveLearner.learn_method = newLearnMethods.join(",");
      }}
      style="height: 36px; border-color: rgb(209 213 219); border-radius: 0.375rem; box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05); font-size: 0.875rem;"
    />
  </div>
  {#if newLearnMethods.includes("level-up")}
    <NumberInput
      id="level-learned"
      bind:value={newMoveLearner.level_learned}
      label="Learn Level"
      class="w-full"
    />
  {/if}
  <Button
    title="Add Move"
    disabled={newMoveLearner.name === "" || newMoveLearner.learn_method === ""}
    onClick={() => addMoveToPokemon()}
  />
</BaseModal>

<BaseModal bind:open={editMoveModalOpen}>
  <h2 class="text-lg font-medium leading-6 text-gray-900">Edit Move</h2>
  <TextInput value={newMoveLearner.name} disabled={true} />
  <div>
    <label
      for="methods"
      class="mb-2 block text-sm font-medium leading-6 text-gray-900"
      >Learn Method(s)</label
    >
    <MultiSelect
      id="methods"
      bind:selected={newLearnMethods}
      options={["level-up", "machine"]}
      on:change={(e) => {
        newMoveLearner.learn_method = newLearnMethods.join(",");
      }}
      style="height: 36px; border-color: rgb(209 213 219); border-radius: 0.375rem; box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05); font-size: 0.875rem;"
    />
  </div>
  {#if newMoveLearner.learn_method.includes("level-up")}
    <NumberInput
      id="level-learned"
      bind:value={newMoveLearner.level_learned}
      label="Learn Level"
      class="w-full"
    />
  {/if}
  <Button
    title="Edit Move"
    disabled={newMoveLearner.name === "" || newMoveLearner.learn_method === ""}
    onClick={editMove}
  />
</BaseModal>

<div class="flex flex-row gap-7">
  <AutoComplete
    bind:value={moveSearch[1]}
    placeholder="Search Moves"
    options={moveListOptions}
    popupId="move-search"
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
    onClick={() => (newMoveModalOpen = true)}
  />
</div>

{#if !_.isEmpty(move)}
  <p class="ml-2 mt-4 mb-4 text-lg">
    {_.capitalize(move.name).replaceAll("-", " ")}
  </p>
  <TabGroup>
    <Tab bind:group={tabSet} value={0} name="details" class="text-sm"
      >Details</Tab
    >
    <Tab bind:group={tabSet} value={1} name="pokemon" class="text-sm"
      >Pokemon</Tab
    >
    <svelte:fragment slot="panel">
      {#if tabSet === 0}
        <div class="ml-2 mt-4">
          <div class="grid grid-cols-2 gap-x-10 gap-y-5 pr-4">
            <NumberInput
              label="Power"
              id="power"
              bind:value={move.power}
              max={255}
            />
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
                };
              })}
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
      {#if tabSet === 1}
        <div class="mt-4 space-y-4 overflow-x-auto px-4">
          <header class="flex items-center justify-between gap-4">
            <div class="flex gap-x-3">
              <TextInput
                id="pokemon"
                bind:value={pokemonSearch}
                inputHandler={() => handler.search(pokemonSearch)}
                placeholder="Search pokemon"
              />
              <Button
                title="Add Move To Pokemon"
                class="mt-2 w-48"
                onClick={() => {
                  addMoveModalOpen = true;
                }}
              />
            </div>
            <aside class="flex items-center gap-x-3">
              <p class="mt-2">Show</p>
              <SelectInput
                bind:value={$rowsPerPage}
                options={rowsPerPageOptions}
              />
            </aside>
          </header>
          <table
            class="table table-hover table-compact w-full table-auto bg-white"
          >
            <thead>
              <tr class="bg-white">
                <ThSort {handler} orderBy="name">Pokemon Name</ThSort>
                <ThSort {handler} orderBy="learn_method">Learn Method</ThSort>
                <ThSort {handler} orderBy="level_learned">Learn Level</ThSort>
              </tr>
            </thead>
            <tbody>
              {#each $rows as row}
                <tr>
                  <td class="flex flex-row gap-1 self-center">
                    {#await getSpriteImage(row.name) then spriteUrl}
                      <img
                        src={spriteUrl}
                        alt={row.name}
                        class="m-0 justify-self-center"
                        width="30"
                        height="30"
                      />
                    {/await}
                    <span class="self-center">
                      {_.capitalize(row.name.replace("-", " "))}
                    </span>
                  </td>
                  <td>{row.learn_method}</td>
                  <td>{row.level_learned}</td>
                  <td
                    class="w-5 rounded-sm hover:cursor-pointer hover:bg-gray-300"
                    on:click={() => {
                      newMoveLearner = row;
                      editMoveModalOpen = true;
                      newLearnMethods = row.learn_method.split(",");
                    }}
                  >
                    <IconEdit size={18} class="text-gray-500" />
                  </td>
                  <td
                    class="w-5 rounded-sm hover:cursor-pointer hover:bg-gray-300"
                    on:click={() => deleteMoveFromPokemon(row.pokemonId)}
                  >
                    <IconTrash size={18} class="text-gray-500" />
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
          <footer class="flex">
            <Pagination {handler} />
          </footer>
        </div>
      {/if}
    </svelte:fragment>
  </TabGroup>
{/if}
<!-- <Button title="Convert Moves to SQLite" onClick={convertMovesToSqlite} /> -->
