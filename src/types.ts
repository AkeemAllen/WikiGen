export type MoveSetChange = {
  operation: string,
  move: string,
  level: number,
  secondaryMove: string,
}

export enum Operation {
  ADD = "add",
  SHIFT = "shift",
  DELETE = "delete",
  REPLACE_MOVE = "replace_move",
  REPLACE_BY_LEVEL = "replace_by_level",
  SWAP_MOVES = "swap_moves",
}