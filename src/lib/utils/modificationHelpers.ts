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
import type { Item, ModifiedItems } from "../../store/items";
import { items, modifiedItems as _mdItems } from "../../store/items";
import {
  natures,
  modifiedNatures as _mdNatures,
  type Nature,
} from "../../store/natures";
import {
  abilities,
  modifiedAbilities as _mdAbilities,
  type Ability,
} from "../../store/abilities";

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
  itemName: string,
  itemDetails: Item,
) {
  let modifiedItems = get(_mdItems);
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

  if (
    modifiedItems[itemName].original.effect === "" ||
    modifiedItems[itemName].original.sprite === ""
  ) {
    modifiedItems[itemName].original.effect = get(items)[itemName].effect;
    modifiedItems[itemName].original.effect = get(items)[itemName].sprite;
  }

  modifiedItems[itemName].modified.effect = itemDetails.effect;
  modifiedItems[itemName].modified.sprite = itemDetails.sprite;

  if (
    _.isEqual(
      modifiedItems[itemName].original,
      modifiedItems[itemName].modified,
    ) &&
    !modifiedItems[itemName].is_new_item
  ) {
    delete modifiedItems[itemName];
  }

  await writeTextFile(
    `${get(selectedWiki).name}/data/modifications/modified_items_natures_abilities.json`,
    JSON.stringify({
      items: modifiedItems,
      natures: get(_mdNatures),
      abilities: get(_mdAbilities),
    }),
    { dir: BaseDirectory.AppData },
  );
}

export async function updateAbilityModifications(
  abilityName: string,
  abilityDetails: Ability,
) {
  let modifiedAbilities = get(_mdAbilities);
  if (!modifiedAbilities[abilityName]) {
    modifiedAbilities[abilityName] = {
      original: {
        effect: "",
      },
      modified: {
        effect: "",
      },
      is_new_ability: false,
    };
  }

  if (modifiedAbilities[abilityName].original.effect === "") {
    modifiedAbilities[abilityName].original.effect =
      get(abilities)[abilityName].effect;
  }

  modifiedAbilities[abilityName].modified.effect = abilityDetails.effect;

  if (
    modifiedAbilities[abilityName].original.effect ===
      modifiedAbilities[abilityName].modified.effect &&
    !modifiedAbilities[abilityName].is_new_ability
  ) {
    delete modifiedAbilities[abilityName];
  }

  await writeTextFile(
    `${get(selectedWiki).name}/data/modifications/modified_items_natures_abilities.json`,
    JSON.stringify({
      items: get(_mdItems),
      natures: get(_mdNatures),
      abilities: modifiedAbilities,
    }),
    { dir: BaseDirectory.AppData },
  );
}

export async function updateNatureModifications(
  natureName: string,
  natureDetails: Nature,
) {
  let modifiedNatures = get(_mdNatures);
  if (!modifiedNatures[natureName]) {
    modifiedNatures[natureName] = {
      original: {
        increased_stat: "",
        decreased_stat: "",
      },
      modified: {
        increased_stat: "",
        decreased_stat: "",
      },
      is_new_nature: false,
    };
  }

  if (
    _.isEqual(modifiedNatures[natureName].original, {
      increased_stat: "",
      decreased_stat: "",
    })
  ) {
    modifiedNatures[natureName].original = {
      increased_stat: get(natures)[natureName].increased_stat,
      decreased_stat: get(natures)[natureName].decreased_stat,
    };
  }

  modifiedNatures[natureName].modified = {
    increased_stat: natureDetails.increased_stat,
    decreased_stat: natureDetails.decreased_stat,
  };

  if (
    _.isEqual(
      modifiedNatures[natureName].original,
      modifiedNatures[natureName].modified,
    ) &&
    !modifiedNatures[natureName].is_new_nature
  ) {
    delete modifiedNatures[natureName];
  }

  await writeTextFile(
    `${get(selectedWiki).name}/data/modifications/modified_items_natures_abilities.json`,
    JSON.stringify({
      items: get(_mdItems),
      natures: modifiedNatures,
      abilities: get(_mdAbilities),
    }),
    { dir: BaseDirectory.AppData },
  );
}
