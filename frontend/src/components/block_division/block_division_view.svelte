<script lang="ts">
	import { mdiOrderNumericAscending } from "@mdi/js";
	import Tooltip, { Wrapper, Content as TooltipContent } from "@smui/tooltip";
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
	import {
		clone_block_division_selections,
		type BlockDivisionSelectionEntry
	} from "../../post/results/block_division_state";
	import Switch from "@smui/switch";
	import FormField from "@smui/form-field";
	import Checkbox from "@smui/checkbox";
	import Button from "@smui/button";
	import Paper, { Content as PaperContent } from "@smui/paper";

	export let view: UserViewResult | undefined = undefined;
	let message: string = "Loading";
	let urlhash: string | null = null;
	let title: string = "Block Division";

	let userview_update = () => {
		console.debug("View change", view);
		if (view !== undefined) {
			title = view.state_id;
			if (view.user_id !== undefined) {
				title = title + " - " + view.state.basis.participant_definitions[view.user_id].name;
				selections = [];
				let current_open_round = view.state.current_open_round;
				if (current_open_round !== null) {
					let picks_allowed =
						view.state.basis.participant_definitions[view.user_id].round_picks_allowed[
							current_open_round
						];

					//Need to deep copy here
					for (const current_selection of view.state.selections.state[current_open_round][
						view.user_id
					]) {
						if (current_selection !== null) {
							selections.push(clone_block_division_selections(current_selection));
						}
					}

					if (selections.length !== picks_allowed) {
						console.error("Backend error. Picks allowed !== current selections array length.");
					}
				}
			}
		} else {
			selections = [];
		}
	};

	$: {
		if (view !== undefined) {
			userview_update();
		}
	}

	let callback = (result: BlockDivisionPostResult) => {
		if (typeof result === "object") {
			if ((result as ErrorResult).error) {
				handle_error((result as ErrorResult).error);
			} else {
				view = result as UserViewResult;
				userview_update();
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

	let bucket_id_keyfunc = (bucket_id: number) => {
		if (view === undefined) {
			return "";
		} else {
			return view.state.basis.bucket_definitions[bucket_id].name;
		}
	};

	let selections_have_changed: () => boolean = () => {
		if (view !== undefined) {
			if (view.user_id !== undefined) {
				let current_open_round = view.state.current_open_round;
				if (current_open_round !== null) {
					if (
						selections.length !==
						view.state.selections.state[current_open_round][view.user_id].length
					) {
						console.debug("Different selection lengths");
						return true;
					}
					for (const selection_index in selections) {
						let current = selections[selection_index];
						let original =
							view.state.selections.state[current_open_round][view.user_id][selection_index];

						if (
							current?.bucket_index === original?.bucket_index
							//current?.state === original?.state //Need to ignore state!
						) {
							//Selections still unchanged here.

							let current_ancillaries = current?.ancillaries;
							let original_ancillaries = original?.ancillaries;
							if (current_ancillaries !== undefined && original_ancillaries !== undefined) {
								//Check the arrays for differences.
								console.debug(current_ancillaries, original_ancillaries);
								if (current_ancillaries.length === original_ancillaries.length) {
									for (const ancillary_i in current_ancillaries) {
										if (current_ancillaries[ancillary_i] !== original_ancillaries[ancillary_i]) {
											console.debug("Different ancillary contents");
											return true;
										}
									}
								} else {
									console.debug("Different ancillary lengths");
									return true;
								}
							} else if (current_ancillaries === undefined && original_ancillaries === undefined) {
								//Do nothing in this case.
							} else {
								//One is undefined but the other is not.
								console.debug("Different ancillary defined states");
								return true;
							}
						} else {
							console.debug("Different bucket index");
							return true;
						}
					}
				}
			}
		}

		return false;
	};

	let selections_changed: boolean;
	$: {
		if (selections) {
			selections_changed = selections_have_changed(); //Default to false and switch to true when equivalence is disproven.
		} else {
			selections_changed = false;
		}
		console.debug("Selection change check result:", selections_changed);
	}

	let submit: undefined | (() => void) = undefined;

	$: {
		if (selections_changed && view !== undefined && view.user_id !== undefined) {
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
												<TooltipContent
													><div>
														<ol>
															{#each get_sorted_rankings(view, selection_round_index, bucket_index) as ranking}
																<li>{ranking}</li>
															{/each}
														</ol>
													</div></TooltipContent
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
										if (view !== undefined) {
											let current_open_round = view.state.current_open_round;
											let user_id = view.user_id;
											if (
												selection === null &&
												current_open_round !== null &&
												user_id !== undefined
											) {
												let current =
													view.state.selections.state[current_open_round][user_id][selection_index];
												if (current !== null) {
													selection = clone_block_division_selections(current);
												}
											} else {
												selection = null;
											}
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
													selections = selections;
												}}
											/>
											<span slot="label">{available_ancillary}</span>
										</FormField>
										{#if !selections_changed && selection?.state !== null}
											{#if selection?.state == "Confirmed"}
												<Paper>Request Currently Confirmed</Paper>
											{:else if selection?.state == "RejectedOutranked"}
												<Paper color="secondary" class="mdc-theme--secondary">
													Request Rejected: Outranked for {view.state.basis.bucket_definitions[
														selection?.bucket_index
													].name} this round
												</Paper>
											{:else if selection?.state == "RejectedNoSelectionsThisRound"}
												<Paper color="secondary" class="mdc-theme--secondary">
													Request Rejected: No more slots available for {view.state.basis
														.bucket_definitions[selection?.bucket_index].name} this round
												</Paper>
											{:else if selection?.state.RejectedAncillaryUnavailable !== undefined}
												{#each selection?.state.RejectedAncillaryUnavailable as unavailable_ancillary}
													<Paper color="secondary" class="mdc-theme--secondary">
														Request Rejected: Could not reserve ancillary {view.state.basis
															.bucket_definitions[selection?.bucket_index].available_ancillaries[
															unavailable_ancillary
														]}
													</Paper>
												{/each}
											{/if}
										{:else}
											<!--No selection state-->
										{/if}
									{/each}
								{:else}
									<Select disabled={true} label="Selection"></Select>
								{/if}
							</div>
						{/each}
						{#if selections_changed}
							<Button on:click={submit} variant="raised">Submit</Button>
						{/if}
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
		align-items: center;
		padding: 10px;
	}
</style>
