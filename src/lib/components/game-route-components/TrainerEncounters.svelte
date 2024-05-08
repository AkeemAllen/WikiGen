<script lang="ts">
import BaseModal from "$lib/components/BaseModal.svelte";

import {
  getToastStore,
  popup,
  type AutocompleteOption,
} from "@skeletonlabs/skeleton";
import NumberInput from "../NumberInput.svelte";
import Button from "../Button.svelte";
import { pokemonList, pokemon as storePokemon } from "../../../store/pokemon";
import { selectedWiki } from "../../../store";
import { routes, type TrainerInfo } from "../../../store/gameRoutes";
import TextInput from "../TextInput.svelte";
import { BaseDirectory, writeTextFile } from "@tauri-apps/api/fs";
import { setUniquePokemonId } from "$lib/utils";
import { IconDots, IconEdit, IconTrash } from "@tabler/icons-svelte";
import _ from "lodash";
import AutoComplete from "../AutoComplete.svelte";

const toastStore = getToastStore();

export let routeName: string;
let trainerName: string = "";
let pokemonName: string = "";
let level: number = 0;

let spriteModalOpen: boolean = false;
let spriteName: string = "";

$: trainers = $routes.routes[routeName].trainers;

let pokemonListOptions: AutocompleteOption<string | number>[] =
  $pokemonList.map(([name, id]) => ({ label: name, value: id }));

function onPokemonNameSelected(
  event: CustomEvent<AutocompleteOption<string | number>>,
): void {
  pokemonName = event.detail.label;
}

async function addPokemonToTrainer() {
  let trainers = {
    ...$routes.routes[routeName].trainers,
  };

  $routes.routes[routeName].trainers = {
    ...$routes.routes[routeName].trainers,
    [trainerName]: {
      ...trainers[trainerName],
      pokemon_team: [
        ...(trainers[trainerName]?.pokemon_team ?? []),
        {
          id: $pokemonList.find(
            ([name, _]) => name === pokemonName,
          )?.[1] as number,
          unique_id: setUniquePokemonId(
            $routes.routes[routeName].trainers as {
              [key: string]: TrainerInfo;
            },
            trainerName,
            pokemonName,
            $pokemonList,
          ),
          name: pokemonName,
          level,
        },
      ],
    },
  };
  await writeTextFile(
    `${$selectedWiki.name}/data/routes.json`,
    JSON.stringify($routes),
    { dir: BaseDirectory.AppData },
  );
}

async function deletePokemonFromTrainer(uniqueId: string, trainerName: string) {
  let updatedTrainers = {
    ...$routes.routes[routeName].trainers,
  };
  updatedTrainers[trainerName].pokemon_team = updatedTrainers[
    trainerName
  ].pokemon_team.filter((pokemon) => pokemon.unique_id !== uniqueId);
  if (updatedTrainers[trainerName].pokemon_team.length === 0) {
    delete updatedTrainers[trainerName];
  }

  $routes.routes[routeName].trainers = updatedTrainers;

  await writeTextFile(
    `${$selectedWiki.name}/data/routes.json`,
    JSON.stringify($routes),
    { dir: BaseDirectory.AppData },
  );
}

async function setTrainerSprite() {
  let updatedTrainers = {
    ...$routes.routes[routeName].trainers,
  };
  updatedTrainers[trainerName].sprite = spriteName.toLowerCase();

  $routes.routes[routeName].trainers = updatedTrainers;

  await writeTextFile(
    `${$selectedWiki.name}/data/routes.json`,
    JSON.stringify($routes),
    { dir: BaseDirectory.AppData },
  );
}
</script>

<div class="flex flex-row gap-x-5">
  <div class="w-40">
    <TextInput
      id="trainer-name"
      label="Trainer Name"
      bind:value={trainerName}
    />
  </div>

  <AutoComplete
    label="Pokemon for current encounter type"
    placeholder="Pokemon Name"
    bind:value={pokemonName}
    options={pokemonListOptions}
    popupId="popupAutoComplete"
    onSelection={onPokemonNameSelected}
  />

  <NumberInput label="Level" bind:value={level} />
  <div class="mt-8 w-32">
    <Button
      title="Add Encounter"
      disabled={pokemonName === "" || level === 0 || trainerName === ""}
      onClick={addPokemonToTrainer}
    />
  </div>
</div>

<!-- Sprite Modal -->
<BaseModal bind:open={spriteModalOpen}>
  <TextInput
    id="sprite-name"
    label="Sprite Name"
    placeholder="Sprites are loaded from pokemon https://play.pokemonshowdown.com/sprites/trainers/"
    bind:value={spriteName}
  />
  <Button
    title="Set Sprite"
    disabled={spriteName === ""}
    onClick={setTrainerSprite}
  />
</BaseModal>

<div class="mt-5 flex flex-col gap-y-5">
  {#each Object.entries($routes.routes[routeName].trainers ?? {}) as [_trainerName, trainerInfo]}
    <div>
      <strong class="flex flex-row items-center gap-x-4">
        {_.capitalize(_trainerName)}
        <button
          class="hover:cursor-pointer"
          use:popup={{
            event: "click",
            target: "popupTrainerMenu",
            placement: "right",
          }}
        >
          <IconDots size={20} color="gray" />
        </button>
        <div class="card z-10 bg-white p-2" data-popup="popupTrainerMenu">
          <button
            class="w-full rounded-md p-2 text-left text-sm hover:bg-slate-300"
            on:click={() => {
                    spriteModalOpen = true;
                    trainerName = _trainerName;
                }}
            >Add Sprite</button
          >
        </div>
      </strong>
      {#if trainerInfo.sprite}
        <img
          src={`https://play.pokemonshowdown.com/sprites/trainers/${trainerInfo.sprite}.png`}
          alt={_trainerName}
          class="m-0 justify-self-center"
        />
      {/if}
      <div class="mt-2 grid grid-cols-6 gap-5">
        {#each trainerInfo.pokemon_team as pokemon}
          <div
            class="group card relative grid !bg-transparent p-2 shadow-md hover:cursor-pointer"
          >
            <img
              src={$storePokemon.pokemon[pokemon.id].sprite}
              alt={pokemon.name}
              class="m-0 justify-self-center"
            />
            <div class="w-full rounded-md border-2">
              <p class="text-center">
                {_.capitalize(pokemon.name)}
              </p>
              <p class="text-center">
                {pokemon.level}
              </p>
            </div>
            <button
              class="invisible absolute right-8 top-2 group-hover:visible"
            >
              <IconEdit size={16} color="grey" />
            </button>
            <button
              class="invisible absolute right-2 top-2 group-hover:visible"
              on:click={() => deletePokemonFromTrainer(pokemon.unique_id, _trainerName)}
            >
              <IconTrash size={16} color="grey" />
            </button>
          </div>
        {/each}
      </div>
      <div></div>
    </div>
  {/each}
</div>
