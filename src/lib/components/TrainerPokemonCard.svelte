<script lang="ts">
import { IconEdit, IconTrash } from "@tabler/icons-svelte";
import { type TrainerInfo, type TrainerPokemon } from "../../store/gameRoutes";
import { pokemon as storePokemon } from "../../store/pokemon";
import _ from "lodash";
import BaseModal from "./BaseModal.svelte";
import AutoComplete from "./AutoComplete.svelte";
import MultiSelect from "svelte-multiselect";
import { abilitiesList } from "../../store/abilities";
import { naturesList } from "../../store/natures";
import { itemsList } from "../../store/items";
import { moveList } from "../../store/moves";
import Button from "./Button.svelte";
import NumberInput from "./NumberInput.svelte";

export let pokemon: TrainerPokemon = {} as TrainerPokemon;
let originalPokemonAttributes = _.cloneDeep(pokemon);
export let trainerVersions: string[];
export let trainerName: string;

export let editModalOpen: boolean = false;
export let deletePokemon = (id: string, name: string) => {};
export let savePokemonChanges = (
  pokemon: TrainerPokemon,
  trainerName: string,
) => {};

let abilityListOptions = $abilitiesList.map((ability) => ({
  label: ability,
  value: ability,
}));

let natureListOptions = $naturesList.map((nature) => ({
  label: nature,
  value: nature,
}));

let itemListOptions = $itemsList.map((item) => ({
  label: item,
  value: item,
}));
</script>

<BaseModal bind:open={editModalOpen} class="grid w-[40rem] grid-cols-2 gap-5">
  <div class="col-span-2 text-lg font-bold">
    {_.capitalize(pokemon.name)}
  </div>
  <NumberInput label="Level" bind:value={pokemon.level} />
  <AutoComplete
    label="Item"
    bind:value={pokemon.item}
    options={itemListOptions}
    popupId="item-popup"
    onSelection={async (e) => {pokemon.item = e.detail.value; }}
  />
  <AutoComplete
    label="Ability"
    bind:value={pokemon.ability}
    options={abilityListOptions}
    popupId="ability-popup"
    onSelection={async (e) => {pokemon.ability = e.detail.value; }}
  />
  <AutoComplete
    label="Nature"
    bind:value={pokemon.nature}
    options={natureListOptions}
    popupId="nature-popup"
    onSelection={async (e) => {pokemon.nature = e.detail.value; }}
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
      options={$moveList}
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
  <Button
    class="mt-8 w-32"
    title="Save Changes"
    disabled={_.isEqual(pokemon, originalPokemonAttributes)}
    onClick={() => {savePokemonChanges(pokemon, trainerName);
    originalPokemonAttributes = _.cloneDeep(pokemon);
    editModalOpen = false;
    }}
  />
</BaseModal>
<div
  class="group card relative grid !bg-transparent p-2 shadow-md transition ease-in-out hover:scale-110 hover:cursor-pointer"
  on:click={() => {
        editModalOpen = true;
      }}
  role="button"
  tabindex={0}
  on:keydown={(e) => {
        if (e.key === "Enter") {
          editModalOpen = true;
        }
      }}
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
    class="invisible absolute right-2 top-2 hover:scale-110 group-hover:visible"
    on:click={() => deletePokemon(pokemon.unique_id, trainerName)}
  >
    <IconTrash size={20} color="grey" />
  </button>
</div>
