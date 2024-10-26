import type { BlockDivisionSelectionEntry } from "../results/block_division_state";
import type { GetUserView } from "./get_user_view";

export interface SubmitSelections {
    user_id: number,
    state_id: string,
    selections: BlockDivisionSelectionEntry[]
}