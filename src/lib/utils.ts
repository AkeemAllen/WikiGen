import type { PokemonMoveSet } from "../store/pokemon";
import { Operation, type MoveSetChange } from "../types";

function addOrShiftMove(moveSet: PokemonMoveSet, moveSetChange: MoveSetChange, previous_learn_method: string[]) {
    if (previous_learn_method.includes("machine")) {
        moveSet[moveSetChange.move] = {
            level_learned: moveSetChange.level,
            learn_method: ["level-up", "machine"],
        }
    } else {
        moveSet[moveSetChange.move] = {
            level_learned: moveSetChange.level,
            learn_method: ["level-up"],
        }
    }

    console.log(moveSetChange.move)
    console.log(moveSet[moveSetChange.move])
}

export function modifyLevelUpMoveSet(moveSetChangeList: MoveSetChange[], moveSet: PokemonMoveSet ): PokemonMoveSet {
    for (const moveSetChange of moveSetChangeList) {
        let previous_learn_method: string[] = []
        if (moveSetChange.move in moveSet) {
            previous_learn_method = moveSet[moveSetChange.move].learn_method;
        }
        
        if (moveSetChange.operation === Operation.ADD || moveSetChange.operation === Operation.SHIFT) {
            addOrShiftMove(moveSet, moveSetChange, previous_learn_method) 
            break;
        } 
        // if (moveSetChange.operation === Operation.REPLACE_MOVE)
    }

    return moveSet;
}