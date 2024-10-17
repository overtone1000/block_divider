
interface BucketDefinition {
    available_slots: number,
    available_ancillaries: [string],
}

type SelectionIndex = number; //index in Basis.bucket_definitions
type SelectedParticipant = number; //index in Basis.participant_round_picks
type RoundIndex = number; //index in Basis.selection_rounds

interface Picks {
    [round_index: RoundIndex]: SelectionIndex
}

interface Basis {
    bucket_definitions: { [bucket_name: string]: BucketDefinition },
    participant_round_picks: { [participant_name: string]: Picks },
    selection_rounds: [string]
}

interface RoundState {
    ancillary_designations: { [ancillary_name: string]: SelectedParticipant }
    designations:
    ranks:
}

interface BucketState {
    round_states: { [round_index: RoundIndex]: RoundState }
}

interface State {
    basis: Basis,
    bucket_states: { [bucket_name: string]: BucketState },
    current_open_round: RoundIndex,
    selections: { [round_index: RoundIndex] }
}

type StateList = [State]