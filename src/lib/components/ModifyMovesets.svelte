<script lang="ts">
  import { getToastStore } from "@skeletonlabs/skeleton";
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
</script>

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
        title="Search"
        onClick={addPokemonToModifyMovesets}
        disabled={pokemonSearch[0] === 0}
        class="mt-2 w-32"
      />
    </div>

    <div class="grid grid-cols-2 gap-4">
      {#each Object.entries($pokemonUnderMoveModification) as [name, { moves: moveset, sprite }]}
        <div class="text-center mt-4">
          <img src={sprite} alt={name} width="80" class="m-auto" />
          <div class="grid grid-cols-2 gap-2 bg-white rounded-lg shadow-md p-4">
            <div>
              <h3>TMs</h3>
              {#each moveset.filter((pokemonMove) => pokemonMove.learn_method === "machine") as pokemonMove}
                <div
                  class="rounded-md mt-2 shadow-sm p-1 bg-gray-200 self-center"
                >
                  {pokemonMove.name}
                </div>
              {/each}
            </div>
            <div>
              <h3>Level Up</h3>
            </div>
          </div>
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
      {#each searchMoves as move}
        <div
          class="rounded-md mt-2 shadow-sm p-1 pl-4 bg-gray-200 w-[80%] self-center"
        >
          {move}
        </div>
      {/each}
    </div>
  </section>
</div>
