<script lang="ts">
	import Container from "./container.svelte";
	import { onMount } from "svelte";
	import { block_division_post } from "../post/block_division_post";
	import type { BlockDivisionPost, BlockDivisionPostResult } from "../post/block_division_post";

	let message = "Loading...";
	let list: BlockDivisionStateList | undefined = undefined;
	let division_id: string | undefined = undefined;

	enum DisplayMode {
		Loading,
		List,
		Create,
		Modify
	}

	let display_mode: DisplayMode = DisplayMode.Loading;

	let handle_error = (e: Error) => {
		message = "Error (see developer console)";
		console.error(e);
	};

	onMount(async () => {
		if (division_id !== undefined) {
			display_mode = DisplayMode.Modify;
		}

		let post: BlockDivisionPost = {
			GetDivisions: {}
		};

		let callback = (result: BlockDivisionPostResult) => {
			if (result.error !== undefined) {
				handle_error(result.error);
			} else {
				let cast_result = result as BlockDivisionStateList;
				console.debug(cast_result);
				list = cast_result;
				display_mode = DisplayMode.List;
			}
		};

		block_division_post(post, callback);
	});
</script>

<Container title="Block Division Administration">
	<div slot="contents">
		{#if display_mode == DisplayMode.Loading}
			<div>{message}</div>
		{:else if display_mode == DisplayMode.List && list !== undefined}
			<div>
				{#each list as block_division_item}
					<a
						onclick={() => {
							division_id = block_division_item[0];
						}}
					>
						{block_division_item[1].basis.label}
					</a>
				{/each}
			</div>
		{:else if display_mode == DisplayMode.Create}
			<div></div>
		{:else if display_mode == DisplayMode.Modify}
			<div></div>
		{:else}
			<div>Error</div>
		{/if}
	</div>
</Container>

<style>
</style>
