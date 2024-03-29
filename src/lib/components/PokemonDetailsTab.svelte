<script lang="ts">
  import SelectInput from "$lib/components/SelectInput.svelte";
  import type { AutocompleteOption } from "@skeletonlabs/skeleton";
  import _ from "lodash";
  import { PokemonTypes, type PokemonDetails } from "../../store/pokemon";
  import NumberInput from "./NumberInput.svelte";
  import TextInput from "./TextInput.svelte";

  export let pokemonDetails: PokemonDetails = {} as PokemonDetails;
  export let pokemonListOptions: AutocompleteOption<string | number>[];
</script>

<div class="scroll-smooth">
  <div class="grid grid-cols-2 gap-x-10 gap-y-5 mt-10">
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
    <SelectInput
      id="pokemon-ability-1"
      bind:value={pokemonDetails.abilities[0]}
      label="Ability 1"
      options={pokemonDetails.abilities.map((ability) => ({
        label: _.capitalize(ability),
        value: ability,
      }))}
    />
    <SelectInput
      id="pokemon-ability-2"
      bind:value={pokemonDetails.abilities[1]}
      label="Ability 2"
      options={pokemonDetails.abilities.map((ability) => ({
        label: _.capitalize(ability),
        value: ability,
      }))}
    />
  </div>
  <div class="flex flex-row gap-x-10 mt-5">
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
      <TextInput
        id="evolution-item"
        value={pokemonDetails.evolution?.item ?? ""}
        label="Evolution Item"
      />
    {/if}
    {#if pokemonDetails.evolution.method === "level_up"}
      <NumberInput
        id="evolution-level"
        bind:value={pokemonDetails.evolution.level}
        label="Evolution Level"
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
        <SelectInput
          id="evolution-to"
          label="Evoles To"
          bind:value={pokemonDetails.evolution.evolves_to}
          options={pokemonListOptions}
        />
      </div>
    {/if}
  </div>
  <p class="text-lg mt-10">Stats</p>
  <div class="grid grid-cols-2 gap-x-10 gap-y-5 mt-2 mb-2">
    <NumberInput
      id="pokemon-hp"
      bind:value={pokemonDetails.stats.hp}
      label="HP"
    />
    <NumberInput
      id="pokemon-attack"
      bind:value={pokemonDetails.stats.attack}
      label="Attack"
    />
    <NumberInput
      id="pokemon-defense"
      bind:value={pokemonDetails.stats.defense}
      label="Defense"
    />
    <NumberInput
      id="pokemon-special-attack"
      bind:value={pokemonDetails.stats.sp_attack}
      label="Special Attack"
    />
    <NumberInput
      id="pokemon-special-defense"
      bind:value={pokemonDetails.stats.sp_defense}
      label="Special Defense"
    />
    <NumberInput
      id="pokemon-speed"
      bind:value={pokemonDetails.stats.speed}
      label="Speed"
    />
  </div>
</div>
