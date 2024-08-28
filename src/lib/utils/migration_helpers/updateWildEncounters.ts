import { get } from "svelte/store";
import type { Routes } from "../../../store/gameRoutes";

export default function updateWildEncounters(routes: any): Routes {
  console.log({ routes });
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
        delete updatedRoutes.routes[routeName].wild_encounters[encounterArea][
          index
        ].encounter_type;
      }
    }
  }
  if (updatedRoutes.encounter_types === undefined) {
    updatedRoutes.encounter_areas = [];
  } else {
    updatedRoutes.encounter_areas = [...updatedRoutes.encounter_types];
    delete updatedRoutes.encounter_types;
  }

  return updatedRoutes;
}
