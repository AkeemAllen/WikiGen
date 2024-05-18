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
import { setUniquePokemonId, sortTrainersByPosition } from "$lib/utils";
import { IconDots, IconEdit, IconTrash } from "@tabler/icons-svelte";
import _ from "lodash";
import AutoComplete from "../AutoComplete.svelte";
import TrainerPokemonCard from "../TrainerPokemonCard.svelte";
import MultiSelect from "svelte-multiselect";
import { invoke } from "@tauri-apps/api/tauri";

const toastStore = getToastStore();

export let routeName: string;
let trainerName: string = "";
let pokemonName: string = "";
let level: number = 0;

let trainerToUpdate: string = "";

let spriteModalOpen: boolean = false;
let trainerVersionsModalOpen: boolean = false;
let positionModalOpen: boolean = false;
let spriteName: string = "";

$: trainers = $routes.routes[routeName].trainers ?? {};

let pokemonListOptions: AutocompleteOption<string | number>[] =
  $pokemonList.map(([name, id]) => ({ label: name, value: id }));

function onPokemonNameSelected(
  event: CustomEvent<AutocompleteOption<string | number>>,
): void {
  pokemonName = event.detail.label;
}

async function generateRoutePage() {
  await invoke("generate_single_route_page_with_handle", {
    wikiName: $selectedWiki.name,
    routeName,
  }).catch((e) => {
    console.error(e);
  });
}

async function addPokemonToTrainer() {
  let trainers = {
    ...$routes.routes[routeName].trainers,
  };

  if (trainers[trainerName] === undefined) {
    trainers[trainerName] = {
      position: Object.keys(trainers).length,
      sprite: "",
      versions: [],
      pokemon_team: [],
    };
  }

  $routes.routes[routeName].trainers = {
    ...$routes.routes[routeName].trainers,
    [trainerName]: {
      ...trainers[trainerName],
      pokemon_team: [
        ...trainers[trainerName].pokemon_team,
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
          item: "-",
          nature: "-",
          ability: "-",
          trainer_versions: [],
          moves: [],
        },
      ],
    },
  };

  let sortedTrainers = sortTrainersByPosition($routes, routeName);
  $routes.routes[routeName].trainers = sortedTrainers;

  await writeTextFile(
    `${$selectedWiki.name}/data/routes.json`,
    JSON.stringify($routes),
    { dir: BaseDirectory.AppData },
  ).then(() => {
    generateRoutePage();
  });
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
  ).then(() => {
    generateRoutePage();
  });
}

async function setTrainerSprite() {
  let updatedTrainers = {
    ...$routes.routes[routeName].trainers,
  };
  updatedTrainers[trainerToUpdate].sprite = spriteName.toLowerCase();

  $routes.routes[routeName].trainers = updatedTrainers;

  await writeTextFile(
    `${$selectedWiki.name}/data/routes.json`,
    JSON.stringify($routes),
    { dir: BaseDirectory.AppData },
  ).then(() => {
    generateRoutePage();
  });

  trainerToUpdate = "";
  spriteName = "";
}

async function setPosition() {
  $routes.routes[routeName].trainers = sortTrainersByPosition(
    $routes,
    routeName,
  );

  positionModalOpen = false;

  await writeTextFile(
    `${$selectedWiki.name}/data/routes.json`,
    JSON.stringify($routes),
    { dir: BaseDirectory.AppData },
  ).then(() => {
    generateRoutePage();
  });
}
</script>

<div class="flex flex-row gap-x-5">
  <TextInput
    class="w-40"
    id="trainer-name"
    label="Trainer Name"
    bind:value={trainerName}
  />

  <AutoComplete
    label="Pokemon for current encounter type"
    placeholder="Pokemon Name"
    bind:value={pokemonName}
    options={pokemonListOptions}
    popupId="popupAutoComplete"
    onSelection={onPokemonNameSelected}
  />

  <NumberInput label="Level" bind:value={level} />
  <Button
    title="Add Encounter"
    class="mt-8 w-32"
    disabled={pokemonName === "" || level === 0 || trainerName === ""}
    onClick={addPokemonToTrainer}
  />
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

<!-- Trainer Versions Modal -->
<BaseModal bind:open={trainerVersionsModalOpen} class="gap-y-1">
  <div>
    <label
      for="versions"
      class="block text-sm font-medium leading-6 text-gray-900"
      >Trainer Versions</label
    >
    <MultiSelect
      id="versions"
      bind:selected={trainers[trainerToUpdate].versions}
      allowUserOptions={true}
      options={trainers[trainerToUpdate].versions ?? []}
      on:change={async (e) => {await writeTextFile(
      `${$selectedWiki.name}/data/routes.json`,
      JSON.stringify($routes),
      { dir: BaseDirectory.AppData },
    )}}
    />
  </div>
</BaseModal>

<BaseModal bind:open={positionModalOpen}>
  <NumberInput
    label="Position"
    bind:value={trainers[trainerToUpdate].position}
  />
  <Button title="Set Position" onClick={setPosition} />
</BaseModal>

<div class="mt-5 flex flex-col gap-y-5">
  {#each Object.entries($routes.routes[routeName].trainers ?? {}) as [name, trainerInfo], index}
    <div>
      <strong class="flex flex-row items-center gap-x-4">
        {_.capitalize(name)}
        <button
          class="hover:cursor-pointer"
          use:popup={{
            event: "click",
            target: "trainerMenu" + index,
            placement: "right",
          }}
        >
          <IconDots size={20} color="gray" />
        </button>
        <div class="card z-10 bg-white p-2" data-popup="trainerMenu{index}">
          <button
            class="w-full rounded-md p-2 text-left text-sm hover:bg-slate-300"
            on:click={() => {
                    spriteModalOpen = true;
                    trainerToUpdate = name;
                }}
            >Add Sprite</button
          >
          <button
            class="w-full rounded-md p-2 text-left text-sm hover:bg-slate-300"
            on:click={() => {
                    trainerVersionsModalOpen = true;
                    trainerToUpdate = name;
                }}
            >Modify Trainer Versions</button
          >
          <button
            class="w-full rounded-md p-2 text-left text-sm hover:bg-slate-300"
            on:click={() => {
                    positionModalOpen = true;
                    trainerToUpdate = name;
                }}
            >Change Position</button
          >
        </div>
      </strong>
      {#if trainerInfo.sprite}
        <img
          src={`https://play.pokemonshowdown.com/sprites/trainers/${trainerInfo.sprite}.png`}
          alt={name}
          class="m-0 justify-self-center"
        />
      {/if}
      <div class="mt-2 grid grid-cols-6 gap-5">
        {#each trainerInfo.pokemon_team as pokemon}
          <TrainerPokemonCard
            pokemon={pokemon}
            trainerName={name}
            trainerVersions={trainerInfo.versions ?? []}
            deletePokemon={deletePokemonFromTrainer}
            routeName={routeName}
          />
        {/each}
      </div>
      <div></div>
    </div>
  {/each}
</div>
