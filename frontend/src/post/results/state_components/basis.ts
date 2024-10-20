type BucketIndex = number;
type ParticipantIndex = number;
type RoundIndex = number;
type AncillaryIndex = number;

interface Basis {
    bucket_definitions: BucketDefinition[],
    participant_definitions: ParticipantDefinition[],
    selection_round_names: string[],
}

interface BucketDefinition {
    name: string,
    available_slots: number,
    available_ancillaries: { [ancillary_index: AncillaryIndex]: string },
}

interface ParticipantDefinition {
    name: string,
    email: string,
    round_picks_allowed: { [round_index: RoundIndex]: number }
}
