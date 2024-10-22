export type BlockDivisionPost =
    { GetUserView: GetUserView } |
    { GetStates: GetStates } |
    { SetState: SetState } |
    { NewBasis: NewBasis } |
    { DeleteState: DeleteState } |
    { SendStartEmail: SendStartEmail };

export type ErrorResult = { error: Error };
export type StateResult = { id: string, state: BlockDivisionState };
export type BlockDivisionPostResult =
    ErrorResult |
    BlockDivisionStateList |
    StateResult |
    boolean;

export let block_division_post = (post: BlockDivisionPost, callback: (result: BlockDivisionPostResult) => void) => {
    fetch("http://localhost:8181/block_division_post", {
        method: "POST",
        body: JSON.stringify(post)
    }).then((result) => {
        result.json().then((json) => {
            callback(json);
        });
    });
};