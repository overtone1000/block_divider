type BucketIndex = number;
type ParticipantIndex = number;
type RoundIndex = number;
type AncillaryIndex = number;

interface Basis {
    label: string,
    bucket_definitions: { [bucket_index: BucketIndex]: BucketDefinition },
    participant_definitions: { [participant_index: ParticipantIndex]: ParticipantDefinition },
    selection_round_names: { [round_index: RoundIndex]: string },
}

interface BucketDefinition {
    name: string,
    available_slots: number,
    available_ancillaries: { [ancillary_index: AncillaryIndex]: string },
}

interface ParticipantDefinition {
    name: string,
    round_picks_allowed: { [round_index: RoundIndex]: number }
}
