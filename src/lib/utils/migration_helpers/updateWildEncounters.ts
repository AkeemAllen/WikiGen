import { get } from "svelte/store";
import type { Routes } from "../../../store/gameRoutes";

export default function updateWildEncounters(routes: Routes): Routes {
  let updatedRoutes = routes;

  for (const [routeName, routeProperties] of Object.entries(
    updatedRoutes.routes,
  )) {
    for (const [encounterArea, wildEncounters] of Object.entries(
      routeProperties.wild_encounters,
    )) {
      for (const [index, encounter] of wildEncounters.entries()) {
        updatedRoutes.routes[routeName].wild_encounters[encounterArea][index] =
          {
            ...encounter,
            route: routeName,
            special_note: "",
            encounter_area: encounterArea,
          };
      }
    }
  }

  return updatedRoutes;
}
