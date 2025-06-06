<script lang="ts">
  import { Button } from "$lib/components/ui/button/index.js";
  import {
    type Pokemon,
    pokemonList,
    type PokemonMove,
  } from "../../store/pokemon";
  import { db } from "../../store/db";
  import { BaseDirectory, writeFile } from "@tauri-apps/plugin-fs";
  import { selectedWiki } from "../../store";
  import { addMoves, base64ToArray, typeColors } from "$lib/utils";
  import capitalizeWords from "$lib/utils/capitalizeWords";
  import type { QueryResult } from "@tauri-apps/plugin-sql";
  import * as Card from "$lib/components/ui/card/index.js";
  import Autocomplete from "./ui/Autocomplete.svelte";
  import { toast } from "svelte-sonner";
  import SaveIcon from "@lucide/svelte/icons/save";
  import PokemonStat from "./ui/PokemonStat.svelte";
  import { Input } from "$lib/components/ui/input/index.js";
  import * as Select from "$lib/components/ui/select/index.js";
  import { Label } from "./ui/label";
  import { types as pokemonTypes } from "../../store/types";
  import { abilitiesList as pokemonAbilities } from "../../store/abilities";

  let pokemonSearch: [number, string] = $state([0, ""]);
  let pokemonSearchOption = $state(false);
  let triggerRef = $state<HTMLButtonElement>(null!);
  let newSpriteImage: string = $state("");
  let searchingPokemon = $state("");
  let newPokemon: Pokemon = $state({
    id: 0,
    dex_number: 0,
    name: "",
    types: "normal",
    abilities: "overgrow",
    hp: 0,
    attack: 0,
    defense: 0,
    sp_attack: 0,
    sp_defense: 0,
    speed: 0,
    evolution_method: "no_change",
    evolves_into: null,
    evolution_item: null,
    evolution_level: null,
    evolution_other: null,
    render: "false",
  });
  let baseTotal = $derived(
    newPokemon.hp +
      newPokemon.attack +
      newPokemon.defense +
      newPokemon.sp_attack +
      newPokemon.sp_defense +
      newPokemon.speed,
  );
  let copiedMoveset: PokemonMove[] = $state([]);

  let pokemonListOptions = $pokemonList.map(([id, _, name]) => ({
    label: capitalizeWords(name),
    value: id,
  }));

  let options = $derived(
    pokemonListOptions
      .filter((pokemon) =>
        pokemon.label.toLowerCase().includes(searchingPokemon.toLowerCase()),
      )
      .slice(0, 8),
  );

  let abilities = $derived.by(() => {
    let result = newPokemon.abilities.split(",");
    if (result.length === 1) {
      result.push("");
    }
    return result;
  });
  let types: string[] = $derived.by(() => {
    let result = newPokemon.types.split(",");
    if (result.length === 1) {
      result.push("");
    }
    return result;
  });

  function onImageUpload(e: any) {
    let file = e.target.files[0];
    let reader = new FileReader();
    reader.onloadend = (e) => {
      let base64 = e.target?.result as string;
      if (!base64.includes("data:image/png;base64,")) {
        toast.error("Invalid image format! Please upload a PNG file!");
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
        toast.error(`Error loading Pokemon moveset!: \n ${err}`);
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
        toast.error(`Error loading Pokemon details!: \n ${err}`);
      });
  }

  async function createNewPokemon() {
    await $db
      .execute(
        `INSERT INTO pokemon (
          dex_number,
          name,
          types,
          abilities,
          hp, attack, defense, sp_attack, sp_defense, speed, evolution_method
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11);`,
        [
          newPokemon.dex_number,
          newPokemon.name.toLowerCase(),
          newPokemon.types,
          newPokemon.abilities,
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
          toast.error(`Error writing image to file!: ${err}`);
        });

        addMoves(copiedMoveset, res.lastInsertId as number).catch((err) => {
          toast.error(err);
        });

        // Add new pokemon to pokemonList
        $pokemonList.push([
          res?.lastInsertId as number,
          newPokemon.dex_number,
          newPokemon.name,
          newPokemon.types,
        ]);

        toast.success("New Pokemon created!");
      })
      .catch((err) => {
        toast.error(`Error creating new Pokemon!: ${err}`);
      });
  }

  function onTypeChange(type: string, type_number: number) {
    if (type_number === 1) {
      if (type === "none") {
        toast.error(
          "First Type cannot be empty. Pokemon need at least one type",
        );
        return;
      }
      types[0] = type;
    } else {
      types[1] = type;
    }

    if (types[0] === types[1] || types[1] === "none") {
      newPokemon.types = types[0];
      types[1] = "";
      return;
    }

    newPokemon.types = types.join(",");

    // This is meant to drop the trailing comma if it exists
    if (newPokemon.types[newPokemon.types.length - 1] === ",") {
      newPokemon.types = newPokemon.types.slice(0, -1);
    }
  }

  function onAbilityChange(ability: string, ability_number: number) {
    if (ability === "None") {
      ability = "";
    }

    if (ability_number === 1) {
      if (ability === "") {
        toast.error(
          "First Ability cannot be empty. Pokemon need at least one ability",
        );
        return;
      }
      abilities[0] = ability;
    } else if (ability_number === 2) {
      abilities[1] = ability;
    } else if (ability_number === 3) {
      abilities[2] = ability;
    }

    if (abilities[0] === abilities[1]) {
      newPokemon.abilities = abilities[0];
      return;
    }
    newPokemon.abilities = abilities.join(",");
  }
