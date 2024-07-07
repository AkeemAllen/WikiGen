<script lang="ts">
import BaseModal from "$lib/components/BaseModal.svelte";

import { getToastStore, type AutocompleteOption } from "@skeletonlabs/skeleton";
import NumberInput from "../NumberInput.svelte";
import Button from "../Button.svelte";
import { pokemonList } from "../../../store/pokemon";
import { selectedWiki } from "../../../store";
import {
  routes,
  type TrainerInfo,
  type TrainerPokemon,
} from "../../../store/gameRoutes";
import TextInput from "../TextInput.svelte";
import { BaseDirectory, writeTextFile } from "@tauri-apps/api/fs";
import { setUniquePokemonId, sortTrainersByPosition } from "$lib/utils";
import _ from "lodash";
import AutoComplete from "../AutoComplete.svelte";
import TrainerPokemonCard from "../TrainerPokemonCard.svelte";
import MultiSelect from "svelte-multiselect";
import { invoke } from "@tauri-apps/api/tauri";
import TrainerMenu from "../modals/TrainerMenu.svelte";
import EditTrainerPokemonModal from "../modals/EditTrainerPokemonModal.svelte";
import { shortcut } from "@svelte-put/shortcut";

const toastStore = getToastStore();

export let routeName: string;
let trainerName: string = "";
let pokemonSearchName: string = "";
let level: number = 0;

let editPokemonModalOpen: boolean = false;
let trainerToUpdate: string = "";
let currentTrainerPokemon: TrainerPokemon = {} as TrainerPokemon;
let currentTrainerVersions: string[] = [];

let spriteModalOpen: boolean = false;
let trainerVersionsModalOpen: boolean = false;
let positionModalOpen: boolean = false;
let spriteName: string = "";

let routeTrainers = _.cloneDeep($routes.routes[routeName].trainers);
let originalTrainers = _.cloneDeep(routeTrainers);
$: trainerOptions = Object.keys(routeTrainers).map((trainer) => ({
  label: trainer,
  value: trainer,
}));

$: console.log({ routeTrainers });

let pokemonListOptions: AutocompleteOption<string | number>[] =
  $pokemonList.map(([name, id]) => ({ label: name, value: id }));

function onPokemonNameSelected(
  event: CustomEvent<AutocompleteOption<string | number>>,
): void {
  pokemonSearchName = event.detail.label;
}

function addPokemonToTrainer() {
  if (routeTrainers[trainerName] === undefined) {
    routeTrainers[trainerName] = {
      position: Object.keys(routeTrainers).length,
      sprite: "",
      versions: [],
      pokemon_team: [],
    };
  }

  let id = $pokemonList.find(
    ([name, _]) => name === pokemonSearchName,
  )?.[1] as number;
  let uniqueId = setUniquePokemonId(
    routeTrainers as {
      [key: string]: TrainerInfo;
    },
    trainerName,
    pokemonSearchName,
    $pokemonList,
  );

  routeTrainers[trainerName].pokemon_team = [
    ...routeTrainers[trainerName].pokemon_team,
    {
      id: id,
      unique_id: uniqueId,
      name: pokemonSearchName,
      level,
      item: "",
      nature: "",
      ability: "",
      trainer_versions: [],
      moves: [],
    },
  ];

  let sortedTrainers = sortTrainersByPosition(routeTrainers);
  routeTrainers = sortedTrainers;
}

function deletePokemonFromTrainer(uniqueId: string, trainerName: string) {
  let updatedTrainers = {
    ...routeTrainers,
  };
  updatedTrainers[trainerName].pokemon_team = updatedTrainers[
    trainerName
  ].pokemon_team.filter((pokemon) => pokemon.unique_id !== uniqueId);
  if (updatedTrainers[trainerName].pokemon_team.length === 0) {
    delete updatedTrainers[trainerName];
  }

  routeTrainers = updatedTrainers;
}

