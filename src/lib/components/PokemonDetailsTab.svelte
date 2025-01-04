<script lang="ts">
  import { run } from "svelte/legacy";

  import SelectInput from "$lib/components/SelectInput.svelte";
  import AutoComplete from "$lib/components/AutoComplete.svelte";
  import TextInput from "$lib/components/TextInput.svelte";
  import { type Pokemon, pokemonList } from "../../store/pokemon";
  import NumberInput from "./NumberInput.svelte";
  import { abilitiesList } from "../../store/abilities";
  import { itemsList } from "../../store/items";
  import { types as pokemonTypes } from "../../store/types";
  import capitalizeWords from "$lib/utils/capitalizeWords";
  import { getToastStore } from "@skeletonlabs/skeleton";
  import { getToastSettings, ToastType } from "$lib/utils/toasts";

  interface Props {
    pokemon: Pokemon;
    isNewPokemon?: boolean;
  }

  let { pokemon = $bindable({} as Pokemon), isNewPokemon = false }: Props =
    $props();

  let pokemonListOptions = $pokemonList.map(([id, _, name]) => ({
    label: capitalizeWords(name),
    value: name,
  }));

  const abilitiesListOptions = $abilitiesList.map(([id, name]) => {
    return {
      label: capitalizeWords(name),
      value: name,
    };
  });

  const itemListOptions = $itemsList.map(([id, name]) => ({
    label: capitalizeWords(name),
    value: name,
  }));

  let type1: string = $derived(pokemon.types.split(",")[0]);
  let type2: string = $derived.by(() => {
    if (pokemon.types.split(",").length < 2) {
      return "none";
    }

    return pokemon.types.split(",")[1];
  });

  function onTypeChange(e: any, type_number: string) {
    let tempType1 = type1;
    let tempType2 = type2;

    if (type_number === "1") {
      tempType1 = e.target.value;
    } else {
      tempType2 = e.target.value;
    }

    // This scenario should be unlikely. So default it to normal
    if (tempType1 === "none" && tempType2 === "none") {
      getToastStore().trigger(
        getToastSettings(
          ToastType.ERROR,
          "Both types cannot be none. Defaulting to normal",
        ),
      );
      pokemon.types = "normal";
      return;
    }

    if (tempType1 !== "none" && tempType2 === "none") {
      pokemon.types = tempType1;
      return;
    }

    if (tempType1 === "none" && tempType2 !== "none") {
      pokemon.types = tempType2;
      return;
    }

    pokemon.types = `${tempType1},${tempType2}`;
  }
</script>

<div class="scroll-smooth px-4">
  <div class="mt-4 grid grid-cols-2 gap-x-10 gap-y-5">
    <SelectInput
      id="pokemon-type-1"
      value={type1}
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
      value={type2}
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
      bind:value={pokemon.ability_1}
      options={abilitiesListOptions}
      popupId="ability-1-popup"
      onSelection={(e) => {
        pokemon.ability_1 = e.detail.value;
      }}
      class="w-full"
    />
    <AutoComplete
      label="Ability 2"
      bind:value={pokemon.ability_2}
      options={abilitiesListOptions}
      popupId="ability-2-popup"
      onSelection={(e) => {
        pokemon.ability_2 = e.detail.value;
      }}
      class="w-full"
    />
    <AutoComplete
      label="Hidden Ability"
      bind:value={pokemon.hidden_ability}
      options={abilitiesListOptions}
      popupId="hidden-ability-popup"
      onSelection={(e) => {
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