</script>

<Card.Root>
  <Card.Content class="flex flex-row gap-3">
    <Autocomplete
      open={pokemonSearchOption}
      value={pokemonSearch[1]}
      bind:searcher={searchingPokemon}
      {options}
      placeholder="Search Pokemon To Copy"
      onSelect={(option) => {
        pokemonSearch = [option.value, option.label];
      }}
      class="w-[15rem]"
    />
    <div class="w-full flex flex-row justify-between">
      <Button
        variant="outline"
        class="cursor-pointer"
        disabled={pokemonSearch[0] === 0}
        onclick={copyPokemonDetails}
      >
        Copy Pokemon Details</Button
      >
      <Button
        class="cursor-pointer"
        onclick={createNewPokemon}
        disabled={newPokemon.dex_number === 0 ||
          newPokemon.name === "" ||
          newSpriteImage === ""}
      >
        <SaveIcon />
        Create New Pokemon</Button
      >
    </div>
  </Card.Content>
</Card.Root>
<div class="bg-slate-200 flex flex-col gap-2 p-1 mt-5 rounded-md w-3/4">
  <p class="p-1 text-sm text-slate-500">
    <strong class="text-yellow-500">Note</strong>: To see New Pokemon in Pokemon
    Tab, go to route page and then back to pokemon page. A slight bug is causing
    the new pokemon not to appear in the list.
  </p>
</div>

