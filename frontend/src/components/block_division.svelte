<script lang="ts">
	import Container from "./container.svelte";
	import { onMount } from "svelte";
	import { block_division_post } from "../post/block_division_post";
	import type { BlockDivisionPost, BlockDivisionPostResult } from "../post/block_division_post";

	let data: undefined | string = "Loading";

	onMount(async () => {
		const urlParams = new URLSearchParams(window.location.search);
		console.debug(urlParams);
		let user_id = urlParams.get("user_id");
		let division_id = urlParams.get("division_id");

		if (user_id !== null && division_id !== null) {
			let post: BlockDivisionPost = {
				GetState: {
					user_id: user_id,
					division_id: division_id
				}
			};

			let callback = (result: BlockDivisionPostResult) => {
				if (result.Error !== undefined) {
				} else {
					data = JSON.stringify(result);
					console.debug(data);
				}
			};

			block_division_post(post, callback);
		}
	});
</script>

<Container title="Block Division">
	<div slot="contents">
		<div>{data}</div>
	</div>
</Container>

<style>
</style>
