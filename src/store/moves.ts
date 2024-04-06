import { writable } from "svelte/store";

export type MoveDetails = {
  accuracy: number;
  pp: number;
  power: number;
  type: string;
  damage_class: string;
  past_values: any[];
  machine_details?: MachineVersion[];
};

type MachineVersion = {
    game_version?: string;
    technical_name?: string;
};

type Moves = {
    moves: { [key: string]: MoveDetails };
}

export let moves = writable<Moves>();
export let moveList = writable<string[]>([]);