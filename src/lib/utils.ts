import { get } from "svelte/store";
import type { Routes, TrainerInfo } from "../store/gameRoutes";
import type { Pokemon, PokemonMove, MoveSetChange } from "../store/pokemon";
import { Operation } from "../store/pokemon";
import { db } from "../store/db";
import { moveList } from "../store/moves";
import type Database from "@tauri-apps/plugin-sql";
import type { QueryResult } from "@tauri-apps/plugin-sql";

export async function addMoves(
  moveAdditions: MoveSetChange[],
  pokemonId: number,
  database: Database,
): Promise<QueryResult> {
  let movesToInsert = "";
  for (let index = 0; index < moveAdditions.length; index++) {
    let move = moveAdditions[index];

    if (index === moveAdditions.length - 1) {
      movesToInsert += `(${pokemonId}, ${move.id}, '${move.method}', ${move.level})`;
      break;
    }
    movesToInsert += `(${pokemonId}, ${move.id}, '${move.method}', ${move.level}), `;
  }
  return await database.execute(`INSERT INTO pokemon_movesets (pokemon, move, learn_method, level_learned)
          VALUES ${movesToInsert}`);
}

export async function addMoves_(movesToAdd: PokemonMove[], pokemonId: number) {
  let movesToInsert = "";
  for (let index = 0; index < movesToAdd.length; index++) {
    let move = movesToAdd[index];

    if (index === movesToAdd.length - 1) {
      movesToInsert += `(${pokemonId}, ${move.id}, '${move.learn_method}', ${move.level_learned})`;
      break;
    }
    movesToInsert += `(${pokemonId}, ${move.id}, '${move.learn_method}', ${move.level_learned}), `;
  }
  return await get(db)
    .execute(`INSERT INTO pokemon_movesets (pokemon, move, learn_method, level_learned)
          VALUES ${movesToInsert}`);
}

async function shiftMoves(
  moveShifts: MoveSetChange[],
  pokemonId: number,
  database: Database,
): Promise<QueryResult> {
  let updateQueries = "";
  for (let index = 0; index < moveShifts.length; index++) {
    let move = moveShifts[index];

    updateQueries += `UPDATE pokemon_movesets SET level_learned = ${move.level} WHERE pokemon = ${pokemonId} AND move = ${move.id}; `;
  }

  return await database.execute(
    `BEGIN TRANSACTION;
        ${updateQueries}
      COMMIT;
      `,
  );
}

async function deleteMoves(
  moveDeletions: MoveSetChange[],
  pokemonId: number,
  database: Database,
): Promise<QueryResult> {
  let moveIds = moveDeletions.map((move) => move.id);
  return await database.execute(
    `DELETE FROM pokemon_movesets WHERE pokemon = ? AND move IN (${moveIds.join(",")})`,
    [pokemonId],
  );
}

async function replaceMoves(
  moveReplacements: MoveSetChange[],
  pokemonId: number,
  database: Database,
): Promise<QueryResult | void> {
  let movesToAdd = moveReplacements.map((move) => {
    return {
      id: move.secondaryMoveId,
      method: move.method,
      level: move.level,
    };
  }) as MoveSetChange[];

  let movesToDelete = moveReplacements.map((move) => {
    return { id: move.id };
  }) as MoveSetChange[];

  await addMoves(movesToAdd, pokemonId, database).then(async () => {
    return await deleteMoves(movesToDelete, pokemonId, database);
  });
}

function replaceByLevel(
  moveReplacementsByLevel: MoveSetChange[],
  pokemonId: number,
  database: Database,
) {}

async function swapMoves(
  movwSwaps: MoveSetChange[],
  pokemonId: number,
  database: Database,
) {
  let updateQueries = "";
  for (let index = 0; index < movwSwaps.length; index++) {
    let move = movwSwaps[index];

    updateQueries += `UPDATE pokemon_movesets SET move = ${move.secondaryMoveId} WHERE pokemon = ${pokemonId} AND move = ${move.id}; `;
  }

  return await database.execute(
    `BEGIN TRANSACTION;
          ${updateQueries}
        COMMIT;
        `,
  );
}

