import type { Routes } from "../../../store/gameRoutes";
import { cloneDeep } from "../cloneDeep";

export default function updateDexNumberReferences(
  routes: Routes,
  pokemonList: [number, number, string, string][],
): Routes {
  let updatedRoutes: Routes = cloneDeep(routes);
  for (const [routeName, properties] of Object.entries(routes.routes)) {
    for (const [encounterArea, wildEncounters] of Object.entries(
      properties.wild_encounters,
    )) {
      for (const [index, encounter] of wildEncounters.entries()) {
        let dexNumber = pokemonList.find(
          (pokemon) => pokemon[2] === encounter.name,
        )?.[1] as number;
        updatedRoutes.routes[routeName].wild_encounters[encounterArea][
          index
        ].id = dexNumber;
      }
    }
  }
  return updatedRoutes;
}
