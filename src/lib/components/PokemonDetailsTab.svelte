<script lang="ts">
  import SelectInput from "$lib/components/SelectInput.svelte";
  import AutoComplete from "$lib/components/AutoComplete.svelte";
  import TextInput from "$lib/components/TextInput.svelte";
  import { type Pokemon, pokemonList } from "../../store/pokemon";
  import NumberInput from "./NumberInput.svelte";
  import { abilitiesList } from "../../store/abilities";
  import { itemsList } from "../../store/items";
  import { types as pokemonTypes } from "../../store/types";
  import capitalizeWords from "$lib/utils/capitalizeWords";
  import { FALSE, TRUE } from "$lib/utils/CONSTANTS";
  import { getToastStore } from "@skeletonlabs/skeleton";

  export let pokemon: Pokemon = {} as Pokemon;
  export let isNewPokemon: boolean = false;

  let pokemonListOptions = $pokemonList.map(([id, _, name]) => ({
    label: capitalizeWords(name),
    value: name,
  }));

  const abilitiesListOptions = $abilitiesList.map(([id, name]) => {
    if (name === "None") {
      return {
        label: "None",
        value: null,
      };
    }
    return {
      label: capitalizeWords(name),
      value: name,
    };
  });

  const itemListOptions = $itemsList.map(([id, name]) => ({
    label: capitalizeWords(name),
    value: name,
  }));

  let ability_1: string = "";
  $: if (pokemon.ability_1 || pokemon.ability_1 === null) setAbility1();
  function setAbility1() {
    if (pokemon.ability_1 === null) {
      ability_1 = "None";
      return;
    }
    ability_1 = capitalizeWords(pokemon.ability_1);
  }

  let ability_2: string = "";
  $: if (pokemon.ability_2 || pokemon.ability_2 === null) setAbility2();
  function setAbility2() {
    if (pokemon.ability_2 === null) {
      ability_2 = "None";
      return;
    }
    ability_2 = capitalizeWords(pokemon.ability_2);
  }

  let hidden_ability: string = "";
  $: if (pokemon.hidden_ability || pokemon.hidden_ability === null)
    setHiddenAbility();
  function setHiddenAbility() {
    if (pokemon.hidden_ability === null) {
      hidden_ability = "None";
      return;
    }
    hidden_ability = capitalizeWords(pokemon.hidden_ability);
  }

  let types: string[];
  $: if (pokemon.types) setTypes();
  function setTypes() {
    types = pokemon.types.split(",");
    if (types.length === 1) {
      types.push("none");
    }
  }

  function onTypeChange(e: any, type_number: string) {
    console.log(e);
    if (type_number === "1") {
      types[0] = e.target.value;
    } else {
      types[1] = e.target.value;
    }

    // This scenario should be unlikely. So default it to normal
    if (types[0] === "none" && types[1] === "none") {
      getToastStore().trigger({
        message: "Both types cannot be none. Defaulting to normal",
        background: "variant-filled-error",
      });
      pokemon.types = "normal";
      return;
    }

    if (types[0] !== "none" && types[1] === "none") {
      pokemon.types = types[0];
      return;
    }

    if (types[0] === "none" && types[1] !== "none") {
      pokemon.types = types[1];
      return;
    }

    pokemon.types = `${types[0]},${types[1]}`;
  }
</script>

