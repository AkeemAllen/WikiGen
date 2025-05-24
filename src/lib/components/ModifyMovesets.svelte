<script lang="ts">
  import { getToastStore, Tab, TabGroup } from "@skeletonlabs/skeleton";
  import {
    pokemonList,
    pokemonUnderMoveModification,
    type PokemonMove,
  } from "../../store/pokemon";
  import AutoComplete from "./AutoComplete.svelte";
  import Button from "./Button.svelte";
  import { getToastSettings, ToastType } from "$lib/utils/toasts";
  import capitalizeWords from "$lib/utils/capitalizeWords";
  import { db } from "../../store/db";
  import TextInput from "./TextInput.svelte";
  import { moveList } from "../../store/moves";
  import { readFile } from "@tauri-apps/plugin-fs";
  import { selectedWiki } from "../../store";
  import { BaseDirectory } from "@tauri-apps/plugin-fs";

  let toastStore = getToastStore();

  let pokemonSearch: [number, string] = $state([0, ""]);
  let moveSearch: string = $state("flame");
  let tabSet: number = $state(0);

  let pokemonListOptions = $pokemonList.map(([id, _, name]) => ({
    label: capitalizeWords(name),
    value: id,
  }));

  let searchMoves: string[] = $derived.by(() => {
    let moves: string[] = [];
    if (moveSearch.length < 3) return moves;

    return $moveList
      .filter(([_, name]) =>
        name
          .toLowerCase()
          .replaceAll("-", "")
          .includes(moveSearch.toLowerCase().replaceAll(" ", "")),
      )
      .map(([_, name]) => name)
      .slice(0, 10);
  });

  async function addPokemonToModifyMovesets() {
    let retrievedPokemon = $pokemonList.find(
      ([__, ___, name]) =>
        name === pokemonSearch[1].toLowerCase().split(" ").join("-"),
    );

    if (!retrievedPokemon) {
      toastStore.trigger(
        getToastSettings(ToastType.ERROR, "Pokemon not found!"),
      );
      return;
    }

    if ($pokemonUnderMoveModification[pokemonSearch[1]]) {
      return;
    }

    $pokemonUnderMoveModification[pokemonSearch[1]] = {
      moves: [],
      sprite: "",
    };

    // Gather moveset
    await $db
      .select<PokemonMove[]>(
        `SELECT moves.id as id, moves.name as name, learn_method, level_learned FROM pokemon_movesets
               INNER JOIN moves on moves.id = pokemon_movesets.move
               WHERE pokemon = $1;`,
        [pokemonSearch[0]],
      )
      .then((res) => {
        $pokemonUnderMoveModification[pokemonSearch[1]]["moves"] = res;
      })
      .catch((err) => {
        toastStore.trigger(
          getToastSettings(
            ToastType.ERROR,
            `Error loading Pokemon moveset!: \n ${err}`,
          ),
        );
      });

    $pokemonUnderMoveModification[pokemonSearch[1]]["sprite"] = await readFile(
      `${$selectedWiki.name}/dist/docs/img/pokemon/${pokemonSearch[1]}.png`,
      { baseDir: BaseDirectory.AppData },
    )
      .then((res) => {
        const blob = new Blob([res], { type: "image/png" });
        return URL.createObjectURL(blob);
      })
      .catch((err) => {
        if (err.includes("No such file or directory")) {
          return "404";
        }
        return "Error loading image";
      });
  }

  function onDragStart(e: DragEvent) {
    if (e.dataTransfer === null) return;
    e.dataTransfer.setData("text", (e.target as HTMLElement).id);
  }

  function onDragDrop(e: DragEvent, pokemonName: string) {
    if (e.dataTransfer === null) return;
    e.preventDefault();
    let data: string = e.dataTransfer.getData("text");
    let moveId = $moveList.find(([id, name]) => name === data)?.[0] as number;

    if (
      $pokemonUnderMoveModification[pokemonName]["moves"].find(
        (move) => move.id === moveId,
      )
    )
      return;

    let newMove: PokemonMove = {
      id: moveId,
      name: data,
      learn_method: "machine",
      level_learned: 0,
    };
    $pokemonUnderMoveModification[pokemonName]["moves"] = [
      newMove,
      ...$pokemonUnderMoveModification[pokemonName]["moves"],
    ];
  }
</script>

<div></div>

<div class="grid grid-cols-3 gap-4">
  <section class="col-span-2">
    <div class="flex flex-row gap-7">
      <AutoComplete
        bind:value={pokemonSearch[1]}
        placeholder="Search Pokemon"
        options={pokemonListOptions}
        popupId="pokemon-search"
        onSelection={(e) => {
          pokemonSearch = [e.detail.value, e.detail.label];
        }}
        showChevron={false}
      />
      <Button
        title="Add"
        onClick={addPokemonToModifyMovesets}
        disabled={pokemonSearch[0] === 0}
        class="mt-2 w-32"
      />
    </div>

    <div class="grid grid-cols-2 gap-4">
      {#each Object.entries($pokemonUnderMoveModification) as [name, { moves: moveset, sprite }]}
        <div class="text-center mt-4">
          <img src={sprite} alt={name} width="80" class="m-auto" />
          <TabGroup
            border=""
            justify="justify-center"
            class="p-3 shadow-lg rounded-lg"
          >
            <Tab bind:group={tabSet} name="machines" value={0} class="text-sm">
              Machines
            </Tab>
            <Tab bind:group={tabSet} name="level-up" value={1} class="text-sm">
              Level-Up
            </Tab>
            <div slot="panel">
              {#if tabSet === 0}
                <div
                  ondragover={(e) => e.preventDefault()}
                  ondrop={(e) => onDragDrop(e, name)}
                  role="none"
                >
                  <div class="grid grid-cols-2 gap-2">
                    {#each moveset.filter((pokemonMove) => pokemonMove.learn_method === "machine") as pokemonMove}
                      <div
                        class="rounded-md mt-2 shadow-sm p-1 bg-gray-200 self-center"
                      >
                        {pokemonMove.name}
                      </div>
                    {/each}
                  </div>
                </div>
              {/if}
              {#if tabSet === 1}
                <div></div>
              {/if}
            </div>
          </TabGroup>
          <!-- <div
            class="grid grid-cols-2 gap-2 bg-white rounded-lg shadow-md p-4"
          ></div> -->
        </div>
      {/each}
    </div>
  </section>
  <section class="col-span-1">
    <div class="flex flex-row gap-7">
      <TextInput
        class="w-full"
        placeholder="Search Moves (min 3 letters)"
        bind:value={moveSearch}
      />
    </div>
    <div class="flex flex-col">
      {#each searchMoves as move, i}
        <div
          class="rounded-md mt-2 shadow-sm p-1 pl-4 bg-gray-200 w-[80%] self-center"
          draggable={true}
          ondragstart={onDragStart}
          id={move}
          role="none"
        >
          {move}
        </div>
      {/each}
    </div>
  </section>
</div>
