<script lang="ts">
	import Textfield from "@smui/textfield";

	import SaveDiscard from "./save_discard_delete.svelte";
	import {
		type BlockDivisionPost,
		type BlockDivisionPostResult,
		type ErrorResult,
		block_division_post
	} from "../../post/block_division_post";
	import type {
		BlockDivisionState,
		BlockDivisionStateList
	} from "../../post/results/block_division_state";
	import ModifiableBucketList from "../modifiable_lists/modifiable_bucket_list.svelte";
	import { DisplayMode, handle_error } from "../../commons/commons";
	import type { Basis } from "../../post/results/state_components/basis";
	import ModifiableStringList from "../modifiable_lists/commons/modifiable_string_list.svelte";

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
		id = selected_division[0] + " (Copy)";
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
</script>

<div class="outer">
	<div class="main">
		<div class="toplabel">
			<Textfield style="width:100%" label="Label" bind:value={id} />
		</div>
		<div class="list">
			Buckets
			<ModifiableBucketList bucket_definitions={basis.bucket_definitions} />
		</div>

		<div class="area">
			Participants
			<div class="list">
				{#each basis.participant_definitions as participant_definition}
					<div class="area row">
						<div class="col textboxwidth">
							<Textfield label="Participant Name" bind:value={participant_definition.name} />
							<Textfield label="E-mail" type="email" bind:value={participant_definition.email} />
						</div>
						<div class="area col">
							Picks allowed per round
							<div class="row">
								{#each Object.keys(participant_definition.round_picks_allowed) as picks_allowed_index}
									<Textfield
										bind:value={participant_definition.round_picks_allowed[
											parseInt(picks_allowed_index)
										]}
										label={basis.selection_round_names[parseInt(picks_allowed_index)]}
										type="number"
										input$step="1"
									/>
								{/each}
							</div>
						</div>
					</div>
				{/each}
			</div>
		</div>

		<div class="area">
			Selection Rounds
			<ModifiableStringList label="Selection Round Name" bind:list={basis.selection_round_names} />
		</div>
	</div>
	<SaveDiscard {save_func} {discard_func} />
</div>

<style lang="scss">
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

	.row {
		display: flex;
		flex-direction: row;
		align-items: center;
	}
	.col {
		display: flex;
		flex-direction: column;
	}
	.textboxwidth {
		width: 300px;
	}
	.list {
		display: flex;
		flex-direction: column;
		margin-left: 20px;
	}
</style>
