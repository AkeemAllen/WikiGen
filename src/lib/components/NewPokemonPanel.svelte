<script lang="ts">
  import Button from "$lib/components/Button.svelte";
  import AutoComplete from "$lib/components/AutoComplete.svelte";
  import PokemonDetailsTab from "./PokemonDetailsTab.svelte";
  import { getToastStore, Tab, TabGroup } from "@skeletonlabs/skeleton";
  import {
    type MoveSetChange,
    type LearnMethod,
    type Pokemon,
    pokemonList,
    type PokemonMove,
  } from "../../store/pokemon";
  import { db } from "../../store/db";
  import NumberInput from "./NumberInput.svelte";
  import TextInput from "./TextInput.svelte";
  import PokemonMovesetTab from "$lib/components/PokemonMovesTab.svelte";
  import { BaseDirectory, writeFile } from "@tauri-apps/plugin-fs";
  import { selectedWiki } from "../../store";
  import { addMoves, base64ToArray } from "$lib/utils";
  import capitalizeWords from "$lib/utils/capitalizeWords";
  import { getToastSettings, ToastType } from "$lib/utils/toasts";
  import type { QueryResult } from "@tauri-apps/plugin-sql";

  let pokemonSearch: [number, string] = $state([0, ""]);
  let newSpriteImage: string = $state("");
  let newPokemon: Pokemon = $state({
    id: 0,
    dex_number: 0,
    name: "",
    types: "normal",
    ability_1: null,
    ability_2: null,
    hidden_ability: null,
    hp: 0,
    attack: 0,
    defense: 0,
    sp_attack: 0,
    sp_defense: 0,
    speed: 0,
    evolution_method: "no_change",
    evolved_pokemon: null,
    evolution_item: null,
    evolution_level: null,
    evolution_other: null,
    render: "false",
  });
  let copiedMoveset: PokemonMove[] = $state([]);

  let tabSet: number = $state(0);
  let pokemonListOptions = $pokemonList.map(([id, _, name]) => ({
    label: capitalizeWords(name),
    value: id,
  }));

  const toastStore = getToastStore();

  function onImageUpload(e: any) {
    let file = e.target.files[0];
    let reader = new FileReader();
    reader.onloadend = (e) => {
      let base64 = e.target?.result as string;
      if (!base64.includes("data:image/png;base64,")) {
        toastStore.trigger(
          getToastSettings(
            ToastType.ERROR,
            "Invalid image format! Please upload a PNG file!",
          ),
        );
        reader.abort();
        return;
      }
      newSpriteImage = e.target?.result as string;
    };
    reader.readAsDataURL(file);
  }

  async function copyPokemonMoveset() {
    await $db
      .select<PokemonMove[]>(
        `SELECT moves.id as id, moves.name as name, learn_method, level_learned FROM pokemon_movesets
              INNER JOIN moves on moves.id = pokemon_movesets.move
              WHERE pokemon = $1;`,
        [pokemonSearch[0]],
      )
      .then((res) => {
        copiedMoveset = res;
        pokemonSearch = [0, ""];
      })
      .catch((err) => {
        toastStore.trigger(
          getToastSettings(
            ToastType.ERROR,
            `Error loading Pokemon moveset!: \n ${err}`,
          ),
        );
      });
  }

  async function copyPokemonDetails() {
    await $db
      .select<Pokemon[]>(`SELECT * FROM pokemon WHERE id = $1;`, [
        pokemonSearch[0],
      ])
      .then((res) => {
        newPokemon = res[0];
      })
      .then(() => {
        copyPokemonMoveset();
      })
      .catch((err) => {
        toastStore.trigger(
          getToastSettings(
            ToastType.ERROR,
            `Error loading Pokemon details!: \n ${err}`,
          ),
        );
      });
  }

  async function createNewPokemon() {
    await $db
      .execute(
        `INSERT INTO pokemon (
          dex_number,
          name,
          types,
          ability_1,
          ability_2,
          hidden_ability,
          hp, attack, defense, sp_attack, sp_defense, speed, evolution_method
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13);`,
        [
          newPokemon.dex_number,
          newPokemon.name.toLowerCase(),
          newPokemon.types,
          newPokemon.ability_1?.toLowerCase().split(" ").join("-"),
          newPokemon.ability_2?.toLowerCase().split(" ").join("-"),
          newPokemon.hidden_ability?.toLowerCase().split(" ").join("-"),
          newPokemon.hp,
          newPokemon.attack,
          newPokemon.defense,
          newPokemon.sp_attack,
          newPokemon.sp_defense,
          newPokemon.speed,
          "no_change",
        ],
      )
      .then((res: QueryResult) => {
        // Write image to file
        const imageBytes = base64ToArray(
          newSpriteImage.replace("data:image/png;base64,", ""),
          "image/png",
        );
        writeFile(
          `${$selectedWiki.name}/dist/docs/img/pokemon/${newPokemon.name}.png`,
          new Uint8Array(imageBytes),
          { baseDir: BaseDirectory.AppData },
        ).catch((err) => {
          toastStore.trigger(
            getToastSettings(
              ToastType.ERROR,
              `Error writing image to file!: ${err}`,
            ),
          );
        });

        // Add moves to pokemon moveset
        const moveset: MoveSetChange[] = copiedMoveset.map((move) => ({
          id: move.id,
          operation: "add",
          move: move.name,
          method: move.learn_method.split(",") as LearnMethod[],
          level: move.level_learned,
          secondaryMoveId: null,
          secondaryMove: "",
        }));
        addMoves(moveset, res.lastInsertId as number, $db).catch((err) => {
          toastStore.trigger(getToastSettings(ToastType.SUCCESS, err));
        });

        // Add new pokemon to pokemonList
        $pokemonList.push([
          res?.lastInsertId as number,
          newPokemon.dex_number,
          newPokemon.name,
          newPokemon.types,
        ]);

        toastStore.trigger(
          getToastSettings(ToastType.SUCCESS, "New Pokemon created!"),
        );
      })
      .catch((err) => {
        toastStore.trigger(
          getToastSettings(
            ToastType.ERROR,
            `Error creating new Pokemon!: ${err}`,
          ),
        );
      });
  }
