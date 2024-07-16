<script lang="ts">
import SelectInput from "$lib/components/SelectInput.svelte";
import AutoComplete from "$lib/components/AutoComplete.svelte";
import TextInput from "$lib/components/TextInput.svelte";
import _, { isEqual } from "lodash";
import {
  PokemonTypes,
  type DBPokemon,
  dbPokemonList,
  type EvolutionChange,
} from "../../store/pokemon";
import NumberInput from "./NumberInput.svelte";
import { abilitiesList } from "../../store/abilities";
import { itemsList } from "../../store/items";
import { db } from "../../store/db";
import Button from "./Button.svelte";
import { getToastStore } from "@skeletonlabs/skeleton";

const toastStore = getToastStore();

export let pokemon: DBPokemon = {} as DBPokemon;

let pokemonListOptions = $dbPokemonList.map(([id, name]) => ({
  label: _.capitalize(name),
  value: id,
}));

const abilitiesListOptions = $abilitiesList.map(([id, name]) => ({
  label: name,
  value: id,
}));

let ability_1: string = "";
let ability_2: string = "";
$: if (pokemon.ability_1) setAbility1();
$: if (pokemon.ability_2) setAbility2();
function setAbility1() {
  ability_1 = _.capitalize(
    abilitiesListOptions.find((ability) => ability.value === pokemon.ability_1)
      ?.label ?? "",
  );
}
function setAbility2() {
  ability_2 = _.capitalize(
    abilitiesListOptions.find((ability) => ability.value === pokemon.ability_2)
      ?.label ?? "",
  );
}

let evolutionChange: EvolutionChange = {
  id: 0,
  method: "no_change",
  evolved_pokemon: null,
  item: null,
  other: null,
  level: null,
} as EvolutionChange;
let originalEvolutionChange: EvolutionChange = _.cloneDeep(evolutionChange);

$: if (pokemon.evolution_change || pokemon.evolution_change === null)
  setEvolutionChange();
async function setEvolutionChange() {
  await $db
    .select<EvolutionChange[]>("SELECT * FROM evolution_change WHERE id=$1", [
      pokemon.evolution_change,
    ])
    .then((res) => {
      if (res.length === 0) {
        console.log("No evolution change found");
        evolutionChange = {
          id: 0,
          method: "no_change",
          evolved_pokemon: null,
          item: null,
          other: null,
          level: null,
        } as EvolutionChange;
        originalEvolutionChange = _.cloneDeep(evolutionChange);
        return;
      }
      evolutionChange = res[0];
      originalEvolutionChange = _.cloneDeep(evolutionChange);
    })
    .catch((err) => {
      console.error(err);
    });
}

let evolvedPokemonName: string = "";
$: if (
  evolutionChange.evolved_pokemon ||
  evolutionChange.evolved_pokemon === null
)
  setEvolvedPokemonName();
function setEvolvedPokemonName() {
  evolvedPokemonName = _.capitalize(
    pokemonListOptions.find(
      (pokemon) => pokemon.value === evolutionChange.evolved_pokemon,
    )?.label ?? "",
  );
}

let evolutionItem: string = "";
$: if (evolutionChange.item || evolutionChange.item === null)
  setEvolutionItem();
function setEvolutionItem() {
  evolutionItem = _.capitalize(
    itemListOptions.find((item) => item.value === evolutionChange.item)
      ?.label ?? "",
  );
}

const itemListOptions = $itemsList.map(([id, name]) => ({
  label: name,
  value: id,
}));

let displaySaveButton: boolean = false;

