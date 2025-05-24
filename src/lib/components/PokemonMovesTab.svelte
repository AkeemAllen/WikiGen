<script lang="ts">
  import Pagination from "$lib/components/Pagination.svelte";
  import SelectInput from "$lib/components/SelectInput.svelte";
  import ThSort from "$lib/components/ThSort.svelte";
  import IconEdit from "@tabler/icons-svelte/icons/edit";
  import IconTrash from "@tabler/icons-svelte/icons/trash";
  import { DataHandler } from "@vincjo/datatables";
  import type { PokemonMove } from "../../store/pokemon";
  import TextInput from "./TextInput.svelte";
  import NumberInput from "./NumberInput.svelte";
  import Button from "./Button.svelte";
  import BaseModal from "./BaseModal.svelte";
  import AutoComplete from "./AutoComplete.svelte";
  import { moveList } from "../../store/moves";
  import { db } from "../../store/db";
  import { getToastStore } from "@skeletonlabs/skeleton";
  import capitalizeWords from "$lib/utils/capitalizeWords";
  import { getToastSettings, ToastType } from "$lib/utils/toasts";

  interface Props {
    pokemonId: number;
    generatePokemonPage: Function;
    moveset?: PokemonMove[];
  }

  let {
    pokemonId = $bindable(),
    generatePokemonPage,
    moveset = $bindable([]),
  }: Props = $props();

  let searchValue: string = $state("");
  let addMoveModalOpen: boolean = $state(false);
  let newMove: PokemonMove = $state({
    id: 0,
    name: "",
    learn_method: "",
    level_learned: 0,
  });

  let editMoveModalOpen: boolean = $state(false);
  let moveToEdit: PokemonMove = $state({
    id: 0,
    name: "",
    learn_method: "",
    level_learned: 0,
  });

  const toastStore = getToastStore();

  let moveListOptions = $derived(
    $moveList.map(([id, name]) => ({
      label: name,
      value: id,
    })),
  );

  const rowsPerPageOptions = [
    { label: "5", value: 5 },
    { label: "10", value: 10 },
    { label: "20", value: 20 },
    { label: "50", value: 50 },
    { label: "100", value: 100 },
  ];

  let handler = $derived(
    new DataHandler(moveset, {
      rowsPerPage: 5,
    }),
  );
  let rows = $derived(handler.getRows());
  let rowsPerPage = $derived(handler.getRowsPerPage());

  async function deleteMove(moveId: number, learn_method: string) {
    await $db
      .execute(
        `DELETE FROM pokemon_movesets WHERE pokemon = $1 AND move = $2 AND learn_method = $3`,
        [pokemonId, moveId, learn_method],
      )
      .then(() => {
        const updatedMoves: PokemonMove[] = moveset.filter(
          (move) => move.id !== moveId && move.learn_method !== learn_method,
        );
        moveset = updatedMoves;
        toastStore.trigger(
          getToastSettings(ToastType.SUCCESS, "Move deleted successfully"),
        );
      })
      .then(() => generatePokemonPage())
      .catch((err) => {
        toastStore.trigger(
          getToastSettings(ToastType.ERROR, `Error deleting move: ${err}`),
        );
      });
  }

  async function addMove() {
    await $db
      .execute(
        `INSERT INTO pokemon_movesets (pokemon, move, learn_method, level_learned) VALUES ($1, $2, $3, $4)`,
        [pokemonId, newMove.id, newMove.learn_method, newMove.level_learned],
      )
      .then(() => {
        moveset = [...moveset, newMove];
        addMoveModalOpen = false;
        newMove = {
          id: 0,
          name: "",
          learn_method: "",
          level_learned: 0,
        };
        toastStore.trigger(
          getToastSettings(ToastType.SUCCESS, "Move added successfully"),
        );
      })
      .then(() => generatePokemonPage())
      .catch((err) => {
        toastStore.trigger(
          getToastSettings(ToastType.ERROR, `Error adding move: ${err}`),
        );
      });
  }

  async function editMove() {
    await $db
      .execute(
        `UPDATE pokemon_movesets SET learn_method = $1, level_learned = $2 WHERE pokemon = $3 AND move = $4`,
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
        editMoveModalOpen = false;
        toastStore.trigger(
          getToastSettings(ToastType.SUCCESS, "Move updated successfully"),
        );
      })
      .then(() => generatePokemonPage())
      .catch((err) => {
        toastStore.trigger(
          getToastSettings(ToastType.ERROR, `Error updating move: ${err}`),
        );
      });
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
    <SelectInput
      id="methods"
      bind:value={newMove.learn_method}
      options={["level-up", "machine"].map((option) => ({
        label: option,
        value: option,
      }))}
    />
  </div>
  {#if newMove.learn_method === "level-up"}
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
    <SelectInput
      id="methods"
      bind:value={moveToEdit.learn_method}
      options={["level-up", "machine"].map((option) => ({
        label: option,
        value: option,
      }))}
    />
  </div>
  {#if moveToEdit.learn_method === "level-up"}
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
              onclick={() => {
                moveToEdit = row;
                editMoveModalOpen = true;
              }}
            >
              <IconEdit size={18} class="text-gray-500" />
            </td>
            <td
              class="w-5 rounded-sm hover:cursor-pointer hover:bg-gray-300"
              onclick={() => deleteMove(row.id, row.learn_method)}
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
