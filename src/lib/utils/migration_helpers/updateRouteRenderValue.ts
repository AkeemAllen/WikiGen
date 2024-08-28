import type { Routes } from "../../../store/gameRoutes";

export default function updateRouteRender(routes: Routes): Routes {
  let updatedRoutes = routes;

  for (const [routeName, routeProperties] of Object.entries(
    updatedRoutes.routes,
  )) {
    if (routeProperties.render !== undefined) {
      continue;
    }
    routeProperties.render = true;
  }

  return updatedRoutes;
}