<div class="flex flex-row gap-5 mb-5">
  <Card.Root class="mt-5">
    <Card.Content class="flex flex-col gap-10">
      <section class="flex flex-row gap-5">
        {#if newSpriteImage !== ""}
          <div
            class={`size-24 rounded-full ${typeColors[newPokemon.types.split(",")[0]].background} grid align-center place-content-center shadow-xl`}
          >
            <img
              src={newSpriteImage}
              alt={newPokemon.name}
              class="size-20 object-contain"
            />
          </div>
        {/if}
        <div class="flex flex-col justify-between">
          <div class="text-[25px] font-bold">
            {capitalizeWords(newPokemon.name)}
          </div>
          <div class="text-slate-400">#{newPokemon.dex_number}</div>
          <div class="flex flex-row gap-2">
            {#each newPokemon.types.split(",") as type}
              <div
                class={`text-sm px-2 rounded-lg font-medium ${typeColors[type].background} ${typeColors[type].text}`}
              >
                {capitalizeWords(type)}
              </div>
            {/each}
          </div>
        </div>
      </section>
      <div>
        <Label
          for="sprite-image"
          class="text-sm font-medium text-slate-700 mb-2 block"
          >Pokemon Sprite*</Label
        >
        <Input
          type="file"
          id="sprite-image"
          accept="image/png"
          onchange={onImageUpload}
          class="cursor-pointer"
          required
        />
      </div>
      <section class="flex flex-row gap-5 justify-between">
        <div>
          <Label
            for="pokemon-name"
            class="text-sm font-medium text-slate-700 mb-2 block"
            >New Pokemon Name*</Label
          >
          <Input
            type="text"
            id="pokemon-name"
            bind:value={newPokemon.name}
            oninput={(e: any) => {
              newPokemon.name = e.target.value
                .toLowerCase()
                .replaceAll(" ", "-");
            }}
            required
          />
        </div>
        <div>
          <Label
            for="dex-number"
            class="text-sm font-medium text-slate-700 mb-2 block"
            >Dex Number</Label
          >
          <Input
            type="number"
            id="dex-number"
            bind:value={newPokemon.dex_number}
            required
          />
        </div>
      </section>
      <section class="flex flex-row gap-5 justify-between">
        <div>
          <Label
            for="pokemon-type"
            class="text-sm font-medium text-slate-700 mb-2 block">Type 1</Label
          >
          <Select.Root type="single" bind:value={types[0]}>
            <Select.Trigger id="pokemon-type" class="w-[11rem]">
              {capitalizeWords(types[0])}
            </Select.Trigger>
            <Select.Content>
              {#each $pokemonTypes as type}
                <Select.Item
                  value={type}
                  onclick={() => onTypeChange(type, 1)}
                  label={type}
                >
                  {capitalizeWords(type)}
                </Select.Item>
              {/each}
            </Select.Content>
          </Select.Root>
        </div>
        <div>
          <Label
            for="pokemon-type-2"
            class="text-sm font-medium text-slate-700 mb-2 block">Type 2</Label
          >
          <Select.Root type="single" bind:value={types[1]}>
            <Select.Trigger id="pokemon-type-2" class="w-[11rem]">
              {#if types[1] === ""}
                None
              {:else}
                {capitalizeWords(types[1])}
              {/if}
            </Select.Trigger>
            <Select.Content>
              {#each $pokemonTypes as type}
                <Select.Item
                  value={type}
                  onclick={() => onTypeChange(type, 2)}
                  label={type}
                >
                  {capitalizeWords(type)}
                </Select.Item>
              {/each}
            </Select.Content>
          </Select.Root>
        </div>
      </section>
      <section class="flex flex-col gap-10">
        <section class="flex flex-row justify-between">
          <div>
            <Label
              for="pokemon-ability-1"
              class="text-sm font-medium text-slate-700 mb-2 block"
              >Ability 1</Label
            >
            <Select.Root type="single" bind:value={abilities[0]}>
              <Select.Trigger id="pokemon-ability-1" class="w-[11rem]">
                {capitalizeWords(abilities[0])}
              </Select.Trigger>
              <Select.Content>
                {#each $pokemonAbilities as [id, ability]}
                  <Select.Item
                    value={ability}
                    label={ability}
                    onclick={() => {
                      onAbilityChange(ability, 1);
                    }}
                  >
                    {capitalizeWords(ability)}
                  </Select.Item>
                {/each}
              </Select.Content>
            </Select.Root>
          </div>
          <div>
            <Label
              for="pokemon-ability-2"
              class="text-sm font-medium text-slate-700 mb-2 block"
              >Ability 2</Label
            >
            <Select.Root type="single" bind:value={abilities[1]}>
              <Select.Trigger id="pokemon-ability-2" class="w-[11rem]">
                {#if abilities[1] === ""}
                  None
                {:else}
                  {capitalizeWords(abilities[1])}
                {/if}
              </Select.Trigger>
              <Select.Content>
                {#each $pokemonAbilities as [id, ability]}
                  <Select.Item
                    value={ability}
                    label={ability}
                    onclick={() => {
                      onAbilityChange(ability, 2);
                    }}
                  >
                    {capitalizeWords(ability)}
                  </Select.Item>
                {/each}
              </Select.Content>
            </Select.Root>
          </div>
        </section>
        <div>
          <Label
            for="pokemon-ability-hidden"
            class="text-sm font-medium text-slate-700 mb-2 block"
            >Hidden Ability</Label
          >
          <Select.Root type="single" bind:value={abilities[2]}>
            <Select.Trigger id="pokemon-ability-hidden" class="w-full">
              {#if abilities[2] === ""}
                None
              {:else}
                {capitalizeWords(abilities[2])}
              {/if}
            </Select.Trigger>
            <Select.Content>
              {#each $pokemonAbilities as [id, ability]}
                <Select.Item
                  value={ability}
                  label={ability}
                  onclick={() => {
                    onAbilityChange(ability, 3);
                  }}
                >
                  {capitalizeWords(ability)}
                </Select.Item>
              {/each}
            </Select.Content>
          </Select.Root>
        </div>
      </section>
    </Card.Content>
  </Card.Root>
  <div class="flex flex-col w-full">
    <Card.Root class="mt-5 h-fit w-full">
      <Card.Header>
        <Card.Title>Base Stats</Card.Title>
        <Card.Description>Base Total: {baseTotal}</Card.Description>
      </Card.Header>
      <Card.Content class="grid grid-cols-2 gap-4">
        <PokemonStat label="HP" bind:value={newPokemon.hp} />
        <PokemonStat label="Attack" bind:value={newPokemon.attack} />
        <PokemonStat label="Defense" bind:value={newPokemon.defense} />
        <PokemonStat label="Special Attack" bind:value={newPokemon.sp_attack} />
        <PokemonStat
          label="Special Defense"
          bind:value={newPokemon.sp_defense}
        />
        <PokemonStat label="Speed" bind:value={newPokemon.speed} />
      </Card.Content>
    </Card.Root>
  </div>
</div>
