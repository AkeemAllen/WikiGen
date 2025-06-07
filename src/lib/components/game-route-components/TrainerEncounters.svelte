<script lang="ts">
  import { pokemonList } from "../../../store/pokemon";
  import { selectedWiki } from "../../../store";
  import {
    routes,
    type TrainerInfo,
    type TrainerPokemon,
  } from "../../../store/gameRoutes";
  import {
    getSpriteImage,
    setUniquePokemonId,
    sortTrainersByPosition,
  } from "$lib/utils";
  import MultiSelect from "svelte-multiselect";
  import EditTrainerPokemonModal from "../modals/EditTrainerPokemonModal.svelte";
  import { cloneDeep } from "$lib/utils/cloneDeep";
  import capitalizeWords from "$lib/utils/capitalizeWords";
  import isEqual from "$lib/utils/isEqual";
  import { generateRoutePages, updateRoutes } from "$lib/utils/generators";
  import * as Card from "$lib/components/ui/card/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { toast } from "svelte-sonner";
  import Autocomplete from "$lib/components/ui/Autocomplete.svelte";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Label } from "$lib/components/ui/label";
  import EditIcon from "@lucide/svelte/icons/edit";
  import XIcon from "@lucide/svelte/icons/x";
  import ImageIcon from "@lucide/svelte/icons/image";
  import SplitIcon from "@lucide/svelte/icons/split";
  import ArrowLeftRightIcon from "@lucide/svelte/icons/arrow-left-right";
  import * as Dialog from "$lib/components/ui/dialog/index";

  type Props = {
    routeName: string;
  };

  let { routeName }: Props = $props();
  let trainerName: string = $state("");
  let trainerSearchOpen: boolean = $state(false);
  let searchingTrainers: string = $state("");
  let pokemonSearchOpen: boolean = $state(false);
  let searchingPokemon = $state("");
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
    Object.keys(routeTrainers)
      .map((trainer) => ({
        label: trainer,
        value: trainer,
      }))
      .filter((option) =>
        option.label.toLowerCase().includes(searchingTrainers.toLowerCase()),
      ),
  );

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

  function addPokemonToTrainer() {
    let searchedPokemon = $pokemonList.find(
      ([_, __, name, ___]) =>
        name === pokemonSearchName.toLowerCase().replaceAll(" ", "-"),
    );

    if (searchedPokemon === undefined) {
      toast.error("Pokemon not found");
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
      toast.error("No more trainer pokemon");
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
      toast.error("No more trainer pokemon");
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
            toast.success(res as string);
          })
          .catch((e) => {
            toast.error(e as string);
          });
      })
      .catch((e) => {
        toast.error(e as string);
      });
  }
</script>

