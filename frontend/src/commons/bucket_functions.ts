import { type UserViewResult } from "../post/block_division_post";

export let get_participant_name = (view: UserViewResult, participant_index: number) => {
    return view.state.basis.participant_definitions[participant_index].name;
};

export let get_ancillary_name = (view: UserViewResult, bucket_index: number, ancillary_index: number) => {
    return view.state.basis.bucket_definitions[bucket_index].available_ancillaries[ancillary_index];
}

export let get_designations = (view: UserViewResult, round_index: number, bucket_index: number) => {
    let designations = view.state.bucket_states[bucket_index].round_states[round_index].designations;

    let retval: string[] = [];
    for (let participant_index of designations) {
        let name: string = get_participant_name(view, participant_index);
        retval.push(name);
    }

    return retval;
};

export let get_ancillary_designations = (view: UserViewResult, round_index: number, bucket_index: number) => {
    let designations = view.state.bucket_states[bucket_index].round_states[round_index].ancillary_designations;

    let retval: string[] = [];
    for (let ancillary_index in designations) {
        let ancillary_name = get_ancillary_name(view, bucket_index, parseInt(ancillary_index));
        let participant_index = designations[ancillary_index];
        let participant_name: string = get_participant_name(view, participant_index);
        retval.push(ancillary_name + ": " + participant_name);
    }

    return retval;
}

export let get_sorted_rankings = (view: UserViewResult, round_index: number, bucket_index: number) => {
    let ranks = view.state.bucket_states[bucket_index].round_states[round_index].ranks;
    let retval: string[] = [];

    if (ranks === null) {
        return retval;
    }

    let map: Map<number, string> = new Map();
    for (let participant_index in ranks) {
        let rank = ranks[participant_index];
        let name = get_participant_name(view, parseInt(participant_index));
        map.set(rank, name);
    }


    let sorted_keys = map.keys().toArray().sort();
    for (let key of sorted_keys) {
        let name: string = map.get(key) as string;
        retval.push(name);
    }

    return retval;
};