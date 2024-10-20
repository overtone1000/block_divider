<script lang="ts">
	import Container from "./container.svelte";
	import { onMount } from "svelte";
	import { handle_error } from "../commons/commons";
	import { block_division_post } from "../post/block_division_post";
	import type { BlockDivisionPost, BlockDivisionPostResult } from "../post/block_division_post";

	let message: string = "Loading";

	onMount(async () => {
		const urlParams = new URLSearchParams(window.location.search);

		//Example: localhost:5173/?user_id=test_user_id&division_id=test_division_id
		console.debug(urlParams);
		let user_id = urlParams.get("user_id");
		let division_id = urlParams.get("division_id");

		if (user_id !== null && division_id !== null) {
			let post: BlockDivisionPost = {
				GetState: {
					id: division_id
				}
			};

			let callback = (result: BlockDivisionPostResult) => {
				if (typeof result === "object") {
					if (result.error !== undefined) {
						handle_error(result.error);
					} else {
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
		<div>{message}</div>
	</div>
</Container>

<style>
</style>
