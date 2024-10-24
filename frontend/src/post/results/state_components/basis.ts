export type BucketIndex = number;
export type ParticipantIndex = number;
export type RoundIndex = number;
export type AncillaryIndex = number;

export interface Basis {
    bucket_definitions: BucketDefinition[],
    participant_definitions: ParticipantDefinition[],
    selection_round_names: string[],
}

export interface BucketDefinition {
    name: string,
    available_slots: number,
    available_ancillaries: string[],
}

export interface ParticipantDefinition {
    name: string,
    email: string,
    round_picks_allowed: number[]
}