<div class="scroll-smooth px-4">
  <div class="mt-4 grid grid-cols-2 gap-x-10 gap-y-5">
    <SelectInput
      id="pokemon-type-1"
      bind:value={types[0]}
      label="Type 1"
      options={$pokemonTypes.map((type) => {
        return {
          label: capitalizeWords(type),
          value: type,
        };
      })}
      onChange={(e) => onTypeChange(e, "1")}
    />
    <SelectInput
      id="pokemon-type-2"
      bind:value={types[1]}
      label="Type 2"
      options={$pokemonTypes.map((type) => {
        return {
          label: capitalizeWords(type),
          value: type,
        };
      })}
      onChange={(e) => onTypeChange(e, "2")}
    />
    <AutoComplete
      label="Ability 1"
      bind:value={ability_1}
      options={abilitiesListOptions}
      popupId="ability-1-popup"
      onSelection={(e) => {
        ability_1 = e.detail.label;
        pokemon.ability_1 = e.detail.value;
      }}
      class="w-full"
    />
    <AutoComplete
      label="Ability 2"
      bind:value={ability_2}
      options={abilitiesListOptions}
      popupId="ability-2-popup"
      onSelection={(e) => {
        ability_2 = e.detail.label;
        pokemon.ability_2 = e.detail.value;
      }}
      class="w-full"
    />
    <AutoComplete
      label="Hidden Ability"
      bind:value={hidden_ability}
      options={abilitiesListOptions}
      popupId="hidden-ability-popup"
      onSelection={(e) => {
        hidden_ability = e.detail.label;
        pokemon.hidden_ability = e.detail.value;
      }}
      class="w-full col-start-2"
    />
  </div>
  <div class="mt-5 flex flex-row gap-x-10">
    <SelectInput
      id="render-page"
      label="Render Pokemon Page"
      bind:value={pokemon.render}
      options={[
        { label: "True", value: "true" },
        { label: "False", value: "false" },
      ]}
    />
    {#if !isNewPokemon}
      <div class="w-44">
        <SelectInput
          id="evolution-method"
          label="Evolution Method"
          bind:value={pokemon.evolution_method}
          options={[
            { label: "No Change", value: "no_change" },
            { label: "Level Up", value: "level_up" },
            { label: "Other", value: "other" },
            { label: "Use Item", value: "item" },
          ]}
        />
      </div>
      {#if pokemon.evolution_method === "item"}
        <AutoComplete
          label="Evolution Item"
          bind:value={pokemon.evolution_item}
          options={itemListOptions}
          popupId="evolution-item-popup"
          onSelection={(e) => {
            pokemon.evolution_item = e.detail.label;
          }}
        />
      {/if}
      {#if pokemon.evolution_method === "level_up"}
        <NumberInput
          id="evolution-level"
          bind:value={pokemon.evolution_level}
          label="Evolution Level"
          max={100}
        />
      {/if}
      {#if pokemon.evolution_method === "other"}
        <TextInput
          id="evolution-other"
          bind:value={pokemon.evolution_other}
          label="Evolution Other"
        />
      {/if}
      {#if pokemon.evolution_method !== "no_change"}
        <div class="w-44">
          <AutoComplete
            label="Evolves To"
            bind:value={pokemon.evolved_pokemon}
            placeholder="Evolves To"
            options={pokemonListOptions}
            popupId="evolved-pokemon-popup"
            onSelection={(e) => {
              pokemon.evolved_pokemon = e.detail.label;
            }}
          />
        </div>
      {/if}
    {/if}
  </div>
  <p class="mt-10 text-lg">Stats</p>
  <div class="mb-2 mt-2 grid grid-cols-2 gap-x-10 gap-y-5">
    <NumberInput id="pokemon-hp" bind:value={pokemon.hp} label="HP" max={255} />
    <NumberInput
      id="pokemon-attack"
      bind:value={pokemon.attack}
      label="Attack"
      max={255}
    />
    <NumberInput
      id="pokemon-defense"
      bind:value={pokemon.defense}
      label="Defense"
      max={255}
    />
    <NumberInput
      id="pokemon-special-attack"
      bind:value={pokemon.sp_attack}
      label="Special Attack"
      max={255}
    />
    <NumberInput
      id="pokemon-special-defense"
      bind:value={pokemon.sp_defense}
      label="Special Defense"
      max={255}
    />
    <NumberInput
      id="pokemon-speed"
      bind:value={pokemon.speed}
      label="Speed"
      max={255}
    />
  </div>
</div>
