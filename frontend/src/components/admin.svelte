<script lang="ts">
	import Container from "./container.svelte";
	import { onMount } from "svelte";
	import {
		block_division_post,
		type BlockDivisionPost,
		type BlockDivisionPostResult
	} from "../post/block_division_post";
	import BlockDivisionEdit from "./block_division/block_division_edit.svelte";
	import { DisplayMode } from "../commons/commons";

	let message = "Loading...";
	let list: BlockDivisionStateList | undefined = undefined;
	let selected_division: [string, BlockDivisionState] | undefined = undefined;

	let display_mode: DisplayMode = DisplayMode.Loading;

	let handle_error = (e: Error) => {
		message = "Error (see developer console)";
		console.error(e);
	};

	let set_display_mode = (m: DisplayMode) => {
		display_mode = m;
	};

	onMount(async () => {
		let post: BlockDivisionPost = {
			GetDivisions: {}
		};

		let callback = (result: BlockDivisionPostResult) => {
			if (typeof result === "object") {
				if (result.error !== undefined) {
					handle_error(result.error);
				} else {
					let cast_result = result as BlockDivisionStateList;
					console.debug(cast_result);
					list = cast_result;
					display_mode = DisplayMode.List;
				}
			}
		};

		block_division_post(post, callback);
	});

	$: {
		if (selected_division !== undefined) {
			display_mode = DisplayMode.Modify;
		}
	}
</script>

<Container title="Block Division Administration">
	<div slot="contents">
		{#if display_mode == DisplayMode.Loading}
			<div>{message}</div>
		{:else if display_mode == DisplayMode.List && list !== undefined}
			<div>
				{#each list as block_division_item}
					<a
						href={window.location.href}
						on:click={() => {
							selected_division = block_division_item;
							return false;
						}}
					>
						{block_division_item[1].basis.label}
					</a>
				{/each}
			</div>
		{:else if display_mode == DisplayMode.Create}
			<div></div>
		{:else if display_mode == DisplayMode.Modify && selected_division !== undefined}
			<div>
				<BlockDivisionEdit {set_display_mode} block_division_tuple={selected_division} />
			</div>
		{:else}
			<div>Error</div>
		{/if}
	</div>
</Container>

<style>
</style>