function nextTrainerPokemon() {
  let existingPokemon = routeTrainers[trainerToUpdate].pokemon_team.find(
    (p) => p.unique_id === currentTrainerPokemon.unique_id,
  ) as TrainerPokemon;

  let index =
    routeTrainers[trainerToUpdate].pokemon_team.indexOf(existingPokemon);
  if (index === routeTrainers[trainerToUpdate].pokemon_team.length - 1) {
    toastStore.trigger({
      message: "No more trainer pokemon",
      timeout: 3000,
      background: "variant-filled-error",
    });
    return;
  }
  currentTrainerPokemon = _.cloneDeep(
    routeTrainers[trainerToUpdate].pokemon_team[index + 1],
  );
}

function prevTrainerPokemon() {
  let existingPokemon = routeTrainers[trainerToUpdate].pokemon_team.find(
    (p) => p.unique_id === currentTrainerPokemon.unique_id,
  ) as TrainerPokemon;

  let index =
    routeTrainers[trainerToUpdate].pokemon_team.indexOf(existingPokemon);
  if (index === 0) {
    toastStore.trigger({
      message: "No more trainer pokemon",
      timeout: 3000,
      background: "variant-filled-error",
    });
    return;
  }
  currentTrainerPokemon = _.cloneDeep(
    routeTrainers[trainerToUpdate].pokemon_team[index - 1],
  );
}

async function savePokemonChanges(trainerName: string) {
  let pokemonToSave = routeTrainers[trainerName].pokemon_team.find(
    (p) => p.unique_id === currentTrainerPokemon.unique_id,
  ) as TrainerPokemon;
  let index = routeTrainers[trainerName].pokemon_team.indexOf(pokemonToSave);

  routeTrainers[trainerName].pokemon_team[index] = currentTrainerPokemon;
  saveChanges();
}

async function saveChanges() {
  routeTrainers = sortTrainersByPosition(routeTrainers);

  $routes.routes[routeName].trainers = routeTrainers;

  routeTrainers = _.cloneDeep($routes.routes[routeName].trainers);
  originalTrainers = _.cloneDeep(routeTrainers);

  await writeTextFile(
    `${$selectedWiki.name}/data/routes.json`,
    JSON.stringify($routes),
    { dir: BaseDirectory.AppData },
  ).then(() => {
    invoke("generate_single_route_page_with_handle", {
      wikiName: $selectedWiki.name,
      routeName,
    })
      .then(() => {
        toastStore.trigger({
          message: "Changes saved successfully",
          timeout: 3000,
          background: "variant-filled-success",
        });
      })
      .catch((e) => {
        console.error(e);
      });
  });
}
</script>

