import type { Routes, TrainerInfo } from "../store/gameRoutes";
import type { PokemonMoveSet } from "../store/pokemon";
import { Operation, type MoveSetChange } from "../types";

function addOrShiftMove(
  moveSet: PokemonMoveSet,
  moveSetChange: MoveSetChange,
  previous_learn_method: string[],
) {
  if (previous_learn_method.includes("machine")) {
    moveSet[moveSetChange.move] = {
      level_learned: moveSetChange.level,
      learn_method: ["level-up", "machine"],
    };
  } else {
    moveSet[moveSetChange.move] = {
      level_learned: moveSetChange.level,
      learn_method: ["level-up"],
    };
  }
}

function replaceMove(moveSet: PokemonMoveSet, moveSetChange: MoveSetChange) {
  moveSet[moveSetChange.secondaryMove] = moveSet[moveSetChange.move];
  delete moveSet[moveSetChange.move];
}

function replaceByLevel(
  moveSet: PokemonMoveSet,
  moveSetChange: MoveSetChange,
  previous_learn_method: string[],
) {
  for (const move of Object.entries(moveSet)) {
    if (move[1].level_learned === moveSetChange.level) {
      delete moveSet[move[0]];
      addOrShiftMove(moveSet, moveSetChange, previous_learn_method);
      break;
    }
  }
}

function swapMoves(
  moveSet: PokemonMoveSet,
  moveSetChange: MoveSetChange,
  previous_learn_method: string[],
) {
  const temp = moveSet[moveSetChange.move];
  moveSet[moveSetChange.move] = moveSet[moveSetChange.secondaryMove];
  moveSet[moveSetChange.secondaryMove] = temp;

  if (previous_learn_method.includes("machine")) {
    moveSet[moveSetChange.move].learn_method = ["level-up", "machine"];
    moveSet[moveSetChange.secondaryMove].learn_method = ["level-up", "machine"];
  } else {
    moveSet[moveSetChange.move].learn_method = ["level-up"];
    moveSet[moveSetChange.secondaryMove].learn_method = ["level-up"];
  }
}

export function modifyLevelUpMoveSet(
  moveSetChangeList: MoveSetChange[],
  moveSet: PokemonMoveSet,
): PokemonMoveSet {
  for (const moveSetChange of moveSetChangeList) {
    let previous_learn_method: string[] = [];
    if (moveSetChange.move in moveSet) {
      previous_learn_method = moveSet[moveSetChange.move].learn_method;
    }

    if (
      moveSetChange.operation === Operation.ADD ||
      moveSetChange.operation === Operation.SHIFT
    ) {
      addOrShiftMove(moveSet, moveSetChange, previous_learn_method);
      continue;
    }

    if (moveSetChange.operation === Operation.REPLACE_MOVE) {
      replaceMove(moveSet, moveSetChange);
      continue;
    }

    if (moveSetChange.operation === Operation.REPLACE_BY_LEVEL) {
      replaceByLevel(moveSet, moveSetChange, previous_learn_method);
      continue;
    }

    if (moveSetChange.operation === Operation.SWAP_MOVES) {
      swapMoves(moveSet, moveSetChange, previous_learn_method);
      continue;
    }

    if (moveSetChange.operation === Operation.DELETE) {
      delete moveSet[moveSetChange.move];
      continue;
    }
  }

  return moveSet;
}

export function sortRoutesByPosition(routes: Routes): Routes {
  // Convert object into an array of key-value pairs
  const routesArray = Object.entries(routes.routes);

  // Sort the array based on the "position" property
  routesArray.sort(
    ([, route1], [, route2]) => route1.position - route2.position,
  );

  // Reconstruct the sorted object
  const sortedRoutes: Routes = { routes: {}, encounter_types: [] };
  routesArray.forEach(([routeName, route]) => {
    sortedRoutes.routes[routeName] = route;
  });

  return sortedRoutes;
}

export const isNullEmptyOrUndefined = (value: any) => {
  return value === null || value === "" || value === undefined;
};

export const setUniquePokemonId = (
  trainers: { [key: string]: TrainerInfo },
  trainerName: string,
  pokemonName: string,
  pokemonList: [string, number][],
) => {
  let teamLength = 0;

  console.log(pokemonList.find(([name, _]) => name === pokemonName)?.[1]);
  if (isNullEmptyOrUndefined(trainers)) {
    return `${
      pokemonList.find(([name, _]) => name === pokemonName)?.[1]
    }_${teamLength}_${Math.floor(Math.random() * 9000 + 1000)}`;
  }

  if (trainerName in trainers) {
    if (!isNullEmptyOrUndefined(trainers[trainerName].pokemon_team)) {
      teamLength = trainers[trainerName].pokemon_team.length;
    }
  }
  return `${
    pokemonList?.find(([name, _]) => name === pokemonName)?.[1]
  }_${teamLength}_${Math.floor(Math.random() * 9000 + 1000)}`;
};
