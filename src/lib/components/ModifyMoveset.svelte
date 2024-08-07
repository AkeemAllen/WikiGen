<script lang="ts">
  import { modifyMoveSet } from "$lib/utils";
  import {
    Autocomplete,
    getToastStore,
    popup,
    type AutocompleteOption,
  } from "@skeletonlabs/skeleton";
  import { IconTrash } from "@tabler/icons-svelte";
  import { moveList } from "../../store/moves";
  import {
    type PokemonMove,
    Operation,
    type MoveSetChange,
  } from "../../store/pokemon";
  import NumberInput from "./NumberInput.svelte";
  import AutoComplete from "./AutoComplete.svelte";
  import Button from "./Button.svelte";
  import SelectInput from "./SelectInput.svelte";
  import { shortcut } from "@svelte-put/shortcut";
  import MultiSelect from "svelte-multiselect";
  import capitalizeWords from "$lib/utils/capitalizeWords";

  const toastStore = getToastStore();

  export let open: boolean = false;
  export let pokemonId: number;
  export let generatePokemonPage: Function;
  let moveSetChangeList: MoveSetChange[] = [];
  export let onClose: Function = () => {
    open = false;
    moveSetChangeList = [];
  };
  export let moveset: PokemonMove[];

  const moveListOptions = $moveList.map(([id, name]) => ({
    label: name,
    value: id,
  }));

  const pokemonMoveListOptions = moveset.map((move) => ({
    label: move.name,
    value: move.id,
  }));

  function onMoveNameSelected(
    event: CustomEvent<AutocompleteOption<string>>,
    field: "move" | "secondaryMove",
    index: number,
  ): void {
    if (field === "move") {
      moveSetChangeList[index].move = event.detail.label;
    } else {
      moveSetChangeList[index].secondaryMove = event.detail.label;
    }
  }

  function addMoveSetChange() {
    moveSetChangeList = [
      ...moveSetChangeList,
      {
        id: 0,
        operation: "add",
        move: "",
        method: [],
        level: 0,
        secondaryMoveId: 0,
        secondaryMove: "",
      },
    ];
  }

  function removeMoveSetChange(index: number) {
    moveSetChangeList = moveSetChangeList.filter((_, i) => i !== index);
  }

  async function saveChanges() {
    if (moveSetChangeList.length === 0) {
      return;
    }
    await modifyMoveSet(moveSetChangeList, moveset, pokemonId)
      .then((res) => {
        moveset = res;
      })
      .then(() => generatePokemonPage())
      .finally(() => {
        toastStore.trigger({
          message: "Moveset changes applied!",
          background: "variant-filled-success",
        });
        onClose();
      });
  }
</script>

<Button title="Add Moveset Change" onClick={addMoveSetChange} />
<div class="relative z-10 w-[60rem] overflow-x-auto">
  <table class="w-full text-left text-sm rtl:text-right">
    <thead class="text-xs uppercase">
      <tr>
        <th class="px-6 py-3" scope="col">Operation</th>
        <th class="px-6 py-3" scope="col">Move</th>
        <th class="px-6 py-3" scope="col">Method</th>
        <th class="px-6 py-3" scope="col">Level</th>
        <th class="px-6 py-3" scope="col">Secondary Move</th>
        <th></th>
      </tr>
    </thead>
    <tbody>
      {#each moveSetChangeList as row, index}
        <tr class="border-b bg-white">
          <td class="w-32 px-6 py-0 pb-1">
            <SelectInput
              bind:value={row.operation}
              options={Object.values(Operation).map((value) => ({
                label: capitalizeWords(value.replaceAll("_", " ")),
                value,
              }))}
            />
          </td>
          <td class="z-10 w-52 px-6 py-0 pb-1">
            <AutoComplete
              bind:value={row.move}
              popupId="movePopupAutoComplete{index}"
              class="z-10 w-full text-sm"
              options={row.operation === Operation.SHIFT ||
              row.operation === Operation.DELETE ||
              row.operation === Operation.REPLACE_MOVE
                ? pokemonMoveListOptions
                : moveListOptions}
              onSelection={(event) => {
                onMoveNameSelected(event, "move", index);
                let existingMove = moveset.find(
                  (move) => move.name === event.detail.label,
                );

                if (existingMove) {
                  row.id = existingMove.id;
                  // @ts-ignore
                  row.method = existingMove.learn_method.split(",");
                  if (existingMove.learn_method.includes("level-up")) {
                    row.level = existingMove.level_learned;
                  }
                } else {
                  row.level = 1;
                  // @ts-ignore
                  row.id = $moveList.find(
                    ([_, name]) => name === event.detail.label,
                  )?.[0];
                }
              }}
            />
          </td>
          <td class="px-6 py-0 pb-1">
            <MultiSelect
              id="versions"
              bind:selected={row.method}
              allowUserOptions={true}
              disabled={row.operation === Operation.DELETE}
              options={["level-up", "machine", "tutor", "egg"]}
              style="margin-top: 0.5rem; height: 36px; border-color: rgb(209 213 219); border-radius: 0.375rem; box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05); font-size: 0.875rem;"
            />
          </td>
          <td class="px-6 py-0 pb-1">
            <NumberInput
              bind:value={row.level}
              disabled={row.operation === Operation.DELETE ||
                row.operation === Operation.REPLACE_MOVE ||
                !row.method.includes("level-up")}
              max={100}
            />
          </td>
          <td class="w-52 px-6 py-0 pb-1">
            <AutoComplete
              bind:value={row.secondaryMove}
              popupId="secondaryMovePopupAutoComplete{index}"
              class="w-full text-sm"
              disabled={row.operation === Operation.DELETE ||
                row.operation === Operation.ADD ||
                row.operation === Operation.SHIFT}
              options={moveListOptions}
              onSelection={(event) => {
                onMoveNameSelected(event, "secondaryMove", index);
                row.secondaryMoveId = $moveList.find(
                  ([_, name]) => name === event.detail.label,
                )?.[0];
              }}
            />
          </td>
          <td
            class="w-5 rounded-sm px-6 py-0 pb-1 hover:cursor-pointer hover:bg-gray-300"
            on:click={() => removeMoveSetChange(index)}
          >
            <IconTrash size={18} class="mt-4 text-gray-500" />
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>
<Button
  title="Save Changes"
  onClick={saveChanges}
  disabled={moveSetChangeList.length === 0}
  class="w-40"
/>

<svelte:window
  use:shortcut={{
    trigger: {
      key: "l",
      modifier: "ctrl",
      callback: () => addMoveSetChange(),
    },
  }}
  use:shortcut={{
    trigger: {
      key: "Escape",
      callback: () => (open = false),
    },
  }}
  use:shortcut={{
    trigger: {
      key: "Enter",
      modifier: ["ctrl", "meta"],
      callback: () => saveChanges(),
    },
  }}
/>
