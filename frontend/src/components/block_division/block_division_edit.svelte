<script lang="ts">
	import Switch from "@smui/switch";
	import FormField from "@smui/form-field";
	import Select, { Option } from "@smui/select";
	import SaveDiscard from "./save_discard.svelte";

	import { DisplayMode, handle_error } from "../../commons/commons";
	import {
		type BlockDivisionPost,
		type BlockDivisionPostResult,
		block_division_post
	} from "../../post/block_division_post";

	export let selected_division: [string, BlockDivisionState];
	export let set_display_mode: (mode: DisplayMode) => void;

	let block_division = selected_division[1];

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
				if (result.error !== undefined) {
					handle_error(result.error);
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
	</div>
	<SaveDiscard {save_func} {discard_func} />
</div>

<style>
	.outer {
		display: flex;
		flex-direction: column;
	}
	.main {
		flex-shrink: 1;
		overflow: auto;
	}
</style>
