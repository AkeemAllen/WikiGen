<script lang="ts">
  import Pagination from "$lib/components/Pagination.svelte";
  import SelectInput from "$lib/components/SelectInput.svelte";
  import ThSort from "$lib/components/ThSort.svelte";
  import { IconEdit, IconTrash } from "@tabler/icons-svelte";
  import { DataHandler } from "@vincjo/datatables";
  import type { PokemonMove } from "../../store/pokemon";
  import TextInput from "./TextInput.svelte";
  import NumberInput from "./NumberInput.svelte";
  import Button from "./Button.svelte";
  import BaseModal from "./BaseModal.svelte";
  import AutoComplete from "./AutoComplete.svelte";
  import ModifyMoveset from "./ModifyMoveset.svelte";
  import { shortcut } from "@svelte-put/shortcut";
  import { moveList } from "../../store/moves";
  import MultiSelect from "svelte-multiselect";
  import { db } from "../../store/db";
  import { getToastStore } from "@skeletonlabs/skeleton";
  import capitalizeWords from "$lib/utils/capitalizeWords";

  const toastStore = getToastStore();

  // export let pokemonDetails: PokemonDetails;
  // export let savePokemonChanges: Function;
  export let pokemonId: number;
  export let generatePokemonPage: Function;

  let searchValue: string = "";
  let modifyMovesetModalOpen: boolean = false;
  let addMoveModalOpen: boolean = false;
  let newMove: PokemonMove = {
    id: 0,
    name: "",
    learn_method: "",
    level_learned: 0,
  };

  let editMoveModalOpen: boolean = false;
  let moveToEditLearnMethods: string[] = [];
  let moveToEdit: PokemonMove = {
    id: 0,
    name: "",
    learn_method: "",
    level_learned: 0,
  };

  let newLearnMethods: string[] = [];

  export let moveset: PokemonMove[] = [];

  $: moveListOptions = $moveList
    .map(([id, name]) => ({
      label: name,
      value: id,
    }))
    .filter((move) => !moveset.some((m) => m.id === move.value));

  const rowsPerPageOptions = [
    { label: "5", value: 5 },
    { label: "10", value: 10 },
    { label: "20", value: 20 },
    { label: "50", value: 50 },
    { label: "100", value: 100 },
  ];

  $: handler = new DataHandler(moveset, {
    rowsPerPage: 5,
  });
  $: rows = handler.getRows();
  $: rowsPerPage = handler.getRowsPerPage();

  async function deleteMove(moveId: number) {
    await $db
      .execute(
        `DELETE FROM pokemon_movesets WHERE pokemon = $1 AND move = $2`,
        [pokemonId, moveId],
      )
      .then(() => {
        const updatedMoves: PokemonMove[] = moveset.filter(
          (move) => move.id !== moveId,
        );
        moveset = updatedMoves;
        toastStore.trigger({
          message: "Move deleted successfully",
          background: "variant-filled-success",
        });
      })
      .then(() => generatePokemonPage());
  }

  async function addMove() {
    await $db
      .execute(
        `INSERT INTO pokemon_movesets (pokemon, move, learn_method, level_learned)
    VALUES ($1, $2, $3, $4)`,
        [
          pokemonId,
          newMove.id,
          newLearnMethods.join(","),
          newMove.level_learned,
        ],
      )
      .then(() => {
        moveset = [
          ...moveset,
          {
            id: newMove.id,
            name: newMove.name,
            learn_method: newLearnMethods.join(","),
            level_learned: newMove.level_learned,
          },
        ];
        addMoveModalOpen = false;
        toastStore.trigger({
          message: "Move added successfully",
          background: "variant-filled-success",
        });
      })
      .then(() => generatePokemonPage());
  }

  async function editMove() {
    await $db
      .execute(
        `UPDATE pokemon_movesets
    SET learn_method = $1, level_learned = $2
    WHERE pokemon = $3 AND move = $4`,
        [
          moveToEdit.learn_method,
          moveToEdit.level_learned,
          pokemonId,
          moveToEdit.id,
        ],
      )
      .then(() => {
        moveset = moveset.map((move) => {
          if (move.id === moveToEdit.id) {
            return {
              ...move,
              learn_method: moveToEdit.learn_method,
              level_learned: moveToEdit.level_learned,
            };
          }
          return move;
        });
        moveToEdit = { id: 0, name: "", learn_method: "", level_learned: 0 };
        moveToEditLearnMethods = [];
        editMoveModalOpen = false;
        toastStore.trigger({
          message: "Move updated successfully",
          background: "variant-filled-success",
        });
      })
      .then(() => generatePokemonPage());
  }
</script>

