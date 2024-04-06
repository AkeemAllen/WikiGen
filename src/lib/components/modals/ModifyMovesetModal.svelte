<script lang="ts">
  import { modifyLevelUpMoveSet } from "$lib/utils";
  import {
    Autocomplete,
    popup,
    type AutocompleteOption,
  } from "@skeletonlabs/skeleton";
  import { IconTrash } from "@tabler/icons-svelte";
  import _ from "lodash";
  import { moveList } from "../../../store/moves";
  import { type PokemonDetails } from "../../../store/pokemon";
  import { Operation, type MoveSetChange } from "../../../types";
  import NumberInput from "../NumberInput.svelte";
  import SelectInput from "../SelectInput.svelte";

  export let open: boolean = false;
  export let onClose: Function = () => (open = false);
  export let pokemonDetails: PokemonDetails;
  export let savePokemonChanges: Function;

  const moveListOptions: AutocompleteOption<string>[] = $moveList.map(
    (name) => ({ label: name, value: name }),
  );

  const pokemonMoveListOptions: AutocompleteOption<string>[] = Object.keys(
    pokemonDetails.moves,
  ).map((name) => ({ label: name, value: name }));

  let moveSetChangeList: MoveSetChange[] = [];

  function onMoveNameSelected(
    event: CustomEvent<AutocompleteOption<string>>,
    field: "move" | "secondaryMove",
    index: number,
  ): void {
    if (field === "move") {
      moveSetChangeList[index].move = event.detail.value;
    } else {
      moveSetChangeList[index].secondaryMove = event.detail.value;
    }
  }
  function addMoveSetChange() {
    moveSetChangeList = [
      ...moveSetChangeList,
      {
        operation: "add",
        move: "",
        level: 1,
        secondaryMove: "",
      },
    ];
  }

  function removeMoveSetChange(index: number) {
    moveSetChangeList = moveSetChangeList.filter((_, i) => i !== index);
  }

  function saveChanges() {
    pokemonDetails.moves = modifyLevelUpMoveSet(
      moveSetChangeList,
      pokemonDetails.moves,
    );
    savePokemonChanges();
    onClose();
  }
</script>

{#if open}
  <div
    class="absolute top-0 bottom-0 left-0 right-0 h-screen grid justify-center items-center"
  >
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div
      on:click={() => onClose()}
      class="absolute top-0 bottom-0 left-0 right-0 h-screen bg-gray-900/90"
      style="z-index: 100;"
    />
    <div
      class="relative flex flex-col gap-y-4 bg-white p-4 rounded-md"
      style="z-index: 1000;"
    >
      <button
        on:click={addMoveSetChange}
        class=" rounded-md bg-indigo-600 w-full px-3 py-2 mt-2 text-sm font-semibold text-white
                   shadow-sm hover:bg-indigo-500 focus-visible:outline
                   focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600
                   disabled:bg-indigo-400"
      >
        Add Moveset Change</button
      >
      <div class="relative overflow-x-auto w-[50rem]">
        <table class="w-full text-sm text-left rtl:text-right">
          <thead class="text-xs uppercase">
            <tr>
              <th class="px-6 py-3" scope="col">Operation</th>
              <th class="px-6 py-3" scope="col">Move</th>
              <th class="px-6 py-3" scope="col">Level</th>
              <th class="px-6 py-3" scope="col">Secondary Move</th>
              <th></th>
            </tr>
          </thead>
          <tbody>
            {#each moveSetChangeList as row, index}
              <tr class="bg-white border-b">
                <td class="px-6 py-0 pb-1 w-52">
                  <SelectInput
                    bind:value={row.operation}
                    options={Object.values(Operation).map((value) => ({
                      label: _.capitalize(value.replaceAll("_", " ")),
                      value,
                    }))}
                  />
                </td>
                <td class="px-6 py-0 pb-1">
                  <input
                    id="move"
                    type="text"
                    placeholder="Move"
                    class="block w-full pl-2 mt-2 rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6 disabled:bg-gray-100 disabled:text-gray-400"
                    bind:value={row.move}
                    use:popup={{
                      event: "focus-click",
                      target: "movePopupAutoComplete" + index,
                      placement: "bottom",
                    }}
                  />
                  <div
                    data-popup="movePopupAutoComplete{index}"
                    class="fixed card w-60 mt-2 overflow-y-auto bg-white rounded-sm"
                    tabindex="-1"
                  >
                    <Autocomplete
                      bind:input={row.move}
                      on:selection={(event) => {
                        onMoveNameSelected(event, "move", index);
                        if (
                          row.move in pokemonDetails.moves &&
                          pokemonDetails.moves[row.move].learn_method.includes(
                            "level-up",
                          )
                        ) {
                          row.level =
                            pokemonDetails.moves[row.move].level_learned;
                        }
                      }}
                      options={row.operation === Operation.SWAP_MOVES ||
                      row.operation === Operation.SHIFT ||
                      row.operation === Operation.DELETE ||
                      row.operation === Operation.REPLACE_MOVE
                        ? pokemonMoveListOptions
                        : moveListOptions}
                      limit={5}
                      class="bg-white w-full text-sm border rounded-md p-2 z-[100000]"
                    />
                  </div>
                </td>
                <td class="px-6 py-0 pb-1">
                  <NumberInput
                    bind:value={row.level}
                    disabled={row.operation === Operation.DELETE ||
                      row.operation === Operation.REPLACE_MOVE ||
                      row.operation === Operation.SWAP_MOVES}
                  />
                </td>
                <td class="px-6 py-0 pb-1">
                  <input
                    id="secondary_move"
                    type="text"
                    placeholder="Secondary Move"
                    disabled={row.operation === Operation.DELETE ||
                      row.operation === Operation.ADD ||
                      row.operation === Operation.SHIFT ||
                      row.operation === Operation.REPLACE_BY_LEVEL}
                    class="block w-full pl-2 mt-2 rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6 disabled:bg-gray-100 disabled:text-gray-400"
                    bind:value={row.secondaryMove}
                    use:popup={{
                      event: "focus-click",
                      target: "secondaryMovePopupAutoComplete" + index,
                      placement: "bottom",
                    }}
                  />
                  <div
                    data-popup="secondaryMovePopupAutoComplete{index}"
                    class="fixed card w-60 mt-2 overflow-y-auto bg-white rounded-sm"
                    tabindex="-1"
                  >
                    <Autocomplete
                      bind:input={row.secondaryMove}
                      on:selection={(event) =>
                        onMoveNameSelected(event, "secondaryMove", index)}
                      options={row.operation === Operation.SWAP_MOVES
                        ? pokemonMoveListOptions
                        : moveListOptions}
                      limit={5}
                      class="bg-white w-full text-sm border rounded-md p-2"
                    />
                  </div>
                </td>
                <td
                  class="px-6 py-0 pb-1 hover:cursor-pointer hover:bg-gray-300 w-5 rounded-sm"
                  on:click={() => removeMoveSetChange(index)}
                >
                  <IconTrash size={18} class="text-gray-500 mt-4" />
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
      <button
        on:click={saveChanges}
        class=" rounded-md bg-indigo-600 w-40 px-3 py-2 mt-2 text-sm font-semibold text-white
               shadow-sm hover:bg-indigo-500 focus-visible:outline
               focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600
               disabled:bg-indigo-400"
      >
        Save Changes</button
      >
    </div>
  </div>
{/if}