<script lang="ts">
	import { mdiOrderNumericAscending } from "@mdi/js";
	import Tooltip, { Wrapper, Content } from "@smui/tooltip";
	import { Icon } from "@smui/icon-button";
	import Fab from "@smui/fab";
	import Container from "./container.svelte";
	import { onMount } from "svelte";
	import { handle_error } from "../commons/commons";
	import { block_division_post } from "../post/block_division_post";
	import type {
		BlockDivisionPost,
		BlockDivisionPostResult,
		ErrorResult,
		UserViewResult
	} from "../post/block_division_post";
	import {
		get_ancillary_designations,
		get_designations,
		get_sorted_rankings
	} from "../commons/bucket_functions";
	import type { BlockDivisionSelection } from "../post/results/block_division_state";

	let message: string = "Loading";
	let view: UserViewResult | undefined = undefined;

	onMount(async () => {
		const urlParams = new URLSearchParams(window.location.search);

		//Example: localhost:5173/?user_id=test_user_id&division_id=test_division_id
		console.debug(urlParams);
		let hash = urlParams.get("hash");

		if (hash !== null) {
			let post: BlockDivisionPost = {
				GetUserView: {
					hash: hash
				}
			};

			let callback = (result: BlockDivisionPostResult) => {
				if (typeof result === "object") {
					if ((result as ErrorResult).error) {
						handle_error((result as ErrorResult).error);
					} else {
						view = result as UserViewResult;
						message = JSON.stringify(result);
						console.debug(message);
					}
				}
			};

			block_division_post(post, callback);
		}
	});

	let selections: BlockDivisionSelection[] = [];

	$: {
		console.debug("View:", view);
		if (view !== undefined) {
			selections = [];
			let current_open_round = view.state.current_open_round;
			console.debug("Current open round:", current_open_round);
			if (current_open_round !== null) {
				let picks_allowed =
					view.state.basis.participant_definitions[view.user_id].round_picks_allowed[
						current_open_round
					];
				console.debug("Picks allowed:", picks_allowed);
				let current_selections = view.state.selections.state[current_open_round][view.user_id];
				if (current_selections.length !== picks_allowed) {
					console.error("Backend error. Picks allowed !== current selections array length.");
				}
				selections = current_selections;
			}
		} else {
			selections = [];
		}
	}
</script>

<Container title="Block Division">
	<div slot="contents">
		{#if view === undefined}
			<div>{message}</div>
		{:else}
			<table>
				<tr>
					<th> {view.state_id}</th>
					{#each view.state.basis.selection_round_names as selection_round, selection_round_index}
						<th>{selection_round}</th>
					{/each}
				</tr>
				{#each view.state.basis.bucket_definitions as bucket, bucket_index}
					<tr>
						<td>{bucket.name}</td>
						{#each view.state.basis.selection_round_names as selection_round, selection_round_index}
							<td>
								<div class="cell_contents">
									<div class="designations">
										{#each get_designations(view, selection_round_index, bucket_index) as designation}
											<ul>{designation}</ul>
										{/each}
									</div>
									<div class="ancillary_designations">
										{#each get_ancillary_designations(view, selection_round_index, bucket_index) as ancillary_designation}
											<ul>{ancillary_designation}</ul>
										{/each}
									</div>
									{#if view.state.bucket_states[bucket_index].round_states[selection_round_index].ranks !== null}
										<Wrapper rich>
											<Fab on:click={() => {}} mini>
												<Icon tag="svg" viewBox="0 0 24 24" on>
													<path fill="currentColor" d={mdiOrderNumericAscending} />
												</Icon>
											</Fab>
											<Tooltip>
												<Content
													><div>
														<ol>
															{#each get_sorted_rankings(view, selection_round_index, bucket_index) as ranking}
																<li>{ranking}</li>
															{/each}
														</ol>
													</div></Content
												></Tooltip
											>
										</Wrapper>
									{/if}
								</div>
							</td>
						{/each}
					</tr>
				{/each}
			</table>
			{#if selections.length > 0}
				<div>
					{#each selections as selection, selection_index}
						Selection {selection_index}
					{/each}
				</div>
			{/if}
		{/if}
	</div>
</Container>

<style>
	table,
	td,
	th,
	tr {
		border-style: solid;
		border-width: 1px;
		border-collapse: collapse;
		padding: 10px;
	}
	.cell_contents {
		display: flex;
		flex-direction: row;
		align-items: center;
	}
	.designations {
		display: flex;
	}
	.ancillary {
		display: flex;
	}
</style>
