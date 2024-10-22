<script lang="ts">
	import Container from "./container.svelte";
	import { onMount } from "svelte";
	import { handle_error } from "../commons/commons";
	import { block_division_post } from "../post/block_division_post";
	import type {
		BlockDivisionPost,
		BlockDivisionPostResult,
		ErrorResult,
		StateResult
	} from "../post/block_division_post";
	import { get_ranking_as_string } from "../commons/bucket_functions";

	let message: string = "Loading";
	let view: StateResult | undefined = undefined;

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
						view = result as StateResult;
						message = JSON.stringify(result);
						console.debug(message);
					}
				}
			};

			block_division_post(post, callback);
		}
	});
</script>

<Container title="Block Division">
	<div slot="contents">
		{#if view == undefined}
			<div>{message}</div>
		{:else}
			<div>{view.id}</div>
			<table>
				<tr>
					<th> </th>
					{#each view.state.basis.selection_round_names as selection_round, selection_round_index}
						<th>{selection_round}</th>
					{/each}
				</tr>
				{#each view.state.basis.bucket_definitions as bucket, bucket_index}
					<tr>
						<td>{bucket.name}</td>
						{#each view.state.basis.selection_round_names as selection_round, selection_round_index}
							<td>{get_ranking_as_string(view, selection_round_index, bucket_index)}</td>
						{/each}
					</tr>
				{/each}
			</table>
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
	}
</style>
