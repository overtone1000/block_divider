<script lang="ts">
	import Container from "./container.svelte";
	import { onMount } from "svelte";
	import { block_division_post } from "../post/block_division_post";
	import type { BlockDivisionPost, BlockDivisionPostResult } from "../post/block_division_post";

	let message: string = "Loading";

	let handle_error = (e: Error) => {
		message = "Error (see developer console)";
		console.error(message);
	};

	onMount(async () => {
		const urlParams = new URLSearchParams(window.location.search);
		console.debug(urlParams);
		let user_id = urlParams.get("user_id");
		let division_id = urlParams.get("division_id");

		if (user_id !== null && division_id !== null) {
			let post: BlockDivisionPost = {
				GetState: {
					division_id: division_id
				}
			};

			let callback = (result: BlockDivisionPostResult) => {
				if (result.Error !== undefined) {
					handle_error(result.Error);
				} else {
					message = JSON.stringify(result);
					console.debug(message);
				}
			};

			block_division_post(post, callback);
		}
	});
</script>

<Container title="Block Division">
	<div slot="contents">
		<div>{message}</div>
	</div>
</Container>

<style>
</style>
