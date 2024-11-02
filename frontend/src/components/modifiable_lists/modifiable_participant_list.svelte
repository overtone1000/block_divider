<script lang="ts">
	import Textfield from "@smui/textfield";
	import ModifiableStringList from "./commons/modifiable_string_list.svelte";
	import ModifiableListDeleteButton from "./commons/modifiable_list_delete_button.svelte";
	import type {
		Basis,
		BucketDefinition,
		ParticipantDefinition
	} from "../../post/results/state_components/basis";
	import ModifiableListContainer from "./commons/modifiable_list_container.svelte";
	import ModifiableNumberList from "./commons/modifiable_number_list.svelte";
	export let basis: Basis;

	let participants: ParticipantDefinition[];
	let round_pick_count: number;

	$: {
		participants = basis.participant_definitions;
		round_pick_count = basis.selection_round_names.length;

		for (let participant of participants) {
			if (participant.round_picks_allowed.length > round_pick_count) {
				participant.round_picks_allowed = participant.round_picks_allowed.slice(
					0,
					round_pick_count
				);
			} else if (participant.round_picks_allowed.length < round_pick_count) {
				while (participant.round_picks_allowed.length < round_pick_count) {
					participant.round_picks_allowed.push(0);
				}
			}
		}
	}
</script>

<ModifiableListContainer
	add={() => {
		participants.push({
			name: "New Participant",
			email: "",
			round_picks_allowed: []
		});
		basis = basis;
	}}
>
	{#each participants as participant, index}
		<div class="area">
			<div class="row">
				<Textfield label="Participant name" bind:value={participant.name} />
				<Textfield label="Email" type="email" bind:value={participant.email} />
				<div class="spacer" />
				<ModifiableListDeleteButton bind:list={participants} {index} />
			</div>
			<div class="area">
				Selections Allowed
				{#each participant.round_picks_allowed as picks, index}
					<Textfield
						label={"Selections for " + basis.selection_round_names[index]}
						type="number"
						bind:value={picks}
					/>
				{/each}
			</div>
		</div>
	{/each}
</ModifiableListContainer>

<style>
	.area {
		margin: 10px;
		padding: 10px;
		border-width: 1px;
		border-style: solid;
		border-radius: 10px;
	}
	.row {
		display: flex;
		flex-direction: row;
	}
	.spacer {
		flex-grow: 1;
	}
</style>
