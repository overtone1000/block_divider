<script lang="ts">
	import { mdiOrderNumericAscending } from "@mdi/js";
	import Tooltip, { Wrapper, Content } from "@smui/tooltip";
	import Select, { Option } from "@smui/select";
	import { Icon } from "@smui/icon-button";
	import Fab from "@smui/fab";
	import Container from "../container.svelte";
	import { onMount } from "svelte";
	import { handle_error } from "../../commons/commons";
	import { block_division_post } from "../../post/block_division_post";
	import type {
		BlockDivisionPost,
		BlockDivisionPostResult,
		ErrorResult,
		UserViewResult
	} from "../../post/block_division_post";
	import {
		get_ancillary_designations,
		get_designations,
		get_sorted_rankings
	} from "../../commons/bucket_functions";
	import type { BlockDivisionSelectionEntry } from "../../post/results/block_division_state";
	import Switch from "@smui/switch";
	import FormField from "@smui/form-field";
	import Checkbox from "@smui/checkbox";
	import Button from "@smui/button";

	export let view: UserViewResult | undefined = undefined;
	let message: string = "Loading";
	let urlhash: string | null = null;
	let title: string = "Block Division";

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

	onMount(async () => {
		const urlParams = new URLSearchParams(window.location.search);
		urlhash = urlParams.get("hash");
		if (urlhash !== null) {
			let post: BlockDivisionPost = {
				GetUserView: {
					hash: urlhash
				}
			};

			block_division_post(post, callback);
		}
	});

	let selections: BlockDivisionSelectionEntry[] = [];

	let submit: undefined | (() => void) = undefined;

	$: {
		if (view !== undefined && view.user_id !== undefined) {
			submit = () => {
				let post: BlockDivisionPost = {
					SubmitSelections: {
						user_id: (view as UserViewResult).user_id as number,
						state_id: (view as UserViewResult).state_id as string,
						selections: selections
					}
				};

				block_division_post(post, callback);
			};
		} else {
			submit = undefined;
		}
	}

	let bucket_id_keyfunc = (bucket_id: number) => {
		if (view === undefined) {
			return "";
		} else {
			return view.state.basis.bucket_definitions[bucket_id].name;
		}
	};

	$: {
		console.debug("View:", view);
		if (view !== undefined) {
			title = view.state_id;
			if (view.user_id !== undefined) {
				title = title + " - " + view.state.basis.participant_definitions[view.user_id].name;
				selections = [];
				let current_open_round = view.state.current_open_round;
				console.debug("Current open round:", current_open_round);
				if (current_open_round !== null) {
					let picks_allowed =
						view.state.basis.participant_definitions[view.user_id].round_picks_allowed[
							current_open_round
						];
					let current_selections = view.state.selections.state[current_open_round][view.user_id];
					console.debug("Picks allowed:", picks_allowed);
					console.debug("Current selections:", current_selections);
					if (current_selections.length !== picks_allowed) {
						console.error("Backend error. Picks allowed !== current selections array length.");
					}
					selections = current_selections;
				}
			}
		} else {
			selections = [];
		}
	}
</script>

<Container {title}>
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
				<div class="selections">
					{#if view.state.current_open_round !== null}
						{#each selections as selection, selection_index}
							<div class="selection">
								<FormField
									align="end"
									on:SMUISwitch:change={(_e) => {
										if (selection === null) {
											selection = {
												bucket_index: 0,
												ancillaries: [],
												state: null
											};
										} else {
											selection = null;
										}
										selections = selections;
									}}
								>
									<Switch checked={selection !== null} />
									<span slot="label"
										>{view.state.basis.selection_round_names[view.state.current_open_round]}
										selection {selection_index + 1}</span
									>
								</FormField>
								{#if selection !== null}
									<Select
										key={bucket_id_keyfunc}
										bind:value={selection.bucket_index}
										label="Selection"
									>
										{#each view.state.basis.bucket_definitions as bucket_definition, bucket_index}
											<Option value={bucket_index}>{bucket_definition.name}</Option>
										{/each}
									</Select>
									{#each view.state.basis.bucket_definitions[selection.bucket_index].available_ancillaries as available_ancillary, ancillary_index}
										<FormField align="end">
											<Checkbox
												checked={selection.ancillaries.includes(ancillary_index)}
												on:click={() => {
													if (selection?.ancillaries.includes(ancillary_index)) {
														selection?.ancillaries.splice(
															selection?.ancillaries.indexOf(ancillary_index),
															1
														);
													} else {
														selection?.ancillaries.push(ancillary_index);
													}
												}}
											/>
											<span slot="label">{available_ancillary}</span>
										</FormField>
										{#if selection?.state !== null}
											<div>
												Result: {selection?.state}
											</div>
										{:else}
											No selection state {JSON.stringify(selection)}
										{/if}
									{/each}
								{:else}
									<Select disabled={true} label="Selection"></Select>
								{/if}
							</div>
						{/each}
						<Button on:click={submit} variant="raised">Submit</Button>
					{:else}
						<div>
							Selections for {view.state_id} are current closed.
						</div>
					{/if}
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
	.selections {
		display: flex;
		flex-direction: column;
		justify-content: flex-start;
		align-content: flex-start;
		align-items: flex-start;
	}
	.selection {
		display: flex;
		flex-direction: row;
	}
</style>
