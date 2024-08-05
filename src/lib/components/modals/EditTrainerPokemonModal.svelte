<script lang="ts">
  import _ from "lodash";
  import { type TrainerPokemon } from "../../../store/gameRoutes";
  import BaseModal from "../BaseModal.svelte";
  import NumberInput from "../NumberInput.svelte";
  import AutoComplete from "../AutoComplete.svelte";
  import MultiSelect from "svelte-multiselect";
  import Button from "../Button.svelte";
  import { IconChevronLeft, IconChevronRight } from "@tabler/icons-svelte";
  import { abilitiesList } from "../../../store/abilities";
  import { naturesList } from "../../../store/natures";
  import { itemsList } from "../../../store/items";
  import { moveList } from "../../../store/moves";
  import { shortcut } from "@svelte-put/shortcut";
  import { cloneDeep } from "$lib/utils/cloneDeep";
  import capitalizeWords from "$lib/utils/capitalizeWords";

  export let pokemon: TrainerPokemon = {} as TrainerPokemon;
  let originalPokemonAttributes = cloneDeep(pokemon);
  export let open: boolean = false;
  export let trainerToUpdate: string;
  export let trainerVersions: string[] = [];

  $: if (pokemon.unique_id !== originalPokemonAttributes.unique_id) {
    originalPokemonAttributes = cloneDeep(pokemon);
  }

  export let savePokemonChanges = (trainerToUpdate: string) => {};

  export let nextTrainerPokemon = () => {};
  export let prevTrainerPokemon = () => {};

  let abilityListOptions = $abilitiesList.map(([id, name]) => ({
    label: name,
    value: name,
  }));

  let natureListOptions = $naturesList.map(([id, name]) => ({
    label: name,
    value: name,
  }));

  let itemListOptions = $itemsList.map(([id, name]) => ({
    label: name,
    value: name,
  }));

  let moveListOptions = $moveList.map(([id, name]) => name);
</script>

<BaseModal bind:open class="grid w-[40rem] grid-cols-2 gap-5">
  <div class="col-span-2 text-lg font-bold">
    {capitalizeWords(pokemon.name)}
  </div>
  <NumberInput label="Level" bind:value={pokemon.level} />
  <AutoComplete
    label="Item"
    bind:value={pokemon.item}
    options={itemListOptions}
    popupId="item-popup"
    onSelection={async (e) => {
      pokemon.item = e.detail.value;
    }}
  />
  <AutoComplete
    label="Ability"
    bind:value={pokemon.ability}
    options={abilityListOptions}
    popupId="ability-popup"
    onSelection={async (e) => {
      pokemon.ability = e.detail.value;
    }}
  />
  <AutoComplete
    label="Nature"
    bind:value={pokemon.nature}
    options={natureListOptions}
    popupId="nature-popup"
    onSelection={async (e) => {
      pokemon.nature = e.detail.value;
    }}
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
      title="Save Changes"
      disabled={_.isEqual(pokemon, originalPokemonAttributes)}
      onClick={() => {
        savePokemonChanges(trainerToUpdate);
        originalPokemonAttributes = cloneDeep(pokemon);
      }}
    />
    <div class="flex flex-row gap-5">
      <button
        class="rounded-md bg-indigo-300 px-3 py-2 hover:cursor-pointer hover:bg-indigo-600"
        on:click={prevTrainerPokemon}
      >
        <IconChevronLeft size={16} color="white" strokeWidth={3} />
      </button>
      <button
        class="rounded-md bg-indigo-300 px-3 py-2 hover:cursor-pointer hover:bg-indigo-600"
        on:click={nextTrainerPokemon}
      >
        <IconChevronRight size={16} color="white" strokeWidth={3} />
      </button>
    </div>
  </div>
</BaseModal>

<svelte:window
  use:shortcut={{
    trigger: {
      key: "]",
      modifier: ["ctrl", "meta"],
      callback: nextTrainerPokemon,
    },
  }}
  use:shortcut={{
    trigger: {
      key: "[",
      modifier: ["ctrl", "meta"],
      callback: prevTrainerPokemon,
    },
  }}
  use:shortcut={{
    trigger: {
      key: "Enter",
      modifier: ["ctrl", "meta"],
      callback: () => {
        if (_.isEqual(pokemon, originalPokemonAttributes)) {
          return;
        }
        savePokemonChanges(trainerToUpdate);
        originalPokemonAttributes = cloneDeep(pokemon);
      },
    },
  }}
/>
