<script lang="ts">
import SelectInput from "$lib/components/SelectInput.svelte";
import AutoComplete from "$lib/components/AutoComplete.svelte";
import type { AutocompleteOption } from "@skeletonlabs/skeleton";
import _ from "lodash";
import {
  PokemonTypes,
  pokemonList,
  type PokemonDetails,
} from "../../store/pokemon";
import NumberInput from "./NumberInput.svelte";
import TextInput from "./TextInput.svelte";
import { abilitiesList } from "../../store/abilities";
import { itemsList } from "../../store/items";

export let pokemonDetails: PokemonDetails = {} as PokemonDetails;
export let formTabSet: number;
let pokemonListOptions: AutocompleteOption<string | number>[] =
  $pokemonList.map(([name, id]) => ({ label: _.capitalize(name), value: id }));

const abilitiesListOptions: AutocompleteOption<string>[] = $abilitiesList.map(
  (name) => ({
    label: name,
    value: name,
  }),
);

const itemListOptions: AutocompleteOption<string>[] = $itemsList.map(
  (name) => ({
    label: name,
    value: name,
  }),
);
</script>

<div class="scroll-smooth px-4">
  <div class="mt-4 grid grid-cols-2 gap-x-10 gap-y-5">
    <SelectInput
      id="pokemon-type-1"
      bind:value={pokemonDetails.types[0]}
      label="Type 1"
      options={PokemonTypes.map((type) => ({
        label: _.capitalize(type),
        value: type,
      }))}
    />
    <SelectInput
      id="pokemon-type-2"
      bind:value={pokemonDetails.types[1]}
      label="Type 2"
      options={PokemonTypes.map((type) => ({
        label: _.capitalize(type),
        value: type,
      }))}
    />
    <AutoComplete
      label="Ability 1"
      bind:value={pokemonDetails.abilities[0]}
      options={abilitiesListOptions}
      popupId="ability-1-popup"
      onSelection={(e) => {
        pokemonDetails.abilities[0] = e.detail.value;
      }}
      class="w-full"
    />
    <AutoComplete
      label="Ability 2"
      bind:value={pokemonDetails.abilities[1]}
      options={abilitiesListOptions}
      popupId="ability-2-popup"
      onSelection={(e) => {
      pokemonDetails.abilities[1] = e.detail.value;
    }}
      class="w-full"
    />
  </div>
  {#if formTabSet === 0}
    <div class="mt-5 flex flex-row gap-x-10">
      <div class="w-44">
        <SelectInput
          id="evolution-method"
          label="Evolution Method"
          bind:value={pokemonDetails.evolution.method}
          options={[
          { label: "No Change", value: "no_change" },
          { label: "Level Up", value: "level_up" },
          { label: "Other", value: "other" },
          { label: "Use Item", value: "item" },
        ]}
        />
      </div>
      {#if pokemonDetails.evolution.method === "item"}
        <AutoComplete
          label="Evolution Item"
          bind:value={pokemonDetails.evolution.item}
          options={itemListOptions}
          popupId="evolution-item-popup"
          onSelection={(e) => {
              pokemonDetails.evolution.item = e.detail.value;
            }}
        />
      {/if}
      {#if pokemonDetails.evolution.method === "level_up"}
        <NumberInput
          id="evolution-level"
          bind:value={pokemonDetails.evolution.level}
          label="Evolution Level"
          max={100}
        />
      {/if}
      {#if pokemonDetails.evolution.method === "other"}
        <TextInput
          id="evolution-other"
          bind:value={pokemonDetails.evolution.other}
          label="Evolution Other"
        />
      {/if}
      {#if pokemonDetails.evolution.method !== "no_change"}
        <div class="w-44">
          <AutoComplete
            label="Evolves To"
            bind:value={pokemonDetails.evolution.evolves_to.pokemon_name}
            placeholder="Evolves To"
            options={pokemonListOptions}
            popupId="ability-popup"
            onSelection={(e) => {
            pokemonDetails.evolution.evolves_to.id = e.detail.value;
            pokemonDetails.evolution.evolves_to.pokemon_name = e.detail.label;
          }}
          />
        </div>
      {/if}
    </div>
  {/if}
  <p class="mt-10 text-lg">Stats</p>
  <div class="mb-2 mt-2 grid grid-cols-2 gap-x-10 gap-y-5">
    <NumberInput
      id="pokemon-hp"
      bind:value={pokemonDetails.stats.hp}
      label="HP"
      max={255}
    />
    <NumberInput
      id="pokemon-attack"
      bind:value={pokemonDetails.stats.attack}
      label="Attack"
      max={255}
    />
    <NumberInput
      id="pokemon-defense"
      bind:value={pokemonDetails.stats.defense}
      label="Defense"
      max={255}
    />
    <NumberInput
      id="pokemon-special-attack"
      bind:value={pokemonDetails.stats.sp_attack}
      label="Special Attack"
      max={255}
    />
    <NumberInput
      id="pokemon-special-defense"
      bind:value={pokemonDetails.stats.sp_defense}
      label="Special Defense"
      max={255}
    />
    <NumberInput
      id="pokemon-speed"
      bind:value={pokemonDetails.stats.speed}
      label="Speed"
      max={255}
    />
  </div>
</div>