<BaseModal
  bind:open={addMoveModalOpen}
  close={() => {
    addMoveModalOpen = false;
    newMove = { id: 0, name: "", learn_method: "", level_learned: 0 };
  }}
  class="w-[20rem]"
>
  <h2 class="text-lg font-medium leading-6 text-gray-900">New Move</h2>
  <AutoComplete
    bind:value={newMove.name}
    label="Move Name"
    popupId="newMovePopup"
    class="z-10 w-full text-sm"
    options={moveListOptions}
    onSelection={(event) => {
      newMove.id = event.detail.value;
      newMove.name = event.detail.label;
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
        newMove.learn_method = newLearnMethods.join(",");
        console.log(newMove.learn_method);
      }}
      style="height: 36px; border-color: rgb(209 213 219); border-radius: 0.375rem; box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05); font-size: 0.875rem;"
    />
  </div>
  {#if newLearnMethods.includes("level-up")}
    <NumberInput
      id="level-learned"
      bind:value={newMove.level_learned}
      label="Learn Level"
      class="w-full"
    />
  {/if}
  <Button
    title="Add Move"
    disabled={newMove.name === "" || newMove.learn_method === ""}
    onClick={addMove}
  />
</BaseModal>

<BaseModal bind:open={editMoveModalOpen}>
  <h2 class="text-lg font-medium leading-6 text-gray-900">Edit Move</h2>
  <TextInput value={moveToEdit.name} disabled={true} />
  <div>
    <label
      for="methods"
      class="mb-2 block text-sm font-medium leading-6 text-gray-900"
      >Learn Method(s)</label
    >
    <MultiSelect
      id="methods"
      bind:selected={moveToEditLearnMethods}
      options={["level-up", "machine"]}
      on:change={(e) => {
        moveToEdit.learn_method = moveToEditLearnMethods.join(",");
        console.log(moveToEdit.learn_method);
      }}
      style="height: 36px; border-color: rgb(209 213 219); border-radius: 0.375rem; box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05); font-size: 0.875rem;"
    />
  </div>
  {#if moveToEditLearnMethods.includes("level-up")}
    <NumberInput
      id="level-learned"
      bind:value={moveToEdit.level_learned}
      label="Learn Level"
      class="w-full"
    />
  {/if}
  <Button
    title="Edit Move"
    disabled={moveToEdit.name === "" || moveToEdit.learn_method === ""}
    onClick={editMove}
  />
</BaseModal>

<BaseModal bind:open={modifyMovesetModalOpen}>
  <ModifyMoveset
    bind:moveset
    bind:pokemonId
    bind:open={modifyMovesetModalOpen}
    {generatePokemonPage}
  />
</BaseModal>

<div>
  <div class="mt-4 space-y-4 overflow-x-auto px-4">
    <header class="flex items-center justify-between gap-4">
      <div class="flex gap-x-3">
        <TextInput
          id="move-name"
          bind:value={searchValue}
          inputHandler={() => handler.search(searchValue)}
          placeholder="Search move name..."
        />
        <Button
          title="Modify Moveset"
          onClick={() => (modifyMovesetModalOpen = true)}
          class="mt-2"
        />
        <Button
          title="Add Move"
          class="mt-2"
          onClick={() => (addMoveModalOpen = true)}
        />
      </div>
      <aside class="flex items-center gap-x-3">
        <p class="mt-2">Show</p>
        <SelectInput bind:value={$rowsPerPage} options={rowsPerPageOptions} />
      </aside>
    </header>
    <table class="table table-hover table-compact w-full table-auto bg-white">
      <thead>
        <tr class="bg-white">
          <ThSort {handler} orderBy="move_name">Move Name</ThSort>
          <ThSort {handler} orderBy="learn_method">Learn Method</ThSort>
          <ThSort {handler} orderBy="level_learned">Learn Level</ThSort>
        </tr>
      </thead>
      <tbody>
        {#each $rows as row}
          <tr>
            <td>{capitalizeWords(row.name.replace("-", " "))}</td>
            <td>{row.learn_method}</td>
            <td>{row.level_learned}</td>
            <td
              class="w-5 rounded-sm hover:cursor-pointer hover:bg-gray-300"
              on:click={() => {
                moveToEdit = row;
                editMoveModalOpen = true;
                moveToEditLearnMethods = row.learn_method.split(",");
              }}
            >
              <IconEdit size={18} class="text-gray-500" />
            </td>
            <td
              class="w-5 rounded-sm hover:cursor-pointer hover:bg-gray-300"
              on:click={() => deleteMove(row.id)}
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
</div>

<svelte:window
  use:shortcut={{
    trigger: {
      key: "m",
      modifier: "ctrl",
      callback: () => {
        modifyMovesetModalOpen = true;
      },
    },
  }}
/>
