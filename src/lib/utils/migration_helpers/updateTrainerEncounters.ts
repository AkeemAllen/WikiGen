import { get } from "svelte/store";
import { db } from "../../../store/db";
import type { Routes } from "../../../store/gameRoutes";
import type { Pokemon } from "../../../store/pokemon";

export default async function updateTrainerEncounters(
  routes: Routes,
): Promise<Routes> {
  let database = get(db);
  let updatedRoutes = routes;

  for (const [routeName, routeProperties] of Object.entries(
    updatedRoutes.routes,
  )) {
    for (const [name, trainerInfo] of Object.entries(
      routeProperties.trainers,
    )) {
      for (const [index, pokemon] of trainerInfo.pokemon_team.entries()) {
        await database
          .select<
            Pokemon[]
          >(`SELECT * FROM pokemon WHERE name = $1;`, [pokemon.name])
          .then((res) => {
            updatedRoutes.routes[routeName].trainers[name].pokemon_team[index] =
              {
                ...pokemon,
                types: res[0].types.split(","),
              };
          });
      }
    }
  }

  return updatedRoutes;
}
