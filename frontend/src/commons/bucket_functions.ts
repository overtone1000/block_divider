import { type StateResult } from "../post/block_division_post";

export let get_participant_name = (view: StateResult, participant_index: number) => {
    return view.state.basis.participant_definitions[participant_index].name;
};

export let get_designations_summary = (view: StateResult, round_index: number, bucket_index: number) => {
    let designations =
        view.state.bucket_states[bucket_index].round_states[round_index].designations;
    let ancillary_designations =
        view.state.bucket_states[bucket_index].round_states[round_index].ancillary_designations;
};

export let get_ranking_as_string = (view: StateResult, round_index: number, bucket_index: number) => {
    let ranks = view.state.bucket_states[bucket_index].round_states[round_index].ranks;

    let map: Map<number, string> = new Map();
    for (let participant_index in ranks) {
        let rank = ranks[participant_index];
        let name = get_participant_name(view, parseInt(participant_index));
        map.set(rank, name);
    }

    let retval = "";
    let sorted_keys = map.keys().toArray().sort();
    for (let key of sorted_keys) {
        retval = retval + key + ": " + map.get(key) + " ";
    }

    console.debug(retval);
    return retval;
};