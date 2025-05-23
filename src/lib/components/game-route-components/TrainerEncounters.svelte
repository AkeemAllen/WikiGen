<script lang="ts">
  import BaseModal from "$lib/components/BaseModal.svelte";

  import {
    getToastStore,
    type AutocompleteOption,
  } from "@skeletonlabs/skeleton";
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
  import { BaseDirectory, writeTextFile } from "@tauri-apps/plugin-fs";
  import { setUniquePokemonId, sortTrainersByPosition } from "$lib/utils";
  import AutoComplete from "../AutoComplete.svelte";
  import TrainerPokemonCard from "../TrainerPokemonCard.svelte";
  import MultiSelect from "svelte-multiselect";
  import { invoke } from "@tauri-apps/api/core";
  import TrainerMenu from "../modals/TrainerMenu.svelte";
  import EditTrainerPokemonModal from "../modals/EditTrainerPokemonModal.svelte";
  import { shortcut } from "@svelte-put/shortcut";
  import { cloneDeep } from "$lib/utils/cloneDeep";
  import capitalizeWords from "$lib/utils/capitalizeWords";
  import isEqual from "$lib/utils/isEqual";
  import { generateRoutePages, updateRoutes } from "$lib/utils/generators";
  import { getToastSettings, ToastType } from "$lib/utils/toasts";

  const toastStore = getToastStore();

  interface Props {
    routeName: string;
  }

  let { routeName }: Props = $props();
  let trainerName: string = $state("");
  let pokemonSearchName: string = $state("");
  let level: number = $state(0);

  let editPokemonModalOpen: boolean = $state(false);
  let trainerToUpdate: string = $state("");
  let currentTrainerPokemon: TrainerPokemon = $state({} as TrainerPokemon);
  let currentTrainerVersions: string[] = $state([]);

  let spriteModalOpen: boolean = $state(false);
  let trainerVersionsModalOpen: boolean = $state(false);
  let positionModalOpen: boolean = $state(false);
  let spriteName: string = $state("");

  let routeTrainers: { [key: string]: TrainerInfo } = $state(
    cloneDeep($routes.routes[routeName].trainers),
  );
  let originalTrainers = $state(cloneDeep($routes.routes[routeName].trainers));
  let trainerOptions = $derived(
    Object.keys(routeTrainers).map((trainer) => ({
      label: trainer,
      value: trainer,
    })),
  );

  let pokemonListOptions: AutocompleteOption<string | number>[] =
    $pokemonList.map(([id, _, name]) => ({
      label: capitalizeWords(name),
      value: id,
    }));

  function onPokemonNameSelected(
    event: CustomEvent<AutocompleteOption<string | number>>,
  ): void {
    pokemonSearchName = event.detail.label;
  }

  function addPokemonToTrainer() {
    let searchedPokemon = $pokemonList.find(
      ([_, __, name, ___]) =>
        name === pokemonSearchName.toLowerCase().replaceAll(" ", "-"),
    );

    if (searchedPokemon === undefined) {
      toastStore.trigger({
        message: "Pokemon not found",
        timeout: 3000,
        background: "variant-filled-error",
      });
      return;
    }

    if (routeTrainers[trainerName] === undefined) {
      routeTrainers[trainerName] = {
        position: Object.keys(routeTrainers).length,
        sprite: "",
        versions: [],
        pokemon_team: [],
      };
    }

    let id = searchedPokemon[1];
    let uniqueId = setUniquePokemonId(
      routeTrainers as {
        [key: string]: TrainerInfo;
      },
      trainerName,
      pokemonSearchName.toLowerCase().replaceAll(" ", "-"),
      $pokemonList,
    );

    routeTrainers[trainerName].pokemon_team = [
      ...routeTrainers[trainerName].pokemon_team,
      {
        id: id,
        unique_id: uniqueId,
        name: pokemonSearchName.toLowerCase().replaceAll(" ", "-"),
        types: searchedPokemon[3].split(","),
        level,
        item: "",
        nature: "",
        ability: "",
        trainer_versions: [],
        moves: [],
      },
    ];

    console.log(routeTrainers[trainerName].pokemon_team);

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
    currentTrainerPokemon = cloneDeep(
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
    currentTrainerPokemon = cloneDeep(
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

    routeTrainers = cloneDeep($routes.routes[routeName].trainers);
    originalTrainers = cloneDeep(routeTrainers);

    await updateRoutes($routes, $selectedWiki.name)
      .then(() => {
        generateRoutePages([routeName], $selectedWiki.name)
          .then((res) => {
            toastStore.trigger(
              getToastSettings(ToastType.SUCCESS, res as string),
            );
          })
          .catch((e) => {
            toastStore.trigger(getToastSettings(ToastType.ERROR, e as string));
          });
      })
      .catch((e) => {
        toastStore.trigger(getToastSettings(ToastType.ERROR, e as string));
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
    disabled={isEqual(routeTrainers, originalTrainers)}
    onClick={() => {
      saveChanges();
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
    disabled={isEqual(routeTrainers, originalTrainers)}
    onClick={() => {
      routeTrainers = sortTrainersByPosition(routeTrainers);
      positionModalOpen = false;
    }}
  />
</BaseModal>

<EditTrainerPokemonModal
  bind:open={editPokemonModalOpen}
  pokemon={currentTrainerPokemon}
  {trainerToUpdate}
  trainerVersions={currentTrainerVersions}
  {nextTrainerPokemon}
  {prevTrainerPokemon}
  {savePokemonChanges}
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
    label="Pokemon for current encounter area"
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
    disabled={isEqual(routeTrainers, originalTrainers)}
    onClick={saveChanges}
  />
</div>

<div class="mt-5 flex flex-col gap-y-5">
  {#each Object.entries(routeTrainers) as [name, trainerInfo], index}
    <div>
      <strong class="flex flex-row items-center gap-x-4">
        {capitalizeWords(name)}
        <TrainerMenu
          {index}
          trainerName={name}
          bind:trainerToUpdate
          bind:positionModalOpen
          bind:spriteModalOpen
          bind:trainerVersionsModalOpen
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
            onclick={() => {
              editPokemonModalOpen = true;
              currentTrainerPokemon = cloneDeep(pokemon);
              trainerToUpdate = name;
              currentTrainerVersions = trainerInfo.versions ?? [];
            }}
          >
            <TrainerPokemonCard
              {pokemon}
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
      key: "Enter",
      modifier: ["ctrl", "meta"],
      callback: () => {
        if (isEqual(routeTrainers, originalTrainers)) {
          return;
        }
        saveChanges();
      },
    },
  }}
/>
