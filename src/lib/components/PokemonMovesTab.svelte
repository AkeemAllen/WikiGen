<script lang="ts">
  import { SelectInput } from "$lib";
  import Pagination from "$lib/components/Pagination.svelte";
  import ThSort from "$lib/components/ThSort.svelte";
  import { IconTrash } from "@tabler/icons-svelte";
  import { DataHandler } from "@vincjo/datatables";
  import _ from "lodash";
  import type { PokemonDetails, PokemonMoveSet } from "../../store/pokemon";
  import TextInput from "./TextInput.svelte";

  export let pokemonDetails: PokemonDetails;
  let searchValue: string = "";

  let handler = new DataHandler(Object.entries(pokemonDetails.moves), {
    rowsPerPage: 5,
  });
  let rows = handler.getRows();
  const rowsPerPage = handler.getRowsPerPage();
  const rowsPerPageOptions = [
    { label: "5", value: 5 },
    { label: "10", value: 10 },
    { label: "20", value: 20 },
    { label: "50", value: 50 },
    { label: "100", value: 100 },
  ];

  function deleteMove(moveName: string) {
    const updatedMoves: PokemonMoveSet = Object.fromEntries(
      Object.entries(pokemonDetails.moves).filter(([key]) => key !== moveName),
    );
    pokemonDetails.moves = updatedMoves;

    // Need to reinitialize the handlers to get immediate row updates
    // when move is deleted. This is a workaround for now.
    handler = new DataHandler(Object.entries(pokemonDetails.moves), {
      rowsPerPage: 5,
    });
    rows = handler.getRows();
  }
</script>

<div>
  <div class="overflow-x-auto space-y-4 mt-4 px-4">
    <header class="flex justify-between items-center gap-4">
      <div class="flex gap-x-3">
        <TextInput
          id="move-name"
          bind:value={searchValue}
          inputHandler={() => handler.search(searchValue)}
          placeholder="Search move name..."
        />
        <button
          class=" rounded-md bg-indigo-600 w-40 px-3 py-2 mt-2 text-sm font-semibold text-white
        shadow-sm hover:bg-indigo-500 focus-visible:outline
        focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600
        disabled:bg-indigo-400"
        >
          Modify Moveset</button
        >
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
            <td>{_.capitalize(row[0].replace("-", " "))}</td>
            <td>{_.capitalize(row[1].learn_method.replace("-", " "))}</td>
            <td>{row[1].level_learned}</td>
            <td
              class="hover:cursor-pointer hover:bg-gray-300 w-5 rounded-sm"
              on:click={() => deleteMove(row[0])}
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
