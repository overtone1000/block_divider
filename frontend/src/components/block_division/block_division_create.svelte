<script lang="ts">
	import Textfield from "@smui/textfield";
	import { DisplayMode, handle_error } from "../../commons/commons";

	import SaveDiscard from "./save_discard.svelte";
	import {
		type BlockDivisionPost,
		type BlockDivisionPostResult,
		block_division_post
	} from "../../post/block_division_post";

	export let selected_division: [string, BlockDivisionState] | undefined = undefined;
	export let set_display_mode: (mode: DisplayMode) => void;

	let id: string;
	let basis: Basis;
	if (selected_division === undefined) {
		id = "New Block Division";
		basis = {
			bucket_definitions: [],
			participant_definitions: [],
			selection_round_names: []
		};
	} else {
		id = selected_division[0];
		basis = selected_division[1].basis;
	}

	//Don't need to manage bucket states. That's internal to the program.
	//Don't need to manage selections. That's controlled by users.

	/*Need to be able to change:
		[x]	block_division.basis.label;
		[ ]	block_division.basis.bucket_definitions;
		[ ]	block_division.basis.participant_definitions;
		[ ] block_division.basis.selection_round_names;
	*/

	let save_func = () => {
		let post: BlockDivisionPost = {
			NewBasis: {
				id: id,
				basis: basis
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
		<Textfield label="Label" bind:value={id} />

		<div class="area">
			Buckets
			<div class="list">
				{#each basis.bucket_definitions as bucket_definition}
					<div class="area">
						<Textfield label="Bucket Name" bind:value={bucket_definition.name} />
					</div>
				{/each}
			</div>
		</div>

		<div class="area">
			Participants
			<div class="list">
				{#each basis.participant_definitions as participant_definition}
					<div class="area">
						<Textfield label="Participant Name" bind:value={participant_definition.name} />
					</div>
				{/each}
			</div>
		</div>

		<div class="area">
			Selection Rounds
			<div class="list">
				{#each basis.selection_round_names as selection_round_name}
					<div class="area">
						<Textfield label="Round Name" bind:value={selection_round_name} />
					</div>
				{/each}
			</div>
		</div>
	</div>
	<SaveDiscard {save_func} {discard_func} />
</div>

<style>
	.outer {
		height: 100%;
		width: 100%;
		display: flex;
		flex-direction: column;
	}
	.main {
		flex-shrink: 1;
		overflow: auto;
	}
	.area {
		margin: 10px;
		padding: 10px;
		border-width: 1px;
		border-style: solid;
		border-radius: 10px;
	}
	.list {
		display: flex;
		flex-direction: column;
		margin-left: 20px;
	}
</style>
