
type BlockDivisionStateList = [[string, BlockDivisionState]]

interface BlockDivisionState {
    basis: Basis,
    bucket_states: { [bucket_index: BucketIndex]: BucketState },
    current_open_round: RoundIndex | undefined,
    selections: { [round_index: RoundIndex]: { [participant_index: ParticipantIndex]: [BucketIndex] } }
}