<!-- Sprite Modal -->
<BaseModal bind:open={spriteModalOpen} class="w-[25rem]">
  {#if spriteName !== ""}
    <img
      src={`https://play.pokemonshowdown.com/sprites/trainers/${spriteName.toLowerCase()}.png`}
      alt="Trainer Sprite"
      class="h-32 w-32"
    />
  {/if}
  <TextInput
    id="sprite-name"
    label="Sprite Name"
    placeholder="Type name to see sprite. Eg. red"
    bind:value={spriteName}
  />
  <Button
    title="Set Sprite"
    class="w-32"
    disabled={spriteName === ""}
    onClick={() => {
        routeTrainers[trainerToUpdate].sprite = spriteName.toLowerCase();
        trainerToUpdate = "";
        spriteName = "";
        spriteModalOpen = false;
    }}
  />
</BaseModal>

<!-- Trainer Versions Modal -->
<BaseModal bind:open={trainerVersionsModalOpen} class="w-[25rem] gap-y-1">
  <div>
    <label
      for="versions"
      class="block text-sm font-medium leading-6 text-gray-900"
      >Trainer Versions</label
    >
    <MultiSelect
      id="versions"
      bind:selected={routeTrainers[trainerToUpdate].versions}
      allowUserOptions={true}
      options={routeTrainers[trainerToUpdate].versions ?? []}
      style="height: 36px; border-color: rgb(209 213 219); border-radius: 0.375rem; box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05); font-size: 0.875rem;"
    />
  </div>
  <Button
    class="w-32"
    title="Save Versions"
    disabled={_.isEqual(routeTrainers, originalTrainers)}
    onClick={() => {
      saveChanges()
      trainerVersionsModalOpen = false;
    }}
  />
</BaseModal>

<!-- Position Modal -->
<BaseModal bind:open={positionModalOpen} class="w-[25rem]">
  <NumberInput
    label="Position"
    bind:value={routeTrainers[trainerToUpdate].position}
  />
  <Button
    class="w-32"
    title="Set Position"
    disabled={_.isEqual(routeTrainers, originalTrainers)}
    onClick={() => {
    routeTrainers = sortTrainersByPosition(routeTrainers);
    positionModalOpen = false;
  }}
  />
</BaseModal>

<EditTrainerPokemonModal
  bind:open={editPokemonModalOpen}
  pokemon={currentTrainerPokemon}
  trainerToUpdate={trainerToUpdate}
  trainerVersions={currentTrainerVersions}
  nextTrainerPokemon={nextTrainerPokemon}
  prevTrainerPokemon={prevTrainerPokemon}
  savePokemonChanges={savePokemonChanges}
/>

<div
  class="sticky top-0 z-10 flex flex-row gap-x-5 rounded-md bg-white pb-1 shadow-sm"
>
  <AutoComplete
    label="Trainer Name"
    bind:value={trainerName}
    options={trainerOptions}
    popupId="popupTrainerNames"
    onSelection={(e) => {
      trainerName = e.detail.label;
    }}
    showChevron={false}
    class="w-40"
  />
  <AutoComplete
    label="Pokemon for current encounter type"
    placeholder="Pokemon Name"
    bind:value={pokemonSearchName}
    options={pokemonListOptions}
    popupId="popupPokemonNames"
    onSelection={onPokemonNameSelected}
    showChevron={false}
  />

  <NumberInput label="Level" bind:value={level} class="w-32" max={100} />
  <Button
    title="Add Encounter"
    class="mt-8 w-32"
    disabled={pokemonSearchName === "" || level === 0 || trainerName === ""}
    onClick={addPokemonToTrainer}
  />
  <Button
    class="mt-8 w-32"
    title="Save Changes"
    disabled={_.isEqual(routeTrainers, originalTrainers)}
    onClick={saveChanges}
  />
</div>

<div class="mt-5 flex flex-col gap-y-5">
  {#each Object.entries(routeTrainers) as [name, trainerInfo], index}
    <div>
      <strong class="flex flex-row items-center gap-x-4">
        {_.capitalize(name)}
        <TrainerMenu
          index={index}
          trainerName={name}
          bind:trainerToUpdate={trainerToUpdate}
          bind:positionModalOpen={positionModalOpen}
          bind:spriteModalOpen={spriteModalOpen}
          bind:trainerVersionsModalOpen={trainerVersionsModalOpen}
        />
      </strong>
      <div class="mt-2 grid grid-cols-6 items-center gap-5">
        {#if trainerInfo.sprite}
          <img
            src={`https://play.pokemonshowdown.com/sprites/trainers/${trainerInfo.sprite}.png`}
            alt={name}
            class="m-0 justify-self-center"
          />
        {/if}
        {#each trainerInfo.pokemon_team as pokemon}
          <button
            class="group card relative grid !bg-transparent p-2 shadow-md transition ease-in-out hover:scale-110 hover:cursor-pointer"
            on:click={() => {
                    editPokemonModalOpen = true;
                    currentTrainerPokemon = _.cloneDeep(pokemon);
                    trainerToUpdate = name;
                    currentTrainerVersions = trainerInfo.versions ?? [];
                  }}
          >
            <TrainerPokemonCard
              pokemon={pokemon}
              trainerName={name}
              deletePokemon={deletePokemonFromTrainer}
            />
          </button>
        {/each}
      </div>
      <div></div>
    </div>
  {/each}
</div>

<svelte:window
  use:shortcut={{
    trigger: {
      key: 'Enter',
      modifier: ["ctrl", "meta"],
      callback: () => {
        if (_.isEqual(routeTrainers, originalTrainers)) {
          return;
        }
        saveChanges();
      }
    },
  }}
/>
