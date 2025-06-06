<script lang="ts">
  import { run } from "svelte/legacy";

  import { type TrainerPokemon } from "../../../store/gameRoutes";
  import MultiSelect from "svelte-multiselect";
  import IconChevronLeft from "@tabler/icons-svelte/icons/chevron-left";
  import IconChevronRight from "@tabler/icons-svelte/icons/chevron-right";
  import { abilitiesList } from "../../../store/abilities";
  import { naturesList } from "../../../store/natures";
  import { itemsList } from "../../../store/items";
  import { moveList } from "../../../store/moves";
  import { cloneDeep } from "$lib/utils/cloneDeep";
  import capitalizeWords from "$lib/utils/capitalizeWords";
  import isEqual from "$lib/utils/isEqual";
  import * as Dialog from "$lib/components/ui/dialog/index";
  import { Button } from "$lib/components/ui/button/index";
  import Input from "../ui/input/input.svelte";
  import { Label } from "../ui/label";
  import Autocomplete from "../ui/Autocomplete.svelte";

  type Props = {
    pokemon?: TrainerPokemon;
    open?: boolean;
    trainerToUpdate: string;
    trainerVersions?: string[];
    savePokemonChanges?: any;
    nextTrainerPokemon?: any;
    prevTrainerPokemon?: any;
  };

  let {
    pokemon = $bindable({} as TrainerPokemon),
    open = $bindable(false),
    trainerToUpdate,
    trainerVersions = [],
    savePokemonChanges = (trainerToUpdate: string) => {},
    nextTrainerPokemon = () => {},
    prevTrainerPokemon = () => {},
  }: Props = $props();

  let itemSearchOpen = $state(false);
  let searchingItems = $state("");

  let abilitySearchOpen = $state(false);
  let searchingAbilities = $state("");

  let natureSearchOpen = $state(false);
  let searchingNatures = $state("");

  let originalPokemonAttributes = $state(cloneDeep(pokemon));

  let moveListOptions = $moveList.map(([id, name]) => name);
  run(() => {
    if (pokemon.unique_id !== originalPokemonAttributes.unique_id) {
      originalPokemonAttributes = cloneDeep(pokemon);
    }
  });
</script>

<Dialog.Root bind:open>
  <Dialog.Content class="grid w-[40rem] grid-cols-2 gap-5">
    <div class="col-span-2 text-lg font-bold">
      {capitalizeWords(pokemon.name)}
    </div>
    <div>
      <Label for="level-input" class="text-sm font-medium text-gray-700 mb-2"
        >Level</Label
      >
      <Input id="level-input" type="number" bind:value={pokemon.level} />
    </div>
    <Autocomplete
      open={itemSearchOpen}
      value={pokemon.item}
      label="Items"
      bind:searcher={searchingItems}
      options={$itemsList
        .map((item) => ({
          value: item[0],
          label: item[1],
        }))
        .filter((item) => item.label.includes(searchingItems))
        .slice(0, 8)}
      placeholder="Search Items"
      onSelect={(option) => {
        pokemon.item = option.label;
      }}
      class="w-full"
    />
    <Autocomplete
      open={abilitySearchOpen}
      value={pokemon.ability}
      label="Ability"
      bind:searcher={searchingAbilities}
      options={$abilitiesList
        .map((ability) => ({
          value: ability[0],
          label: ability[1],
        }))
        .filter((ability) => ability.label.includes(searchingAbilities))
        .slice(0, 8)}
      placeholder="Search Abilities"
      onSelect={(option) => {
        pokemon.ability = option.label;
      }}
      class="w-full"
    />
    <Autocomplete
      open={natureSearchOpen}
      value={pokemon.nature}
      label="Items"
      bind:searcher={searchingNatures}
      options={$naturesList.map((nature) => ({
        value: nature[0],
        label: nature[1],
      }))}
      placeholder="Search Nature"
      onSelect={(option) => {
        pokemon.nature = option.label;
      }}
      class="w-full"
    />
    <div class="col-span-2">
      <label
        for="moveSet"
        class="mb-2 block text-sm font-medium leading-6 text-gray-900"
        >Moves</label
      >
      <MultiSelect
        id="moveSet"
        bind:selected={pokemon.moves}
        options={moveListOptions}
        maxSelect={4}
        maxOptions={5}
        style="height: 36px; border-color: rgb(209 213 219); border-radius: 0.375rem; box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05); font-size: 0.875rem;"
      />
    </div>
    <div class="col-span-2">
      <label
        for="versions"
        class="mb-2 block text-sm font-medium leading-6 text-gray-900"
        >Trainer Versions</label
      >
      <MultiSelect
        id="versions"
        bind:selected={pokemon.trainer_versions}
        options={trainerVersions}
        on:change={async (e) => {}}
        style="height: 36px; border-color: rgb(209 213 219); border-radius: 0.375rem; box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05); font-size: 0.875rem;"
      />
    </div>
    <div class="col-span-2 mt-6 flex flex-row items-center justify-between">
      <Button
        class="w-32"
        disabled={isEqual(pokemon, originalPokemonAttributes)}
        onclick={() => {
          savePokemonChanges(trainerToUpdate);
          originalPokemonAttributes = cloneDeep(pokemon);
        }}
      >
        Save Changes
      </Button>
      <div class="flex flex-row gap-5">
        <Button
          class="rounded-md px-3 py-2 hover:cursor-pointer"
          onclick={prevTrainerPokemon}
        >
          <IconChevronLeft size={16} color="white" stroke={3} />
        </Button>
        <Button
          class="rounded-md px-3 py-2 hover:cursor-pointer"
          onclick={nextTrainerPokemon}
        >
          <IconChevronRight size={16} color="white" stroke={3} />
        </Button>
      </div>
    </div>
  </Dialog.Content>
</Dialog.Root>
