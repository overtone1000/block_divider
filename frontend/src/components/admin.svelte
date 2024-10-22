<script lang="ts">
	import Container from "./container.svelte";
	import { onMount } from "svelte";
	import BlockDivisionEdit from "./block_division/block_division_edit.svelte";
	import { DisplayMode } from "../commons/commons";
	import BlockDivisionCreate from "./block_division/block_division_create.svelte";
	import BlockDivisionList from "./block_division/block_division_list.svelte";
	import {
		block_division_post,
		ErrorResult,
		type BlockDivisionPost,
		type BlockDivisionPostResult
	} from "../post/block_division_post";

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

		if (m === DisplayMode.List) {
			selected_division = undefined;
			loadlist();
		}
	};

	let loadlist = () => {
		let post: BlockDivisionPost = {
			GetStates: {}
		};

		let callback = (result: BlockDivisionPostResult) => {
			if (typeof result === "object") {
				if ((result as ErrorResult).error) {
					handle_error((result as ErrorResult).error);
				} else {
					let cast_result = result as BlockDivisionStateList;
					console.debug(cast_result);
					list = cast_result;
					display_mode = DisplayMode.List;
				}
			}
		};

		block_division_post(post, callback);
	};

	onMount(async () => {
		loadlist();
	});
</script>

<Container title="Block Division Administration">
	<div class="contents" slot="contents">
		{#if display_mode == DisplayMode.Loading}
			<div>{message}</div>
		{:else if display_mode == DisplayMode.List && list !== undefined}
			<BlockDivisionList {set_display_mode} bind:list bind:selected_division />
		{:else if display_mode == DisplayMode.Create}
			<BlockDivisionCreate {set_display_mode} {selected_division} />
		{:else if display_mode == DisplayMode.Modify && selected_division !== undefined}
			<BlockDivisionEdit {set_display_mode} {selected_division} />
		{:else}
			<div>Error</div>
		{/if}
	</div>
</Container>

<style>
	.contents {
		height: 100%;
		width: 100%;
	}
</style>
