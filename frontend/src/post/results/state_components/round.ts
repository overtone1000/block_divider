interface RoundState {
    ancillary_designations: { [ancillary_index: AncillaryIndex]: ParticipantIndex }
    designations: [ParticipantIndex]
    ranks: { [participant_index: ParticipantIndex]: number }
}