</script>

<Button
  title="Save New Pokemon"
  class="mt-2 w-40"
  onClick={createNewPokemon}
  disabled={newPokemon.dex_number === 0 ||
    newPokemon.name === "" ||
    newSpriteImage === ""}
/>

<div class="flex flex-row gap-5">
  <AutoComplete
    bind:value={pokemonSearch[1]}
    placeholder="Search Pokemon"
    options={pokemonListOptions}
    popupId="pokemon-search"
    onSelection={(e) => {
      pokemonSearch = [e.detail.value, e.detail.label];
    }}
    showChevron={false}
    class="w-48"
  />
  <Button
    title="Copy Pokemon Details"
    class="mt-2 w-48"
    disabled={pokemonSearch[0] === 0}
    onClick={copyPokemonDetails}
  />
</div>

<div class="mb-4 mt-4">
  <label
    for="sprite-image"
    class="block text-sm font-medium leading-6 text-gray-900"
    >Pokemon Sprite*</label
  >
  {#if newSpriteImage !== ""}
    <img src={newSpriteImage} alt="Sprite" />
  {/if}
  <input
    id="sprite-image"
    type="file"
    accept="image/png"
    class="mt-2"
    onchange={onImageUpload}
  />
</div>

<div class="mb-5 mt-5 flex flex-row gap-5">
  <TextInput
    label="Pokemon Name*"
    bind:value={newPokemon.name}
    class="w-40"
    inputHandler={(e: any) => {
      newPokemon.name = e.target.value.toLowerCase().replaceAll(" ", "-");
    }}
  />
  <NumberInput
    label="Dex Number*"
    bind:value={newPokemon.dex_number}
    class="w-40"
  />
</div>

<TabGroup>
  <Tab bind:group={tabSet} name="pokemon-details" value={0} class="text-sm"
    >Details</Tab
  >
  <Tab bind:group={tabSet} name="pokemon-moves" value={1} class="text-sm"
    >Moves</Tab
  >
  <div slot="panel">
    {#if tabSet === 0}
      <PokemonDetailsTab bind:pokemon={newPokemon} isNewPokemon={true} />
    {/if}
    {#if tabSet === 1}
      <PokemonMovesetTab
        moveset={copiedMoveset}
        pokemonId={0}
        generatePokemonPage={() => {}}
      />
    {/if}
  </div>
  <!-- {#snippet panel()}
  {/snippet} -->
</TabGroup>
