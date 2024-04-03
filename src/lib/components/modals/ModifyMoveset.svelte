<script lang="ts">
  import {
    Autocomplete,
    popup,
    type AutocompleteOption,
    type PopupSettings,
  } from "@skeletonlabs/skeleton";
  import { IconTrash } from "@tabler/icons-svelte";
  import _ from "lodash";
  import { moveList } from "../../../store/moves";
  import { Operation } from "../../../types";
  import NumberInput from "../NumberInput.svelte";
  import SelectInput from "../SelectInput.svelte";

  const moveListOptions: AutocompleteOption<string>[] = $moveList.map(
    (name) => ({ label: name, value: name }),
  );
  let moveSetChangeList: {
    operation: string;
    move: string;
    level: number;
    secondaryMove: string;
  }[] = [];

  const moveAutoCompletePopup: PopupSettings = {
    event: "focus-click",
    target: "movePopupAutoComplete",
    placement: "bottom",
  };

  const secondaryMoveAutoCompletePopup: PopupSettings = {
    event: "focus-click",
    target: "secondaryMovePopupAutoComplete",
    placement: "bottom",
  };

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
  $: console.log(moveSetChangeList);
</script>

<div class="flex flex-col gap-y-4 bg-white p-4 rounded-md">
  <button
    on:click={addMoveSetChange}
    class=" rounded-md bg-indigo-600 w-full px-3 py-2 mt-2 text-sm font-semibold text-white
            shadow-sm hover:bg-indigo-500 focus-visible:outline
            focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600
            disabled:bg-indigo-400"
  >
    Add Moveset Change</button
  >
  <div class="table-container w-[50rem]">
    <table class="table table-hover">
      <thead>
        <tr>
          <th>Operation</th>
          <th>Move</th>
          <th>Level</th>
          <th>Secondary Move</th>
          <th></th>
        </tr>
      </thead>
      <tbody>
        {#each moveSetChangeList as row, index}
          <tr>
            <td class="w-48">
              <SelectInput
                bind:value={row.operation}
                options={Object.values(Operation).map((value) => ({
                  label: _.capitalize(value),
                  value,
                }))}
              />
            </td>
            <td>
              <input
                id="move"
                type="text"
                placeholder="Move"
                class="block w-full pl-2 mt-2 rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6 disabled:bg-gray-100 disabled:text-gray-400"
                bind:value={row.move}
                use:popup={moveAutoCompletePopup}
              />
              <div
                data-popup="movePopupAutoComplete"
                class="card w-60 mt-2 overflow-y-auto bg-white rounded-sm"
                tabindex="-1"
              >
                <Autocomplete
                  bind:input={row.move}
                  on:selection={(event) =>
                    onMoveNameSelected(event, "move", index)}
                  options={moveListOptions}
                  limit={5}
                  class="bg-white w-full text-sm border rounded-md p-2"
                />
              </div>
            </td>
            <td>
              <NumberInput
                bind:value={row.level}
                disabled={row.operation === Operation.DELETE ||
                  row.operation === Operation.SWAP_MOVES}
              />
            </td>
            <td>
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
                use:popup={secondaryMoveAutoCompletePopup}
              />
              <div
                data-popup="secondaryMovePopupAutoComplete"
                class="card w-60 mt-2 overflow-y-auto bg-white rounded-sm"
                tabindex="-1"
              >
                <Autocomplete
                  bind:input={row.secondaryMove}
                  on:selection={(event) =>
                    onMoveNameSelected(event, "secondaryMove", index)}
                  options={moveListOptions}
                  limit={5}
                  class="bg-white w-full text-sm border rounded-md p-2"
                />
              </div>
            </td>
            <td
              class="hover:cursor-pointer hover:bg-gray-300 w-5 rounded-sm"
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
    class=" rounded-md bg-indigo-600 w-40 px-3 py-2 mt-2 text-sm font-semibold text-white
        shadow-sm hover:bg-indigo-500 focus-visible:outline
        focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600
        disabled:bg-indigo-400"
  >
    Save Changes</button
  >
</div>
