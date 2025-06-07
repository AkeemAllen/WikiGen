import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";
import { get } from "svelte/store";
import type { Routes, TrainerInfo } from "../store/gameRoutes";
import type { PokemonMove } from "../store/pokemon";
import { db } from "../store/db";
import type { QueryResult } from "@tauri-apps/plugin-sql";
import { BaseDirectory, readFile } from "@tauri-apps/plugin-fs";
import { selectedWiki } from "../store";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type WithoutChild<T> = T extends { child?: any } ? Omit<T, "child"> : T;
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type WithoutChildren<T> = T extends { children?: any }
  ? Omit<T, "children">
  : T;
export type WithoutChildrenOrChild<T> = WithoutChildren<WithoutChild<T>>;
export type WithElementRef<T, U extends HTMLElement = HTMLElement> = T & {
  ref?: U | null;
};

export async function addMoves(movesToAdd: PokemonMove[], pokemonId: number) {
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

export async function shiftMoves(
  moveToEdit: PokemonMove[],
  pokemonId: number,
): Promise<QueryResult> {
  let updateQueries = "";
  for (let index = 0; index < moveToEdit.length; index++) {
    let move = moveToEdit[index];

    updateQueries += `UPDATE pokemon_movesets SET level_learned = ${move.level_learned} WHERE pokemon = ${pokemonId} AND move = ${move.id} AND learn_method = \'${move.learn_method}\'; `;
  }

  return await get(db).execute(
    `BEGIN TRANSACTION;
        ${updateQueries}
      COMMIT;
      `,
  );
}

export async function deleteMoves(
  movesToDelete: PokemonMove[],
  pokemonId: number,
): Promise<QueryResult> {
  let deleteQueries = "";
  for (let index = 0; index < movesToDelete.length; index++) {
    let move = movesToDelete[index];

    deleteQueries += `DELETE FROM pokemon_movesets WHERE pokemon = ${pokemonId} AND move = ${move.id} AND learn_method = \'${move.learn_method}\'; `;
  }

  return await get(db).execute(
    `BEGIN TRANSACTION;
        ${deleteQueries}
      COMMIT;
      `,
  );
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

type TypeColorScheme = {
  background: string;
  text: string;
};

export const typeColors: Record<string, TypeColorScheme> = {
  normal: {
    background: "bg-gray-200",
    text: "text-gray-400",
  },
  fire: {
    background: "bg-red-300",
    text: "text-red-500",
  },
  water: {
    background: "bg-blue-300",
    text: "text-blue-500",
  },
  grass: {
    background: "bg-green-300",
    text: "text-green-700",
  },
  electric: {
    background: "bg-yellow-200",
    text: "text-yellow-400",
  },
  ice: {
    background: "bg-cyan-100",
    text: "text-cyan-200",
  },
  fighting: {
    background: "bg-sanguine",
    text: "text-sanguine-dark",
  },
  poison: {
    background: "bg-purple-400",
    text: "text-purple-600",
  },
  ground: {
    background: "bg-yellow-500",
    text: "text-yellow-700",
  },
  flying: {
    background: "bg-sky-200",
    text: "text-sky-400",
  },
  psychic: {
    background: "bg-pink-300",
    text: "text-pink-500",
  },
  bug: {
    background: "bg-lime-300",
    text: "text-lime-500",
  },
  rock: {
    background: "bg-stone-300",
    text: "text-stone-500",
  },
  ghost: {
    background: "bg-violet-600",
    text: "text-violet-800",
  },
  dragon: {
    background: "bg-indigo-500",
    text: "text-indigo-700",
  },
  dark: {
    background: "bg-gray-600",
    text: "text-gray-800",
  },
  steel: {
    background: "bg-zinc-300",
    text: "text-zinc-500",
  },
  fairy: {
    background: "bg-pink-200",
    text: "text-pink-300",
  },
};

export async function getSpriteImage(pokemonName: string): Promise<string> {
  return await readFile(
    `${get(selectedWiki).name}/dist/docs/img/pokemon/${pokemonName}.png`,
    { baseDir: BaseDirectory.AppData },
  )
    .then((res) => {
      const blob = new Blob([res], { type: "image/png" });
      return URL.createObjectURL(blob);
    })
    .catch((err) => {
      if (err.includes("No such file or directory")) {
        return "Image Not Found";
      }
      return "Error loading image";
    });
}
