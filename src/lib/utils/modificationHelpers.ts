import _ from "lodash";
import type {
  Evolution,
  ModifiedPokemon,
  ModifiedPokemonDetails,
  PokemonDetails,
} from "../../store/pokemon";
import { BaseDirectory, writeTextFile } from "@tauri-apps/api/fs";
import { selectedWiki } from "../../store";
import { get } from "svelte/store";
import type { Item, ModifiedItem, ModifiedItems } from "../../store/items";
import { items } from "../../store/items";
import { modifiedNatures } from "../../store/natures";
import { modifiedAbilities } from "../../store/abilities";

export async function updatePokemonModifications(
  modifiedPokemon: ModifiedPokemon,
  originalPokemonDetails: PokemonDetails,
  pokemonDetails: PokemonDetails,
) {
  if (
    _.isEqual(originalPokemonDetails.types, pokemonDetails.types) &&
    _.isEqual(originalPokemonDetails.evolution, pokemonDetails.evolution)
  ) {
    return;
  }

  let pokemonName = pokemonDetails.name;

  if (!modifiedPokemon[pokemonName]) {
    modifiedPokemon[pokemonName] = {
      id: pokemonDetails.id,
      evolution: {
        method: "no_change",
        level: 0,
        item: "",
        other: "",
        evolves_to: { id: 0, pokemon_name: "" },
      },
      types: {
        original: [],
        modified: [],
      },
    };
  }

  if (!_.isEqual(originalPokemonDetails.types, pokemonDetails.types)) {
    checkModifiedTypes(
      modifiedPokemon[pokemonName],
      pokemonDetails.types,
      originalPokemonDetails.types,
    );
  }
  if (!_.isEqual(originalPokemonDetails.evolution, pokemonDetails.evolution)) {
    checkModifiedEvolutions(
      modifiedPokemon[pokemonName],
      pokemonDetails.evolution,
    );
  }

  if (
    modifiedPokemon[pokemonName].evolution.method === "no_change" &&
    modifiedPokemon[pokemonName].types.original.length === 0
  ) {
    delete modifiedPokemon[pokemonName];
  }

  await writeTextFile(
    `${get(selectedWiki).name}/data/modifications/modified_pokemon.json`,
    JSON.stringify(modifiedPokemon),
    { dir: BaseDirectory.AppData },
  );
}

function checkModifiedTypes(
  modifiedPokemonDetails: ModifiedPokemonDetails,
  currentTypes: string[],
  originalTypes: string[],
) {
  if (modifiedPokemonDetails.types.original.length === 0) {
    modifiedPokemonDetails.types.original = originalTypes;
  }
  modifiedPokemonDetails.types.modified = currentTypes;

  if (
    _.isEqual(
      modifiedPokemonDetails.types.original.sort(),
      modifiedPokemonDetails.types.modified.sort(),
    )
  ) {
    modifiedPokemonDetails.types = {
      original: [],
      modified: [],
    };
  }
}

function checkModifiedEvolutions(
  modifiedPokemonDetails: ModifiedPokemonDetails,
  evolution: Evolution,
) {
  if (evolution.method !== "no_change") {
    modifiedPokemonDetails.evolution = evolution;
  } else {
    modifiedPokemonDetails.evolution = {
      method: "no_change",
      level: 0,
      item: "",
      other: "",
      evolves_to: { id: 0, pokemon_name: "" },
    };
  }
}

export async function updateItemModifications(
  modifiedItems: ModifiedItems,
  itemName: string,
  itemDetails: Item,
) {
  if (!modifiedItems[itemName]) {
    modifiedItems[itemName] = {
      original: {
        effect: "",
        sprite: "",
      },
      modified: {
        effect: "",
        sprite: "",
      },
      is_new_item: false,
    };
  }

  if (modifiedItems[itemName].original.effect === "") {
    modifiedItems[itemName].original.effect = get(items)[itemName].effect;
  }

  modifiedItems[itemName].modified.effect = itemDetails.effect;

  if (
    modifiedItems[itemName].original.effect ===
      modifiedItems[itemName].modified.effect &&
    !modifiedItems[itemName].is_new_item
  ) {
    delete modifiedItems[itemName];
  }

  await writeTextFile(
    `${get(selectedWiki).name}/data/modifications/modified_items_natures_abilities.json`,
    JSON.stringify({
      items: modifiedItems,
      natures: get(modifiedNatures),
      abilities: get(modifiedAbilities),
    }),
    { dir: BaseDirectory.AppData },
  );
}
