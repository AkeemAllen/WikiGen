import { get } from "svelte/store";
import { routes } from "../../../store/gameRoutes";

export function load({params}) {
    const routeDetails = get(routes).routes[params.slug];
    return {
        title: params.slug,
        routeDetails
    }
}