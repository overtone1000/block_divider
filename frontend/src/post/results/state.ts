
interface BucketDefinition {
    available_slots: number,
    available_ancillaries: [string],
}

type SelectionIndex = number;
type SelectedParticipant = number; //index in Basis.participant_round_picks

interface Picks {
    [round_index: number]: SelectionIndex
}

interface Basis {
    bucket_definitions: { [bucket_name: string]: BucketDefinition },
    participant_round_picks: { [participant_name: string]: Picks },
    selection_rounds: [string]
}

interface RoundState {
    ancillary_designations: { [ancillary_name: string]: number }
}

interface BucketState {
    round_states: { [round_index: number]: RoundState }
}

interface State {
    basis: Basis,
    bucket_states: { [bucket_name: string]: BucketState }

}

type StateList = [State]