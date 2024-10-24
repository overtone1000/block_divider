import type { BucketIndex, AncillaryIndex, Basis, RoundIndex, ParticipantIndex } from "./state_components/basis";
import type { BucketState } from "./state_components/state";

export type BlockDivisionStateList = { [label: string]: BlockDivisionState }


export interface BlockDivisionSelection {
    bucket_index: BucketIndex | null;
    ancillaries: AncillaryIndex[]
}

export interface BlockDivisionState {
    basis: Basis,
    bucket_states: { [bucket_index: BucketIndex]: BucketState },
    current_open_round: RoundIndex | null,
    selections: { state: { [round_index: RoundIndex]: { [participant_index: ParticipantIndex]: BlockDivisionSelection[] } } }
}