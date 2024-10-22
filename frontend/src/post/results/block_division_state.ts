
type BlockDivisionStateList = { [label: string]: BlockDivisionState }

interface BlockDivisionState {
    basis: Basis,
    bucket_states: { [bucket_index: BucketIndex]: BucketState },
    current_open_round: RoundIndex | null,
    selections: { state: { [round_index: RoundIndex]: { [participant_index: ParticipantIndex]: [BucketIndex] } } }
}