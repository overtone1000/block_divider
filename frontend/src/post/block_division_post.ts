export type BlockDivisionPost = { GetState: GetState } | { GetDivisions: GetDivisions } | { SetState: SetState } | { NewBasis: NewBasis };
export type BlockDivisionPostResult = { State?: BlockDivisionState, error?: Error } | boolean;

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