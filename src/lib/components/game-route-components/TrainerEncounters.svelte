<script lang="ts">
import {
  Autocomplete,
  getToastStore,
  popup,
  type AutocompleteOption,
} from "@skeletonlabs/skeleton";
import SelectInput from "../SelectInput.svelte";
import NumberInput from "../NumberInput.svelte";
import Button from "../Button.svelte";
import { pokemonList, pokemon as storePokemon } from "../../../store/pokemon";
import { selectedWiki } from "../../../store";
import { routes, type TrainerInfo } from "../../../store/gameRoutes";
import TextInput from "../TextInput.svelte";
import { BaseDirectory, writeTextFile } from "@tauri-apps/api/fs";
import { setUniquePokemonId } from "$lib/utils";
import { IconTrash } from "@tabler/icons-svelte";
import _ from "lodash";

const toastStore = getToastStore();

export let routeName: string;
let trainerName: string = "";
let pokemonName: string = "";
let level: number = 0;

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
  ).then(() => {
    toastStore.trigger({
      message: "Encounter Deleted",
      background: "variant-filled-success",
    });
  });
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
  <div class="w-60">
    <label
      for="pokemon-name"
      class="block text-sm font-medium leading-6 text-gray-900"
    >
      Pokemon for current encounter type
    </label>
    <input
      id="pokemon-name"
      type="text"
      placeholder="Pokemon Name"
      class="mt-2 block w-full rounded-md border-0 py-1.5 pl-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 disabled:bg-gray-100 disabled:text-gray-400 sm:text-sm sm:leading-6"
      bind:value={pokemonName}
      use:popup={{
        event: "focus-click",
        target: "popupAutoComplete",
        placement: "bottom",
      }}
    />
    <div
      data-popup="popupAutoComplete"
      class="card mt-2 w-60 overflow-y-auto rounded-sm bg-white"
      tabindex="-1"
    >
      <Autocomplete
        bind:input={pokemonName}
        options={pokemonListOptions}
        limit={5}
        on:selection={onPokemonNameSelected}
        class="w-full rounded-md border bg-white p-2 text-sm"
      />
    </div>
  </div>
  <NumberInput label="Level" bind:value={level} />
  <div class="mt-8 w-32">
    <Button
      title="Add Encounter"
      disabled={pokemonName === "" || level === 0}
      onClick={addPokemonToTrainer}
    />
  </div>
</div>

<div class="mt-5 flex flex-col gap-y-5">
  {#each Object.entries($routes.routes[routeName].trainers ?? {}) as [_trainerName, trainerInfo]}
    <div>
      <strong>
        {_.capitalize(_trainerName)}
      </strong>
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
            <div
              class="invisible absolute right-2 top-2 group-hover:visible"
              on:click={() => deletePokemonFromTrainer(pokemon.unique_id, _trainerName)}
            >
              <IconTrash size={16} color="grey" />
            </div>
          </div>
        {/each}
      </div>
      <div></div>
    </div>
  {/each}
</div>
