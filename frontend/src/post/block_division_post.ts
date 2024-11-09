import type { DeleteState } from "./posts/delete_state";
import type { GetStates } from "./posts/get_states";
import type { GetUserView } from "./posts/get_user_view";
import type { NewBasis } from "./posts/new_basis";
import type { SendStartEmail } from "./posts/send_start_email";
import type { SetState } from "./posts/set_open_round";
import type { SubmitSelections } from "./posts/submit_selections";
import type { BlockDivisionState, BlockDivisionStateList } from "./results/block_division_state";

export type BlockDivisionPost =
    { GetUserView: GetUserView } |
    { SubmitSelections: SubmitSelections } |
    { GetStates: GetStates } |
    { SetState: SetState } |
    { NewBasis: NewBasis } |
    { DeleteState: DeleteState } |
    { SendStartEmail: SendStartEmail };

export type ErrorResult = { error: Error };
export type UserViewResult = { user_id: number, state_id: string, state: BlockDivisionState };
export type BlockDivisionPostResult =
    ErrorResult |
    BlockDivisionStateList |
    UserViewResult |
    boolean;

export let block_division_post = (post: BlockDivisionPost, callback: (result: BlockDivisionPostResult) => void) => {
    //fetch("http://localhost:8181/block_division_post", {
    fetch(import.meta.env.VITE_POST_ROOT + "block_division_post", {
        method: "POST",
        body: JSON.stringify(post)
    }).then((result) => {
        result.json().then((json) => {
            callback(json);
        });
    });
};