<!-- Sprite Modal -->
<Dialog.Root bind:open={spriteModalOpen}>
  <Dialog.Content class="w-[25rem]">
    {#if spriteName !== ""}
      <img
        src={`https://play.pokemonshowdown.com/sprites/trainers/${spriteName.toLowerCase()}.png`}
        alt="Trainer Sprite"
        class="h-32 w-32"
      />
    {/if}
    <div>
      <Label
        for="sprite-name"
        class="block text-sm font-medium leading-6 text-gray-900 mb-2"
        >Sprite Name</Label
      >
      <Input
        type="text"
        id="sprite-name"
        placeholder="Type name to see sprite. Eg. red"
        bind:value={spriteName}
      />
    </div>
    <Button
      class="w-32"
      disabled={spriteName === ""}
      onclick={() => {
        routeTrainers[trainerToUpdate].sprite = spriteName.toLowerCase();
        trainerToUpdate = "";
        spriteName = "";
        spriteModalOpen = false;
      }}
    >
      Set Sprite
    </Button>
  </Dialog.Content>
</Dialog.Root>

<!-- Trainer Versions Modal -->
<Dialog.Root bind:open={trainerVersionsModalOpen}>
  <Dialog.Content class="w-[25rem] gap-y-1">
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
      disabled={isEqual(routeTrainers, originalTrainers)}
      onclick={() => {
        saveChanges();
        trainerVersionsModalOpen = false;
      }}
    >
      Save Versions
    </Button>
  </Dialog.Content>
</Dialog.Root>

<!-- Position Modal -->
<Dialog.Root bind:open={positionModalOpen}>
  <Dialog.Content class="w-[25rem]">
    <div>
      <Label class="text-sm font-medium text-gray-700 mb-2" for="position"
        >Position</Label
      >
      <Input
        id="position"
        type="number"
        bind:value={routeTrainers[trainerToUpdate].position}
      />
    </div>
    <Button
      class="w-32"
      disabled={isEqual(routeTrainers, originalTrainers)}
      onclick={() => {
        routeTrainers = sortTrainersByPosition(routeTrainers);
        positionModalOpen = false;
      }}
    >
      Change Position
    </Button>
  </Dialog.Content>
</Dialog.Root>

<EditTrainerPokemonModal
  bind:open={editPokemonModalOpen}
  pokemon={currentTrainerPokemon}
  {trainerToUpdate}
  trainerVersions={currentTrainerVersions}
  {nextTrainerPokemon}
  {prevTrainerPokemon}
  {savePokemonChanges}
/>

<Card.Root>
  <Card.Content class="flex flex-row gap-3">
    <section class="flex flex-row gap-5 justify-between">
      <Autocomplete
        open={trainerSearchOpen}
        value={trainerName}
        label="Trainer"
        creationEnabled={true}
        bind:searcher={searchingTrainers}
        options={trainerOptions}
        placeholder="Search Trainer"
        onSelect={(option) => {
          trainerName = option.label;
        }}
        class="w-[10rem]"
      />
      <Autocomplete
        open={pokemonSearchOpen}
        value={pokemonSearchName}
        label="Pokemon"
        bind:searcher={searchingPokemon}
        {options}
        placeholder="Search Pokemon"
        onSelect={(option) => {
          pokemonSearchName = option.label;
        }}
        class="w-[10rem]"
      />
      <div>
        <Label for="level" class="block text-sm font-medium mb-2 text-gray-700"
          >Level</Label
        >
        <Input type="number" id="level" bind:value={level} class="w-[10rem]" />
      </div>
      <Button
        class="mt-7 w-32"
        disabled={pokemonSearchName === "" || level === 0 || trainerName === ""}
        onclick={addPokemonToTrainer}
      >
        Add Encounter
      </Button>
      <Button
        class="mt-7 w-32"
        disabled={isEqual(routeTrainers, originalTrainers)}
        onclick={saveChanges}
      >
        Save Changes
      </Button>
    </section></Card.Content
  >
</Card.Root>

<main class="mt-5 grid grid-cols-3 gap-5">
  {#each Object.entries(routeTrainers) as [name, trainerInfo], index}
    <Card.Root>
      <Card.Header>
        <div class="flex flex-row justify-between">
          <Card.Title class="text-slate-900 text-lg"
            >{capitalizeWords(name)}</Card.Title
          >
          <div class="flex flex-row gap-3">
            <button
              class="rounded-md p-1 mr-2 hover:cursor-pointer hover:bg-slate-200"
              onclick={() => {
                spriteModalOpen = true;
                trainerToUpdate = name;
              }}
            >
              <ImageIcon class="text-slate-400 size-4 self-center" />
            </button>
            <button
              class="rounded-md p-1 mr-2 hover:cursor-pointer hover:bg-slate-200"
              onclick={() => {
                trainerVersionsModalOpen = true;
                trainerToUpdate = name;
              }}
            >
              <SplitIcon class="text-slate-400 size-4 self-center" />
            </button>
            <button
              class="rounded-md p-1 mr-2 hover:cursor-pointer hover:bg-slate-200"
              onclick={() => {
                positionModalOpen = true;
                trainerToUpdate = name;
              }}
            >
              <ArrowLeftRightIcon class="text-slate-400 size-4 self-center" />
            </button>
          </div>
        </div>
        <Card.Description>
          {#if trainerInfo.sprite}
            <img
              src={`https://play.pokemonshowdown.com/sprites/trainers/${trainerInfo.sprite}.png`}
              alt={name}
              class="m-0 justify-self-center"
            />
          {/if}
        </Card.Description>
      </Card.Header>
      <Card.Content>
        <div class="space-y-2 max-h-80 overflow-y-auto">
          {#each trainerInfo.pokemon_team as pokemon}
            <div
              class="flex items-center gap-3 p-2 bg-slate-100 dark:bg-slate-800/60 rounded-lg group hover:bg-slate-200 dark:hover:bg-slate-700/80 transition-colors"
            >
              {#await getSpriteImage(pokemon.name) then spriteUrl}
                <img
                  src={spriteUrl}
                  alt={pokemon.name}
                  class="size-13 object-contain"
                />
              {/await}
              <div class="flex-1 min-w-0">
                <div
                  class="font-medium text-slate-900 dark:text-slate-100 truncate"
                >
                  {capitalizeWords(pokemon.name)}
                </div>
                <div
                  class="flex items-center gap-2 text-xs text-slate-600 dark:text-slate-400"
                >
                  <span class="flex items-center gap-1">
                    Lv. {pokemon.level}
                  </span>
                </div>
              </div>
              <div
                class="flex gap-1 opacity-0 group-hover:opacity-100 transition-opacity"
              >
                <Button
                  size="sm"
                  variant="ghost"
                  class="h-6 w-6 p-0 cursor-pointer"
                  onclick={() => {
                    editPokemonModalOpen = true;
                    currentTrainerPokemon = cloneDeep(pokemon);
                    trainerToUpdate = name;
                    currentTrainerVersions = trainerInfo.versions ?? [];
                  }}
                >
                  <EditIcon class="w-3 h-3" />
                </Button>
                <Button
                  size="sm"
                  variant="ghost"
                  class="h-6 w-6 p-0 text-red-600 hover:text-red-700 cursor-pointer"
                  onclick={() => {
                    deletePokemonFromTrainer(pokemon.unique_id, name);
                  }}
                >
                  <XIcon class="w-3 h-3" />
                </Button>
              </div>
            </div>
          {/each}
        </div>
      </Card.Content>
    </Card.Root>
  {/each}
</main>
