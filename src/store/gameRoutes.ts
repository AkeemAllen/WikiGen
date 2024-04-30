import { writable } from "svelte/store"

export type Routes = {
    routes: {[key: string]: RouteProperties}
}

type RouteProperties = {
    position: number,
    trainers?: {
        [key: string]: Trainer
    },
    wild_encounters?: {
        [key: string]: WildEncounter
    },
    wild_encounter_area_levels?: {
        [key: string]: string
    }
}

type Trainer = {
    pokemon_team: TrainerPokemon[];
}

type TrainerPokemon = {
    id: number,
    unique_id: string,
    name: string,
    level: number,
    moves?: string[],
    item?: string,
    nature?: string,
    ability?: string,
    trainer_version?: string[]
}

type WildEncounter = {
    id: number,
    name: string,
    encounter_rate: number
}

export let routes = writable<Routes>()
// export let routesList = writable<>()