export async function modifyMoveSet(
  moveSetChangeList: MoveSetChange[],
  pokemonMoveset: PokemonMove[],
  pokemonId: number,
): Promise<PokemonMove[]> {
  let database = get(db);

  let moveAdditions: MoveSetChange[] = [];
  let moveShifts: MoveSetChange[] = [];
  let moveDeletions: MoveSetChange[] = [];
  let moveReplacements: MoveSetChange[] = [];
  let moveReplacementsByLevel: MoveSetChange[] = [];
  for (let index = 0; index < moveSetChangeList.length; index++) {
    let moveSetChange = moveSetChangeList[index];

    if (moveSetChange.operation === Operation.SHIFT) {
      moveShifts.push(moveSetChange);
      continue;
    }
    if (moveSetChange.operation === Operation.ADD) {
      moveAdditions.push(moveSetChange);
      continue;
    }

    if (moveSetChange.operation === Operation.DELETE) {
      moveDeletions.push(moveSetChange);
      continue;
    }

    if (moveSetChange.operation === Operation.REPLACE_MOVE) {
      moveReplacements.push(moveSetChange);
      continue;
    }
  }
  if (moveAdditions.length > 0) {
    await addMoves(moveAdditions, pokemonId, database).then((res) => {
      moveAdditions.forEach((move) => {
        pokemonMoveset.push({
          id: move.id,
          name: move.move,
          learn_method: move.method,
          level_learned: move.level,
        });
      });
    });
  }
  if (moveShifts.length > 0) {
    await shiftMoves(moveShifts, pokemonId, database).then((res) => {
      moveShifts.forEach((move) => {
        let index = pokemonMoveset.findIndex((m) => m.id === move.id);
        console.log(index);
        pokemonMoveset[index].level_learned = move.level;
      });
    });
  }
  if (moveDeletions.length > 0) {
    await deleteMoves(moveDeletions, pokemonId, database).then((res) => {
      moveDeletions.forEach((move) => {
        let index = pokemonMoveset.findIndex((m) => m.id === move.id);
        pokemonMoveset.splice(index, 1);
      });
    });
  }
  if (moveReplacements.length > 0) {
    await replaceMoves(moveReplacements, pokemonId, database).then((res) => {
      moveReplacements.forEach((move) => {
        let index = pokemonMoveset.findIndex((m) => m.id === move.id);
        pokemonMoveset[index].id = move.secondaryMoveId as number;
        pokemonMoveset[index].name = move.secondaryMove;
        pokemonMoveset[index].learn_method = move.method;
        pokemonMoveset[index].level_learned = move.level;
      });
    });
  }

  return pokemonMoveset;
}

export function sortRoutesByPosition(routes: Routes): Routes {
  // Convert object into an array of key-value pairs
  const routesArray = Object.entries(routes.routes);

  // Sort the array based on the "position" property
  routesArray.sort(
    ([, route1], [, route2]) => route1.position - route2.position,
  );

  // Reconstruct the sorted object
  const sortedRoutes: Routes = { routes: {}, encounter_areas: [] };
  routesArray.forEach(([routeName, route]) => {
    sortedRoutes.routes[routeName] = route;
  });

  sortedRoutes.encounter_areas = routes.encounter_areas;

  return sortedRoutes;
}

export function sortTrainersByPosition(trainers: {
  [key: string]: TrainerInfo;
}): { [key: string]: TrainerInfo } {
  // Convert object into an array of key-value pairs
  const trainersArray = Object.entries(trainers);

  // Sort the array based on the "position" property
  trainersArray.sort(
    ([, trainer1], [, trainer2]) => trainer1.position - trainer2.position,
  );

  // Reconstruct the sorted object
  const sortedTrainers: { [key: string]: TrainerInfo } = {};
  trainersArray.forEach(([trainerName, trainerInfo]) => {
    sortedTrainers[trainerName] = trainerInfo;
  });

  return sortedTrainers;
}

export const isNullEmptyOrUndefined = (value: any) => {
  return value === null || value === "" || value === undefined;
};

export const setUniquePokemonId = (
  trainers: { [key: string]: TrainerInfo },
  trainerName: string,
  pokemonName: string,
  pokemonList: [number, number, string, string][],
) => {
  let teamLength = 0;

  if (isNullEmptyOrUndefined(trainers)) {
    return `${
      pokemonList.find(([id, _, name, __]) => name === pokemonName)?.[1]
    }_${teamLength}_${Math.floor(Math.random() * 9000 + 1000)}`;
  }

  if (trainerName in trainers) {
    if (!isNullEmptyOrUndefined(trainers[trainerName].pokemon_team)) {
      teamLength = trainers[trainerName].pokemon_team.length;
    }
  }
  return `${
    pokemonList?.find(([id, _, name, __]) => name === pokemonName)?.[1]
  }_${teamLength}_${Math.floor(Math.random() * 9000 + 1000)}`;
};

export function getShardToWrite(pokemonId: number): number {
  let roundedId = Math.ceil(pokemonId / 100) * 100;

  if (roundedId > 900) {
    return 10;
  }

  // The first number of the rounded ID indicates the shard to write to
  return parseInt(roundedId.toString().charAt(0));
}

export function convertToTitle(input: string): string {
  return input
    .split("-") // Split the string by hyphens
    .map((word) => {
      // Capitalize the first letter and make the rest lowercase
      if (word.length === 0) return word;
      return word[0].toUpperCase() + word.slice(1).toLowerCase();
    })
    .join(" "); // Join the words with spaces
}

export function base64ToArray(base64String: string, contentType = "") {
  const byteCharacters = atob(base64String);
  const byteArrays = [];

  for (let i = 0; i < byteCharacters.length; i++) {
    byteArrays.push(byteCharacters.charCodeAt(i));
  }
  return byteArrays;
}
