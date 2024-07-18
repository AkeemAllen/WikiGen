<script lang="ts">
import SelectInput from "$lib/components/SelectInput.svelte";
import AutoComplete from "$lib/components/AutoComplete.svelte";
import TextInput from "$lib/components/TextInput.svelte";
import _ from "lodash";
import { PokemonTypes, type Pokemon, pokemonList } from "../../store/pokemon";
import NumberInput from "./NumberInput.svelte";
import { abilitiesList } from "../../store/abilities";
import { itemsList } from "../../store/items";

export let pokemon: Pokemon = {} as Pokemon;

let pokemonListOptions = $pokemonList.map(([id, name]) => ({
  label: _.capitalize(name),
  value: id,
}));

const abilitiesListOptions = $abilitiesList.map(([id, name]) => ({
  label: _.capitalize(name),
  value: id,
}));

const itemListOptions = $itemsList.map(([id, name]) => ({
  label: name,
  value: id,
}));

let evolutionItemName: string = "";
$: if (
  pokemon.evolution_change_object.item ||
  pokemon.evolution_change_object.item === null
) {
  let itemName =
    itemListOptions.find(
      (item) => item.value === pokemon.evolution_change_object.item,
    )?.label || "";
  evolutionItemName = _.capitalize(itemName);
}
</script>

<div class="scroll-smooth px-4">
  <div class="mt-4 grid grid-cols-2 gap-x-10 gap-y-5">
    <SelectInput
      id="pokemon-type-1"
      bind:value={pokemon.type_1}
      label="Type 1"
      options={PokemonTypes.map(type => {
        if (type === null) {
          return { label: "None", value: null };
        }

        return {
          label: _.capitalize(type),
          value: type,
        }
      })}
    />
    <SelectInput
      id="pokemon-type-2"
      bind:value={pokemon.type_2}
      label="Type 2"
      options={PokemonTypes.map(type => {
        if (type === null) {
          return { label: "None", value: null };
        }

        return {
          label: _.capitalize(type),
          value: type,
        }
      })}
    />
    <AutoComplete
      label="Ability 1"
      bind:value={pokemon.ability_1_name}
      options={abilitiesListOptions}
      popupId="ability-1-popup"
      onSelection={(e) => {
        pokemon.ability_1 = e.detail.value;
        pokemon.ability_1_name = e.detail.label;
      }}
      class="w-full"
    />
    <AutoComplete
      label="Ability 2"
      bind:value={pokemon.ability_2_name}
      options={abilitiesListOptions}
      popupId="ability-2-popup"
      onSelection={(e) => {
        pokemon.ability_2 = e.detail.value;
        pokemon.ability_2_name = e.detail.label;
    }}
      class="w-full"
    />
  </div>
  <div class="mt-5 flex flex-row gap-x-10">
    <div class="w-44">
      <SelectInput
        id="evolution-method"
        label="Evolution Method"
        bind:value={pokemon.evolution_change_object.method}
        options={[
            { label: "No Change", value: "no_change" },
            { label: "Level Up", value: "level_up" },
            { label: "Other", value: "other" },
            { label: "Use Item", value: "item" },
          ]}
      />
    </div>
    {#if pokemon.evolution_change_object.method === "item"}
      <AutoComplete
        label="Evolution Item"
        bind:value={evolutionItemName}
        options={itemListOptions}
        popupId="evolution-item-popup"
        onSelection={(e) => {
                pokemon.evolution_change_object.item= e.detail.value;
                evolutionItemName = e.detail.label;
              }}
      />
    {/if}
    {#if pokemon.evolution_change_object.method === "level_up"}
      <NumberInput
        id="evolution-level"
        bind:value={pokemon.evolution_change_object.level}
        label="Evolution Level"
        max={100}
      />
    {/if}
    {#if pokemon.evolution_change_object.method === "other"}
      <TextInput
        id="evolution-other"
        bind:value={pokemon.evolution_change_object.other}
        label="Evolution Other"
      />
    {/if}
    {#if pokemon.evolution_change_object.method !== "no_change"}
      <div class="w-44">
        <AutoComplete
          label="Evolves To"
          bind:value={pokemon.evolution_change_object.evolved_pokemon.name}
          placeholder="Evolves To"
          options={pokemonListOptions}
          popupId="ability-popup"
          onSelection={(e) => {
            pokemon.evolution_change_object.evolved_pokemon.id = e.detail.value;
              pokemon.evolution_change_object.evolved_pokemon.name = _.capitalize(e.detail.label);
            }}
        />
      </div>
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
