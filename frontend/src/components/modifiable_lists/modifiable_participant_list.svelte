<script lang="ts">
	import Textfield from "@smui/textfield";
	import ModifiableStringList from "./commons/modifiable_string_list.svelte";
	import ModifiableListDeleteButton from "./commons/modifiable_list_delete_button.svelte";
	import type {
		BucketDefinition,
		ParticipantDefinition
	} from "../../post/results/state_components/basis";
	import ModifiableListContainer from "./commons/modifiable_list_container.svelte";
	import ModifiableNumberList from "./commons/modifiable_number_list.svelte";
	export let participants: ParticipantDefinition[];
</script>

<ModifiableListContainer
	add={() => {
		participants.push({
			name: "New Participant",
			email: "",
			round_picks_allowed: []
		});
		participants = participants;
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
				<ModifiableNumberList bind:list={participant.round_picks_allowed} label="Round Picks" />
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
