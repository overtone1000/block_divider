<script lang="ts">
	import Switch from "@smui/switch";
	import FormField from "@smui/form-field";
	import Select, { Option } from "@smui/select";
	import SaveDiscard from "./save_discard_delete.svelte";
	import Button, { Label } from "@smui/button";
	import {
		type BlockDivisionState,
		type BlockDivisionStateList
	} from "../../post/results/block_division_state";
	import { DisplayMode, handle_error } from "../../commons/commons";
	import {
		type BlockDivisionPost,
		type BlockDivisionPostResult,
		type ErrorResult,
		block_division_post
	} from "../../post/block_division_post";

	export let selected_division: [string, BlockDivisionState];
	export let set_display_mode: (mode: DisplayMode) => void;

	let block_division = selected_division[1];

	console.debug("Block division:", block_division);

	//Don't need to manage bucket states. That's internal to the program.
	//Don't need to manage selections. That's controlled by users.

	//Need to be able to change current open round or close the block division. That's all that should be allowed.

	let enabled: boolean = block_division.current_open_round !== null;

	let current_open_round: number;
	if (block_division.current_open_round !== null) {
		current_open_round = block_division.current_open_round;
	} else {
		current_open_round = 0;
	}

	let label: string;

	$: {
		if (enabled) {
			block_division.current_open_round = current_open_round;
			label = "Selection open";
		} else {
			block_division.current_open_round = null;
			label = "Selection closed";
		}
		console.debug(block_division);
	}

	let round_indices: number[] = [];
	for (let n = 0; n < block_division.basis.selection_round_names.length; n++) {
		console.debug("Pushing index " + n);
		round_indices.push(n);
	}

	let keyfunc = (round_key: number) => {
		if (round_key === undefined) {
			return "";
		} else {
			return block_division.basis.selection_round_names[round_key].toString();
		}
	};

	let save_func = () => {
		let post: BlockDivisionPost = {
			SetState: {
				id: selected_division[0],
				state: block_division
			}
		};

		let callback = (result: BlockDivisionPostResult) => {
			if (typeof result === "object") {
				if ((result as ErrorResult).error) {
					handle_error((result as ErrorResult).error);
				}
			} else {
				if (result) {
					set_display_mode(DisplayMode.List);
				}
			}
		};

		block_division_post(post, callback);
	};

	let discard_func = () => {
		set_display_mode(DisplayMode.List);
	};

	let delete_func = () => {
		let post: BlockDivisionPost = {
			DeleteState: {
				id: selected_division[0]
			}
		};

		let callback = (result: BlockDivisionPostResult) => {
			if (typeof result === "object") {
				if ((result as ErrorResult).error) {
					handle_error((result as ErrorResult).error);
				}
			} else {
				if (result) {
					set_display_mode(DisplayMode.List);
				}
			}
		};

		block_division_post(post, callback);
	};

	let send_intro_email = (id: number) => {
		let post: BlockDivisionPost = {
			SendStartEmail: {
				user_id: id,
				state_id: selected_division[0]
			}
		};

		let callback = (result: BlockDivisionPostResult) => {
			if (typeof result === "object") {
				if ((result as ErrorResult).error) {
					handle_error((result as ErrorResult).error);
				}
			} else {
				console.debug("Would be nice to display confirmation here.");
			}
		};

		block_division_post(post, callback);
	};
</script>

<div class="outer">
	<div class="main">
		<FormField>
			<Switch bind:checked={enabled} />
			<span slot="label">{label}</span>
		</FormField>

		<Select
			disabled={!enabled}
			key={keyfunc}
			bind:value={current_open_round}
			label="Current Open Round"
		>
			{#each round_indices as round_index}
				<Option value={round_index}
					>{block_division.basis.selection_round_names[round_index]}</Option
				>
			{/each}
		</Select>

		{#each block_division.basis.participant_definitions as participant_definition, participant_index}
			<div class="participant">
				<div class="participant_name">
					{participant_definition.name}
				</div>
				<Button
					color="primary"
					on:click={() => send_intro_email(participant_index)}
					variant="raised"
				>
					<Label>Send Introduction E-mail</Label>
				</Button>
			</div>
		{/each}
	</div>
	<SaveDiscard {save_func} {discard_func} {delete_func} />
</div>

<style>
	.outer {
		display: flex;
		flex-direction: column;
		flex-grow: 1;
		overflow: auto;
		padding: 20px;
		min-height: 50%; /* Needed for select to display correctly */
	}
	.main {
		display: flex;
		flex-direction: column;
		flex-grow: 1;
		padding: 20px;
	}
	.participant {
		display: flex;
		flex-direction: row;
		padding: 20px;
		justify-content: flex-start;
		align-items: center;
	}
	.participant_name {
		padding: 20px;
	}
</style>