async function updateEvolutionChange() {
  if (evolutionChange.method === "no_change") {
    evolutionChange.level = null;
    evolutionChange.item = null;
    evolutionChange.other = null;
    evolutionChange.evolved_pokemon = null;

    await $db
      .execute(`UPDATE pokemon SET evolution_change=NULL WHERE id=$1`, [
        pokemon.id,
      ])
      .then(() => {
        $db
          .execute(`DELETE FROM evolution_change WHERE id=$1`, [
            evolutionChange.id,
          ])
          .then(() => {
            evolutionChange = {
              id: 0,
              method: "no_change",
            } as EvolutionChange;
            originalEvolutionChange = _.cloneDeep(evolutionChange);
            displaySaveButton = false;
            toastStore.trigger({
              message: "Evolution Change Removed",
              background: "variant-filled-success",
            });
          });
      });
  }

  if (evolutionChange.method === "item") {
    evolutionChange.level = null;
    evolutionChange.other = null;
  } else if (evolutionChange.method === "level_up") {
    evolutionChange.item = null;
    evolutionChange.other = null;
  } else if (evolutionChange.method === "other") {
    evolutionChange.level = null;
    evolutionChange.item = null;
  }

  if (evolutionChange.id == 0) {
    await $db
      .execute(
        `INSERT INTO evolution_change (
          method,
          level,
          item,
          other,
          evolved_pokemon
        ) VALUES ($1, $2, $3, $4, $5)`,
        [
          evolutionChange.method,
          evolutionChange.level,
          evolutionChange.item,
          evolutionChange.other,
          evolutionChange.evolved_pokemon,
        ],
      )
      .then((res) => {
        toastStore.trigger({
          message: "Evolution Change Added",
          background: "variant-filled-success",
        });
        $db
          .execute(`UPDATE pokemon SET evolution_change=$1 WHERE id=$2`, [
            res.lastInsertId,
            pokemon.id,
          ])
          .then((res) => {
            evolutionChange.id = res.lastInsertId;
            originalEvolutionChange = _.cloneDeep(evolutionChange);
            displaySaveButton = false;
          });
      });

    return;
  }

  await $db
    .execute(
      `UPDATE evolution_change SET
      method=$1,
      level=$2,
      item=$3,
      other=$4,
      evolved_pokemon=$5
    WHERE id=$6`,
      [
        evolutionChange.method,
        evolutionChange.level,
        evolutionChange.item,
        evolutionChange.other,
        evolutionChange.evolved_pokemon,
        evolutionChange.id,
      ],
    )
    .then(() => {
      toastStore.trigger({
        message: "Evolution Change Updated",
        background: "variant-filled-success",
      });
      originalEvolutionChange = _.cloneDeep(evolutionChange);
      displaySaveButton = false;
      hasEvolutionChanged(evolutionChange);
    });
}

$: displaySaveButton = hasEvolutionChanged(evolutionChange);

function hasEvolutionChanged(evolutionChange: EvolutionChange) {
  if (evolutionChange.method !== originalEvolutionChange.method) {
    return true;
  }

  if (
    evolutionChange.evolved_pokemon !== originalEvolutionChange.evolved_pokemon
  ) {
    return true;
  }

  switch (evolutionChange.method) {
    case "level_up":
      if (evolutionChange.level !== originalEvolutionChange.level) {
        return true;
      }
      break;
    case "item":
      if (evolutionChange.item !== originalEvolutionChange.item) {
        return true;
      }
      break;
    case "other":
      if (evolutionChange.other !== originalEvolutionChange.other) {
        return true;
      }
      break;
  }

  return false;
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
      bind:value={ability_1}
      options={abilitiesListOptions}
      popupId="ability-1-popup"
      onSelection={(e) => {
        pokemon.ability_1 = e.detail.value;
        ability_1 = e.detail.label;
      }}
      class="w-full"
    />
    <AutoComplete
      label="Ability 2"
      bind:value={ability_2}
      options={abilitiesListOptions}
      popupId="ability-2-popup"
      onSelection={(e) => {
        pokemon.ability_2 = e.detail.value;
        ability_2 = e.detail.label;
    }}
      class="w-full"
    />
  </div>
  <div class="mt-5 flex flex-row gap-x-10">
    <div class="w-44">
      <SelectInput
        id="evolution-method"
        label="Evolution Method"
        bind:value={evolutionChange.method}
        options={[
            { label: "No Change", value: "no_change" },
            { label: "Level Up", value: "level_up" },
            { label: "Other", value: "other" },
            { label: "Use Item", value: "item" },
          ]}
      />
    </div>
    {#if evolutionChange.method === "item"}
      <AutoComplete
        label="Evolution Item"
        bind:value={evolutionItem}
        options={itemListOptions}
        popupId="evolution-item-popup"
        onSelection={(e) => {
                evolutionChange.item = e.detail.value;
                evolutionItem = e.detail.label;
              }}
      />
    {/if}
    {#if evolutionChange.method === "level_up"}
      <NumberInput
        id="evolution-level"
        bind:value={evolutionChange.level}
        label="Evolution Level"
        max={100}
      />
    {/if}
    {#if evolutionChange.method === "other"}
      <TextInput
        id="evolution-other"
        bind:value={evolutionChange.other}
        label="Evolution Other"
      />
    {/if}
    {#if evolutionChange.method !== "no_change"}
      <div class="w-44">
        <AutoComplete
          label="Evolves To"
          bind:value={evolvedPokemonName}
          placeholder="Evolves To"
          options={pokemonListOptions}
          popupId="ability-popup"
          onSelection={(e) => {
            console.log(e.detail);
              evolutionChange.evolved_pokemon = e.detail.value;
              evolvedPokemonName = e.detail.label;
            }}
        />
      </div>
    {/if}
    {#if displaySaveButton}
      <Button title="Save" class="mt-8" onClick={updateEvolutionChange} />
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
